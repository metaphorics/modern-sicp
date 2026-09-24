// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.10

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution

/**
 * The section's typed form of the book's "Insufficient funds" string,
 * shared by the withdrawal and account exercises of this section.
 */
public sealed interface WithdrawError {
    public data object InsufficientFunds : WithdrawError
}

/**
 * Exercise 3.10: the alternate `make-withdraw`, whose local state
 * variable is created by an explicit binding instead of a parameter. In
 * Scheme the binding is a `let`, sugar for a procedure call that builds a
 * frame of its own; here it is a local `var`, which binds in the frame
 * the call already built. Analyze the interactions `w1 =
 * makeWithdrawLet(100L)`, `w1(50L)`, `w2 = makeWithdrawLet(100L)`, show
 * that the two versions of `make-withdraw` create objects with the same
 * behavior, and explain how the environment structures differ.
 */
public fun makeWithdrawLet(initialAmount: Long): (Long) -> Either<WithdrawError, Long> = throw PendingSolution()
