#include "print.h"

int fib(int n) {
    if(n <= 1) {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

int main() {
    int x = fib(10);
    print_int(x);
}