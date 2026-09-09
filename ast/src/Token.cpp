#include "Token.hpp"

#include <ostream>

using namespace ast;

namespace {
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
}  // namespace
