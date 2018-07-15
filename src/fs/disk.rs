use fs::mbr::Mbr; 
//use core_io;

pub struct Disk {
    pub current_pos: usize,
    pub id: u8
}

#[derive(Copy, Clone, Debug)]
pub enum Fs {
    FAT32,
    Other
}


pub struct SystemId(u8);

impl SystemId {
    fn get_fs(&self) -> Fs {
        match self.0 {
            0x0c => Fs::FAT32,
            _ => panic!("Unsupported partition")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Partition {
    // The starting sector number
    pub start_sector: u32,
    // Length of the partition in sectors
    pub length: usize,
    // Seek position wrt start of partition
    pub relative_pos: usize,
    // Type of the filesystem
    pub fs: Fs,
    // Partition bootable flag
    pub is_boot: bool,
}

impl Partition {

    pub fn get_bootable(mbr: Mbr) -> Option<Partition> {
        for partition in mbr.partition_table.iter() {
            if partition.is_bootable() {
                println!("Found bootable partition");
                let systemid = SystemId(partition.system_id());
                return Some(Partition {
                    start_sector: partition.starting_lba(),
                    length: partition.partition_length() as usize,
                    relative_pos: 0,
                    fs: systemid.get_fs(),
                    is_boot: true
                })
            }
        }
        None
    }

}

/*
impl Read for Partition {
    

    

}
*/
