CC ?= gcc
CFLAGS ?= -Wall -Wextra -std=c11 -Iinclude -Itests

all: test

test: test_universal_bin
	./test_universal_bin

test_universal_bin: tests/test_universal.c include/omni_base.h \
                    tests/test_common.h
	$(CC) $(CFLAGS) $< -o $@

clean:
	rm -f test_universal_bin

.PHONY: all test clean
