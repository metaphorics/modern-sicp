// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import arrow.core.raise.Raise
import sicp.runtime.PendingSolution

/**
 * The error set of exercise 0.4: blank input, or input with a character
 * outside `0..9`.
 *
 * Section 0.6.
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
 * Exercise 0.4: keep [parseAmountEager]'s behavior but raise instead of
 * throw. Trim spaces; [ParseError.Blank] when nothing is left,
 * [ParseError.NotDigits] when any character is outside `0..9`. Test both
 * branches through `either { }`. The statement lives in the section 0.6
 * chapter text.
 */
context(r: Raise<ParseError>)
public fun parseAmount(s: String): Long = throw PendingSolution()
