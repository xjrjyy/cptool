#include <cctype>
#include <iostream>
#include <iterator>
#include <string>

constexpr int V = 1e9;

int main() {
    std::cin >> std::noskipws;

    std::istream_iterator<char> it(std::cin), end;
    std::string s(it, end);

    auto cur = s.begin();
    auto peekChar = [&]() -> char {
        if (cur == s.end()) {
            std::exit(1);
        }
        return *cur;
    };
    auto consumeChar = [&]() {
        ++cur;
    };

    auto readInt = [&](int low, int high) {
        char c = peekChar();
        if (!std::isdigit(c)) {
            std::exit(1);
        }
        int result = 0;
        while (std::isdigit(c)) {
            consumeChar();
            long long newResult = 1ll * result * 10 + c - '0';
            if (newResult < low || newResult > high) {
                std::exit(1);
            }
            result = newResult;
            c = peekChar();
        }
        return result;
    };

    int a = readInt(1, V);
    if (peekChar() != ' ') {
        std::exit(1);
    }
    consumeChar();
    int b = readInt(1, V);
    if (peekChar() != '\n') {
        std::exit(1);
    }
    consumeChar();
    if (cur != s.end()) {
        std::exit(1);
    }

    return 0;
}