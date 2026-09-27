// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.4.1, the nature of time in concurrent systems

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.yield

/** The section's shared balance: one field, two processes, one holder. */
public class Cell(
    public var value: Long,
)

// Named `sharedBalance`/`sharedWithdraw` here: S3_1_1 owns plain
// `balance`/`withdraw` in this shared examples package.
val sharedBalance = Cell(100L)

fun sharedWithdraw(amount: Long): Either<WithdrawError, Long> =
    if (sharedBalance.value < amount) {
        Either.Left(WithdrawError.InsufficientFunds)
    } else {
        sharedBalance.value -= amount
        Either.Right(sharedBalance.value)
    }

public class S3_4_1TimeInConcurrentSystemsTest :
    FunSpec({
        test("the intro session: successive withdrawals give different answers") {
            sharedBalance.value = 100L
            sharedWithdraw(25L) shouldBe Either.Right(75L)
            sharedWithdraw(25L) shouldBe Either.Right(50L)
        }

        test("the three-step statement, run alone: balance halves") {
            val b = Cell(100L)
            b.value = b.value - b.value / 2
            b.value shouldBe 50L
        }

        test("the figure 3.29 anomaly, forced: Peter 10, Paul 25, the bank keeps 75") {
            runTest {
                val bank = Cell(100L)
                var petersRead = 0L
                var paulsRead = 0L
                coroutineScope {
                    launch {
                        petersRead = bank.value
                        yield()
                        bank.value = petersRead - 10L
                    }
                    launch {
                        paulsRead = bank.value
                        yield()
                        bank.value = paulsRead - 25L
                    }
                }
                petersRead shouldBe 100L
                paulsRead shouldBe 100L
                bank.value shouldBe 75L
            }
        }
    })
