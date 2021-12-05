use std::time::SystemTime;

pub fn print_std_time() {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("std nanos: {}", n.as_nanos()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };  
}
