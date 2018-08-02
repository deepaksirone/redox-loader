extern crate fat;
use fat::{FatFileSystem, StorageDevice};
use core::slice;
use alloc::Vec;
use memory::Frame;
use paging;
use paging::{ActivePageTable, Page, VirtualAddress, PhysicalAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;

pub const KERNEL: &'static str = "kernel.dat";
pub const KERNEL_LOAD_ADDRESS: usize = 0x400000;

fn init_kernel_copy(active_table: &mut ActivePageTable, filesize: usize)
{
    let start_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS));
    let end_page = Page::containing_address(VirtualAddress::new(KERNEL_LOAD_ADDRESS + filesize));
    
    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }

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
        println!("In loop");
        
        let read_bytes = kernel_file.read(vec.as_mut_slice()).expect("Kernel Read Error") as usize;
        addr += copy_slice_to_addr(&vec.as_slice()[0..read_bytes], addr);
//        vec.clear();
/*        for byte in vec.iter() {
            print!("{}", *byte as char);
        }
*/
    }

}

fn copy_slice_to_addr(slice: &[u8], addr: usize) -> usize
{
    let ptr = addr as *mut u8;
    let mut dest_slice = unsafe { slice::from_raw_parts_mut(ptr, slice.len()) };
    dest_slice[0..slice.len()].clone_from_slice(&slice);
    slice.len()
}
