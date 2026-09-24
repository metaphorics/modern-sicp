// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.4

package sicp.ch3.exercises

import arrow.core.Either

// AccountError and Account are exercise 3.3's public declarations, reused here from the same package.

/**
 * A second captured `var`, `consecutiveWrong`, alongside `balance`: a
 * correct password resets it to zero, a wrong one increments it, and
 * past seven in a row every further request raises `CallTheCops` instead
 * of `WrongPassword` until a correct password arrives.
 */
public fun makeAccountWithLockout(
    balance: Long,
    correctPassword: String,
): Account {
    var b = balance
    var consecutiveWrong = 0

    fun checkPassword(password: String): AccountError? {
        if (password == correctPassword) {
            consecutiveWrong = 0
            return null
        }
        consecutiveWrong += 1
        return if (consecutiveWrong > 7) AccountError.CallTheCops else AccountError.WrongPassword
    }

    return object : Account {
        override fun withdraw(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> {
            val failure = checkPassword(password)
            return when {
                failure != null -> {
                    Either.Left(failure)
                }

                amount > b -> {
                    Either.Left(AccountError.InsufficientFunds)
                }

                else -> {
                    b -= amount
                    Either.Right(b)
                }
            }
        }

        override fun deposit(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> {
            val failure = checkPassword(password)
            return if (failure != null) {
                Either.Left(failure)
            } else {
                b += amount
                Either.Right(b)
            }
        }
    }
}
