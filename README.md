# Redox Loader Stub

## What?
The Redox loader was written in order to support loading the Redox kernel from standard filesystems.

## Why?
Currently the Redox bootloader supports booting only from a RedoxFS partition

## Building
Make sure to have the following dependencies installed:
* [Rust](https://www.rust-lang.org/en-US/install.html)
* [xargo](https://github.com/japaric/xargo)

Then run `make run_kvm REDOXFS=<path_to_redoxfs_filesystem>`. `REDOXFS` defaults to `sample_images/fs_redoxfs.bin`

## Tweaks
* `redox-loader` currently boots from the first bootable partition it sees. Modify `bootloader/x86_64/bootsector.asm` to set the bootable partition.

