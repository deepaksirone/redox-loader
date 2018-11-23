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
//#![feature(global_allocator)]
#![feature(core_intrinsics)]
//#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(integer_atomics)]
#![feature(panic_implementation)]
#![feature(extern_prelude)]

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

pub mod devices;

pub use consts::*;

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;
use core::slice;
use core::sync::atomic::{AtomicU8, ATOMIC_U8_INIT, Ordering};
use fs::disk::{File, Fs};
pub static mut DISK: AtomicU8 = ATOMIC_U8_INIT;

#[no_mangle]
pub unsafe extern fn rust_main(args_ptr: *const arch::x86_64::start::KernelArgs) -> !
{
        DISK.store((*args_ptr).disk, Ordering::SeqCst);
        let mut active_table  = arch::x86_64::start::kstart(args_ptr);
        fs::init_real_mode(&mut active_table);
        let mut mbr = fs::read_bootsector(*(DISK.get_mut()));
        let part_table = fs::disk::PartitionTable::new(&mbr);

        println!("{:?}", part_table);
        
        let boot_partition = match part_table.get_bootable() {
            Some((boot, idx)) => boot,
            None => panic!("No bootable partition found!")
        };
        
        println!("Booting Kernel from {:?}", boot_partition);

        match boot_partition.fs {
               Fs::FAT32 => {
                        let mut fs = fat::FatFileSystem::<fs::disk::Partition>::mount(*(DISK.get_mut())).expect("FS error");
                        let mut fs_root = fs.root().expect("Root Error");
                        let kernel_file = File { file: fs_root.open_file("kernel.dat").expect("Kernel not found").expect("Unwrap Error"), args: vec![] };
                        let mut env = format!(""); 
                        loader::load_kernel(&mut active_table, kernel_file, env);
               },
               Fs::RedoxFS => { 
                        let mut f = fs::redoxfs::FileSystem::open(boot_partition).expect("RedoxFS open error");
                        let root = f.header.1.root;
                        let node = f.find_node("kernel", root).expect("Kernel Node Error");
                        let mut env = format!("REDOXFS_UUID=");
                        for i in 0..f.header.1.uuid.len() {
                            if i == 4 || i == 6 || i == 8 || i == 10 {
                                env.push('-');
                            }
                            env.push_str(&format!("{:>02x}", f.header.1.uuid[i]));
                        }
                        env.push('\0');
                        let kernel_file = File { file: fs::redoxfs::FileSystem::open(boot_partition).expect("RedoxFS open error"), args: vec![0, node.0 as usize] };
                        loader::load_kernel(&mut active_table, kernel_file, env);
                                
               }, 
               Fs::Other => panic!("Unsupported boot partition")
        };
            
        loop { }
}
/*
#[lang = "eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang = "panic_fmt"] pub extern "C" fn panic_fmt() -> !{ loop {} }
*/
