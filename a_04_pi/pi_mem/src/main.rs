use core::panic;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io;
use std::os::unix::prelude::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use std::ptr;

use libc::{self, off_t, MAP_FAILED, MAP_SHARED, O_SYNC, PROT_READ, PROT_WRITE};


const PIN_ID_BCM : usize = 23;
const PERI_BASE : usize = 0x3F000000;
const GPIO_OFST : usize = 0x00200000;
const GPIO_BASE : usize = PERI_BASE + GPIO_OFST;

const OUTPUT_MASK : usize = 0b001;

// 3*10 bits used per register -> 10 pins per register
const LED_GPFSEL : usize = PIN_ID_BCM /10;
// Offset within the select register
const LED_GPFBIT_OFST : usize = (PIN_ID_BCM % 10) * 3;

const LED_GPSET : usize = 7; // set0
const LED_GPCLR : usize = 10; // clr0

fn main() {

    let mem_ptr = map_gpio_mem() as *mut usize;

    set_as_output(mem_ptr);
  
    loop{
        write(mem_ptr, LED_GPSET, 1<<PIN_ID_BCM);
        sleep(Duration::from_millis(100));
        write(mem_ptr, LED_GPCLR, 1<<PIN_ID_BCM);
        sleep(Duration::from_millis(100));
    }

}

fn set_as_output(mem_ptr: *mut usize) {
    let reg = read(mem_ptr, LED_GPFSEL);
    println!("Pre_select: {:b}", reg);

    let new = reg | OUTPUT_MASK<<LED_GPFBIT_OFST;

    write(mem_ptr, LED_GPFSEL, new);

    let reg = read(mem_ptr, LED_GPFSEL);
    println!("Post_select: {:b}", reg);
}

fn read(mem_ptr: *const usize, offset: usize) -> usize{
    unsafe{ptr::read_volatile(mem_ptr.add(offset ))}
}
fn write(mem_ptr: *mut usize, offset: usize, value: usize){
    unsafe{ ptr::write_volatile(mem_ptr.add(offset), value)}; 
}

fn map_gpio_mem() -> *mut libc::c_void {

    let mem_file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_SYNC)
        .open("/dev/mem").unwrap();

    let mem_ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            512,        // mhm...
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            mem_file.as_raw_fd(),
            GPIO_BASE as off_t,
            )
    };

    if mem_ptr == MAP_FAILED {
        panic!("Broken: {}", io::Error::last_os_error());
    }

    mem_ptr
}
