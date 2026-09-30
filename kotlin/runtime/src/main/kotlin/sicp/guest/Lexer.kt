// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.raise.Raise

/** One lexical token: the closed vocabulary of grammar section 2. */
public sealed class Token(
    public val span: Span,
) {
    public class Ident(
        public val text: String,
        span: Span,
    ) : Token(span)

    /** A numeric literal, sign and underscores kept as written; [suffix] is
     * `L` for a Long literal and empty otherwise. */
    public class Number(
        public val text: String,
        public val suffix: String,
        public val floating: Boolean,
        span: Span,
    ) : Token(span)

    public class Bool(
        public val value: Boolean,
        span: Span,
    ) : Token(span)

    /** A string literal; templates arrive as [Fragment]s. */
    public class Str(
        public val fragments: List<Fragment>,
        span: Span,
    ) : Token(span) {
        public sealed interface Fragment {
            public class Text(
                public val text: String,
            ) : Fragment

            public class Name(
                public val name: String,
            ) : Fragment

            public class Embedded(
                public val tokens: List<Token>,
            ) : Fragment
        }
    }

    public class Keyword(
        public val text: String,
        span: Span,
    ) : Token(span)

    public class Symbol(
        public val text: String,
        span: Span,
    ) : Token(span)

    public class Eof(
        span: Span,
    ) : Token(span)
}

private val KEYWORDS: Set<String> =
    setOf(
        "fun",
        "val",
        "var",
        "if",
        "else",
        "when",
        "is",
        "in",
        "return",
        "break",
        "continue",
        "while",
        "for",
        "data",
        "class",
        "sealed",
        "interface",
        "object",
        "typealias",
        "this",
        "null",
        "true",
        "false",
        "tailrec",
        "public",
    )

/** Longest-match-first: `===` must precede `==`, `!!` must precede `!`. */
private val SYMBOLS: List<String> =
    listOf(
        "?:",
        "?.",
        "->",
        "<=",
        ">=",
        "===",
        "!==",
        "==",
        "!=",
        "&&",
        "||",
        "!!",
        "::",
        "..",
        "+=",
        "-=",
        "*=",
        "/=",
        "%=",
        "@",
        "+",
        "-",
        "*",
        "/",
        "%",
        "<",
        ">",
        "!",
        "?",
        ".",
        ",",
        "(",
        ")",
        "[",
        "]",
        "{",
        "}",
        ":",
        ";",
        "=",
    )

/** Scans admitted guest source into tokens, preserving source locations.
 * Unadmitted surface forms fail here with their admission class, before any
 * parser or evaluator sees the text. */
