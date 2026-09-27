// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.58

package sicp.ch2.exercises

/** One parse position over a token list, advanced in place as parsing proceeds. */
private class Tokens(
    private val tokens: List<String>,
) {
    var pos: Int = 0

    fun peek(): String? = tokens.getOrNull(pos)

    fun next(): String = tokens[pos++]

    fun expect(token: String) {
        require(next() == token) { "expected '$token'" }
    }
}

private fun tokenize(s: String): List<String> {
    val out = mutableListOf<String>()
    var i = 0
    while (i < s.length) {
        val c = s[i]
        when {
            c.isWhitespace() -> {
                i++
            }

            c == '(' || c == ')' || c == '+' || c == '*' -> {
                out.add(c.toString())
                i++
            }

            else -> {
                val start = i
                while (i < s.length && !s[i].isWhitespace() && s[i] !in "()+*") i++
                out.add(s.substring(start, i))
            }
        }
    }
    return out
}

private fun atom(token: String): Expr = token.toLongOrNull()?.let { Expr.Num(it) } ?: Expr.Var(token)

/** Part a): every expression is `NUM`, `VAR`, `(a + b)`, or `(a * b)`, fully parenthesized. */
private fun parseParenthesized(tokens: Tokens): Expr {
    val token = tokens.next()
    if (token != "(") return atom(token)
    val left = parseParenthesized(tokens)
    val op = tokens.next()
    val right = parseParenthesized(tokens)
    tokens.expect(")")
    return if (op == "+") Expr.Sum(left, right) else Expr.Product(left, right)
}

/** Part a): parses a fully-parenthesized infix expression into this section's [Expr]. */
public fun parseFullyParenthesized(s: String): Expr = parseParenthesized(Tokens(tokenize(s)))

/** Part b): `*` binds tighter than `+`; a sum is one or more terms joined by `+`. */
private fun parseSum(tokens: Tokens): Expr {
    var left = parseProduct(tokens)
    while (tokens.peek() == "+") {
        tokens.next()
        left = Expr.Sum(left, parseProduct(tokens))
    }
    return left
}

/** Part b): a product is one or more factors joined by `*`. */
private fun parseProduct(tokens: Tokens): Expr {
    var left = parseFactor(tokens)
    while (tokens.peek() == "*") {
        tokens.next()
        left = Expr.Product(left, parseFactor(tokens))
    }
    return left
}

/** Part b): a factor is a parenthesized sum, a number, or a variable. */
private fun parseFactor(tokens: Tokens): Expr {
    val token = tokens.next()
    if (token != "(") return atom(token)
    val inner = parseSum(tokens)
    tokens.expect(")")
    return inner
}

/** Part b): parses an infix expression with standard `*`-over-`+` precedence into this section's [Expr]. */
public fun parseWithPrecedence(s: String): Expr = parseSum(Tokens(tokenize(s)))

/** The derivative of `"x + 3 * (x + y + 2)"` w.r.t. `x`, parsed with part b)'s precedence-aware parser. */
public fun ex_2_58(): String = printExpr(deriv(parseWithPrecedence("x + 3 * (x + y + 2)"), "x"))
