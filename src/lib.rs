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
#![feature(core_intrinsics)]
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

/// Test of zero values in BSS.
static BSS_TEST_ZERO: usize = 0;
/// Test of non-zero values in data.
static DATA_TEST_NONZERO: usize = 0xFFFF_FFFF_FFFF_FFFF;
/// Test of zero values in thread BSS
#[thread_local]
static mut TBSS_TEST_ZERO: usize = 0;
/// Test of non-zero values in thread data.
#[thread_local]
static mut TDATA_TEST_NONZERO: usize = 0xFFFF_FFFF_FFFF_FFFF;

pub static KERNEL_BASE: AtomicUsize = ATOMIC_USIZE_INIT;
pub static KERNEL_SIZE: AtomicUsize = ATOMIC_USIZE_INIT;
pub static CPU_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
pub static AP_READY: AtomicBool = ATOMIC_BOOL_INIT;
pub static BSP_READY: AtomicBool = ATOMIC_BOOL_INIT;

/*
#[repr(packed)]
pub struct KernelArgs {
    kernel_base: u64,
    kernel_size: u64,
    stack_base: u64,
    stack_size: u64,
    env_base: u64,
    env_size: u64,
}
*/
 
#[no_mangle]
pub unsafe extern fn rust_main(args_ptr: *const arch::x86_64::start::KernelArgs) -> !
{
        arch::x86_64::start::kstart(args_ptr);
//        vga_buffer::clear_screen();
  /*  let env = {
        let args = &*args_ptr;

        let kernel_base = args.kernel_base as usize;
        let kernel_size = args.kernel_size as usize;
        let stack_base = args.stack_base as usize;
        let stack_size = args.stack_size as usize;
        let env_base = args.env_base as usize;
        let env_size = args.env_size as usize;

        // BSS should already be zero
        {
            assert_eq!(BSS_TEST_ZERO, 0);
            assert_eq!(DATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFF);
        }

        KERNEL_BASE.store(kernel_base, Ordering::SeqCst);
        KERNEL_SIZE.store(kernel_size, Ordering::SeqCst);

        println!("Kernel: {:X}:{:X}", kernel_base, kernel_base + kernel_size);
        println!("Stack: {:X}:{:X}", stack_base, stack_base + stack_size);
        println!("Env: {:X}:{:X}", env_base, env_base + env_size);
        // Set up GDT before paging
        gdt::init();

    };*/
    /*    // Set up IDT before paging
        idt::init();

        // Initialize memory management
        memory::init(0, kernel_base + ((kernel_size + 4095)/4096) * 4096);

        // Initialize paging
        let (mut active_table, tcb_offset) = paging::init(0, kernel_base, kernel_base + kernel_size, stack_base, stack_base + stack_size);

        // Set up GDT after paging with TLS
        gdt::init_paging(tcb_offset, stack_base + stack_size);

        // Set up IDT
        idt::init_paging();

        // Test tdata and tbss
        {
            assert_eq!(TBSS_TEST_ZERO, 0);
            TBSS_TEST_ZERO += 1;
            assert_eq!(TBSS_TEST_ZERO, 1);
            assert_eq!(TDATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFF);
            TDATA_TEST_NONZERO -= 1;
            assert_eq!(TDATA_TEST_NONZERO, 0xFFFF_FFFF_FFFF_FFFE);
        }

        // Reset AP variables
        CPU_COUNT.store(1, Ordering::SeqCst);
        AP_READY.store(false, Ordering::SeqCst);
        BSP_READY.store(false, Ordering::SeqCst);

        // Setup kernel heap
        allocator::init(&mut active_table);

        // Use graphical debug
        #[cfg(feature="graphical_debug")]
        graphical_debug::init(&mut active_table);

        // Initialize devices
        device::init(&mut active_table);

        // Read ACPI tables, starts APs
        #[cfg(feature = "acpi")]
        acpi::init(&mut active_table);

        // Initialize all of the non-core devices not otherwise needed to complete initialization
        device::init_noncore();

        // Initialize memory functions after core has loaded
        memory::init_noncore();

        // Stop graphical debug
        #[cfg(feature="graphical_debug")]
        graphical_debug::fini(&mut active_table);

        BSP_READY.store(true, Ordering::SeqCst);

        slice::from_raw_parts(env_base as *const u8, env_size)
    };


        serial::init();
*/
        println!("Hello World!");
        println!("Loader Stub Initialized");
    
        loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }

