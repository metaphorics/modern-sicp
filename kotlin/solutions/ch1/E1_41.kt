// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.41

package sicp.ch1.exercises

/** Generic so it can be applied to a numeric function or, self-applied, to itself. */
public fun <T> double(f: (T) -> T): (T) -> T = { x -> f(f(x)) }

private fun inc(n: Long): Long = n + 1L

/**
 * `double(double)` needs `double` instantiated twice, at two different types: once at
 * `T = Long` for the innermost `double`, and once at `T = (Long) -> Long` for the `double`
 * that takes that innermost one as its argument. Kotlin's `T` is not self-referential, so
 * this is two ordinary, unrelated instantiations of one generic function, not a paradox:
 * `double<Long>` has type `((Long) -> Long) -> (Long) -> Long`, exactly the `((Long) -> Long)`
 * that `double<(Long) -> Long>` expects.
 */
private val doubleLong: ((Long) -> Long) -> (Long) -> Long = { f -> double(f) }

public fun ex_1_41(): Long {
    val quadrupled = double<(Long) -> Long>(doubleLong) // double(double)
    val sixteenFold = double<(Long) -> Long>(quadrupled) // double(double(double))
    return sixteenFold(::inc)(5L)
}
