// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1, the source contract (given code, D23):
// admission round trips over the shared grammar before any guest effect,
// and the section 3.7 printer contract for what a program may write.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Admission
import sicp.guest.GValue
import sicp.guest.Mode
import sicp.guest.RunResult
import sicp.guest.renderPrinted

private val squareSource: String =
    """
    fun square(x: Long): Long = x * x

    fun main() {
        println(square(6L))
    }
    """.trimIndent()

/** The rejection class of [source], asserting it never admits. */
private fun rejection(source: String): String =
    Admission.admit(source, Mode.CORE).fold(
        { e -> e.category },
        { throw AssertionError("the unit was admitted: $source") },
    )

/** Runs [source] as a core program, asserting admission. */
private fun runCore(source: String): RunResult =
    Direct.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_0ReaderPrinterTest :
    FunSpec({
        test("a well-formed unit admits to a checked program") {
            val checked =
                Admission.admit(squareSource, Mode.CORE).fold(
                    { e -> throw AssertionError("rejected: ${e.category}: ${e.message}") },
                    { it },
                )
            checked.syntax.declarations.isNotEmpty() shouldBe true
            runCore(squareSource).output shouldBe "36\n"
        }

        test("malformed source fails typed, before any guest effect") {
            rejection("fun main() {\n    println(\"open)\n") shouldBe "Literal"
            rejection("fun main() {\n    println(1L) )\n}\n") shouldBe "Syntax"
        }

        test("rejected Kotlin surface names its rejection class") {
            rejection("fun main() {\n    try {\n        println(1L)\n    } finally {\n    }\n}\n") shouldBe "Exceptions"
            rejection("private fun f(): Long = 1L\n") shouldBe "MiscKotlin"
            rejection("val c = 'c'\n") shouldBe "CharSurface"
            rejection("fun main() {\n    println(\"${'$'}{listOf(1L)}\")\n}\n") shouldBe "StructuredOutput"
        }

        test("the printer renders exactly the four printable shapes") {
            renderPrinted(GValue.VLong(42)) shouldBe "42"
            renderPrinted(GValue.VDouble(2.25)) shouldBe "2.25"
            renderPrinted(GValue.VBool(true)) shouldBe "true"
            renderPrinted(GValue.VString("a\"b")) shouldBe "a\"b"
            // a structured value is not printable output at all
            renderPrinted(GValue.VList(mutableListOf(), mutable = false)) shouldBe null
        }

        test("decimal rendering is the pinned double text") {
            renderPrinted(GValue.VDouble(5.0)) shouldBe "5.0"
            renderPrinted(GValue.VDouble(0.001)) shouldBe "0.001"
            renderPrinted(GValue.VDouble(1.0e22)) shouldBe "1.0E22"
            renderPrinted(GValue.VDouble(2.5e-7)) shouldBe "2.5E-7"
        }

        test("a string source escapes only quote and backslash, and prints unescaped") {
            runCore(
                """
                fun main() {
                    println("a\"b\\c")
                    println("plain")
                }
                """.trimIndent(),
            ).output shouldBe "a\"b\\c\nplain\n"
        }
    })
