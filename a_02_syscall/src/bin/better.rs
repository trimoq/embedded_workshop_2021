use a_02_syscall::print_std_time;


static NANOS_IN_SEC : u128= 1_000_000_000;

fn main() {
    print_std_time();  

    match my_get_time(){
        Ok(nanos) => println!("sys nanos: {}",nanos),
        Err(e) => panic!("Failed to read time: {}", e),
    }  
}
pub fn my_get_time() -> std::io::Result<u128> {
    let mut t =  libc::timespec { tv_sec: 0, tv_nsec: 0 } ;
    let res = unsafe { 
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut t) 
    };
    if res != 0 { 
        Err(std::io::Error::last_os_error()) 
    } else { 
        Ok( t.tv_sec as u128 * NANOS_IN_SEC + t.tv_nsec as u128) 
    }
}
