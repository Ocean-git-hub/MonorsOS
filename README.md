# MonorsOS

MonorsOS is an OS written in Rust from scratch. It is stil incomplete, and I am still continuing to learn Rust, an OS
and a hardware.   
Its bootloader is following UEFI and kernel is a monolithic kernel.

## Requirement

* x86_64

## Environment

* Rust Nightly

#### QEMU

* OVMF

## Building and Running

### QEMU

#### Debian

```
$ apt install make ovmf qemu curl
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ make
```
