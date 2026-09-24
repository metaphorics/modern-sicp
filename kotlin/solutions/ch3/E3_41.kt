// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.41

package sicp.ch3.exercises

import arrow.core.Either
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.yield

/**
 * Ben's version of the serialized account: the same as the text's, but
 * the balance read is serialized too (the changed method of the
 * statement).
 */
public fun makeBenAccount(balance: Long): SerializedAccount {
    var b = balance
    val s = Serializer()
    return object : SerializedAccount {
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

        override suspend fun balance(): Long = s.serialized { b }()
    }
}

/**
 * An observer sums the two accounts while a transfer of `amount` moves
 * money from a1 to a2, and the observer yields between its two reads,
 * so the transfer's writes land between them. Both reads are committed
 * balances of one account each; their sum is still a total that never
 * existed.
 */
public suspend fun sumStraddling(
    a1: SerializedAccount,
    a2: SerializedAccount,
    amount: Long,
): Long {
    var total = 0L
    coroutineScope {
        launch {
            val first = a1.balance()
            yield()
            total = first + a2.balance()
        }
        launch {
            a1.withdraw(amount)
            a2.deposit(amount)
        }
    }
    return total
}

/**
 * The same observer and the same transfer, but the observer takes both
 * its reads before the transfer runs, so it cannot straddle.
 */
public suspend fun sumUnstraddled(
    a1: SerializedAccount,
    a2: SerializedAccount,
    amount: Long,
): Long {
    var total = 0L
    coroutineScope {
        launch {
            val first = a1.balance()
            val second = a2.balance()
            total = first + second
        }
        launch {
            a1.withdraw(amount)
            a2.deposit(amount)
        }
    }
    return total
}
