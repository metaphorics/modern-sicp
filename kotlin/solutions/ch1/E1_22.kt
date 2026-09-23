// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.22

package sicp.ch1.exercises

private fun divides(
    a: Long,
    b: Long,
): Boolean = b % a == 0L

private tailrec fun findDivisor(
    n: Long,
    testDivisor: Long,
): Long =
    when {
        testDivisor * testDivisor > n -> n
        divides(testDivisor, n) -> testDivisor
        else -> findDivisor(n, testDivisor + 1L)
    }

public fun isPrime(n: Long): Boolean = n >= 2L && n == findDivisor(n, 2L)

/** The elapsed time is measured but never asserted on; wall-clock timing is not test-deterministic. */
public data class TimedPrimeResult(
    val n: Long,
    val elapsedNanos: Long,
)

public fun timedPrimeTest(n: Long): TimedPrimeResult? {
    val start = System.nanoTime()
    return if (isPrime(n)) TimedPrimeResult(n, System.nanoTime() - start) else null
}

/** The [count] smallest primes strictly greater than [start], checking consecutive odd integers. */
public fun searchForPrimes(
    start: Long,
    count: Int,
): List<Long> {
    var candidate = if (start % 2L == 0L) start + 1L else start + 2L
    val found = mutableListOf<Long>()
    while (found.size < count) {
        if (timedPrimeTest(candidate) != null) {
            found.add(candidate)
        }
        candidate += 2L
    }
    return found
}

public fun ex_1_22(): Map<Long, List<Long>> = listOf(1_000L, 10_000L, 100_000L, 1_000_000L).associateWith { searchForPrimes(it, 3) }
