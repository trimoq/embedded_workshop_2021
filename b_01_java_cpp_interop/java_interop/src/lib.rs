use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

static NANOS_IN_SEC : u128= 1_000_000_000;

#[no_mangle]
pub extern "system" fn Java_Clock_myGetTime(
    env: JNIEnv,
    _class: JClass,
    reason: JString) 
    -> jstring{

    let reason : String = env.get_string(reason).unwrap().into();
    println!("Getting time for reason: {}", reason);

    let s = match my_get_time(){
        Ok(nanos) => format!("Rust: {}",nanos/1_000_000),
        Err(e) => format!("Failed to read time: {}", e),
    };

    env.new_string(s).unwrap().into_inner()
}

fn my_get_time() -> std::io::Result<u128> {
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
