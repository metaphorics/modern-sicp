// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.3

package sicp.ch3.exercises

import arrow.core.Either

/**
 * The domain's errors for a password-protected account. `CallTheCops` is
 * exercise 3.4's lockout, declared here so the whole family of account
 * exercises (3.3, 3.4, 3.4a, 3.7) shares one closed error set.
 */
public sealed interface AccountError {
    public data object WrongPassword : AccountError

    public data object InsufficientFunds : AccountError

    public data object CallTheCops : AccountError
}

/**
 * The book's dispatch protocol for a bank account, now requiring a
 * password on every request.
 */
public interface Account {
    public fun withdraw(
        password: String,
        amount: Long,
    ): Either<AccountError, Long>

    public fun deposit(
        password: String,
        amount: Long,
    ): Either<AccountError, Long>
}

/** A captured `var balance`, guarded by comparing every request's password against the one closed over at creation. */
public fun makeAccount(
    balance: Long,
    correctPassword: String,
): Account {
    var b = balance
    return object : Account {
        override fun withdraw(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> =
            when {
                password != correctPassword -> {
                    Either.Left(AccountError.WrongPassword)
                }

                amount > b -> {
                    Either.Left(AccountError.InsufficientFunds)
                }

                else -> {
                    b -= amount
                    Either.Right(b)
                }
            }

        override fun deposit(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> =
            if (password != correctPassword) {
                Either.Left(AccountError.WrongPassword)
            } else {
                b += amount
                Either.Right(b)
            }
    }
}
