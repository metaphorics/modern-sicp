// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1, the reader and printer (given code, D23):
// round trips over the shared grammar and the printer contract.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.printFloat
import sicp.ch4.printValue
import sicp.ch4.readDatum
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VStr

private fun readOne(text: String): String = either { printValue(readDatum(text)) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

// readDatum("(cons 1 2)") => (cons 1 2)
// readDatum("(a . b)")    => (a . b)
// printValue(VReal(2.25)) => 2.25
// printValue(VStr("a\"b")) => "a\"b"

public class S4_1_0ReaderPrinterTest :
    FunSpec({
        test("lists and dotted pairs round trip through read and print") {
            readOne("(1 2 3)") shouldBe "(1 2 3)"
            readOne("(a . b)") shouldBe "(a . b)"
            readOne("(a (b . c) d)") shouldBe "(a (b . c) d)"
            readOne("()") shouldBe "()"
        }

        test("quote sugar reads as the quote form the evaluator sees") {
            readOne("'x") shouldBe "(quote x)"
        }

        test("booleans, integers, and comments parse") {
            either { readDatum("#t") } shouldBe either { VBool(true) }
            readOne("42") shouldBe "42"
            readOne("-7") shouldBe "-7"
            readOne("; a comment\n1.5") shouldBe "1.5"
        }

        test("strings escape only quote and backslash") {
            printValue(VStr("a\"b\\c")) shouldBe "\"a\\\"b\\\\c\""
            readOne("\"plain\"") shouldBe "\"plain\""
        }

        test("floats print shortest round-trip, point always, e outside the window") {
            printFloat(2.25) shouldBe "2.25"
            printFloat(5.0) shouldBe "5.0"
            printFloat(0.001) shouldBe "0.001"
            printFloat(1.0e22) shouldBe "1.0e22"
            printFloat(2.5e-7) shouldBe "2.5e-7"
        }

        test("malformed surface raises the typed parse error") {
            either { readDatum("(a b") }.isLeft() shouldBe true
            either { readDatum("\"open") }.isLeft() shouldBe true
            either<SchemeError, Any> { readDatum("(a . b c)") }.isLeft() shouldBe true
        }
    })
