#include "print.h"

int mod(int x, int r) {
    while (x >= r) {
        x -= r;
    }
    return x;
}

void fizzbuzz(int i) {
    if(mod(i, 3) == 0) {
        print("Fizz");
    }
    if(mod(i, 5) == 0) {
        print("Buzz");
    }
    print_char('\n');
}

int main() {
    for (int i=0; i<15; i++) {
        fizzbuzz(i);
    }
}