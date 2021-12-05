#include "point.h"

int manhattan( Point* point){
    return point->x + point->y;
}

void invert( Point* point){
    point->x *= -1;
    point->y *= -1;
}