# Rust Microkernel based OS

**[WARNING: Under development]**

A Microkernel based OS in Rust.

## Prerequisites
Make sure you have installed all of the following prerequisites on your development machine (_tested on Ubuntu 20.04.1 LTS_):

* build-essential
```
sudo apt install build-essential
```
* Rust (nightly channel)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain=nightly
```
* Rust component - Source
```
rustup component add rust-src
```
* Rust component - LLVM Tools Preview
```
rustup component add llvm-tools-preview
```
* Bootimage
```
cargo install bootimage
```

## Building Kernel

```
cd kernel
cargo bootimage
```

## Running Kernel on QEMU (MacOS)

```
qemu-system-x86_64 -display none -m 64M -M accel=hvf --cpu host -serial stdio -drive format=raw,file=kernel/target/x86_64-blog_os/debug/bootimage-myos-kernel.bin
```
