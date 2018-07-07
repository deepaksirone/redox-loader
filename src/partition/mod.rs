use memory::Frame;
use paging::{ActivePageTable, Page, PhysicalAddress, VirtualAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;
use core::mem;
use paging;

use self::mbr::Mbr;
use paging::PAGE_SIZE;
pub mod mbr;

const SECTOR_SIZE: usize = 512;

#[derive(Clone, Debug, Copy)]
pub struct SectorIter {
    pub start_addr: usize,
    pub end_addr: usize
}

pub fn read_bootsector(active_table: &mut ActivePageTable) -> Mbr {
    let mut mbr = Mbr::default();
    let bootsector_addr = 0x7c00;
    let follow_up = 0x7c00 + PAGE_SIZE;

    let ret;
    {
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

    {
            let bootsector = unsafe { &mut *(bootsector_addr as *mut Mbr) };
            ret = bootsector.clone();
            println!("Checking if Bootsector is valid: {}", bootsector.is_valid());
    }

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


    ret 
}

// Set the page table mappings for disk reads
// Pages 0x9000 and 0xa000 serve as the real mode stack
// Pages 0xb000 is where the real.asm code is put
// Pages 0xc000 to 0x70000 are for the reads into memory
pub unsafe fn init_real_mode(active_table: &mut ActivePageTable)
{
    let start_page = Page::containing_address(VirtualAddress::new(0x9000));
    let end_page = Page::containing_address(VirtualAddress::new(0x70000));
    for page in paging::Page::range_inclusive(start_page, end_page) {
        let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
        let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
        result.flush(active_table);
    }
        
}

// The main entry point into real mode code
pub unsafe fn read_drive(id: u8, buf: &mut [u8], start_sector: u32)
{

    let real_func_addr = 0xb000;
    let ptr = 0xb000 as *const ();
    let n_sectors: usize = (buf.len() + SECTOR_SIZE - 1) / 512;

    let read_func: extern "C" fn(start_lba: u32, num_sectors: u16, id: u8) = unsafe { mem::transmute(ptr) };
/*
    {
            let page = Page::containing_address(VirtualAddress::new(real_func_addr));
            let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
            println!("{:x}", page.start_address().get()); 
            let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            result.flush(active_table);

    }
    {
            let page = Page::containing_address(VirtualAddress::new(real_func_addr - 1));
            let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
            println!("{:x}", page.start_address().get()); 
            let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            result.flush(active_table);
    }

*/    
    asm!("push rax
         push rbx
         push rcx
         push rdx
         push rdi
         push rsi
         push r8
         push r9
         push r10
         push r11
         push fs"
         : : : : "intel", "volatile");

    // Invokes the code in bootsector/x86_64/real.asm
    (read_func)(start_sector, n_sectors as u16, id);

    asm!("pop fs
          pop r11
          pop r10
          pop r9
          pop r8
          pop rsi
          pop rdi
          pop rdx
          pop rcx
          pop rbx
          pop rax"
          : : : : "intel", "volatile");
/*
    {
        let page = Page::containing_address(VirtualAddress::new(real_func_addr));
        let (result, _frame) = active_table.unmap_return(page, false);
        result.flush(active_table);
    }

    {
        let page = Page::containing_address(VirtualAddress::new(real_func_addr - 1));
        let (result, _frame) = active_table.unmap_return(page, false);
        result.flush(active_table);
    }
*/
}
