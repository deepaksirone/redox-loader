use fs;
use fs::{read_bootsector};
use fs::disk::{PartitionTable, Partition};
use fat::{FatResult, DeviceStatus, StorageDevice, SectorOffset, FatError};
use fat::DeviceStatus::Normal;
use fs::SECTOR_SIZE; 

impl StorageDevice for Partition {

    // Here id is the partition number
    fn initialize(id: u8, partition_id: u8) -> FatResult<Partition> {
        let idx = partition_id as usize;
        let mbr = read_bootsector();
        let part_table = PartitionTable::new(&mbr);
        Ok(part_table.partitions[idx].clone())
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


