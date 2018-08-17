pub use self::disk::Disk;
pub use self::extent::Extent;
pub use self::filesystem::FileSystem;
pub use self::header::Header;
pub use self::node::Node;

use syscall::error::{Result, Error, EIO};
use fs::disk::{Partition, FileOps, FsError, SeekFrom, File};
use fs;
use super::SECTOR_SIZE;
use DISK;

mod disk;
mod extent;
mod filesystem;
mod header;
mod node;

pub const BLOCK_SIZE: u64 = 4096;
pub const SIGNATURE: &'static [u8; 8] = b"RedoxFS\0";
pub const VERSION: u64 = 3;

impl Disk for Partition {

    fn read_at(&mut self, block: u64, buffer: &mut[u8]) -> Result<usize>
    {
        let rel_offset = self.relative_pos + ((block as usize * BLOCK_SIZE as usize) as usize); 
        let num_blocks = (buffer.len() + rel_offset + BLOCK_SIZE as usize - 1) / BLOCK_SIZE as usize ;
        let real_offset = (self.start_sector as usize * SECTOR_SIZE) + rel_offset;
        let tot_blocks = ((self.length * SECTOR_SIZE) + BLOCK_SIZE as usize - 1) / BLOCK_SIZE as usize;
        if num_blocks <= tot_blocks {
            unsafe { fs::read(*(DISK.get_mut()), buffer, real_offset); }
            Ok(buffer.len())
        }
        else {
            Err(Error::new(EIO))
        }
 
    }

    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize>
    {
        Ok(0)
    }

    fn size(&mut self) -> Result<u64>
    {
        Ok((self.length * SECTOR_SIZE) as u64)
    }
}

impl FileOps for File<FileSystem<Partition>> {

    fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, FsError> 
    {
        unimplemented!()
    }
   
    fn write(&mut self, buf: &mut [u8]) -> core::result::Result<usize, FsError>
    {
        Ok(0)
    }

    fn seek(&mut self, pos: SeekFrom) -> core::result::Result<u64, FsError>
    {
        Ok(0)
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}
