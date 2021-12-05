use a_02_syscall::{print_std_time};

fn main() {
    print_std_time();
    let mut t =  libc::timespec { tv_sec: 0, tv_nsec: 0 } ;
    unsafe { libc::clock_gettime(99, &mut t) };   
    println!("wrong: {} {}", t.tv_sec, t.tv_nsec);
}