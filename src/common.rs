// Chip Frequency
pub const CHIP_FREQ: u32 = 100000000;

// GPIO Constants
pub const GPIO: u32 =                  	0x80000000;
pub const GPIO_DATA: u32 =             	GPIO + 0x04;
pub const GPIO_DATA_DIRECTION: u32 =   	GPIO + 0x08;
pub const GPIO_INTERRUPT_ENABLE: u32 =	GPIO + 0x0C;
pub const GPIO_POSITIVE_EDGE: u32 =     GPIO + 0x10;
pub const GPIO_NEGATIVE_EDGE: u32 =     GPIO + 0x14;
pub const GPIO_INTERRUPT_CLEAR: u32 =	GPIO + 0x18;
pub const GPIO_INTERRUPT_STATUS: u32 =	GPIO + 0x1C;
pub const GPIOALL_AFTX06: u32 =         0xFF;
pub const GPIOALL: u32 =                0xFFFFFFFF;

// PWM Constants
pub const PWM: u32 =                      0x80010000;
pub const PWM_PERIOD: u32 =               PWM + 0x00;
pub const PWM_DUTY: u32 =                 PWM + 0x04;
pub const PWM_CONTROL: u32 =              PWM + 0x08;
pub const PWM_CONTROL_DISABLE: u32 =      !(1 << 0);
pub const PWM_CONTROL_ENABLE: u32 =       1 << 0;
pub const PWM_CONTROL_ACTIVE_HIGH: u32 =  !(1 << 1);
pub const PWM_CONTROL_ACTIVE_LOW: u32 =   1 << 1;
pub const PWM_CONTROL_ALIGN_LEFT: u32 =   !(1 << 2);
pub const PWM_CONTROL_ALIGN_CENTER: u32 = 1 << 2;
pub const PWM_CHANNEL_SIZE: u32 =         0x0C;
pub const PWM_MAX_FREQ: u32 =             CHIP_FREQ / 2;
pub const AFTX06_DUTY_OFFSET: u32 =       1;

pub fn gpion(pin: i32) -> u32 {
    1 << pin
}

pub fn pwmn(channel: u32) -> u32 {
    channel * PWM_CHANNEL_SIZE
}

pub fn rounding_division(dividend: u32, divisor: u32) -> u32 {
    if divisor == 0 { return 0; }
    (dividend + (divisor / 2)) / divisor
}
