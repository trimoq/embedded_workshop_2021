#pragma once
#include <memory>
#include <vector>


struct Point2D;

class Board {
  public:
    Board();
    uint32_t manhattan( const Point2D &point) const;
    void add(const Point2D &point) const;
    uint32_t total_manhattan() const;
  private:
    class store;
    std::shared_ptr<store> store;
};

std::unique_ptr<Board> new_board();
