// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.3

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution

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

/**
 * Exercise 3.3: Modify the `makeAccount` function so that it creates
 * password-protected accounts. That is, `makeAccount` should take a
 * password as an additional argument, as in
 * `makeAccount(100, "secret-password")`. The resulting account object
 * should process a request only if it is accompanied by the password
 * with which the account was created, and should otherwise raise
 * `AccountError.WrongPassword`.
 *
 * The scaffold always raises `WrongPassword`, ignoring the balance and
 * every password given.
 */
public fun makeAccount(
    balance: Long,
    correctPassword: String,
): Account = throw PendingSolution()
