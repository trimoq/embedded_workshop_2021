use core::ptr;

const PIN_ID_BCM: usize = 23;
const PERI_BASE: usize = 0x3F000000;
const GPIO_OFST: usize = 0x00200000;
const GPIO_BASE: usize = PERI_BASE + GPIO_OFST;

const OUTPUT_MASK: usize = 0b001;

// 3*10 bits used per register -> 10 pins per register
const LED_GPFSEL: usize = PIN_ID_BCM / 10;
// Offset within the select register
const LED_GPFBIT_OFST: usize = (PIN_ID_BCM % 10) * 3;

const LED_GPSET: usize = 7; // set0
const LED_GPCLR: usize = 10; // clr0


fn set_as_output(mem_ptr: *mut usize) {
    let reg = read_gpio_reg(mem_ptr, LED_GPFSEL);
    let new = reg | OUTPUT_MASK << LED_GPFBIT_OFST;
    write_gpio_reg(mem_ptr, LED_GPFSEL, new);
}

fn read_gpio_reg(mem_ptr: *const usize, offset: usize) -> usize {
    unsafe { 
        ptr::read_volatile(mem_ptr.add(offset)) 
    }
}

fn write_gpio_reg(mem_ptr: *mut usize, offset: usize, value: usize) {
    unsafe { 
        ptr::write_volatile(mem_ptr.add(offset), value) 
    }
}



pub struct Led23 {
    state: bool,
    mem_ptr: *mut usize
}

impl Led23 {

    pub fn new() -> Self {
        let mem_ptr = GPIO_BASE as *mut usize;
        set_as_output(mem_ptr);
        Self { state: false, mem_ptr }
    }

    pub fn set(&mut self) {
        write_gpio_reg(self.mem_ptr, LED_GPSET, 1 << PIN_ID_BCM);
        self.state = true;
    }

    pub fn clear(&mut self) {
        write_gpio_reg(self.mem_ptr, LED_GPCLR, 1 << PIN_ID_BCM);
        self.state = false;
    }

    pub fn toggle(&mut self) {
        if self.state {
            self.clear()
        }
        else{
            self.set()
        }
    }

}
