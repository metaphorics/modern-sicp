// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.1.1, local state variables

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The one book string this edition types: every `withdraw` and `make-account`
 * of this section answers "Insufficient funds" with this error instead.
 */
public sealed interface WithdrawError {
    public data object InsufficientFunds : WithdrawError
}

/**
 * The book's first `withdraw`: `balance` is an ordinary file-level `var`,
 * freely visible and modifiable by any other declaration in the program.
 * The rest of 3.1.1 exists to show why that is a problem.
 */
public var balance: Long = 100L

public fun withdraw(amount: Long): Either<WithdrawError, Long> =
    if (amount > balance) {
        Either.Left(WithdrawError.InsufficientFunds)
    } else {
        balance -= amount
        Either.Right(balance)
    }

/**
 * The book's `new-withdraw`: a `let`-bound `balance` closed over by the
 * returned lambda, so the cell is encapsulated and no other declaration in
 * the program can reach it. This is the captured-`var` decision section
 * 3.1 exists to teach.
 */
public fun newWithdraw(): (Long) -> Either<WithdrawError, Long> {
    var b = 100L
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
 * The book's `make-withdraw`: a withdrawal-processor factory. Each call
 * captures its own `balance` cell, so two withdrawers built from two calls
 * never interfere.
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
 * The book's bank-account object. Scheme's `dispatch` procedure, which
 * hands back the `withdraw` or `deposit` closure for a message symbol, is
 * this edition's `Account` interface: the dispatch the book's `cond`
 * approximates becomes an ordinary method call.
 */
public interface Account {
    public fun withdraw(amount: Long): Either<WithdrawError, Long>

    public fun deposit(amount: Long): Long
}

/**
 * The book's `make-account`: sets up one environment with a local
 * `balance`, and returns an object exposing `withdraw` and `deposit` over
 * that one captured cell.
 */
public fun makeAccount(balance: Long): Account {
    var b = balance
    return object : Account {
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

public class S3_1_1LocalStateVariablesTest :
    FunSpec({
        test("the book's withdraw session: 75, 50, insufficient at 60, then 35") {
            balance = 100L
            withdraw(25L) shouldBe Either.Right(75L)
            withdraw(25L) shouldBe Either.Right(50L)
            withdraw(60L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            withdraw(15L) shouldBe Either.Right(35L)
        }

        test("newWithdraw encapsulates its own balance, starting at 100") {
            val w = newWithdraw()
            w(30L) shouldBe Either.Right(70L)
            w(40L) shouldBe Either.Right(30L)
        }

        test("two make-withdraw objects never interfere") {
            val w1 = makeWithdraw(100L)
            val w2 = makeWithdraw(100L)
            w1(50L) shouldBe Either.Right(50L)
            w2(70L) shouldBe Either.Right(30L)
            w2(40L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            w1(40L) shouldBe Either.Right(10L)
        }

        test("the book's make-account session: 50, insufficient at 60, deposit to 90, then 30") {
            val acc = makeAccount(100L)
            acc.withdraw(50L) shouldBe Either.Right(50L)
            acc.withdraw(60L) shouldBe Either.Left(WithdrawError.InsufficientFunds)
            acc.deposit(40L) shouldBe 90L
            acc.withdraw(60L) shouldBe Either.Right(30L)
        }

        test("a second make-account call is a fully separate object") {
            val acc = makeAccount(100L)
            val acc2 = makeAccount(100L)
            acc.withdraw(10L) shouldBe Either.Right(90L)
            acc2.withdraw(10L) shouldBe Either.Right(90L)
            acc.deposit(5L) shouldBe 95L
            acc2.withdraw(90L) shouldBe Either.Right(0L)
        }
    })
