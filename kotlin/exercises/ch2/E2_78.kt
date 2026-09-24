// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.78

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.78: the book's ordinary numbers carry a `scheme-number` tag
 * because Lisp data is untyped pairs; exercise 2.78 rewrites `type-tag`,
 * `contents`, and `attach-tag` so ordinary numbers ride bare. This
 * edition's tower went further from the start -- a [Num] presents its
 * level as its own type, so there was never a wrapper to strip. What the
 * exercise still teaches here is the bare case: [typeTagOfAny] reads the
 * level off the host's own `Long` and `Double` too, and the bare integer
 * package computes on `Long` values directly, returning bare results.
 */
public fun ex_2_78(): Boolean = throw PendingSolution()
