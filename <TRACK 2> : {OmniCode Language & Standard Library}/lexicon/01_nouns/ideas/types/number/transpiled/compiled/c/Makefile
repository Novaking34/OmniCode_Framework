CC ?= gcc
CFLAGS ?= -Wall -Wextra -std=c11 -Iinclude \
          -I../../../../universal/transpiled/compiled/c/include \
          -I../../../../universal/transpiled/compiled/c/tests

all: test

test: test_number_bin
	./test_number_bin

test_number_bin: src/number.c tests/test_number.c include/number.h
	$(CC) $(CFLAGS) src/number.c tests/test_number.c -o $@

clean:
	rm -f test_number_bin

.PHONY: all test clean
