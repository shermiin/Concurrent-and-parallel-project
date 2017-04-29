# Foundations of Concurrent and Distributed Systems Lab: Summer semester 2017 #

This repository contains programming tasks, with their descriptions, sequential C sources, and test inputs.
The tasks are taken from the **[Marathon of Parallel Programming 2016](https://bitbucket.org/r0bcrane/fcds-lab-2017/src/b1a657cd5eacfcf7d6ede9a664c25d59989b7c99/Marathon%20of%20Parallel%20Programming%20problemset.pdf?at=master)**

# Contributors #

Robert Krahn

Franz Gregor

# CURRENT STATE #

We are still configuring the lab. Source folder is incomplete!


# How to develop #

Look in the folder *11mopp* for the four sub folders resembling the four tasks.
In the subfolders e.g. *game-of-life* you may change the source code and the makefile.

You must not change the *.in files!

The file 11mopp/cds_server.json contains a lookup table for the paths to the binaries of the tasks.
Change the paths if necessary. (It works by default if the makefiles are not changed.)

# How to use #

1. Build the binaries of the tasks

	```cd 11mopp```

	```make```

2. Create the docker container

	```cd ..``` (to where 'Dockerfile' is located)

	```docker build .```

	Cearefully look for a line at the end that prints the container number:


	```Successfully built 2e32ab3296ea```

3. Start a measurement with the docker container:

	```./cds-tool/bin/cds-tool run --measure --image [DOCKER ID] -c [NUMBER OF CPUs] --input [INPUT FOR THE TASK] [NAME OF TASK as is 11mopp/cds_server.json]```
	```./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1 --input 11mopp/game-of-life/life.in 11mopp-game-of-life```

4. Start a 'judge' with the docker container:

	```./cds-tool/bin/cds-tool run --measure --image [DOCKER ID] -c [NUMBER OF CPUs] --input [JUDGE FOR THE TASK] [NAME OF TASK as is 11mopp/cds_server.json]```
	```./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1 --input 11mopp/game-of-life/judge.in 11mopp-game-of-life```