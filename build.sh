#!/bin/bash

# this script builds our software

pushd 11mopp
make clean
make
popd
