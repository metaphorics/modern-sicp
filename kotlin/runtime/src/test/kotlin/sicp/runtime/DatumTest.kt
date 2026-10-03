// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class DatumTest :
    FunSpec({
        test("proper lists preserve typed values and pair aliases") {
            val member = pair(Symbol("member"), Empty)
            val value = datumList(Whole(1), member, Truth(true))

            val result = either { elements(value) }
            result shouldBe Either.Right(listOf(Whole(1), member, Truth(true)))
        }

        test("improper and cyclic lists raise typed errors") {
            val improper = pair(Whole(1), Whole(2))
            val cyclic = pair(Whole(3), Empty)
            cyclic.second = cyclic

            either { elements(improper) } shouldBe
                Either.Left(DatumError.TypeMismatch("not a proper list", listOf(improper)))
            either { elements(cyclic) } shouldBe
                Either.Left(DatumError.TypeMismatch("not a finite proper list", listOf(cyclic)))
        }

        test("pair identity stays distinct from structural equality") {
            val left = pair(Whole(1), Empty)
            val right = pair(Whole(1), Empty)
            assertSoftly {
                (left == right) shouldBe false
                structurallyEqual(left, right) shouldBe true
                structurallyEqual(Tagged("tag", left), Tagged("tag", right)) shouldBe true
                structurallyEqual(Tagged("tag", left), Tagged("other", right)) shouldBe false
                structurallyEqual(Tagged("tag", left), Whole(1)) shouldBe false
                structurallyEqual(left, pair(Whole(2), Empty)) shouldBe false
            }
        }

        test("structural equality handles cyclic pairs") {
            val left = pair(Whole(1), Empty)
            val right = pair(Whole(1), Empty)
            left.second = left
            right.second = right
            structurallyEqual(left, right) shouldBe true
            right.first = Whole(2)
            structurallyEqual(left, right) shouldBe false
        }

        test("datum rendering is deterministic host syntax and escapes text") {
            val value = pair(Text("a\n\""), Empty)
            value.toString() shouldBe """PairCell(first=Text(value="a\n\""), second=Empty)"""
            Truth(true).toString() shouldBe "Truth(value=true)"
            Tagged("rectangular", Empty).toString() shouldBe """Tagged(tag="rectangular", payload=Empty)"""

            val cyclic = pair(Whole(1), Empty)
            cyclic.second = cyclic
            cyclic.toString() shouldBe "PairCell(first=Whole(value=1), second=<cycle>)"
        }

        test("rendering handles deeply nested pairs without recursion") {
            val depth = 4_096
            var value: Datum = Whole(0)
            repeat(depth) { value = pair(value, Empty) }
            val expected =
                "PairCell(first=".repeat(depth) +
                    "Whole(value=0)" +
                    ", second=Empty)".repeat(depth)

            value.toString() shouldBe expected
        }

        test("key projection is structural and rejects unsupported or cyclic data") {
            val list = datumList(Symbol("a"), Whole(1))
            either { keyOf(list) } shouldBe
                Either.Right(
                    DatumKey.Pair(
                        DatumKey.Symbol("a"),
                        DatumKey.Pair(DatumKey.Integer(1), DatumKey.Empty),
                    ),
                )
            either { keyOf(Truth(true)) } shouldBe
                Either.Left(DatumError.TypeMismatch("not a table key: Truth(value=true)", listOf(Truth(true))))

            val cyclic = pair(Whole(1), Empty)
            cyclic.first = cyclic
            either { keyOf(cyclic) } shouldBe
                Either.Left(DatumError.TypeMismatch("cyclic datum cannot be a table key", listOf(cyclic)))
        }
    })
