// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.7

package sicp.ch3.exercises

import arrow.core.Either

// AccountError and Account are exercise 3.3's public declarations, reused here from the same package.

/**
 * A thin `Account` wrapping `account`: the wrapper checks its `password`
 * argument against `newPassword` itself, then forwards to `account`
 * under the original `originalPassword` it closed over. No new balance
 * cell is created, so every transaction through either name touches the
 * one shared account.
 */
public fun makeJoint(
    account: Account,
    originalPassword: String,
    newPassword: String,
): Account =
    object : Account {
        override fun withdraw(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> =
            if (password != newPassword) {
                Either.Left(AccountError.WrongPassword)
            } else {
                account.withdraw(originalPassword, amount)
            }

        override fun deposit(
            password: String,
            amount: Long,
        ): Either<AccountError, Long> =
            if (password != newPassword) {
                Either.Left(AccountError.WrongPassword)
            } else {
                account.deposit(originalPassword, amount)
            }
    }
