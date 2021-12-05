#[cxx::bridge]
mod ffi {

    // Struct shared between both languages
    #[derive(Debug)]
    struct Point2D{
        x: u32,
        y: u32
    }

    unsafe extern "C++" {
        include!("cpp_interop/include/points.h");

        type Board;
        fn manhattan(&self, point: &Point2D) -> u32 ;
        fn add(&self, point: &Point2D) ;
        fn new_board() -> UniquePtr<Board>;
        fn total_manhattan(&self) -> u32 ;
    }
}

fn main() {
    let board = ffi::new_board();

    for i in 0..5{

        let point = ffi::Point2D{
            x: i,
            y: 2*i
        };
    
        println!("point: {:#?}", &point);
        println!("manhattan: {:#?}", board.manhattan(&point));

        board.add(&point);
    }

    let res = board.total_manhattan();
    println!("total_manhattan: {}", res);

}
