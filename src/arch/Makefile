AS=nasm
FMT=-f elf64
LD=ld

.PHONY: all, clean, arch_obj

all: arch_obj
	${LD} -n -o arch.o -T boot.ld -r *.o

arch_obj: header.o boot.o boot.ld long_boot.o console.o idt.o mem.o

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

mem.o: util/mem.asm
	${AS} ${FMT} util/mem.asm -o mem.o

clean:
	rm -rf *.o *.bin
