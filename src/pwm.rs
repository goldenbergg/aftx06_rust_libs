use crate::common;

// APB Registers
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
pub const PWM_MAX_FREQ: u32 =             common::CHIP_FREQ / 2;
pub const AFTX06_DUTY_OFFSET: u32 =       1;

pub unsafe fn pwmn(channel: u32) -> u32 {
    channel * PWM_CHANNEL_SIZE
}

pub unsafe fn pwm_set_frequency(channel: u32, mut frequency: u32) {
    if PWM_MAX_FREQ < frequency { frequency = PWM_MAX_FREQ; }
    let period : u32 = common::rounding_division(common::CHIP_FREQ, frequency);
    core::ptr::write_volatile((PWM_PERIOD + channel) as *mut u32, period);
    core::ptr::write_volatile((PWM_DUTY + channel) as *mut u32, common::rounding_division(period, 2) + AFTX06_DUTY_OFFSET);
}

// Period
pub unsafe fn pwm_set_period(channel: u32, period: u32) {
   core::ptr::write_volatile((PWM_PERIOD + channel) as *mut u32, period);
}

// Duty
pub unsafe fn pwm_set_duty(channel: u32, duty: u32) {
    core::ptr::write_volatile((PWM_DUTY + channel) as *mut u32, duty + AFTX06_DUTY_OFFSET);
}

// Control
pub unsafe fn pwm_disable(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as *mut u32);
    value &= PWM_CONTROL_DISABLE;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}

pub unsafe fn pwm_enable(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as *mut u32);
    value |= PWM_CONTROL_ENABLE;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}

pub unsafe fn pwm_set_active_high(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as *mut u32);
    value &= PWM_CONTROL_ACTIVE_HIGH;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}

pub unsafe fn pwm_set_active_low(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as *mut u32);
    value |= PWM_CONTROL_ACTIVE_LOW;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}

pub unsafe fn pwm_set_align_left(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as * mut u32);
    value &= PWM_CONTROL_ALIGN_LEFT;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}

pub unsafe fn pwm_set_align_center(channel: u32) {
    let mut value: u32 = core::ptr::read_volatile((PWM_CONTROL + channel) as * mut u32);
    value |= PWM_CONTROL_ALIGN_CENTER;
    core::ptr::write_volatile((PWM_CONTROL + channel) as *mut u32, value);
}
