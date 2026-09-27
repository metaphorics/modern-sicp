// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.42

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution

/**
 * Ben's account: the serialized procedures are created once, when the
 * account is created, and every request answers the same one.
 */
public class BenAccount(
    balance: Long,
) : SerializedAccount {
    override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> = throw PendingSolution()

    override suspend fun deposit(amount: Long): Long = throw PendingSolution()

    override suspend fun balance(): Long = throw PendingSolution()
}

/** The exercise's constructor for Ben's account. */
public fun bensAccount(balance: Long): SerializedAccount = throw PendingSolution()

/**
 * Run the exercise's scenario against an account: a deposit of 40 and a
 * withdrawal of 20 in two coroutines that yield between their own two
 * calls, then a further deposit of 5, answering the final balance.
 */
public suspend fun runBenScenario(account: SerializedAccount): Long = throw PendingSolution()
