use crate::common;
use volatile_register::{RW};

pub struct PWM {
    p: &'static mut PWMRegisterBlock
}

#[repr(C)]
struct PWMRegisterBlock {
    pub pwm0_period: RW<u32>,
    pub pwm0_duty:   RW<u32>,
    pub pwm0_ctrl:   RW<u32>,
}

impl PWM {
    pub fn new() -> PWM {
        PWM {
            p: unsafe { &mut *(0x8001_0000 as *mut PWMRegisterBlock) }
        }
    }

    pub fn pwm_set_frequency(&mut self, channel: i32, mut frequency: u32) {
        unsafe {
            if common::PWM_MAX_FREQ < frequency { frequency = common::PWM_MAX_FREQ; }
            let period: u32 = common::rounding_division(common::CHIP_FREQ, frequency);
            match channel {
                0 => {
                    self.p.pwm0_period.write(period);
                    self.p.pwm0_duty.write(common::rounding_division(period, 2) + common::AFTX06_DUTY_OFFSET);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_set_period(&mut self, channel: i32, period: u32) {
        unsafe {
            match channel {
                0 => self.p.pwm0_period.write(period),
                _ => { }
            }
        }
    }

    pub fn pwm_set_duty(&mut self, channel: i32, duty: u32) {
        unsafe {
            match channel {
                0 => self.p.pwm0_duty.write(duty + common::AFTX06_DUTY_OFFSET),
                _ => { }
            }
        }
    }

    pub fn pwm_disable(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_DISABLE;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_enable(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ENABLE;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_set_active_high(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_ACTIVE_HIGH;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_set_active_low(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ACTIVE_LOW;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_set_align_left(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr &= common::PWM_CONTROL_ALIGN_LEFT;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }

    pub fn pwm_set_align_center(&mut self, channel: i32) {
        unsafe {
            match channel {
                0 => {
                    let mut curr: u32 = self.p.pwm0_ctrl.read();
                    curr |= common::PWM_CONTROL_ALIGN_CENTER;
                    self.p.pwm0_ctrl.write(curr);
                }
                _ => { }
            }
        }
    }
}