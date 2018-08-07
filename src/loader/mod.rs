extern crate fat;
use fat::{FatFileSystem, StorageDevice};
use core::{slice, mem, ptr};
use alloc::{Vec, String};
use memory::Frame;
use paging;
use paging::{ActivePageTable, Page, VirtualAddress, PhysicalAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;
use consts;
use interrupt;

pub const KERNEL: &'static str = "kernel.dat";
static KERNEL_LOAD_ADDRESS: usize = 0x400000;
static mut KERNEL_SIZE: usize = 0;
static mut KERNEL_ENTRY: u64 = 0;

static STACK_PHYSICAL: usize = 0x80000;
static STACK_VIRTUAL: usize = 0xFFFFFF0000080000;
static STACK_SIZE: usize = 0x1F000;

static mut ENV_SIZE: usize = 0x0;

#[repr(packed)]
struct EntryArgs {
    kernel_base: u64,
    kernel_size: u64,
    stack_base: u64,
    stack_size: u64,
    env_base: u64,
    env_size: u64,
}

unsafe fn enter() -> ! {
    let args = EntryArgs {
        kernel_base: KERNEL_LOAD_ADDRESS as u64,
        kernel_size: KERNEL_SIZE as u64,
        stack_base: STACK_VIRTUAL as u64,
        stack_size: STACK_SIZE as u64,
        env_base: STACK_VIRTUAL as u64,
        env_size: ENV_SIZE as u64,
    };

    let entry_fn: extern "C" fn(args_ptr: *const EntryArgs) -> ! = mem::transmute(KERNEL_ENTRY);
    entry_fn(&args);
}

fn init_kernel_copy(active_table: &mut ActivePageTable, filesize: usize)
{
    let start_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS));
    let end_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS + filesize));
    
    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }
    
    let start_page = Page::containing_address(VirtualAddress::new(0x0));
    let end_page = Page::containing_address(VirtualAddress::new(0x7000 - 1));
    
    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }


    let start_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS + consts::KERNEL_OFFSET));
    let end_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS + consts::KERNEL_OFFSET + 0x40000000));
    
    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get() - consts::KERNEL_OFFSET));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }

/*
    //identity map page table store
    let page_table_start = Page::containing_address(VirtualAddress::new(0x70000));
    let page_table_end = Page::containing_address(VirtualAddress::new(0x70000 - 1));

    for page in paging::Page::range_inclusive(page_table_start, page_table_end) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }
*/
}

pub fn load_kernel<T: StorageDevice> (active_table: &mut ActivePageTable, fs: &mut FatFileSystem::<T>)
{
    let root = fs.root().expect("Root Error");
    let mut kernel_file = root.open_file(KERNEL).expect("Kernel Open Error").expect("Kernel Open Error");
    let mut vec: Vec<u8> = vec![0; 1024 * 1024];
    println!("Kernel File Size: {}", kernel_file.size());
    init_kernel_copy(active_table, kernel_file.size() as usize);
    let num_invokes = (kernel_file.size() + (1024 * 1024) - 1) / (1024 * 1024);
    let mut addr = KERNEL_LOAD_ADDRESS;
    
    println!("Num Invokes: {}", num_invokes);
    for i in 0..num_invokes {
//        println!("In loop");
        
        let read_bytes = kernel_file.read(vec.as_mut_slice()).expect("Kernel Read Error") as usize;
        addr += copy_slice_to_addr(&vec.as_slice()[0..read_bytes], addr);
//        vec.clear();
/*        for byte in vec.iter() {
            print!("{}", *byte as char);
        }
*/
    }
    let env = String::from("REDOXFS_UUID=4bf86d4a-28ae-4ad6-8cc3-a0e447192168");

    unsafe {
        KERNEL_SIZE = kernel_file.size() as usize;
        KERNEL_ENTRY = *((KERNEL_LOAD_ADDRESS + 0x18) as usize as *const u64);
        ENV_SIZE = env.len() as usize;
        ptr::copy(env.as_ptr(), STACK_VIRTUAL as *mut u8, env.len());
        println!("Running kernel");
        asm!("mov rsp, $0" : : "r"(STACK_VIRTUAL + STACK_SIZE) : "memory" : "intel", "volatile");
        interrupt::disable();
        enter();
    }
    
}

fn copy_slice_to_addr(slice: &[u8], addr: usize) -> usize
{
    let ptr = addr as *mut u8;
    let mut dest_slice = unsafe { slice::from_raw_parts_mut(ptr, slice.len()) };
    dest_slice[0..slice.len()].clone_from_slice(&slice);
    slice.len()
}
