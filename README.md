# Redox Loader Stub

## What?
The Redox loader was written in order to support loading the Redox kernel from standard filesystems.

## Why?
Currently the Redox bootloader supports booting only from a RedoxFS partition

## Building
Make sure to have the following dependencies installed:
* [Rust](https://www.rust-lang.org/en-US/install.html)
* [xargo](https://github.com/japaric/xargo)

Untar `sample_images/fs_redoxfs.tar.gz` and then run `make run_kvm`. A custom RedoxFS image can be passed using `make run_kvm REDOXFS=<path_to_redox_fs>`.`REDOXFS` defaults to `sample_images/fs_redoxfs.bin`

## Tweaks
* `redox-loader` currently boots from the first bootable partition it sees. Modify `bootloader/x86_64/bootsector.asm` to set the bootable partition.

## Blog Posts
* [Implementing a FAT32 filesystem in Redox - 1](https://www.redox-os.org/news/rsoc-fat32-1/)
* [Implementing a FAT32 filesystem in Redox - 2](https://www.redox-os.org/news/rsoc-fat32-2/)
* [Implementing a FAT32 filesystem in Redox - 3](https://www.redox-os.org/news/rsoc-fat32-3/)
## License
See [LICENSE](https://github.com/deepaksirone/redox-loader/blob/master/LICENSE)

