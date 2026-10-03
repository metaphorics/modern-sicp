// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.37

package sicp.ch4.solutions

// Exercise 4.37: Ben's triple generator. Ben computes the hypotenuse
// instead of choosing it: given i and j he takes the integer square root
// of i*i + j*j and keeps the pair only when that root is exact and within
// the bound. Both programs first answer (3 4 5), but their guest-side
// failed-requirement counts differ.
// These counters are not the removed evaluator's internal backtracks.

/** Ben's generator computes the hypotenuse; the probe prints each triple. */
internal val BEN_TRIPLE_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun tripleLine(i: Long, j: Long, k: Long): String = "(" + showLong(i) + " " + showLong(j) + " " + showLong(k) + ")"

fun isqrtTry(n: Long, k: Long): Long {
    if (k * k > n) {
        return 0L - 1L
    }
    if (k * k == n) {
        return k
    }
    return isqrtTry(n, k + 1L)
}

fun aPythagoreanTripleBen(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val ksq = i * i + j * j
    requireThat(ksq <= high * high)
    val k = isqrtTry(ksq, 1L)
    requireThat(k >= 0L)
    println(tripleLine(i, j, k))
}

fun main() {
    budgetCap = 1000000L
    aPythagoreanTripleBen(1L, 20L)
}
        """.trimIndent()

/** The same generator with its backtracks printed at every answer. */
internal val BEN_COUNTED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun isqrtTry(n: Long, k: Long): Long {
    if (k * k > n) {
        return 0L - 1L
    }
    if (k * k == n) {
        return k
    }
    return isqrtTry(n, k + 1L)
}

fun aCountedBenTriple(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val ksq = i * i + j * j
    requireThat(ksq <= high * high)
    val k = isqrtTry(ksq, 1L)
    requireThat(k >= 0L)
    println(showLong(backtracks))
}

fun main() {
    budgetCap = 1000000L
    aCountedBenTriple(1L, 20L)
}
        """.trimIndent()

/** The 4.35 program with its backtracks printed at every answer. */
internal val BOOK_ORDER_COUNTED_PROGRAM: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun aCountedTriple(low: Long, high: Long): Unit {
    val i = anIntegerBetween(low, high)
    val j = anIntegerBetween(i, high)
    val k = anIntegerBetween(j, high)
    requireThat(i * i + j * j == k * k)
    println(showLong(backtracks))
}

fun main() {
    budgetCap = 1000000L
    aCountedTriple(1L, 20L)
}
        """.trimIndent()

/** Ben's generator answers (3 4 5) first, like the book-order program.
 * => "(3 4 5)" */
public fun benFirstTriple(): String = searchLines(BEN_TRIPLE_PROGRAM).first()

/** Failed requirements before the book-order program's first triple. */
public fun bookOrderBacktracksToFirst(): Long = searchLines(BOOK_ORDER_COUNTED_PROGRAM).first().toLong()

/** Failed requirements before Ben's first triple. */
public fun benBacktracksToFirst(): Long = searchLines(BEN_COUNTED_PROGRAM).first().toLong()
