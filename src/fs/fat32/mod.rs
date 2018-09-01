use fs;
use fs::{read_bootsector};
use fs::disk::{PartitionTable, Partition};
use fat::{FatResult, DeviceStatus, StorageDevice, SectorOffset, FatError};
use fat::DeviceStatus::Normal;
use fs::SECTOR_SIZE; 
use fs::disk::{FileOps, File, FsError, SeekFrom};
use ::DISK;

impl StorageDevice for Partition {

    // Here id is the partition number
    fn initialize(id: u8) -> FatResult<Partition> {
        //let idx = partition_id as usize;
       // unsafe {
       //     let disk = *(DISK.get_mut());
       // }
        let mbr = unsafe { read_bootsector(*(DISK.get_mut())) };
        let part_table = PartitionTable::new(&mbr);
        Ok(part_table.get_bootable().unwrap().0)
    }

    fn status(&self, id: u8) -> FatResult<DeviceStatus> {
        Ok(Normal)
    }

    fn read(&mut self, id: u8, buffer: &mut [u8], sector: SectorOffset) -> FatResult<()> {
        let rel_offset = self.relative_pos + u32::from(sector * (SECTOR_SIZE as u32)) as usize;
        let num_sectors = (buffer.len() + rel_offset + SECTOR_SIZE - 1) / SECTOR_SIZE;
        let real_offset = (self.start_sector as usize * SECTOR_SIZE) + rel_offset;
        if num_sectors <= self.length {
            fs::read(id, buffer, real_offset);
            Ok(())
        }
        else {
            Err(FatError::IoError)
        }
    }

    fn write(&mut self, id: u8, buffer: &[u8], sector: SectorOffset) -> FatResult<()> {
        Ok(())
    }


}

impl<'a> FileOps for File<fat::File<'a, Partition>> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, FsError>
    {
        match self.file.read(buf) {
            Ok(size) => Ok(size as usize),
            Err(err) => Err(FsError::ReadError)
        }
    }

    fn write(&mut self, buf: &mut [u8]) -> Result<usize, FsError> {
        Ok(0)
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<u64, FsError> {
        Ok(0)
    }

    fn size(&mut self) -> usize {
        self.file.size() as usize
    }
}

