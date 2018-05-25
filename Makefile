
default: run

.PHONY: clean

clean:
	rm -rf build
	rm -rf target

run: build/os.iso
	qemu-system-x86_64 -cdrom build/os.iso

build/os.iso: build/kernel.bin kernel/grub.cfg
	mkdir -p build/isofiles/boot/grub
	cp kernel/grub.cfg build/isofiles/boot/grub
	cp build/kernel.bin build/isofiles/boot/
	grub2-mkrescue -o build/os.iso build/isofiles/

build/multiboot_header.o: kernel/multiboot_header.asm
	mkdir -p build 
	nasm -f elf64 kernel/multiboot_header.asm -o build/multiboot_header.o

build/long_mode_init.o: kernel/long_mode_init.asm
	mkdir -p build
	nasm -f elf64 kernel/long_mode_init.asm -o build/long_mode_init.o
build/boot.o: kernel/boot.asm
	mkdir -p build
	nasm -f elf64 kernel/boot.asm  -o build/boot.o

build/kernel.bin: build/multiboot_header.o build/long_mode_init.o build/boot.o kernel/linker.ld cargo
	ld -n  --gc-sections -o build/kernel.bin -T kernel/linker.ld build/multiboot_header.o build/boot.o build/long_mode_init.o build/libpamb_os.a
cargo:
	mkdir -p build
	RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-pamb_os
	cp target/x86_64-pamb_os/debug/libpamb_os.a build/libpamb_os.a	
