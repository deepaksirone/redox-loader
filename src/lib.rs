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
//#![feature(allocator_internals)]
//#![default_lib_allocator]
#![feature(allocator_api, heap_api)]
#![feature(global_allocator)]

extern crate rlibc;
extern crate spin;
extern crate syscall;
extern crate linked_list_allocator;

#[cfg(feature = "slab")]
extern crate slab_allocator;

pub extern crate x86;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;


#[macro_use]
pub mod macros;

//pub mod arch;
//pub use arch::*;

//pub mod devices;
pub mod allocator;
pub mod memory;
pub mod time;
pub mod paging;
pub mod consts;
pub mod panic;

#[cfg(feature = "graphical_debug")]
pub mod graphical_debug;

pub mod debug;
pub mod serial;
pub mod devices;
pub mod io;
//pub mod interrupt;
pub use consts::*;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

//pub mod scheme;
//pub mod syscall;
#[no_mangle]
pub unsafe extern fn rust_main()
{
//        vga_buffer::clear_screen();
        serial::init();
        println!("Hello World!");
        println!("Loader Stub Initialized");
    
        loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }


