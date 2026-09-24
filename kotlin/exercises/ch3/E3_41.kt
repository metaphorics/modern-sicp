// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.41

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution

/**
 * Ben's version of the serialized account: the same as the text's, but
 * the balance read is serialized too (the changed method of the
 * statement).
 */
public fun makeBenAccount(balance: Long): SerializedAccount = throw PendingSolution()

/**
 * An observer sums the two accounts while a transfer of `amount` moves
 * money from a1 to a2, and the observer yields between its two reads,
 * so the transfer's writes land between them.
 */
public suspend fun sumStraddling(
    a1: SerializedAccount,
    a2: SerializedAccount,
    amount: Long,
): Long = throw PendingSolution()

/**
 * The same observer and the same transfer, but the observer takes both
 * its reads before the transfer runs, so it cannot straddle.
 */
public suspend fun sumUnstraddled(
    a1: SerializedAccount,
    a2: SerializedAccount,
    amount: Long,
): Long = throw PendingSolution()
