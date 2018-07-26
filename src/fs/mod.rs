use memory::Frame;
use paging::{ActivePageTable, Page, PhysicalAddress, VirtualAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;
use core::{mem, slice};
use paging;

use self::mbr::Mbr;
use paging::PAGE_SIZE;
pub mod mbr;
pub mod disk;
mod fat32;

pub const SECTOR_SIZE: usize = 512;
const BOOTSECTOR_ADDR: usize = 0x7c00;
const DISK_READ_PAGE_START: usize = 0x9000;
const DISK_READ_STORAGE_START: usize = 0xc000;
const DISK_READ_PAGE_END: usize = 0x70000 - 1;
const READ_FUNC_ADDR: usize = 0xb000;
const NUM_STORAGE_SECTORS: usize = (DISK_READ_PAGE_END - DISK_READ_STORAGE_START + 1) / SECTOR_SIZE;

pub fn read_bootsector() -> Mbr {
//    let mut mbr = Mbr::default();
//    let bootsector_addr = 0x7c00;
//    let follow_up = 0x7c00 + PAGE_SIZE;

/*    {
            let page = Page::containing_address(VirtualAddress::new(bootsector_addr));
            let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
            let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            result.flush(active_table);
    }

    {
            let page = Page::containing_address(VirtualAddress::new(follow_up));
            let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
            let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            result.flush(active_table);
    }
*/
    let bootsector = unsafe { &mut *(BOOTSECTOR_ADDR as *mut Mbr) };
    let ret = bootsector.clone();
    println!("Checking if Bootsector is valid: {}", bootsector.is_valid());
    
/*
    {
        let page = Page::containing_address(VirtualAddress::new(bootsector_addr));
        let (result, _frame) = active_table.unmap_return(page, false);
        result.flush(active_table);
    }

    {
        let page = Page::containing_address(VirtualAddress::new(follow_up));
        let (result, _frame) = active_table.unmap_return(page, false);
        result.flush(active_table);
    }

*/
    ret 
}

// Set the page table mappings for disk reads
// Pages 0x9000 and 0xa000 serve as the real mode stack
// Pages 0xb000 is where the real.asm code is put
// Pages 0xc000 to 0x70000 are for the reads into memory
// 0x70000 onwards is where the paging structures start
pub unsafe fn init_real_mode(active_table: &mut ActivePageTable)
{
    let start_page = Page::containing_address(VirtualAddress::new(BOOTSECTOR_ADDR));
    let end_page = Page::containing_address(VirtualAddress::new(DISK_READ_PAGE_END));

    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }
        
}

fn copy_bytes(buf: &mut [u8], stored_bytes: usize, buffer_offset: usize, sector_offset: usize) -> usize
{
    println!("In copy_sectors");
    let ptr = (DISK_READ_STORAGE_START + sector_offset) as *const u8;
    let buf_cap = buf.len() - buffer_offset;
    
    let avail_bytes = stored_bytes - sector_offset;
    let n_bytes = if buf_cap < avail_bytes { buf_cap } else { avail_bytes };

    let mut slice = unsafe { slice::from_raw_parts(ptr, n_bytes) };
    println!("stored_bytes: {}, buffer_offser: {}, sector_offset: {}, n_bytes: {}", 
             stored_bytes, buffer_offset, sector_offset, n_bytes);
    buf[buffer_offset..buffer_offset+n_bytes].clone_from_slice(&slice);
    n_bytes
}

// The main entry point into real mode code
pub fn read(id: u8, buf: &mut [u8], offset: usize)
{
    println!("In Read");
    println!("id : {}, offset : {}", id, offset);
    let ptr = READ_FUNC_ADDR as *const ();
    let start_sector = offset / SECTOR_SIZE;
    let end_sector = (offset + buf.len() - 1) / SECTOR_SIZE;
    let mut num_sectors = end_sector - start_sector + 1;

//  let mut n_sectors: usize = (buf.len() + SECTOR_SIZE - 1) / SECTOR_SIZE;
    let num_invokes = (num_sectors + NUM_STORAGE_SECTORS - 1) / NUM_STORAGE_SECTORS;

//  TODO: Add exit status
    let read_func: extern "C" fn(start_lba: u32, num_sectors: u16, id: u8) = unsafe { mem::transmute(ptr) };
    let mut buffer_offset = 0;     
    for i in 0..num_invokes {
        let num_copy_sectors: u16 = if num_sectors > NUM_STORAGE_SECTORS 
                    { NUM_STORAGE_SECTORS as u16 } else { num_sectors as u16 };
        unsafe {
            scratch_push!();
            fs_push!();

            // Invokes the code in bootsector/x86_64/real.asm
            (read_func)(start_sector as u32 + (i as u32 * NUM_STORAGE_SECTORS as u32), num_copy_sectors, id);

            fs_pop!();
            scratch_pop!();
        }
        let sector_offset = if i == 0 { offset % SECTOR_SIZE } else { 0 }; 
        buffer_offset += copy_bytes(buf, num_copy_sectors as usize * SECTOR_SIZE, buffer_offset, sector_offset);
        num_sectors -= num_copy_sectors as usize; 
    }

}

