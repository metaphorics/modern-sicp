// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf

public class E3_09aTest :
    FunSpec({
        test("on a 64 KiB stack the recursive factorial overflows at n = 100000") {
            runRecursive(100_000L, 64L * 1024L) shouldBe StackOutcome.Overflow
        }

        test("on the same 64 KiB stack the tailrec factorial answers") {
            runTailrec(100_000L, 64L * 1024L).shouldBeInstanceOf<StackOutcome.Completed>()
        }

        test("at n = 5 both versions answer 120 on the small stack") {
            runRecursive(5L, 64L * 1024L) shouldBe StackOutcome.Completed(120L)
            runTailrec(5L, 64L * 1024L) shouldBe StackOutcome.Completed(120L)
        }
    })
