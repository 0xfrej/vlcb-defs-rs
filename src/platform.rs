use crate::{ArmProcessor, MicrochipProcessor};

pub type CpuManufacturerId = [char; 4];

pub enum Processor {
    Arm(ArmProcessor),
    Microchip(MicrochipProcessor),
    Atmel,
    Unknown{cpu_id: u8, manufacturer: u8}
}