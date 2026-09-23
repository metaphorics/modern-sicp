// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E0_04Test :
    FunSpec({
        test("digits parse to their value, spaces trim away") {
            either { parseAmount("42") } shouldBe Either.Right(42L)
            either { parseAmount("  12 ") } shouldBe Either.Right(12L)
        }
        test("blank input raises Blank") {
            either { parseAmount("   ") } shouldBe Either.Left(ParseError.Blank)
        }
        test("a stray character raises NotDigits carrying the input") {
            either { parseAmount("1x") } shouldBe Either.Left(ParseError.NotDigits("1x"))
        }
        test("the eager original throws where the Raise version returns a value") {
            val outcome =
                try {
                    parseAmountEager("1x").toString()
                } catch (e: IllegalArgumentException) {
                    e::class.simpleName ?: "IllegalArgumentException"
                }
            outcome shouldBe "IllegalArgumentException"
        }
    })
