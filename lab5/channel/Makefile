FLAGS = -Wall -lpthread -std=gnu11

example_3a: example_3a.c channel.o
	gcc example_3a.c channel.o -o $@ $(FLAGS)

example_3b: example_3b.c channel.o
	gcc example_3b.c channel.o -o $@ $(FLAGS)

channel.o: channel.c channel.h
	gcc -c channel.c $(FLAGS)