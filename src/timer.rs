// APB Registers
pub const TIMER: u32 =       0x80020000;
pub const TIMER_IOS: u32 =   TIMER + 0x00;
pub const TIMER_TCF: u32 =   TIMER + 0x04;
pub const TIMER_TCNT: u32 =  TIMER + 0x08;
pub const TIMER_TSCR: u32 =  TIMER + 0x0C;
pub const TIMER_TOV: u32 =   TIMER + 0x10;
pub const TIMER_TCR: u32 =   TIMER + 0x14;
pub const TIMER_TIE: u32 =   TIMER + 0x18;
pub const TIMER_TSCR2: u32 = TIMER + 0x1C;
pub const TIMER_FLG1: u32 =  TIMER + 0x20;
pub const TIMER_FLG2: u32 =  TIMER + 0x24;

pub unsafe fn timer_tcn(channel: u32) -> u32 {
    TIMER + 0x28 + (0x4 * channel)
}

// Timer APB Register Bits
// IOS
pub unsafe fn timer_ios_input(channel: u32) -> u32 {
    !(1 << channel)
}

pub unsafe fn timer_ios_output(channel: u32) -> u32 {
    1 << channel
}

// TCF
pub const TIMER_TCF_MASK: u32 = 0xFF;

// TSCR
pub const TIMER_TSCR_ENABLE: u32 =  1 << 7;
pub const TIMER_TSCR_DISABLE: u32 = !(1 << 7);

// TOV
pub const TIMER_TOV_MASK: u32 = 0xFF;

// TCR
pub unsafe fn timer_tcr_edge_mask(channel: u32) -> u32 {
    0x101 << channel
}

pub const TIMER_TCR_EDGE_DISABLE: u32 = 0x000;
pub const TIMER_TCR_EDGE_FALLING: u32 = 0x001;
pub const TIMER_TCR_EDGE_RISING: u32 =  0x100;
pub const TIMER_TCR_EDGE_EITHER: u32 =  0x101;

pub unsafe fn timer_tcr_output_mask(channel: u32) -> u32 {
    (0x101 << 16) << channel
}

pub const TIMER_TCR_OUTPUT_DISCONNECT: u32 = 0x000 << 16;
pub const TIMER_TCR_OUTPUT_TOGGLE: u32 =     0x001 << 16;
pub const TIMER_TCR_OUTPUT_CLEAR: u32 =      0x100 << 16;
pub const TIMER_TCR_OUTPUT_SET: u32 =        0x101 << 16;

// TIE
pub unsafe fn timer_tie_mask(channel: u32) -> u32 {
    1 << channel
}

pub const TIMER_TIE_ENABLE: u32 =  1;
pub const TIMER_TIE_DISABLE: u32 = !1;

// TSCR2
pub const TIMER_TSCR2_TOI_ENABLE: u32 =   1 << 7;
pub const TIMER_TSCR2_TOI_DISABLE: u32 =  !(1 << 7);
pub const TIMER_TSCR2_TCRE_ENABLE: u32 =  1 << 6;
pub const TIMER_TSCR2_TCRE_DISABLE: u32 = !(1 << 6);
pub const TIMER_TSCR2_PRE_MASK: u32 =     0x7;
pub const TIMER_TSCR2_PRE_DIV1: u32 =     0;
pub const TIMER_TSCR2_PRE_DIV2: u32 =     1;
pub const TIMER_TSCR2_PRE_DIV4: u32 =     2;
pub const TIMER_TSCR2_PRE_DIV8: u32 =     3;
pub const TIMER_TSCR2_PRE_DIV16: u32 =    4;
pub const TIMER_TSCR2_PRE_DIV32: u32 =    5;
pub const TIMER_TSCR2_PRE_DIV64: u32 =    6;
pub const TIMER_TSCR2_PRE_DIV128: u32 =   7;

// FLG1 CHECK: may want to just make a general mask for the channels
pub const TIMER_FLG1_MASK: u32 = 0xFF;

// FLG2
pub const TIMER_FLG2_CLEAR: u32 = 1 << 7;

