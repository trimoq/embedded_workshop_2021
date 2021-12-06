fn foo(){
    let mut number = 0x01_02_u16;
    println!("Number: {}", number);
    let reference = &mut number;
    let pointer = reference as *mut u16;
    let pointer2 = 0x100000000123 as *const u16;
    println!("Reference: {:p}", reference);
    println!("Pointer: {:p}", pointer);
    println!("Pointer: {:p}", pointer2);

    // println!("pointer: {}", *pointer);
    // println!("pointer: {}", *pointer2);

    unsafe{
        println!("*pointer: {}", *pointer);
    }

    let bad_ptr = unsafe{
        println!("*bad_cast: {}", *(pointer as *mut u8));
        let mut ptr = pointer as u64;
        ptr += 1;
        ptr as *mut u8
    };
    println!("bad ptr: {:p}", bad_ptr);
    unsafe{
        println!("bad_ptr: {}", *bad_ptr);
    }
    // unsafe{
    //     println!("probably worse: {}", *((bad_ptr as u64 + 100)as *mut u8));
    //     println!("even worse: {}", *(0 as *mut u8));
    // }
}

fn main(){
    
    let num = 42u16;
    println!("num: {}", num);
    let brw = &num;
    let ptr = brw as *const u16;
    println!("ptr: {:p}", ptr);
    unsafe{
        println!("*ptr: {}", *ptr);
    }

    foo();

}