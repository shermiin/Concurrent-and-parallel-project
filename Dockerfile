FROM ubuntu:16.04


# copy the cds-tool such that it is in the search PATH
COPY cds-tool/bin/cds-tool /usr/local/bin

# copy over your source code & cds server configuration
COPY . /cds-lab

# install your dependencies
RUN /cds-lab/install_deps.sh

# build your programs
RUN cd cds-lab && ./build.sh

# start the cds server by default
# make sure it reads your configuration
CMD cds-tool server -c /cds-lab/cds-tool/cds_server.json
