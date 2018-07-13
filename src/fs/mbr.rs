use byteorder::{LittleEndian, ByteOrder}; 
use core::{slice, mem};
use core::ops::{Deref, DerefMut};

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct MbrTableEntry
{
    data: [u8; 16]
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Mbr
{
    /// The bootstrap code, last 10 bytes can be used as the uuid 
    pub code: [u8; 446],
    /// The partition partition table
    pub partition_table: [MbrTableEntry; 4],
    /// Signature bytes
    pub signature: [u8; 2],
}

impl Mbr 
{
    pub fn default() -> Mbr {
        Mbr {
            code: [0; 446],
            partition_table: [MbrTableEntry::new(); 4],
            signature: [0; 2],
        }
    }
    
    pub fn is_valid(&self) -> bool {
        self.signature[0] == 0x55 && self.signature[1] == 0xaa
    }

}

impl MbrTableEntry
{
    pub fn new() -> MbrTableEntry {
        MbrTableEntry {
            data: [0; 16]
        }
    }

    pub fn is_bootable(&self) -> bool
    {
        self.data[0] == 0x80
    }

    pub fn system_id(&self) -> u8
    {
        self.data[4]
    }

    pub fn starting_lba(&self) -> u32
    {
        let mut buf = &self.data[8..12];
        LittleEndian::read_u32(&buf)
    }

    pub fn partition_length(&self) -> u32
    {
        let mut buf = &self.data[12..16];
        LittleEndian::read_u32(&buf)
    }

}
        
impl Deref for MbrTableEntry {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const MbrTableEntry as *const u8, mem::size_of::<MbrTableEntry>()) as &[u8]
        }
    }
}

impl DerefMut for MbrTableEntry {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut MbrTableEntry as *mut u8, mem::size_of::<MbrTableEntry>()) as &mut [u8]
        }
    }
}

