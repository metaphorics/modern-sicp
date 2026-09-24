// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.10

package sicp.ch3.exercises

import arrow.core.Either

/**
 * The section's typed form of the book's "Insufficient funds" string,
 * shared by the withdrawal and account exercises of this section.
 */
public sealed interface WithdrawError {
    public data object InsufficientFunds : WithdrawError
}

/**
 * The parameter version, repeated here so the two versions of the
 * exercise can be compared in one place: the state cell is the parameter
 * `b`, rebound inside the lambda.
 */
public fun makeWithdraw(balance: Long): (Long) -> Either<WithdrawError, Long> {
    var b = balance
    return { amount ->
        if (amount > b) {
            Either.Left(WithdrawError.InsufficientFunds)
        } else {
            b -= amount
            Either.Right(b)
        }
    }
}

/**
 * The book's alternate `make-withdraw`: the state variable is a local
 * `var balance` instead of a parameter. The returned lambda captures that
 * one cell, exactly as the parameter version captures its own, so the two
 * versions build objects with the same behavior.
 */
public fun makeWithdrawLet(initialAmount: Long): (Long) -> Either<WithdrawError, Long> {
    var balance = initialAmount
    return { amount ->
        if (amount > balance) {
            Either.Left(WithdrawError.InsufficientFunds)
        } else {
            balance -= amount
            Either.Right(balance)
        }
    }
}
