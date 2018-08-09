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

pub use arch::*;
pub mod allocator;
pub mod memory;
pub mod time;
pub mod elf;
pub mod fs;
pub mod loader;
pub mod consts;
pub mod panic;

#[cfg(feature = "graphical_debug")]
pub mod graphical_debug;

pub mod devices;

pub use consts::*;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;
use core::slice;
use core::sync::atomic::{AtomicU8, ATOMIC_U8_INIT, Ordering};
use alloc::Vec;
pub static mut DISK: AtomicU8 = ATOMIC_U8_INIT;

#[no_mangle]
pub unsafe extern fn rust_main(args_ptr: *const arch::x86_64::start::KernelArgs) -> !
{
        DISK.store((*args_ptr).disk, Ordering::SeqCst);
        let mut active_table  = arch::x86_64::start::kstart(args_ptr);
        fs::init_real_mode(&mut active_table);
        let mut mbr = fs::read_bootsector();
        let part_table = fs::disk::PartitionTable::new(&mbr);

        println!("{:?}", part_table);
        
        let boot_partition = part_table.get_bootable().unwrap();
        let mut fat_fs = fat::FatFileSystem::<fs::disk::Partition>::mount(*(DISK.get_mut()), 0).expect("FS error");

        println!("Kernel Offset: {:x}", consts::KERNEL_OFFSET);
        println!("Loader Stub Initialized");
        println!("Loading Kernel..");
        loader::load_kernel(&mut active_table, &mut fat_fs);
        println!("Kernel Loaded :)");
        loop { }
}
/*
#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }
*/
