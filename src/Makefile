CC=rustc
ARGS=-v --emit obj --crate-type lib --target x86_64-RustOS
PKGMGR=xargo
LD=LD

.PHONY: all, clean, kernel.o

all: kernel.o

kernel.o: src/kernel.rs 
	xargo build --release --target x86_64-RustOS

clean:
	rm -rf *.o