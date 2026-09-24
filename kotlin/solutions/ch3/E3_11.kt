// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.11

package sicp.ch3.exercises

import arrow.core.Either

/**
 * The book's dispatch protocol for a bank account: the `cond` that hands
 * back the `withdraw` or `deposit` closure for a message symbol becomes
 * an ordinary method call on an object.
 */
public interface BankAccount {
    public fun withdraw(amount: Long): Either<WithdrawError, Long>

    public fun deposit(amount: Long): Long
}

/**
 * The book's `make-account`: each call opens a frame binding `balance`,
 * the object's methods capture that one cell, and two calls produce two
 * objects whose states are two distinct cells sharing the compiled
 * method bodies.
 */
public fun makeBankAccount(balance: Long): BankAccount {
    var b = balance
    return object : BankAccount {
        override fun withdraw(amount: Long): Either<WithdrawError, Long> =
            if (amount > b) {
                Either.Left(WithdrawError.InsufficientFunds)
            } else {
                b -= amount
                Either.Right(b)
            }

        override fun deposit(amount: Long): Long {
            b += amount
            return b
        }
    }
}
