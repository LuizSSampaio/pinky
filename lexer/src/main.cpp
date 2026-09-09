#include <iostream>

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <data>" << '\n';
        return 1;
    }
    return 0;
}
