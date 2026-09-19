// Derived from caddyserver/tree-sitter-caddyfile, MIT, Matthew Penner (2025).
// RedVim: combine end/body scanning so failed delimiter probes never discard
// lookahead; require an exact token boundary and reset deserialized state.
#include "tree_sitter/parser.h"
#include <stdlib.h>
#include <string.h>

enum TokenType { HEREDOC_START, HEREDOC_BODY, HEREDOC_END };
typedef struct {
    char delimiter[32];
    unsigned delimiter_len;
    bool has_heredoc;
} Scanner;

static void advance(TSLexer *lexer) { lexer->advance(lexer, false); }
static bool marker_char(int32_t c) {
    return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') ||
           (c >= '0' && c <= '9') || c == '_';
}
static bool boundary(TSLexer *lexer) {
    return lexer->eof(lexer) || lexer->lookahead == ' ' ||
           lexer->lookahead == '\t' || lexer->lookahead == '\n' ||
           lexer->lookahead == '\r';
}

static bool scan_start(Scanner *scanner, TSLexer *lexer) {
    char delimiter[32] = {0};
    unsigned length = 0;
    while (marker_char(lexer->lookahead) && length < 31) {
        delimiter[length++] = lexer->lookahead;
        advance(lexer);
    }
    if (!length || (lexer->lookahead != '\n' && lexer->lookahead != '\r'))
        return false;
    memcpy(scanner->delimiter, delimiter, length);
    scanner->delimiter_len = length;
    scanner->has_heredoc = true;
    lexer->mark_end(lexer);
    lexer->result_symbol = HEREDOC_START;
    return true;
}

static bool scan_content(Scanner *scanner, TSLexer *lexer, const bool *valid) {
    if (!scanner->has_heredoc || lexer->eof(lexer)) return false;
    bool consumed = false;
    if (lexer->get_column(lexer) == 0) {
        while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
            advance(lexer);
            consumed = true;
        }
        unsigned matched = 0;
        while (matched < scanner->delimiter_len &&
               lexer->lookahead == scanner->delimiter[matched]) {
            advance(lexer);
            consumed = true;
            matched++;
        }
        if (matched == scanner->delimiter_len && boundary(lexer)) {
            if (!valid[HEREDOC_END]) return false;
            lexer->mark_end(lexer);
            lexer->result_symbol = HEREDOC_END;
            scanner->has_heredoc = false;
            return true;
        }
        // Partial/prefix matches remain body text. The consumed indentation and
        // prefix bytes still belong to this token; never restart another scan.
    }
    if (!valid[HEREDOC_BODY]) return false;
    while (!lexer->eof(lexer)) {
        int32_t c = lexer->lookahead;
        advance(lexer);
        consumed = true;
        if (c == '\n') break;
    }
    if (!consumed) return false;
    lexer->mark_end(lexer);
    lexer->result_symbol = HEREDOC_BODY;
    return true;
}

void *tree_sitter_caddyfile_external_scanner_create(void) {
    return calloc(1, sizeof(Scanner));
}
void tree_sitter_caddyfile_external_scanner_destroy(void *payload) {
    free(payload);
}
bool tree_sitter_caddyfile_external_scanner_scan(void *payload, TSLexer *lexer,
                                               const bool *valid) {
    Scanner *scanner = payload;
    if (scanner->has_heredoc && (valid[HEREDOC_END] || valid[HEREDOC_BODY]))
        return scan_content(scanner, lexer, valid);
    if (valid[HEREDOC_START]) return scan_start(scanner, lexer);
    return false;
}
unsigned tree_sitter_caddyfile_external_scanner_serialize(void *payload, char *buffer) {
    Scanner *scanner = payload;
    buffer[0] = scanner->has_heredoc;
    if (!scanner->has_heredoc) return 1;
    memcpy(buffer + 1, scanner->delimiter, scanner->delimiter_len);
    return 1 + scanner->delimiter_len;
}
void tree_sitter_caddyfile_external_scanner_deserialize(void *payload,
                                                      const char *buffer,
                                                      unsigned length) {
    Scanner *scanner = payload;
    memset(scanner, 0, sizeof(*scanner));
    if (length > 1 && length <= sizeof(scanner->delimiter) && buffer[0] == 1) {
        scanner->has_heredoc = true;
        scanner->delimiter_len = length - 1;
        memcpy(scanner->delimiter, buffer + 1, scanner->delimiter_len);
    }
}