public class Lexer(
    private val source: String,
) {
    private var pos: Int = 0
    private var line: Int = 1
    private var column: Int = 1

    /** The token stream for [source], `Eof` included. */
    context(r: Raise<AdmissionError>)
    public fun tokens(): List<Token> {
        val out = mutableListOf<Token>()
        while (true) {
            skipIgnorable()
            if (atEnd()) {
                out.add(Token.Eof(Span(here(), here())))
                return out
            }
            out.add(scanToken(out.lastOrNull()))
        }
    }

    private fun atEnd(): Boolean = pos >= source.length

    private fun here(): Place = Place(pos, line, column)

    private fun peek(k: Int = 0): Char {
        val i = pos + k
        return if (i < source.length) source[i] else ' '
    }

    private fun advance(): Char {
        val c = source[pos++]
        if (c == '\n') {
            line++
            column = 1
        } else {
            column++
        }
        return c
    }

    private fun spanFrom(start: Place): Span = Span(start, here())

    context(r: Raise<AdmissionError>)
    private fun failUnsupported(
        category: String,
        start: Place,
        what: String,
    ): Nothing = r.raise(AdmissionError.Unsupported(category, spanFrom(start), what))

    context(r: Raise<AdmissionError>)
    private fun failInvalid(
        category: String,
        start: Place,
        what: String,
    ): Nothing = r.raise(AdmissionError.HostInvalid(category, spanFrom(start), what))

    context(r: Raise<AdmissionError>)
    private fun skipIgnorable() {
        while (!atEnd()) {
            val c = peek()
            if (c.isWhitespace()) {
                advance()
                continue
            }
            if (c == '/' && peek(1) == '/') {
                while (!atEnd() && peek() != '\n') advance()
                continue
            }
            if (c == '/' && peek(1) == '*') {
                val start = here()
                advance()
                advance()
                while (!atEnd() && !(peek() == '*' && peek(1) == '/')) advance()
                if (atEnd()) failInvalid("Comment", start, "unterminated block comment")
                advance()
                advance()
                continue
            }
            return
        }
    }

    context(r: Raise<AdmissionError>)
    private fun scanToken(previous: Token?): Token {
        val start = here()
        val c = peek()
        if (c == '"') return scanString(start)
        if (c == '\'') failUnsupported("CharSurface", start, "character literal")
        if (c == '`') failUnsupported("MiscKotlin", start, "backtick identifier")
        if (c.isDigit()) return scanNumber(start)
        if (c == '-' && peek(1).isDigit() && signStartsLiteral(previous)) {
            advance()
            return scanNumber(start)
        }
        if (c.isLetter() || c == '_') return scanWord(start)
        val symbol = SYMBOLS.firstOrNull { source.startsWith(it, pos) }
        if (symbol != null) {
            repeat(symbol.length) { advance() }
            return Token.Symbol(symbol, spanFrom(start))
        }
        return failInvalid("BadToken", start, "unrecognized character: $c")
    }

    /** A `-` directly before a digit belongs to the literal only where a
     * unary minus may start an expression; `a-1` stays subtraction. */
    private fun signStartsLiteral(previous: Token?): Boolean =
        when (previous) {
            is Token.Eof, is Token.Keyword, is Token.Symbol -> true
            else -> false
        }

    private fun scanWord(start: Place): Token {
        while (peek().isLetterOrDigit() || peek() == '_') advance()
        val text = source.substring(start.offset, pos)
        val span = spanFrom(start)
        return when {
            text == "true" -> Token.Bool(true, span)
            text == "false" -> Token.Bool(false, span)
            text in KEYWORDS -> Token.Keyword(text, span)
            else -> Token.Ident(text, span)
        }
    }

    context(r: Raise<AdmissionError>)
    private fun scanNumber(start: Place): Token {
        var floating = false
        scanDigits()
        // A point belongs to the literal only before its fraction; `1..3`
        // leaves the range operator to the symbol scanner.
        if (peek() == '.' && peek(1) != '.' && peek(1).isDigit()) {
            floating = true
            advance()
            scanDigits()
        }
        if (peek() == 'e' || peek() == 'E') {
            floating = true
            advance()
            if (peek() == '-' || peek() == '+') advance()
            if (!peek().isDigit()) failInvalid("Literal", start, "malformed exponent")
            scanDigits()
        }
        var suffix = ""
        if (peek() == 'L' && !floating) {
            suffix = "L"
            advance()
        } else if (peek() == 'f' || peek() == 'F') {
            failUnsupported("MiscKotlin", start, "Float literal")
        } else if (peek() == 'u' || peek() == 'U') {
            failUnsupported("MiscKotlin", start, "unsigned literal")
        }
        if (peek().isLetter()) failInvalid("Literal", start, "malformed literal")
        val text = source.substring(start.offset, pos).replace("_", "")
        return Token.Number(text, suffix, floating, spanFrom(start))
    }

    context(r: Raise<AdmissionError>)
    private fun scanDigits() {
        if (!peek().isDigit()) failInvalid("Literal", here(), "expected a digit")
        while (peek().isDigit() || peek() == '_') advance()
    }

    context(r: Raise<AdmissionError>)
    private fun scanString(start: Place): Token {
        advance() // opening quote
        val fragments = mutableListOf<Token.Str.Fragment>()
        val text = StringBuilder()
        while (true) {
            if (atEnd()) failInvalid("Literal", start, "unterminated string")
            val c = advance()
            if (c == '"') {
                if (text.isNotEmpty()) fragments.add(Token.Str.Fragment.Text(text.toString()))
                return Token.Str(fragments, spanFrom(start))
            }
            if (c == '\\') {
                text.append(scanEscape(start))
                continue
            }
            if (c == '$') {
                if (text.isNotEmpty()) {
                    fragments.add(Token.Str.Fragment.Text(text.toString()))
                    text.clear()
                }
                fragments.add(scanTemplate(start))
                continue
            }
            text.append(c)
        }
    }

    context(r: Raise<AdmissionError>)
    private fun scanEscape(start: Place): Char {
        if (atEnd()) failInvalid("Literal", start, "unterminated escape")
        return when (val e = advance()) {
            'n' -> '\n'
            't' -> '\t'
            'r' -> '\r'
            '\\' -> '\\'
            '"' -> '"'
            '$' -> '$'
            else -> failInvalid("Literal", start, "unsupported string escape: \\$e")
        }
    }

    context(r: Raise<AdmissionError>)
    private fun scanTemplate(start: Place): Token.Str.Fragment {
        if (peek() == '{') {
            advance()
            return Token.Str.Fragment.Embedded(scanEmbedded(start))
        }
        if (peek().isLetter() || peek() == '_') {
            val nameStart = here()
            while (peek().isLetterOrDigit() || peek() == '_') advance()
            return Token.Str.Fragment.Name(source.substring(nameStart.offset, pos))
        }
        return failInvalid("Literal", start, "malformed string template")
    }

    /** Scans the tokens of a `${ ... }` template until its closing brace. */
    context(r: Raise<AdmissionError>)
    private fun scanEmbedded(start: Place): List<Token> {
        val inner = Lexer(source)
        inner.pos = pos
        inner.line = line
        inner.column = column
        val tokens = mutableListOf<Token>()
        var depth = 1
        while (true) {
            inner.skipIgnorable()
            if (inner.atEnd()) failInvalid("Literal", start, "unterminated template")
            if (depth == 0) {
                pos = inner.pos
                line = inner.line
                column = inner.column
                return tokens
            }
            val t = inner.scanToken(tokens.lastOrNull())
            if (t is Token.Symbol && t.text == "{") depth++
            if (t is Token.Symbol && t.text == "}") {
                depth--
                if (depth == 0) continue
            }
            tokens.add(t)
        }
    }
}