pub unsafe fn timer_enable() {
    let mut value = core::ptr::read_volatile(TIMER_TSCR as *mut u32);
    value |= TIMER_TSCR_ENABLE;
    core::ptr::write_volatile(TIMER_TSCR as *mut u32, value);
}

pub unsafe fn timer_disable() {
    let mut value = core::ptr::read_volatile(TIMER_TSCR as *mut u32);
    value &= TIMER_TSCR_DISABLE;
    core::ptr::write_volatile(TIMER_TSCR as *mut u32, value);
}

pub unsafe fn timer_set_output_action(channel: u32, output_action: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TCR as *mut u32);
    value = (value & !(timer_tcr_output_mask(channel))) | (timer_tcr_output_mask(channel) & (output_action << channel));
    core::ptr::write_volatile(TIMER_TCR as *mut u32, value);
}

pub unsafe fn timer_set_input_capture_edge(channel: u32, capture_edge: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TCR as *mut u32);
    value = (value & !(timer_tcr_edge_mask(channel))) | (timer_tcr_edge_mask(channel) & (capture_edge << channel));
    core::ptr::write_volatile(TIMER_TCR as *mut u32, value);
}

pub unsafe fn timer_set_prescaler(pre_div: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TSCR2 as *mut u32);
    value = (value & !TIMER_TSCR2_PRE_MASK) | (TIMER_TSCR2_PRE_MASK & pre_div);
    core::ptr::write_volatile(TIMER_TSCR2 as *mut u32, value);
}

pub unsafe fn timer_set_output_compare(channel: u32, output_action: u32, interrupt_enable: u32, value: u32) {
    let mut hold = core::ptr::read_volatile(TIMER_TCR as *mut u32);
    hold = (hold & !timer_tcr_output_mask(channel)) | (timer_tcr_output_mask(channel) & (output_action << channel));
    core::ptr::write_volatile(TIMER_TCR as *mut u32, hold);
    core::ptr::write_volatile(timer_tcn(channel) as *mut u32, value);
    hold = core::ptr::read_volatile(TIMER_TIE as *mut u32);
    hold = (hold & !timer_tie_mask(channel)) | (timer_tie_mask(channel) & (interrupt_enable << channel));
    core::ptr::write_volatile(TIMER_TIE as *mut u32, hold);
}

pub unsafe fn timer_set_input_capture(channel: u32, capture_edge: u32, interrupt_enable: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TCR as *mut u32);
    value = (value & !timer_tcr_edge_mask(channel)) | (timer_tcr_edge_mask(channel) & (capture_edge << channel));
    core::ptr::write_volatile(TIMER_TCR as *mut u32, value);
    value = core::ptr::read_volatile(TIMER_TIE as *mut u32);
    value = (value & !timer_tie_mask(channel)) | (timer_tie_mask(channel) & (interrupt_enable << channel));
    core::ptr::write_volatile(TIMER_TIE as *mut u32, value);
}

// CHECK
pub unsafe fn timer_read_input_input_capture(channel: u32) -> u32 {
    core::ptr::read_volatile(timer_tcn(channel) as *mut u32)
}

// CHECK: this only clears the selectee channels? not all?
pub unsafe fn timer_clear_interrupt(channels: u32) {
    let mut value = core::ptr::read_volatile(TIMER_FLG1 as *mut u32);
    value |= TIMER_FLG1 & channels;
    core::ptr::write_volatile(TIMER_FLG1 as *mut u32, value);
}

pub unsafe fn timer_enable_cf(channels: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TCF as *mut u32);
    value |= TIMER_TCF_MASK & channels;
    core::ptr::write_volatile(TIMER_TCF as *mut u32, value);
}

pub unsafe fn timer_enable_tov(channels: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TOV as *mut u32);
    value |= TIMER_TOV_MASK & channels;
    core::ptr::write_volatile(TIMER_TOV as *mut u32, value);
}

pub unsafe fn timer_disable_tov(channels: u32) {
    let mut value = core::ptr::read_volatile(TIMER_TOV as *mut u32);
    value &= !TIMER_TOV_MASK | !channels;
    core::ptr::write_volatile(TIMER_TOV as *mut u32, value);
}

pub unsafe fn timer_read_count() -> u32 {
    core::ptr::read_volatile(TIMER_TCNT as *mut u32)
}