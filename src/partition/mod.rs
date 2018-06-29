use memory::Frame;
use paging::{ActivePageTable, Page, PhysicalAddress, VirtualAddress};
use paging::entry::EntryFlags;
use paging::mapper::MapperFlushAll;
use core::mem;

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

pub unsafe fn drop_to_real(active_table: &mut ActivePageTable)
{

    let real_func_addr = 0xf000;
    let ptr = 0xf000 as *const ();
    let code: extern "C" fn() = unsafe { mem::transmute(ptr) };

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


    (code)();

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

}
