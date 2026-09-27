// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.14

package sicp.ch1.exercises

private fun firstDenomination(kindsOfCoins: Int): Long =
    when (kindsOfCoins) {
        1 -> 1L
        2 -> 5L
        3 -> 10L
        4 -> 25L
        else -> 50L
    }

/** Every call this local `cc` makes is one node of the tree the exercise asks to draw. */
public fun ex_1_14(): Long {
    var nodeCount = 0L

    fun cc(
        amount: Long,
        kindsOfCoins: Int,
    ): Long {
        nodeCount += 1L
        return when {
            amount == 0L -> 1L
            amount < 0L || kindsOfCoins == 0 -> 0L
            else -> cc(amount, kindsOfCoins - 1) + cc(amount - firstDenomination(kindsOfCoins), kindsOfCoins)
        }
    }
    cc(11L, 5)
    return nodeCount
}
