AS=nasm
FMT=-f elf64
LD=ld

.PHONY: all, clean

all: boot.bin
	@echo "boot section binary generation complete"

boot.bin: header.o boot.o boot.ld
	ld -n -o boot.bin -T boot.ld header.o boot.o

header.o: boot/mult_boot_hdr.asm
	${AS} ${FMT} boot/mult_boot_hdr.asm -o header.o

boot.o: boot/boot.asm
	${AS} ${FMT} boot/boot.asm -o boot.o

clean: 
	rm -rf *.o *.bin
