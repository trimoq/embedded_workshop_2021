use std::os::raw::c_int;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

#[link(name = "point")]
extern "C" {
    pub fn invert(point: *mut Point);
}


fn main() {
    let mut point = Point{
        x: 3,
        y: 4
    };

    dbg!(point);
    unsafe{invert(&mut point as *mut Point)};
    dbg!(point);

}
