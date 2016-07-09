#![feature(lang_items, asm)]
#![crate_type= "staticlib"]
#![no_std]

const GPIO_BASE: u32 = 0x3F200000;
const LED_GPFSEL: isize = 4;
const LED_GPFBIT: isize = 21;
const LED_GPIO_BIT: isize = 15;
const LED_GPSET: isize = 8;
const LED_GPCLR: isize = 11;
const SLEEP: u32 = 100000;

fn sleep(value: u32) {  
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}

#[no_mangle]
pub extern fn kernel() -> ! {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(LED_GPSET) as *mut u32 };
    let led_off = unsafe { gpio.offset(LED_GPCLR) as *mut u32 };
    let gpfsel = unsafe { gpio.offset(LED_GPFSEL) as *mut u32 };
    
    unsafe { *gpfsel |= 1 << LED_GPFBIT };
    
    loop {
        sleep(SLEEP);
        unsafe { *led_on |= 1 << LED_GPIO_BIT };
        sleep(SLEEP);
        unsafe { *led_off |= 1 << LED_GPIO_BIT };
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}
