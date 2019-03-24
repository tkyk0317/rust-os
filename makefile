ifeq ($(shell uname),Linux)
CC=riscv64-linux-gnu-gcc
else
CC=riscv64-unknown-elf-gcc
endif
CFLAGS="-march=rv32i -mabi=ilp32"

all:
	@CC=$(CC) CFLAGS=$(CFLAGS) cargo b -vv

run:
	@CC=$(CC) CFLAGS=$(CFLAGS) cargo r

clean:
	@cargo clean
