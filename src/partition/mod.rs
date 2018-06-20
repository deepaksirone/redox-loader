use memory::Frame;
use paging::{ActivePageTable, Page, PhysicalAddress, VirtualAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;

use self::mbr::Mbr;
use paging::PAGE_SIZE;
pub mod mbr;


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

