CC ?= gcc
CFLAGS ?= -Wall -Wextra -std=c11 -Iinclude \
          -I../../../../universal/transpiled/compiled/c/include \
          -I../../../../universal/transpiled/compiled/c/tests

all: test

test: test_character_bin
	./test_character_bin

test_character_bin: src/character.c tests/test_character.c include/character.h
	$(CC) $(CFLAGS) src/character.c tests/test_character.c -o $@

clean:
	rm -f test_character_bin

.PHONY: all test clean
