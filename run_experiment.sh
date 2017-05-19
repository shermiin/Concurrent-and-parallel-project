#!/bin/bash

set -x

cd $HOME

# Stop and remove old container
CONTAINER=$(docker ps -a --no-trunc | grep '$USERNAME' | awk '{ print $1 }' | tr '\n' ' ')
if [ ! -z "$CONTAINER" ]; then
	docker stop $CONTAINER
	docker rm $CONTAINER
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
    # replace cds-tool in student image
    CONTAINER=$(docker run -v $(pwd)/cds-root/cds-tool/bin/:/mnt -d $USERNAME bash -c 'eval "cp /mnt/cds-tool `which cds-tool`"')
    docker wait ${CONTAINER}
    ORIG_IMAGE=${IMAGE}
    # put modified image instead
    IMAGE=`docker commit ${CONTAINER} ${USERNAME}`
    docker rm ${CONTAINER}
    # delete original image
    docker rmi ${ORIG_IMAGE}
fi

if [ ! -z "$IMAGE" ]; then
    cd /data/cdslab/$USERNAME
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/string-parsing/judge.in -o 11mopp/string-parsing/judge.out 11mopp-string-parsing > sp.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/sudokount/judge.in -o 11mopp/sudokount/judge.out 11mopp-sudokount > sd.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/histogram/judge.in -o 11mopp/histogram/judge.out 11mopp-histogram > hg.$BUILD_TAG.log
    RUST_LOG=debug ./cds-root/cds-tool/bin/cds-tool run --measure --image $USERNAME -c 1,2,4,8 --input 11mopp/game-of-life/judge.in -o 11mopp/game-of-life/judge.out 11mopp-game-of-life > gl.$BUILD_TAG.log
fi

CONTAINER=$( docker ps -a --no-trunc | grep '$USERNAME' | awk '{ print $1 }' | tr '\n' ' ')
if [ ! -z "$CONTAINER" ]; then
	docker stop $CONTAINER
    docker rm $CONTAINER
fi


IMAGE=$(docker images | grep '$USERNAME' | awk '{ print $3 }' | tr '\n' ' ')
if [ ! -z "$IMAGE" ]; then
	 docker rmi $IMAGE
fi