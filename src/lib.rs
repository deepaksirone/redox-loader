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
//#![feature(type_ascription)]
#![feature(repr_align)]
#![feature(attr_literals)]

extern crate rlibc;
extern crate spin;
extern crate syscall;
extern crate linked_list_allocator;
extern crate byteorder;

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
use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

//pub const BLOCK_SIZE: u64 = 4096;

#[no_mangle]
pub unsafe extern fn rust_main(args_ptr: *const arch::x86_64::start::KernelArgs) -> !
{
        let mut active_table  = arch::x86_64::start::kstart(args_ptr);
        let mut mbr = fs::read_bootsector(&mut active_table);
        let mut s = [0;1000];
        fs::init_real_mode(&mut active_table);
        fs::read_drive(0x80, &mut s, 0x10); 
//        assert_eq!(core::mem::align_of::<real_mode::DescriptorTablePointer>(), 17);
        println!("Kernel Offset: {:x}", consts::KERNEL_OFFSET);
        println!("Hello World!");
        println!("Loader Stub Initialized");
    
        loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }

