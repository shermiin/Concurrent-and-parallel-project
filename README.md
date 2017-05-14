# Foundations of Concurrent and Distributed Systems Lab: Summer semester 2017 #

This repository contains programming tasks, with their descriptions, sequential C/C++ sources, test inputs and outputs as well as the testing infrastructure for the 2017 CDS Lab.
The tasks are taken from the [Marathon of Parallel Programming 2016](https://bitbucket.org/r0bcrane/fcds-lab-2017/src/b1a657cd5eacfcf7d6ede9a664c25d59989b7c99/Marathon%20of%20Parallel%20Programming%20problemset.pdf?at=master).
Find further information at the [Lab's website](https://tu-dresden.de/ing/informatik/sya/se/studium/labs-seminars/concurrent_and_distributed_systems_lab/summer-semester-2017/index).

Please use this repository as a template for your work.
We will a procedure similar to the one described in the next section to evaluate your solution's performance.
So, *make sure what is described here works for your repository prior submission*.

# How to get started #

These steps, build the docker container and run concurrency tests.
In this mode the program is executed a number of times with different amounts of CPUs cores.

1. Fork this repository to your account. (Look for 'Fork this repository' in the bitbucket menu.) 

1. Add this public key to the list of access keys of your just forked repository:

        at https://bitbucket.org/[YOUR_USERNAME]/[YOUR_PROJECT]/admin/access-keys/

        `ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDV/3jxo4Qk8ZrlppV0CHytcZfHF1zZxcUJ07RWqKVNax8AoCezkrBitZJyV8htVJ09jKZPw5b01L5ZSmrZZP5QFO4SXpF6OIId4D7zEOXxRw2DEEq0D4mQiWXaKLqRZYNd4NEHvbQAjqcXAVTEhYrrPPw2D5bTPSFBKuGN8qcU9xorQz4LUPQKdmp1ofGNw3etG8akAhj3V/hRUfenKlYL5dS+Ubgf9N4ZcTZVuo4PHfc2x2pfIQgnTixzgo6PcfY0yxGt1X8HML2EQRbZZnD7heDp6nYOqhXJSt5eLne5UsHiW2ojmLXHKS5TXYymMxHZCPXCySq6iT+4TDPymCjb`

1. Clone your repository and switch your working directory to it:

        $ git clone git@bitbucket.org:[YOUR_USERNAME]/fcds-lab-2017.git && cd fcds-lab-2017

2. Create the Docker image:

        $ docker build .

	Carefully look for a line at the end that prints the image's id:

        Successfully built 2e32ab3296ea

	You can also use the `-t` option to name the image and use the given name afterwards with the `cds-tool`.

3. Run a measurement with the created Docker image:

        $ ./cds-tool/bin/cds-tool run --measure --image [IMAGE ID] -c [NUMBER OF CPUs] --input [INPUT FOR THE TASK] [NAME OF TASK as is 11mopp/cds_server.json]
        $ ./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1,2,3,4 --input 11mopp/game-of-life/judge.in 11mopp-game-of-life

# About this Repository #

The `11mopp` directory contains four sub directories resembling sequential solutions of the four marathon tasks.
Everything is built via Makefiles. There is a global Makefile in `11mopp` and task-specific ones in their respective sub directories.
The global Makefile is invoked in `build.sh` which gets called during the Docker image creation (see `Dockerfile`).

In `cds-tool` you'll find the source code of the program that 1) creates the server inside of the running Docker container and invokes your programs and 2) queries this server and starts the docker container if necessary.
A precompiled binary can be found at `cds-tool/bin/cds-tool`. You can recompile the tool with `build_cds-tool.sh`. This script will install [Rust](https://www.rust-lang.org/) on your machine if it's not already installed. (You can run it in an container if you don't want Rust to be installed on your machine ;) )

The file `11mopp/cds_server.json` contains a lookup table for the paths to the binaries of the tasks used by the server.
Change the paths if your setup puts the binaries to other locations (which is likely).
Update `11mopp/cds_server.json` to reflect the situation in your image.
It links program names to the executable's location INSIDE of the image.
This allows the server to invoke the correct program.
For example with an entry of `["11mopp-histogram", "/11mopp/histogram/histogram"], ...` the
server will invoke `/11mopp/histogram/histogram` when the client requests `11mopp-histogram`
to be executed.
The four tasks have to be named as follows for our script to invoke the correct program:

* 11mopp-string-parsing
* 11mopp-game-of-life
* 11mopp-sudokount
* 11mopp-histogram

# General Advice #

* Develop your software your way. Use tools, languages, and libraries as you wish.
* Adapt the image building process such that
  a) runtime and build dependencies are available in the generated image and
  b) your software is built during image creation.
  Have a look at `Dockerfile`, `install_deps.sh` and `build.sh` to understand how we did it in this 
  template. We recommend you a comparable separation as this eases development.
  Make sure the CDS server is started by default (see `CMD` line in `Dockerfile`).
  *Otherwise, automatic measuring will fail.*
* If the container was started by the `cds-tool` it's environment will contain `MAX_CPUS` which you can read
  in your program to learn how many cpus are availabe for it.

## Development within an Container ##

During development you should also use the container environment.
This might be cumbersome at first but will reduce the pain near the deadline.

Start by creating an interactive container of the base image used in your `Dockerfile`, mount the
cds repository where it would be copied to by the `Dockerfile` and expose port 8080 of the container.
For our seqeuntial template repository this would be done like this (assuming we are currently
in the repositories root directory):


```
#!bash

$ docker run -it --rm -v `pwd`:/cds-lab -p8080 ubuntu:16.04 /bin/bash
```


The container starts and you have an interactive bash session on the inside. Please note the `--rm`
argument which instructs the Docker engine to remove the container once its stopped. Make sure your
code changes are not removed with the container ;).

