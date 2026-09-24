// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.48

package sicp.ch3.exercises

import arrow.core.Either

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
): NumberedAccount {
    var b = balance
    val s = Serializer()
    return object : NumberedAccount {
        override suspend fun withdraw(amount: Long): Either<WithdrawError, Long> =
            if (b < amount) {
                Either.Left(WithdrawError.InsufficientFunds)
            } else {
                b -= amount
                Either.Right(b)
            }

        override suspend fun deposit(amount: Long): Long {
            b += amount
            return b
        }

        override suspend fun balance(): Long = b

        override fun serializer(): Serializer = s

        override fun number(): Int = number
    }
}

/**
 * The rewritten serialized-exchange: always enter the lower-numbered
 * account's serializer first, so concurrent exchanges can never hold
 * each other's head of the order.
 */
public suspend fun orderedSerializedExchange(
    x: NumberedAccount,
    y: NumberedAccount,
): Long {
    val first = if (x.number() < y.number()) x else y
    val second = if (first === x) y else x
    return first.serializer().serialized {
        second.serializer().serialized {
            exchange(first, second)
        }()
    }()
}
