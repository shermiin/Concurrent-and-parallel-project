#!/bin/bash

set -x

cd $HOME

# Stop and remove old container
CONTAINER=$(docker ps -a --no-trunc | grep '$USERNAME' | awk '{ print $1 }' | tr '\n' ' ')
if [ ! -z "$CONTAINER" ]; then
	docker rm --force $CONTAINER
fi

# Remove old container image
IMAGE=$(docker images | grep '$USERNAME' | awk '{ print $3 }' | tr '\n' ' ')
if [ ! -z "$IMAGE" ]; then
	docker rmi $IMAGE
fi

cd /data/cdslab/$USERNAME
docker build -t $USERNAME . > docker.$BUILD_TAG.log
IMAGE=$(grep 'Successfully built' docker.$BUILD_TAG.log | awk '{ print $3 }')


if [ ! -z "$IMAGE" ]; then
    set -e

    # replace cds-tool in student image

    # the subsequent `docker commit` replaces the image's CMD
    # thus we extract the orignal CMD here
    CMD=`docker inspect -f {{.ContainerConfig.Cmd}} ${USERNAME} | sed -e 's/.* \"-c\" \"\(.*\)\"]]/\1/g'`
    # replace cds-tool inside the container
    CONTAINER=$(docker run -v $(pwd)/cds-root/cds-tool/bin/:/mnt -d $USERNAME bash -c 'eval "cp /mnt/cds-tool `which cds-tool`"')
    # wait for cp to finish
    docker wait ${CONTAINER}
    ORIG_IMAGE=${IMAGE}
    # turn the container into an image and tag it with the student's username
    IMAGE=`docker commit -c "CMD ${CMD}" ${CONTAINER} ${USERNAME}`
    # remove the container
    docker rm ${CONTAINER}
    # and the original image
    #docker rmi ${ORIG_IMAGE}
    set +e

    # run experiments
    cd /data/cdslab/$USERNAME
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/string-parsing/judge.in -o 11mopp/string-parsing/judge.out 11mopp-string-parsing | tee sp.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/sudokount/judge.in -o 11mopp/sudokount/judge.out 11mopp-sudokount | tee sd.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/histogram/judge.in -o 11mopp/histogram/judge.out 11mopp-histogram | tee hg.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/game-of-life/judge.in -o 11mopp/game-of-life/judge.out 11mopp-game-of-life | tee gl.$BUILD_TAG.log

    set +e
fi

CONTAINER=$( docker ps -a --no-trunc | grep '$USERNAME' | awk '{ print $1 }' | tr '\n' ' ')
if [ ! -z "$CONTAINER" ]; then
    docker rm --force $CONTAINER
fi


IMAGE=$(docker images | grep '$USERNAME' | awk '{ print $3 }' | tr '\n' ' ')
if [ ! -z "$IMAGE" ]; then
	 docker rmi $IMAGE
fi
