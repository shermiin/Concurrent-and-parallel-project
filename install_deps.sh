#!/bin/bash

# This script installs all dependencies we have for building
# as well as running our software. The idea being that no other
# installation steps are necessary after this script ran.

# have to update repository database, since image is distributed
# with an empty database
apt-get update
# for (simple) c and c++ there is a single metapackage that is
# needed:
apt-get install -y build-essential
