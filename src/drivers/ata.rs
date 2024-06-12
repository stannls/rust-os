use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static!{
    pub static ref ATA1: Mutex<AtaDevice> = Mutex::new( AtaDevice::from_address(0x1F0));
}

// Status constants
const STATUS_BSY: u8 = 0x80;
const STATUS_RDY: u8 = 0x40;
const STATUS_DRQ: u8 = 0x08;
const STATUS_DF: u8 = 0x20;
const STATUS_ERR: u8 = 0x01;

// Command constants
const READ_COMMAND: u8 = 0x20;
const WRITE_COMMAND: u8 = 0x30;

#[allow(non_snake_case)]
pub struct AtaDevice {
    ATA_DATA: Port<u8>,
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

    pub fn write_sector(&mut self, sector_number: u32, sector_count: u8, data: &[u8]) -> Option<()>{
        // Wait until controller is no longer busy
        self.wait_busy();
        
        // Setup the controller to write the correct sector
        unsafe {
            self.ATA_DRIVE_HEAD.write(0xE0 | ((sector_number >> 24) as u8 & 0xF));
            self.ATA_SECTOR_COUNT.write(sector_count);
            self.ATA_SECTOR_NUMBER.write(sector_number as u8);
            self.ATA_CYLINDER_LOW.write((sector_number >> 8) as u8);
            self.ATA_CYLINDER_HIGH.write((sector_number >> 16) as u8);
            self.ATA_COMMAND.write(WRITE_COMMAND);
        }

        // Write each sector
        for current_sector in 0..sector_count as usize {
            self.wait_busy();
            self.wait_drq();

            // Write byte onto the disk if there is enough data. Else fill with zeroes.
            for current_byte in 0..256 {
                if data.len() >= current_sector * 256 + current_byte {
                    unsafe {
                        self.ATA_DATA.write(data[current_sector * 256 + current_byte])
                    }
                }
                else {
                    unsafe {
                        self.ATA_DATA.write(0);
                    }
                }
            }
        }
        self.wait_busy();

        // Check for errors and report if an error has ocurred
        return if unsafe {self.ATA_ERROR.read()} == 0 {
                Some(())
            } else {
                None
            }

    }

    pub fn read_sectors(&mut self, sector_number: u32, sector_count: u8, buffer: &mut [u8]) -> Option<()> {
        // Wait until controller is no longer busy
        self.wait_busy();
        
        // Setup the controller to read the correct sector
        unsafe {
            self.ATA_DRIVE_HEAD.write(0xE0 | ((sector_number >> 24) as u8 & 0xF));
            self.ATA_SECTOR_COUNT.write(sector_count);
            self.ATA_SECTOR_NUMBER.write(sector_number as u8);
            self.ATA_CYLINDER_LOW.write((sector_number >> 8) as u8);
            self.ATA_CYLINDER_HIGH.write((sector_number >> 16) as u8);
            self.ATA_COMMAND.write(READ_COMMAND);
        }
        // Read each sector
        for current_sector in 0..sector_count as usize {
            self.wait_busy();
            self.wait_drq();

            // Write each byte into the buffer if it has enough space
            for current_byte in 0..256 {
                if buffer.len() >= current_sector * 256 + current_byte {
                    unsafe {
                        buffer[current_sector * 256 + current_byte] = self.ATA_DATA.read()
                    }
                }
            }
        }
        self.wait_busy();

        // Check for errors and report if an error has ocurred
        return if unsafe {self.ATA_ERROR.read()} == 0 {
                Some(())
            } else {
                None
            }

    }

    fn wait_busy(&mut self) {
        unsafe {
            while self.ATA_COMMAND.read() & STATUS_BSY == 1 {}
        }
    }

    fn wait_drq(&mut self) {
        unsafe {
            while self.ATA_COMMAND.read() & STATUS_DRQ == 1 {}
        }
    }

}
