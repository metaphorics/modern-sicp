// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.48

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * An account with the unique identification number of exercise 3.48.
 * Its own operations stay raw; the number orders who acquires whose
 * serializer first.
 */
public interface NumberedAccount : RawAccount {
    public fun number(): Int
}

/** The numbered account constructor the exercise asks for. */
public fun makeNumberedAccount(
    number: Int,
    balance: Long,
): NumberedAccount = throw PendingSolution()

/**
 * The rewritten serialized-exchange: always enter the lower-numbered
 * account's serializer first, so concurrent exchanges can never hold
 * each other's head of the order.
 */
public suspend fun orderedSerializedExchange(
    x: NumberedAccount,
    y: NumberedAccount,
): Long = throw PendingSolution()
