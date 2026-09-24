// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.45

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Louis's account: deposits and withdrawals are serialized by the
 * account AND the serializer is exported for procedures such as
 * serialized-exchange.
 */
public fun makeLouisAccount(balance: Long): RawAccount = throw PendingSolution()

/** The plain deposit of Louis's proposal: no explicit serialization. */
public suspend fun plainDeposit(
    account: RawAccount,
    amount: Long,
): Long = throw PendingSolution()
