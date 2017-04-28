# Foundations of Concurrent and Distributed Systems Lab: Summer semester 2017 #

This repository contains programming tasks, with their descriptions, sequential C sources, and test inputs.
The tasks are taken from the **[Marathon of Parallel Programming 2016](https://bitbucket.org/r0bcrane/fcds-lab-2017/src/b1a657cd5eacfcf7d6ede9a664c25d59989b7c99/Marathon%20of%20Parallel%20Programming%20problemset.pdf?at=master)**


# Contributors #

Robert Krahn

Franz Gregor

# How to use #

1. Build binaries

	```cd 11mopp
	make```

2. Create the docker container

	```docker build .```

	Cearefully look for a line at the end that prints the container number:


	```Successfully built 2e32ab3296ea```

3. Start a measurement with the docker container:

	```./cds-tool/bin/cds-tool run --measure --image 2e32ab3296ea -c 1 --input 11mopp/game-of-life/life.in 11mopp-game-of-life```
