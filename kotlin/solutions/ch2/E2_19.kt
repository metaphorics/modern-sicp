// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.19

package sicp.ch2.exercises

/** The book's `us-coins`. */
public val usCoins: List<Long> = listOf(50L, 25L, 10L, 5L, 1L)

/**
 * The book's `uk-coins`. The original list's half-penny (0.5) is not a
 * whole number of cents, so this edition omits it: coin values are Long.
 */
public val ukCoins: List<Long> = listOf(100L, 50L, 20L, 10L, 5L, 2L, 1L)

/** The book's `first-denomination`: the head of the coin-value list. */
public fun firstDenomination(coinValues: List<Long>): Long = coinValues.first()

/** The book's `except-first-denomination`: the rest of the coin-value list. */
public fun exceptFirstDenomination(coinValues: List<Long>): List<Long> = coinValues.drop(1)

/** The book's `no-more?`: true when no coin values remain. */
public fun noMore(coinValues: List<Long>): Boolean = coinValues.isEmpty()

/** The book's `cc`, counting the ways to make [amount] from [coinValues]. */
public fun cc(
    amount: Long,
    coinValues: List<Long>,
): Long =
    when {
        amount == 0L -> {
            1L
        }

        amount < 0L || noMore(coinValues) -> {
            0L
        }

        else -> {
            cc(amount, exceptFirstDenomination(coinValues)) +
                cc(amount - firstDenomination(coinValues), coinValues)
        }
    }

/** The book's `(cc 100 us-coins)`: 292 ways. */
public fun ex_2_19(): Long = cc(100L, usCoins)
