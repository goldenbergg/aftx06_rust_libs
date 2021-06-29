// Common constants
pub const CHIP_FREQ: u32 = 100000000;

pub unsafe fn rounding_division(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 { return 0; }
    (dividend + (divisor / 2)) / divisor
}
