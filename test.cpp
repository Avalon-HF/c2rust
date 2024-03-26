#include <iostream>

const int GLOBAL_A = 5;

int GLBOAL_B = 10;

// int add func
int add(int a, int b) {
    // comment before return
    return a + b;
}

// float add func
float add(float a, float b) {
    return a + b; // comment right after return
}

// string add function
std::string add(std::string a, std::string b) {
    return a + b;
}

int sum(int c, int d, int e) {
    return c + d + e;
}

void print() {
    std::cout << "Hello, World!" << std::endl;
}

/*
 main function
 calls some other functions
*/
int main() {
    std::cout << add(1, 2) << std::endl;
    return 0;
}
