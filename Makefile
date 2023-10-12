test_code:
	riscv64-unknown-elf-gcc -S code.c
	riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -nostdlib -march=rv64im -mabi=lp64 -o code code.s
	riscv64-unknown-elf-objcopy -O binary code code.bin
	cargo r -r -j8
