#include <iostream>

int add(int a, int b) {
    return a + b;
}

float add(float a, float b) {
    return a + b;
}

std::string add(std::string a, std::string b) {
    return a + b;
}

int sum(int c, int d, int e) {
    return c + d + e;
}

void print() {
    std::cout << "Hello, World!" << std::endl;
}

int main() {
    std::cout << add(1, 2) << std::endl;
    return 0;
}
