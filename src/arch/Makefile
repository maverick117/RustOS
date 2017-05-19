AS=nasm
FMT=-f elf64
LD=ld

.PHONY: all, clean

all: boot.bin

boot.bin: header.o boot.o boot.ld long_boot.o console.o idt.o
	ld -n -o boot.bin -T boot.ld header.o boot.o long_boot.o console.o idt.o

header.o: boot/mult_boot_hdr.asm
	${AS} ${FMT} boot/mult_boot_hdr.asm -o header.o

boot.o: boot/boot.asm
	${AS} ${FMT} boot/boot.asm -o boot.o

long_boot.o: boot/long_boot.asm
	${AS} ${FMT} boot/long_boot.asm -o long_boot.o

console.o: util/console.asm
	${AS} ${FMT} util/console.asm -o console.o

idt.o: util/idt.asm
	${AS} ${FMT} util/idt.asm -o idt.o

clean:
	rm -rf *.o *.bin
