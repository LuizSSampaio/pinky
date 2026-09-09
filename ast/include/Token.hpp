#pragma once

#include <cstddef>
#include <cstdint>
#include <string>

namespace ast {
enum class TokenKind : uint8_t { EndOfFile };

class Token {
public:
    struct Position {
        size_t row, col;

        Position(size_t row, size_t col) : row(row), col(col) {}
    };

    Token(TokenKind kind, std::string lexeme, Position pos);
    Token(TokenKind kind, std::string lexeme, size_t row, size_t col);

    TokenKind kind;
    std::string lexeme;
    Position pos;
};
}  // namespace ast
