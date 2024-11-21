CC = gcc
CFLAGS = -Wall -std=c99 -fopenmp

# Source files and corresponding executables (add more as needed)
SOURCES = histogram.c merge_sort.c 
EXECUTABLES = $(SOURCES:.c=)

all: $(EXECUTABLES)

# Rule to compile each program
%: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(EXECUTABLES)

.PHONY: all clean