Now, you can manually execute the two setup steps of the `Dockerfile` inside your interactive container:
Install your dependencies:

```
#!bash

$ /cds-lab/install_deps.sh
```

and build your software:

```
#!bash

$ pushd cds-lab && ./build.sh && popd
```


With this done you can start the CDS server:

```$ /cds-lab/cds-tool/bin/cds-tool server -c cds-lab/cds-tool/cds_server.json```

The CDS server is waiting for requests now. So you can switch to a console on your host and invoke
the CDS measurement tool:

* Find the container's name or id with docker ps:

```
#!bash

   $ docker ps 
   CONTAINER ID        IMAGE               COMMAND             CREATED              STATUS              PORTS                     NAMES
   3979aea71b8e        ubuntu:16.04        "/bin/bash"         About a minute ago   Up About a minute   0.0.0.0:32778->8080/tcp   tender_stonebraker
```


* Invoke the tool normally:

```
#!bash

   ./cds-tool/bin/cds-tool run --container tender_stonebraker --cpus 2 -i ./11mopp/sudokount/sudokount1.in 11mopp-sudokount
   ran program 11mopp-sudokount
   exit status: 0
   duration: 643 micro seconds
   stdout:
   --------------
   1
   --------------
   stderr:
   --------------
   --------------
```


Now you can alter your code, stop the server, invoke the build script, restart the server and retry.
You can omit the hassle of stopping and restarting the server if you get yourself another bash
session in the container:

```$ docker exec -it $CONTAINER_ID_OR_NAME /bin/bash```

This creates another bash session inside of the container which you can use to reinvoke the build script.

One last hint: If you encounter issues with the way your program is run there are two options:

* Activate debug or even trace output of the CDS server and client to see in more detail what is going on:

```
#!bash

$ RUST_LOG=trace ./cds-tool/bin/cds-tool run --container tender_stonebraker --cpus 2 -i ./11mopp/sudokount/sudokount1.in 11mopp-sudokount
$ RUST_LOG=debug /cds-lab/cds-tool/bin/cds-tool server -c cds-lab/cds-tool/cds_server.json
```


   This will help you understand what these tools do in more detail.

* Obviously, you can run your program without the CDS server and client allowing you to verify the
   correct function of your program. Running sudokount inside of the container:
   
```
#!bash

$ /cds-lab/11mopp/sudokount/sudokount < /cds-lab/11mopp/sudokount/sudokount2.in 
     300064
```