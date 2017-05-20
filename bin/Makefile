.PHONY: all, clean

all:
	${LD} -n -o kernel.bin -T image.ld arch.o libkernel.a --gc-sections
	cp kernel.bin isofiles/boot/boot.bin
	grub-mkrescue -o ../iso/os.iso isofiles -d /usr/lib/grub/i386-pc

clean:
	rm -rf *.o image.ld libkernel.a kernel.bin