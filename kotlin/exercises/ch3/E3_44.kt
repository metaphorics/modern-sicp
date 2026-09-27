// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.44

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Ben's transfer: withdraw from one account, deposit into the other,
 * with no lock spanning the two accounts.
 */
public suspend fun transfer(
    fromAccount: SerializedAccount,
    toAccount: SerializedAccount,
    amount: Long,
): Unit = throw PendingSolution()

/**
 * Two transfers run concurrently in forced schedules: one moves
 * `amount` from a to b, the other moves `smaller` from b to a. Each
 * transfer yields between its withdraw and its deposit, so the second
 * transfer's withdraw lands between the first's two steps.
 */
public suspend fun concurrentTransfers(
    amount: Long,
    smaller: Long,
): Pair<Long, Long> = throw PendingSolution()
