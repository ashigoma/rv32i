#include "print.h"

int mod(int x, int r) {
    while (x >= r) {
        x -= r;
    }
    return x;
}

void fizzbuzz(int i) {
    bool is_fizz = mod(i, 3) == 0;
    bool is_buzz = mod(i, 5) == 0;
    if(is_fizz) {
        print("Fizz");
    }
    if(is_buzz) {
        print("Buzz");
    }
    if (!is_fizz && !is_buzz) {
        print_int(i);
    }
    print_char('\n');
}

int main() {
    for (int i=0; i<15; i++) {
        fizzbuzz(i);
    }
}