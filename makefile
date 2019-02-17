CC=riscv64-linux-gnu-gcc
CFLAGS="-march=rv32i -mabi=ilp32"

all:
	@CC=$(CC) CFLAGS=$(CFLAGS) cargo b

run:
	@CC=$(CC) CFLAGS=$(CFLAGS) cargo r

clean:
	@cargo clean
