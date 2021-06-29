// Note, this file is only required for AFTx05.
// CRC is not implemented in AFTx06.
// APB Registers
pub const CRC: u32 =                  0x80030000;
pub const CRC_NAND_NOR_CONTROL: u32 = CRC + 0x00;
pub const CRC_NAND_NOR_INPUT: u32 =   CRC + 0x04;
pub const CRC_NAND_NOR_OUTPUT: u32 =  CRC + 0x08;
pub const CRC_XOR_BUF_CONTROL: u32 =  CRC + 0x0C;
pub const CRC_XOR_BUF_INPUT: u32 =    CRC + 0x10;
pub const CRC_XOR_BUF_OUTPUT: u32 =   CRC + 0x14;
pub const CRC_CONTROL: u32 =          CRC + 0x18;
pub const CRC_CONFIG: u32 =           CRC + 0x1C;
pub const CRC_STATUS: u32 =           CRC + 0x20;
pub const CRC_INPUT: u32 =            CRC + 0x24;
pub const CRC_OUTPUT: u32 =           CRC + 0x28;

// CRC ABP Register Bits
// NAND/NOR Control
pub const CRC_NAND_CONTROL: u32 =     0 << 0;
pub const CRC_NOR_CONTROL: u32 =      1 << 0;

// NAND/NOR Input
pub const CRC_NAND_NOR_INPUT_A: u32 = 1 << 0;
pub const CRC_NAND_NOR_INPUT_B: u32 = 1 << 1;

// XOR/BUF Control
pub const CRC_BUF_CONTROL: u32 =      0 << 0;
pub const CRC_XOR_CONTROL: u32 =      1 << 0;

// XOR/BUF Input
pub const CRC_XOR_BUF_INPUT_A: u32 =  1 << 0;
pub const CRC_XOR_BUF_INPUT_B: u32 =  1 << 1;

// CRC Control
pub const CRC_CONTROL_START: u32 =    1 << 0;
pub const CRC_CONTROL_RESET: u32 =    1 << 1;

// CRC CONFIG
pub const CRC_CONFIG_BUF_ALL: u32 =   0x00000000;
pub const CRC_CONFIG_XOR_ALL: u32 =   0xFFFFFFFF;

pub unsafe fn crc_start() {
    let mut value: u32 = core::ptr::read_volatile(CRC_CONTROL as *mut u32);
    value |= CRC_CONTROL_START;
    core::ptr::write_volatile(CRC_CONTROL as *mut u32, value);
}

pub unsafe fn crc_reset() {
    let mut value: u32 = core::ptr::read_volatile(CRC_CONTROL as *mut u32);
    value |= CRC_CONTROL_RESET;
    core::ptr::write_volatile(CRC_CONTROL as *mut u32, value);
}

pub unsafe fn crc_set_polynomial(polynomial: u32) {
    core::ptr::write_volatile(CRC_CONFIG as *mut u32, polynomial);
}

pub unsafe fn crc_set_input(input: u32) {
    core::ptr::write_volatile(CRC_INPUT as *mut u32, input);
}

pub unsafe fn crc_ready() -> u32 {
    core::ptr::read_volatile(CRC_STATUS as *mut u32) 
}

pub unsafe fn crc_output() -> u32 {
    core::ptr::read_volatile(CRC_OUTPUT as *mut u32)
}
