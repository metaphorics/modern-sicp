// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import arrow.core.raise.Raise

/**
 * The error set of exercise 0.4: blank input, or input with a character
 * outside `0..9`.
 */
public sealed interface ParseError {
    public data object Blank : ParseError

    public data class NotDigits(
        val input: String,
    ) : ParseError
}

/** The throwing original, kept for contrast. */
public fun parseAmountEager(s: String): Long {
    val t = s.trim()
    if (t.isEmpty()) throw IllegalArgumentException("empty amount")
    if (t.any { c -> c !in '0'..'9' }) throw IllegalArgumentException("not digits: $s")
    return t.toLong()
}

/**
 * Same behavior as [parseAmountEager], raised as values: `r.raise` aborts
 * the surrounding `either { }` with the error, and no exception crosses
 * library code.
 */
context(r: Raise<ParseError>)
public fun parseAmount(s: String): Long {
    val t = s.trim()
    if (t.isEmpty()) r.raise(ParseError.Blank)
    if (t.any { c -> c !in '0'..'9' }) r.raise(ParseError.NotDigits(s))
    return t.toLong()
}
