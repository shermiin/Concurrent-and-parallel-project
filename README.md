# Foundations of Concurrent and Distributed Systems Lab: Summer semester 2017 #

This repository contains programming tasks, with their descriptions, sequential C sources, and test inputs.
The tasks are taken from the **[Marathon of Parallel Programming 2016](https://bitbucket.org/r0bcrane/fcds-lab-2017/src/b1a657cd5eacfcf7d6ede9a664c25d59989b7c99/Marathon%20of%20Parallel%20Programming%20problemset.pdf?at=master)**

# How to get started #

These steps build the docker container and run concurrency tests.
In this mode the program is executed a number of times with different amounts of cpu cores.

1. Create the docker container

	```$ docker build .```

	Cearefully look for a line at the end that prints the container number:

	```Successfully built 2e32ab3296ea```

2. Start a measurement with the docker container:

	```$ ./cds-tool/bin/cds-tool run --measure --image [DOCKER ID] -c [NUMBER OF CPUs] --input [INPUT FOR THE TASK] [NAME OF TASK as is 11mopp/cds_server.json]```
	```$ ./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1,2,3,4 --input 11mopp/game-of-life/life.in 11mopp-game-of-life```

3. Start a 'judge' with the docker container:

    ```$ ./cds-tool/bin/cds-tool run --measure --image [DOCKER ID] -c [NUMBER OF CPUs] --input [JUDGE FOR THE TASK] [NAME OF TASK as is 11mopp/cds_server.json]```
    ```$ ./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1,2,3,4 --input 11mopp/game-of-life/judge.in 11mopp-game-of-life```


The difference between using judge.in and e.g. life.in is simply the extend of tests executed. Judge.in uses a larger input.

# How to develop #

Look in the folder *11mopp* for the four sub folders resembling the four tasks.
In the subfolders e.g. *game-of-life* you may change the source code and the makefile.

You must not change the *.in files!

The file `11mopp/cds_server.json` contains a lookup table for the paths to the binaries of the tasks.
Change the paths if necessary. (It works by default if the makefiles are not changed.)
Update `11mopp/cds_server.json` to reflect the situation in your image.
It links program names to the executable's location INSIDE of the image.
This allows the server to invoke the correct program.
For example with an entry of `["11mopp-histogram", "/11mopp/histogram/histogram"], ...` the
server will invoke `/11mopp/histogram/histogram` when the client requests `11mopp-histogram`
to be executed.
The following programs have to be listed:
* 11mopp-string-parsing
* 11mopp-game-of-life
* 11mopp-sudokount
* 11mopp-histogram

# General Advice #

* Develop your software your way.
* Adapt the image building process such that
  * a) runtime and build dependencies are available in the generated image and
  * b) your software is build during image creation.
  Have a look at `Dockerfile`, `install_deps.sh` and `build.sh` to understand how we did it in this
  template. We recommend you a comparable separation as this eases development.

  Make sure the cds server is started by default (see `CMD` line in `Dockerfile`).
  *Otherwise, automatic measuring will fail.*

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