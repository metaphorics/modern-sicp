// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.45

package sicp.ch3.exercises

import arrow.core.Either

/**
 * Louis's account: deposits and withdrawals are serialized by the
 * account AND the serializer is exported for procedures such as
 * serialized-exchange. The two uses collide: kotlinx's Mutex is not
 * reentrant, so a serialized-exchange that enters through the exported
 * serializer and then calls the account's own serialized withdraw waits
 * forever on the mutex its own process holds.
 */
public fun makeLouisAccount(balance: Long): RawAccount {
    var b = balance
    val s = Serializer()
    return object : RawAccount {
        override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> =
            s.serialized {
                if (b < amount) {
                    Either.Left(WithdrawError.InsufficientFunds)
                } else {
                    b -= amount
                    Either.Right(b)
                }
            }()

        override suspend fun deposit(amount: Long): Long =
            s.serialized {
                b += amount
                b
            }()

        override suspend fun balance(): Long = b

        override fun serializer(): Serializer = s
    }
}

/** The plain deposit of Louis's proposal: no explicit serialization. */
public suspend fun plainDeposit(
    account: RawAccount,
    amount: Long,
): Long = account.deposit(amount)
