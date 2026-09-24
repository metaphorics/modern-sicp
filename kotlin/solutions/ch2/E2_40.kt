// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.40

package sicp.ch2.exercises

private fun isPrime(n: Long): Boolean {
    if (n < 2L) return false
    var divisor = 2L
    while (divisor * divisor <= n) {
        if (n % divisor == 0L) return false
        divisor += 1L
    }
    return true
}

/** The book's `unique-pairs`: for each i, pair it with every `1 <= j < i`. */
public fun uniquePairs(n: Long): List<List<Long>> = flatMapSeq({ i -> (1L until i).map { j -> listOf(i, j) } }, (1L..n).toList())

/** `prime-sum-pairs` re-derived over [uniquePairs]: generate, filter on the prime sum, and produce the triple. */
public fun primeSumPairsViaUniquePairs(n: Long): List<List<Long>> =
    uniquePairs(n)
        .filter { pair -> isPrime(pair[0] + pair[1]) }
        .map { pair -> listOf(pair[0], pair[1], pair[0] + pair[1]) }

/** `uniquePairs(6)`, the fifteen pairs with `1 <= j < i <= 6`. */
public fun ex_2_40(): List<List<Long>> = uniquePairs(6L)
