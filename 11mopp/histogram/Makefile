CC=gcc

FLAGS=-O3 -lm

all: histogram

histogram: histogram.c
	$(CC) -fopenmp histogram.c $(FLAGS)  -o histogram

clean:
	rm -f histogram
