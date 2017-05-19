LD=ld
TARGET=x86_64-RustOS

.PHONY: all, clean, arch_obj, kernel_obj, kernel_ld

all: arch_obj kernel_obj kernel_ld
	cd bin; make
	qemu-system-x86_64 -cdrom iso/os.iso -m 64

arch_obj: 
	cd src/arch; make
	cp src/arch/arch.o bin/
	
kernel_obj:
	xargo build --release --target ${TARGET}
	

kernel_ld: src/image.ld
	cp src/image.ld bin/
	cp target/${TARGET}/release/libkernel.a bin/

clean:
	xargo clean
	cd src/arch; make clean
	cd bin/; make clean
	rm -rf bin/isofiles/boot/boot.bin iso/os.iso
	rm -rf bin/*.o bin/image.ld
