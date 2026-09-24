// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.1.3, the costs of introducing assignment

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeSameInstanceAs

/**
 * The book's `make-simplified-withdraw`: no balance check, so calling it
 * keeps changing the same cell and the answers keep shrinking past zero.
 */
public fun makeSimplifiedWithdraw(balance: Long): (Long) -> Long {
    var b = balance
    return { amount ->
        b -= amount
        b
    }
}

/**
 * The book's `make-decrementer`: the same shape with no `var`, so calling
 * it twice with the same argument always answers the same way.
 */
public fun makeDecrementer(balance: Long): (Long) -> Long = { amount -> balance - amount }

/** The book's `factorial`: an iterative loop that threads its state through arguments, no assignment. */
public fun factorial(n: Long): Long {
    tailrec fun iter(
        product: Long,
        counter: Long,
    ): Long = if (counter > n) product else iter(product * counter, counter + 1)
    return iter(1L, 1L)
}

/**
 * The book's imperative `factorial`: `product` and `counter` are captured
 * `var`s updated by explicit assignment, in the order the book warns
 * about -- swap the two assignments and the result is wrong.
 */
public fun factorialImperative(n: Long): Long {
    var product = 1L
    var counter = 1L
    while (counter <= n) {
        product *= counter
        counter += 1
    }
    return product
}

public class S3_1_3CostsOfIntroducingAssignmentTest :
    FunSpec({
        test("makeSimplifiedWithdraw keeps subtracting past zero") {
            val w = makeSimplifiedWithdraw(25L)
            w(20L) shouldBe 5L
            w(10L) shouldBe -5L
        }

        test("makeDecrementer never accumulates an effect") {
            val d = makeDecrementer(25L)
            d(20L) shouldBe 5L
            d(10L) shouldBe 15L
        }

        test("two decrementers built alike are interchangeable, unlike two withdrawers") {
            val d1 = makeDecrementer(25L)
            val d2 = makeDecrementer(25L)
            d1(20L) shouldBe d2(20L)
            val w1 = makeSimplifiedWithdraw(25L)
            val w2 = makeSimplifiedWithdraw(25L)
            w1(20L) shouldBe 5L
            w1(20L) shouldBe -15L
            w2(20L) shouldBe 5L
        }

        test("a joint account is the same object under two names") {
            var peterBalance = 100L
            val peterAcc = { amount: Long ->
                peterBalance -= amount
                peterBalance
            }
            val paulAcc = peterAcc
            paulAcc(30L) shouldBe 70L
            peterAcc(0L) shouldBe 70L
            paulAcc shouldBeSameInstanceAs peterAcc
        }

        test("both factorial styles agree, order matters only in the imperative one") {
            factorial(5L) shouldBe 120L
            factorialImperative(5L) shouldBe 120L
        }
    })
