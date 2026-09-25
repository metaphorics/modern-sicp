// SPDX-License-Identifier: GPL-3.0-only
// The S-expression reader of section 4.1 (the given-code `read` the book's
// driver loop assumes), porting the shared grammar of
// spec/scheme-subset/grammar.md. Given code per D23: the text does not
// derive it; it is presented the way SICP JS presents `parse`, so the
// conformance corpus in `tests` runs through it unchanged.

package sicp.ch4

import arrow.core.raise.Raise
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VReal
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons

/**
 * The whole text as a sequence of data: one datum per top-level form. The
 * reader accepts exactly the grammar's surface: integers (checked `Long`),
 * floats (IEEE 754), `#t` and `#f`, double-quoted strings with `\"` and
 * `\\` escapes, symbols, `'` quote sugar, proper and dotted lists, and `;`
 * comments that run to end of line.
 */
context(r: Raise<SchemeError>)
public fun readProgram(text: String): List<Value> {
    val scanner = Scanner(text)
    val forms = mutableListOf<Value>()
    while (true) {
        scanner.skipAtmosphere()
        if (scanner.atEnd()) return forms
        forms.add(readDatum(scanner))
    }
}

/** One datum from `text`; a trailing form or an empty text is a parse error. */
context(r: Raise<SchemeError>)
public fun readDatum(text: String): Value {
    val scanner = Scanner(text)
    scanner.skipAtmosphere()
    if (scanner.atEnd()) r.raise(SchemeError.Parse("empty input"))
    return readDatum(scanner)
}

/** One datum from wherever `scanner` stands. */
context(r: Raise<SchemeError>)
private fun readDatum(scanner: Scanner): Value {
    scanner.skipAtmosphere()
    if (scanner.atEnd()) r.raise(SchemeError.Parse("unexpected end of input"))
    return when (scanner.peek()) {
        '(' -> {
            readList(scanner)
        }

        '"' -> {
            readString(scanner)
        }

        '\'' -> {
            scanner.advance()
            cons(VSym("quote"), cons(readDatum(scanner), VNil))
        }

        ')' -> {
            r.raise(SchemeError.Parse("unexpected )"))
        }

        else -> {
            readAtom(scanner)
        }
    }
}

/** A list: `(datum ...)` proper, or `(datum ... . datum)` dotted. */
context(r: Raise<SchemeError>)
private fun readList(scanner: Scanner): Value {
    scanner.advance() // the opening paren
    val items = mutableListOf<Value>()
    var tail: Value = VNil
    while (true) {
        scanner.skipAtmosphere()
        if (scanner.atEnd()) r.raise(SchemeError.Parse("unterminated list"))
        when (scanner.peek()) {
            ')' -> {
                scanner.advance()
                return items.foldRight(tail) { v, acc -> cons(v, acc) }
            }

            else -> {
                val datum = readDatum(scanner)
                if (datum == DOT) {
                    if (items.isEmpty()) r.raise(SchemeError.Parse("misplaced . in list"))
                    tail = readDatum(scanner)
                    scanner.skipAtmosphere()
                    if (scanner.atEnd() || scanner.peek() != ')') {
                        r.raise(SchemeError.Parse("bad dotted list"))
                    }
                    scanner.advance()
                    return items.foldRight(tail) { v, acc -> cons(v, acc) }
                }
                items.add(datum)
            }
        }
    }
}

/** The datum marker a list reads a `.` token as; never escapes [readList]. */
private val DOT: Value = VSym(".")

/** A double-quoted string; `\"` and `\\` are the only recognized escapes. */
context(r: Raise<SchemeError>)
private fun readString(scanner: Scanner): Value {
    scanner.advance() // the opening quote
    val out = StringBuilder()
    while (true) {
        if (scanner.atEnd()) r.raise(SchemeError.Parse("unterminated string"))
        when (val c = scanner.advance()) {
            '"' -> {
                return VStr(out.toString())
            }

            '\\' -> {
                if (scanner.atEnd()) r.raise(SchemeError.Parse("unterminated string escape"))
                when (val e = scanner.advance()) {
                    '"' -> out.append('"')
                    '\\' -> out.append('\\')
                    else -> r.raise(SchemeError.Parse("bad string escape: \\$e"))
                }
            }

            else -> {
                out.append(c)
            }
        }
    }
}

/** An atom: integer, float, boolean, or symbol, longest fit first. */
context(r: Raise<SchemeError>)
private fun readAtom(scanner: Scanner): Value {
    val start = scanner.pos
    while (!scanner.atEnd() && !scanner.atDelimiter()) scanner.advance()
    val token = scanner.text.substring(start, scanner.pos)
    if (token == ".") return DOT
    token.toLongOrNull()?.let { return VInt(it) }
    if (isFloatToken(token)) {
        val d = token.toDoubleOrNull()
        if (d != null && !d.isInfinite()) return VReal(d)
        r.raise(SchemeError.Parse("float out of range: $token"))
    }
    if (token == "#t") return VBool(true)
    if (token == "#f") return VBool(false)
    if (isSymbolToken(token)) return VSym(token)
    r.raise(SchemeError.Parse("bad token: $token"))
}

/** A float token: optional sign, digits, a point or an exponent (or both). */
private fun isFloatToken(token: String): Boolean {
    var i = 0
    val n = token.length
    if (i < n && (token[i] == '+' || token[i] == '-')) i++
    var digits = 0
    while (i < n && token[i].isDigit()) {
        i++
        digits++
    }
    var sawPoint = false
    if (i < n && token[i] == '.') {
        sawPoint = true
        i++
        while (i < n && token[i].isDigit()) {
            i++
            digits++
        }
    }
    var sawExponent = false
    if (i < n && (token[i] == 'e' || token[i] == 'E')) {
        sawExponent = true
        i++
        if (i < n && (token[i] == '+' || token[i] == '-')) i++
        var exponentDigits = 0
        while (i < n && token[i].isDigit()) {
            i++
            exponentDigits++
        }
        if (exponentDigits == 0) return false
    }
    if (i != n) return false
    return (sawPoint || sawExponent) && digits > 0
}

/** A symbol: the grammar's letters, digits, and `-?!*+</>=_.` characters. */
private fun isSymbolToken(token: String): Boolean = token.isNotEmpty() && token.all { it.isLetterOrDigit() || it in "-?!*+</>=_." }

/** A cursor over the input text; the scanner holds no lookahead buffer. */
private class Scanner(
    val text: String,
) {
    var pos: Int = 0

    fun atEnd(): Boolean = pos >= text.length

    fun peek(): Char = text[pos]

    fun advance(): Char = text[pos++]

    /** Whitespace and `;` comments, per the grammar's atmosphere. */
    fun skipAtmosphere() {
        while (pos < text.length) {
            val c = text[pos]
            if (c.isWhitespace()) {
                pos++
            } else if (c == ';') {
                while (pos < text.length && text[pos] != '\n') pos++
            } else {
                return
            }
        }
    }

    /** Whether `pos` stands on a character that ends an atom token. */
    fun atDelimiter(): Boolean {
        val c = text[pos]
        return c.isWhitespace() || c == '(' || c == ')' || c == '"' || c == ';'
    }
}
