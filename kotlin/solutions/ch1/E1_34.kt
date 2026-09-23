// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.34

package sicp.ch1.exercises

private fun square(x: Long): Long = x * x

public fun f(g: (Long) -> Long): Long = g(2L)

// f(::f) is refused at compile time, not run time:
//
//   error: argument type mismatch: actual type is '(Long) -> Long', but 'Long' was expected
//
// f needs a (Long) -> Long, but f itself has type ((Long) -> Long) -> Long: passing f to
// itself asks the compiler to unify Long with (Long) -> Long, which it will not do. SICP's
// own (f f) instead runs, applies 2 as though it were a procedure, and fails at that point
// with "The object 2 is not applicable."

public fun ex_1_34(): Pair<Long, Long> = Pair(f(::square), f { z -> z * (z + 1L) })
