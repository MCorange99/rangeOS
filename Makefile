
all: build.linux

build.linux:
	cargo build 
	cargo bootimage

build.linux.release:
	cargo build 
	cargo bootimage

run.linux: build.linux
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rangeos/debug/bootimage-rangeos.bin

run.linux.release: build.linux.release
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rangeos/release/bootimage-rangeos.bin

clean:
	rm -rf ./target