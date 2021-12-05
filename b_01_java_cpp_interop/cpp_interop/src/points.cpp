#include "cpp_interop/include/points.h"

// Header from cxx crate
#include "cpp_interop/src/main.rs.h"

class Board::store {
  friend Board;
  std::vector<Point2D> points;
};

Board::Board() : store(new class Board::store) {}

std::unique_ptr<Board> new_board() {
  return std::unique_ptr<Board>(new Board());
}

uint32_t Board::manhattan(const Point2D &point) const{
  return point.x + point.y;
}

void Board::add(const Point2D &point) const {
  store->points.push_back(point);
}

uint32_t Board::total_manhattan() const{
  uint32_t sum = 0;
  for(
    std::vector<Point2D>::iterator it = std::begin(store->points); 
    it != std::end(store->points); 
    ++it
    ) {
      sum += manhattan(*it);
  }
  return sum;
}
