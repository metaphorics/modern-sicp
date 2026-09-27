// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.33

package sicp.ch1.exercises

public fun filteredAccumulate(
    combiner: (Long, Long) -> Long,
    nullValue: Long,
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
    filter: (Long) -> Boolean,
): Long =
    when {
        a > b -> nullValue
        filter(a) -> combiner(term(a), filteredAccumulate(combiner, nullValue, term, next(a), next, b, filter))
        else -> filteredAccumulate(combiner, nullValue, term, next(a), next, b, filter)
    }

// isPrime is exercise 1.22's, reused here from the same package.
private fun square(x: Long): Long = x * x

private fun inc(n: Long): Long = n + 1L

/** (a): the sum of the squares of the primes in `[a, b]`. */
public fun sumSquaresOfPrimes(
    a: Long,
    b: Long,
): Long = filteredAccumulate({ x, y -> x + y }, 0L, ::square, a, ::inc, b, ::isPrime)

private fun identity(x: Long): Long = x

private tailrec fun gcd(
    a: Long,
    b: Long,
): Long = if (b == 0L) a else gcd(b, a % b)

/** (b): the product of the positive integers below `n` relatively prime to `n`. */
public fun productOfRelativePrimes(n: Long): Long =
    filteredAccumulate({ x, y -> x * y }, 1L, ::identity, 1L, ::inc, n - 1L) { i -> gcd(i, n) == 1L }

public fun ex_1_33(): Pair<Long, Long> = Pair(sumSquaresOfPrimes(2L, 20L), productOfRelativePrimes(10L))
