#include "Token.hpp"

#include <ostream>

namespace ast {
Token::Token(TokenKind kind, std::string lexeme, Position pos)
    : kind(kind), lexeme(std::move(lexeme)), pos(pos) {}

Token::Token(TokenKind kind, std::string lexeme, size_t row, size_t col)
    : kind(kind), lexeme(std::move(lexeme)), pos(row, col) {}

std::ostream& operator<<(std::ostream& strm, const TokenKind& kind) {
    std::string name;
    if (kind == TokenKind::EndOfFile) {
        name = "EOF";
    }

    return strm << name;
}

std::ostream& operator<<(std::ostream& strm, const Token& token) {
    return strm << "(" << token.kind << ", " << token.lexeme << ")";
}
}  // namespace ast
