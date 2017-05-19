LD=ld

.PHONY: all, clean, arch_obj, kernel_obj, kernel_ld

all: arch_obj kernel_obj kernel_ld
	cd bin; make
	qemu-system-x86_64 -cdrom iso/os.iso -m 64

arch_obj: 
	cd src/arch; make
	cp src/arch/arch.o bin/
	
kernel_obj:
	cd src/kernel; make
	cp src/kernel/kernel.o bin/

kernel_ld: src/image.ld
	cp src/image.ld bin/

clean:
	cd src/arch; make clean
	cd src/kernel; make clean
	rm -rf bin/isofiles/boot/boot.bin iso/os.iso
	rm -rf bin/*.o bin/image.ld
