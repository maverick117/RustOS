.PHONY: all, clean

all:
	cd src/arch; make
	cp src/arch/boot.bin bin/isofiles/boot/boot.bin
	grub-mkrescue -o iso/os.iso bin/isofiles -d /usr/lib/grub/i386-pc
	qemu-system-x86_64 -cdrom iso/os.iso -m 64


clean:
	cd src/arch; make clean
	rm bin/isofiles/boot/boot.bin iso/os.iso
