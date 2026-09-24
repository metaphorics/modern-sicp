// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.42

package sicp.ch3.exercises

import arrow.core.Either
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.yield

/**
 * Ben's account: the serialized procedures are created once, when the
 * account is created, and every request answers the same one. The
 * serializer lives in the account either way, so the set of protected
 * procedures -- and with it the allowed concurrency -- is the same as
 * the text's version.
 */
public class BenAccount(
    balance: Long,
) : SerializedAccount {
    private var b = balance
    private val s = Serializer()
    private val protectedWithdraw =
        s.serializedArg { amount: Long ->
            if (b < amount) {
                Either.Left(WithdrawError.InsufficientFunds)
            } else {
                b -= amount
                Either.Right(b)
            }
        }
    private val protectedDeposit =
        s.serializedArg { amount: Long ->
            b += amount
            b
        }

    override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> = protectedWithdraw(amount)

    override suspend fun deposit(amount: Long): Long = protectedDeposit(amount)

    override suspend fun balance(): Long = b
}

/** The exercise's constructor for Ben's account. */
public fun bensAccount(balance: Long): SerializedAccount = BenAccount(balance)

/**
 * Run the exercise's scenario against an account: a deposit of 40 and a
 * withdrawal of 20 in two coroutines that yield between their own two
 * calls, then a further deposit of 5, answering the final balance.
 */
public suspend fun runBenScenario(account: SerializedAccount): Long {
    coroutineScope {
        launch {
            account.deposit(40L)
            yield()
            account.withdraw(15L)
        }
        launch {
            account.withdraw(20L)
            yield()
            account.deposit(5L)
        }
    }
    return account.balance()
}
