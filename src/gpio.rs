// APB Registers
pub const GPIO: u32 =                  	0x80000000;
pub const GPIO_DATA: u32 =             	GPIO + 0x04;
pub const GPIO_DATA_DIRECTION: u32 =   	GPIO + 0x08;
pub const GPIO_INTERRUPT_ENABLE: u32 =	GPIO + 0x0C;
pub const GPIO_POSITIVE_EDGE: u32 =     GPIO + 0x10;
pub const GPIO_NEGATIVE_EDGE: u32 =     GPIO + 0x14;
pub const GPIO_INTERRUPT_CLEAR: u32 =	GPIO + 0x18;
pub const GPIO_INTERRUPT_STATUS: u32 =	GPIO + 0x1C;
pub const GPIOALL_AFTX06: u32 =		    0xFF;
pub const GPIOALL: u32 = 		        0xFFFFFFFF;

// General bit-shift
pub unsafe fn gpion(pin: i32) -> u32 {
	1 << pin
}

// Input
pub unsafe fn gpio_enable_input(pins: u32) {
	let mut value: u32 = core::ptr::read_volatile(GPIO_DATA_DIRECTION as *mut u32);
	value &= !pins;
	core::ptr::write_volatile(GPIO_DATA_DIRECTION as *mut u32, value);		
}

pub unsafe fn gpio_read_input(pins: u32) -> u32 {
	let mut value: u32 = core::ptr::read_volatile(GPIO_DATA as *mut u32);
	value &= pins;
	value
}

// Output
pub unsafe fn gpio_enable_output(pins: u32) {
	let mut value: u32 = core::ptr::read_volatile(GPIO_DATA_DIRECTION as *mut u32);
	value |= pins;
	core::ptr::write_volatile(GPIO_DATA_DIRECTION as *mut u32, value);
}

pub unsafe fn gpio_set_output(pins: u32, pin_outputs: u32) {
	let mut value: u32 = core::ptr::read_volatile(GPIO_DATA as *mut u32);
	value &= !pins;
	value |= pins & pin_outputs;
	core::ptr::write_volatile(GPIO_DATA as *mut u32, value);
}

// Interrupts
pub unsafe fn gpio_enable_interrupt_posedge(pins: u32) {
    let mut value: u32 = core::ptr::read_volatile(GPIO_NEGATIVE_EDGE as *mut u32);
    value &= !pins;
    core::ptr::write_volatile(GPIO_NEGATIVE_EDGE as *mut u32, value);
    value = core::ptr::read_volatile(GPIO_POSITIVE_EDGE as *mut u32);
    value |= pins;
    core::ptr::write_volatile(GPIO_POSITIVE_EDGE as *mut u32, value);
    value = core::ptr::read_volatile(GPIO_INTERRUPT_ENABLE as *mut u32);
    value |= pins;
    core::ptr::write_volatile(GPIO_INTERRUPT_ENABLE as *mut u32, value);
}

pub unsafe fn gpio_disable_interrupt_posedge(pins: u32) {
    let mut value: u32 = core::ptr::read_volatile(GPIO_INTERRUPT_ENABLE as *mut u32);
    value &= !pins;
    core::ptr::write_volatile(GPIO_INTERRUPT_ENABLE as *mut u32, value);
    value = core::ptr::read_volatile(GPIO_POSITIVE_EDGE as *mut u32);
    value &= !pins;
    core::ptr::write_volatile(GPIO_POSITIVE_EDGE as *mut u32, value);
}

pub unsafe fn gpio_clear_interrupt(pins: u32) {
    let mut value: u32 = core::ptr::read_volatile(GPIO_INTERRUPT_CLEAR as *mut u32);
    value |= pins;
    core::ptr::write_volatile(GPIO_INTERRUPT_CLEAR as *mut u32, value);
}

pub unsafe fn gpio_interrupt_status(pins: u32) -> u32 {
    let mut value: u32 = core::ptr::read_volatile(GPIO_INTERRUPT_STATUS as *mut u32);
    value &= pins;
    value
}
