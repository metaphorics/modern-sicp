// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.38

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_38Test :
    FunSpec({
        test("foldRight divides to 3/2, foldLeft to 1/6") {
            ex_2_38() shouldBe listOf(1.5, 1.0 / 6.0)
        }
        test("under list building the two folds reverse each other's order") {
            val sequence = listOf(1L, 2L, 3L, 4L)
            foldRight({ x, acc -> listOf(x) + acc }, emptyList<Long>(), sequence) shouldBe listOf(1L, 2L, 3L, 4L)
            foldLeft({ acc, x -> listOf(x) + acc }, emptyList<Long>(), sequence) shouldBe listOf(4L, 3L, 2L, 1L)
        }
        test("a commutative operation gives the same answer under both folds") {
            val sequence = listOf(1L, 2L, 3L, 4L)
            foldRight({ x: Long, acc: Long -> x + acc }, 0L, sequence) shouldBe
                foldLeft({ acc: Long, x: Long -> acc + x }, 0L, sequence)
        }
    })
