#define CPPMAX(x, y) ((x) + (y) < 5 ? y : x)

int main() {
    int x = 1, y = 2;
    CPPMAX(x, y);
}
