#include <iostream>
#include <cstdarg>

void print_values(int n, ...) {
    va_list vl;
    va_start(vl, n);
    for (int i = 0; i < n; i++) {
        int value = va_arg(vl, int);
        std::cout << "Value: " << value << "\n";
    }
    va_end(vl);
}
int main() {
    print_values(3, 10, 20, 30);
    return 0;
}
