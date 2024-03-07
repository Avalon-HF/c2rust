#include <iostream>
#include <memory>
struct MyStruct {
    int value;
    MyStruct(int v) : value(v) {
        std::cout << "Constructor\n";
    }

    ~MyStruct() {
        std::cout << "Destructor\n";
    }
};
int main() {
    std::shared_ptr<MyStruct> ptr = std::make_shared<MyStruct>(10);
    std::weak_ptr<MyStruct> weak_ptr = ptr; // 创建弱指针

    if (auto shared_ptr = weak_ptr.lock()) { // 尝试获取共享指针
        shared_ptr->value = 20; // 修改数据
        std::cout << "Value: " << shared_ptr->value << "\n";
    }

    return 0;
}
