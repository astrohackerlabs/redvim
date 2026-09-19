// café: generics
#include <string>
template<typename T> T twice(T value) { return value + value; }
int main() {
    std::string message = "hello";
    return twice(21);
}
