use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static!{
    pub static ref ATA1: Mutex<AtaDevice> = Mutex::new( AtaDevice::from_address(0x1F0));
}

#[allow(non_snake_case)]
pub struct AtaDevice {
    ATA_DATA: Port<u16>,
    ATA_ERROR: Port<u8>,
    ATA_SECTOR_COUNT: Port<u8>,
    ATA_SECTOR_NUMBER: Port<u8>,
    ATA_CYLINDER_LOW: Port<u8>,
    ATA_CYLINDER_HIGH: Port<u8>,
    ATA_DRIVE_HEAD: Port<u8>,
    ATA_COMMAND: Port<u8>
}

impl AtaDevice {
    // Create an ata device at the given offset
    pub fn from_address(start_adress: u16) -> AtaDevice {
        return AtaDevice {
            ATA_DATA: Port::new(start_adress),
            ATA_ERROR: Port::new(start_adress + 1),
            ATA_SECTOR_COUNT: Port::new(start_adress + 2),
            ATA_SECTOR_NUMBER: Port::new(start_adress + 3),
            ATA_CYLINDER_LOW: Port::new(start_adress + 4),
            ATA_CYLINDER_HIGH: Port::new(start_adress + 5),
            ATA_DRIVE_HEAD: Port::new(start_adress + 6),
            ATA_COMMAND: Port::new(start_adress + 7)
        };
    }

    #[allow(arithmetic_overflow)]
    pub fn write_sector(&mut self, sector_number: u32, sector_count: u8, data: &[u8]) -> Option<()>{
        unsafe {
            // Wait for drive to be ready
            self.wait_busy();

            // Set the sector count
            self.ATA_SECTOR_COUNT.write(sector_count);

            // Set the sector number
            self.ATA_SECTOR_NUMBER.write((sector_number) as u8);
            self.ATA_CYLINDER_LOW.write((sector_number >> 8) as u8);
            self.ATA_CYLINDER_HIGH.write((sector_number >> 16) as u8);

            // Set the drive and head (assuming LBA28 addressing)
            self.ATA_DRIVE_HEAD.write(0xA0 | (sector_number >> 24 & 0xF) as u8);

            // Send the write sectors command
            self.ATA_COMMAND.write(0x30);

            // Write the data into the ATA device
            for current_sector in 0..sector_count as usize {
                self.wait_busy();
                for byte in 0..256 {
                    if current_sector * 256 + byte < data.len() {
                        self.ATA_DATA.write(data[current_sector * 256 + byte] as u16)
                    } else {
                        self.ATA_DATA.write(0);
                    }
                }
            }

            self.wait_busy();

            // Check for errors
            return if self.ATA_ERROR.read() == 0 {
                Some(())
            } else {
                None
            }
        } 
    }

    #[allow(arithmetic_overflow)]
    pub fn read_sectors(&mut self, sector_number: u8, sector_count: u8, buffer: &mut [u8]) -> Option<()> {
        unsafe {
            self.wait_busy();

            // Set the sector count
            self.ATA_SECTOR_COUNT.write(sector_count);

            // Set the sector number
            self.ATA_SECTOR_NUMBER.write(sector_number);
            self.ATA_CYLINDER_LOW.write(sector_number.wrapping_shr(8));
            self.ATA_CYLINDER_HIGH.write(sector_number.wrapping_shr(16));

            // Set the drive and head (assuming LBA28 addressing)
            self.ATA_DRIVE_HEAD.write(0xA0 | (sector_number.wrapping_shr(24)));

            // Set the sector count
            self.ATA_SECTOR_COUNT.write(sector_count);

            // Send the read sectors command
            self.ATA_COMMAND.write(0x20);

            // Read the data from the ATA data port
            for current_sector in 0..sector_count as usize {
                self.wait_busy();
                for byte in 0..256 {
                    if current_sector * 256 + byte < buffer.len() {
                        buffer[current_sector * 256 + byte] = self.ATA_DATA.read() as u8;
                    } else {
                        break;
                    }
                }
            }

            self.wait_busy();

            // Check for errors
            return if self.ATA_ERROR.read() == 0 {
                Some(())
            } else {
                None
            }

        }
    }

    // Wait until the controller is no longer busy
    fn wait_busy(&mut self) {
        unsafe {
            while self.ATA_COMMAND.read() & 0x80 == 1 {}
        }
    }
}
