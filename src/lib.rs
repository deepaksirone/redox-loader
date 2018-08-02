#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(ptr_internals)]
#![feature(asm)]
#![feature(concat_idents)]
#![feature(naked_functions)]
#![feature(thread_local)]
#![feature(alloc)]
#![feature(allocator_api, heap_api)]
#![feature(global_allocator)]
#![feature(core_intrinsics)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(integer_atomics)]
#![feature(panic_implementation)]

extern crate rlibc;
extern crate spin;
extern crate syscall;
extern crate linked_list_allocator;
extern crate byteorder;
extern crate fat;

#[cfg(feature = "slab")]
extern crate slab_allocator;

pub extern crate x86;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

extern crate goblin;

#[macro_use]
pub mod arch;

//pub mod arch;
pub use arch::*;

//pub mod devices
pub mod allocator;
pub mod memory;
pub mod time;
pub mod elf;
pub mod fs;
pub mod loader;
//pub mod externs;
//pub mod paging;
pub mod consts;
pub mod panic;
//pub mod pti;
//pub mod interrupt;
#[cfg(feature = "graphical_debug")]
pub mod graphical_debug;

//pub mod debug;
//pub mod serial;
//pub mod device;
pub mod devices;
//pub mod real_mode;
//pub mod io;
//pub mod gdt;
//pub mod arch;

//pub mod interrupt;
pub use consts::*;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;
//pub mod scheme;
//pub mod syscall;
use core::slice;
use core::sync::atomic::{AtomicU8, ATOMIC_U8_INIT, Ordering};
use alloc::Vec;
//pub const BLOCK_SIZE: u64 = 4096;
pub static mut DISK: AtomicU8 = ATOMIC_U8_INIT;

#[no_mangle]
pub unsafe extern fn rust_main(args_ptr: *const arch::x86_64::start::KernelArgs) -> !
{
        DISK.store((*args_ptr).disk, Ordering::SeqCst);
        let mut active_table  = arch::x86_64::start::kstart(args_ptr);
        fs::init_real_mode(&mut active_table);
        let mut mbr = fs::read_bootsector();
        let mut s = [0;78];
        let mut vec: Vec<u8> = vec![0; 78];
//        let b = fs::disk::PartitionTable::get_bootable(mbr).unwrap();
        let part_table = fs::disk::PartitionTable::new(&mbr);

        println!("{:?}", part_table);
//        fs::read(*(DISK.get_mut()), &mut s, 510); 
        
        let boot_partition = part_table.get_bootable().unwrap();
        let mut fat_fs = fat::FatFileSystem::<fs::disk::Partition>::mount(*(DISK.get_mut()), 0).expect("FS error");
//        let root = fat_fs.root().expect("Root Error");
//        root.open_file("ice.txt").expect("Open Error").unwrap().read(vec.as_mut_slice());

        println!("Kernel Offset: {:x}", consts::KERNEL_OFFSET);
        println!("Hello World!");
        println!("Loader Stub Initialized");
        loader::load_kernel(&mut active_table, &mut fat_fs);
        println!("Kernel Loaded :)");
/*        for byte in vec.iter() {
            print!("{}", *byte as char);
        }
*/
        loop { }
}
/*
fn read_kernel(filesystem: &mut FatFileSystem::<fs::disk::Partition>)
{
      let root = filesystem.root().expect("Root Error");
      let kernel_file = filesystem.open_file("kernel").expect("Kernel Open Error");
      let vec: Vec<u8> = Vec::new();
      println!("Kernel File Size : {}", kernel_file.size());
}
*/
/*
#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }
*/
