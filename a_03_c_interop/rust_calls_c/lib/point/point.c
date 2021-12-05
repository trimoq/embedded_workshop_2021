
typedef struct Point {
    int x;
    int y;
} Point;

void invert( Point* point){
    point->x *= -1;
    point->y *= -1;
}