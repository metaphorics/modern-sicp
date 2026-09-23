// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.34

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.34: suppose we define `fun f(g: (Long) -> Long): Long =
 * g(2L)`. Then `f(::square)` is 4 and `f { z -> z * (z + 1L) }` is 6.
 * What happens if we ask the compiler to check `f(::f)`? Explain. The
 * statement lives in the section 1.3 chapter text.
 *
 * SICP's own `(f f)` fails at run time, printing "The object 2 is not
 * applicable." Kotlin catches the same mistake earlier, at compile
 * time: `f` needs a `(Long) -> Long`, and `f` itself has type
 * `((Long) -> Long) -> Long`, so `f(::f)` does not type-check and the
 * build never runs at all. The scaffold returns `f(square)` and
 * `f { z -> z * (z + 1) }`, the two calls that do compile.
 */
public fun ex_1_34(): Pair<Long, Long> = throw PendingSolution()
