// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.6

package sicp.ch1.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_06Test :
    FunSpec({
        test("newIf gives the demonstration values of the statement") {
            newIf(2L == 3L, 0L, 5L) shouldBe 5L
            newIf(1L == 1L, 0L, 5L) shouldBe 0L
        }
        test("both clause expressions evaluate on every call") {
            ex_1_06() shouldBe (1 to 1)
        }
        test("the rewritten sqrtIter never terminates: the stack runs out") {
            shouldThrow<StackOverflowError> { sqrtIterNewIf(1.0, 9.0) }
        }
    })
