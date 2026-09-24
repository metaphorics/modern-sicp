// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.44

package sicp.ch3.exercises

import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.yield

/**
 * Ben's transfer: withdraw from one account, deposit into the other,
 * with no lock spanning the two accounts. Per-account serialization is
 * all it needs, because it never reads a balance that must stay
 * consistent across the two accounts.
 */
public suspend fun transfer(
    fromAccount: SerializedAccount,
    toAccount: SerializedAccount,
    amount: Long,
) {
    fromAccount.withdraw(amount)
    toAccount.deposit(amount)
}

/**
 * Two transfers run concurrently in forced schedules: one moves
 * `amount` from a to b, the other moves `smaller` from b to a. Each
 * transfer yields between its withdraw and its deposit, so the second
 * transfer's withdraw lands between the first's two steps. Whatever the
 * launch order, the two accounts still hold their starting total,
 * because each account's own withdraw and deposit are serialized.
 */
public suspend fun concurrentTransfers(
    amount: Long,
    smaller: Long,
): Pair<Long, Long> {
    val a = makeSerializedAccount(100L)
    val b = makeSerializedAccount(100L)
    coroutineScope {
        launch {
            a.withdraw(amount)
            yield()
            b.deposit(amount)
        }
        launch {
            b.withdraw(smaller)
            yield()
            a.deposit(smaller)
        }
    }
    return a.balance() to b.balance()
}
