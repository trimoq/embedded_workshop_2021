
use generated_sys::{Point, manhattan, invert};
fn main() {
    let mut point = Point{
        x: 3,
        y: 2
    };

    let dist_m = unsafe{manhattan(&mut point as *mut Point)};
    dbg!(dist_m);

    dbg!(point);
    unsafe{invert(&mut point as *mut Point)};
    dbg!(point);

}
