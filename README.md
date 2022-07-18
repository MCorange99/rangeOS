# rangeOS
An os in rust because i was bored
## setup

### For x86_64 linux  
```shell
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-none
cargo install bootimage
rustup component add llvm-tools-preview
```  
you also need qemu