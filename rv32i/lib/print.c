#include "print.h"

#define MMIO_PUTCHAR 0x20000000
#define MMIO_PUTHEX 0x20000004
#define MMIO_PUTINT 0x20000008

void print_char(char c) {
    *(volatile char *)MMIO_PUTCHAR = c;
}

void print_hex(int x) {
    *(volatile int *)MMIO_PUTHEX = x;
}

void print_int(int x) {
    *(volatile int *)MMIO_PUTINT = x;
}

void print(const char* s) {
    while(*s != '\0') {
        print_char(*s);
        s++;
    }
}