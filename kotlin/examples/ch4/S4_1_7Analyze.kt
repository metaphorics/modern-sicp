// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.7, separating syntactic analysis from execution:
// admission builds the typed syntax once, before any guest effect, and the
// analyzed run answers exactly what the direct run answers -- the same
// checked program executed repeatedly stays itself.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Analyzed
import sicp.ch4.Direct
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.Mode

private val FACTORIAL: String =
    """
    fun factorial(n: Long): Long = if (n < 2L) 1L else factorial(n - 1L) * n

    fun main() {
        println(factorial(5L))
        println(factorial(10L))
    }
    """.trimIndent()

private val REPEATED_CALL: String =
    """
    fun factorial(n: Long): Long = if (n < 2L) 1L else factorial(n - 1L) * n

    fun main() {
        println(factorial(6L))
    }
    """.trimIndent()

private val SHAPE_SESSION: String =
    """
    fun square(x: Long): Long = x * x

    fun render(xs: List<Long>): String {
        var out = ""
        var i = 0
        while (i < xs.size) {
            if (i > 0) { out = out + " " }
            out = out + "${'$'}{xs.get(i)}"
            i = i + 1
        }
        return out
    }

    fun main() {
        println(render(listOf(square(3L))))
        println(render(listOf(1L, 2L)))
        println(square(4L) > square(3L))
    }
    """.trimIndent()

private val ILL_TYPED: String =
    """
    fun main() {
        println("this unit never runs")
        val broken: Long = "not a number"
    }
    """.trimIndent()

private fun checked(source: String): CheckedProgram =
    Admission.admit(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_7AnalyzeTest :
    FunSpec({
        test("the analyzed run answers exactly what the direct run answers") {
            val program = checked(FACTORIAL)
            val direct = Direct.run(program)
            val analyzed = Analyzed.run(program)
            direct.output shouldBe "120\n3628800\n"
            analyzed.output shouldBe direct.output
            analyzed.mainValue shouldBe direct.mainValue
            analyzed.error shouldBe null
        }

        test("one checked program runs many times, answering the same thing") {
            val program = checked(REPEATED_CALL)
            val first = Analyzed.run(program)
            val second = Analyzed.run(program)
            first.output shouldBe "720\n"
            second.output shouldBe first.output
        }

        test("the two engines agree on the shapes a program can answer") {
            val program = checked(SHAPE_SESSION)
            Direct.run(program).output shouldBe "9\n1 2\ntrue\n"
            Analyzed.run(program).output shouldBe Direct.run(program).output
        }

        test("analysis rejects an ill-typed unit before any guest effect") {
            val admitted = Admission.admit(ILL_TYPED, Mode.CORE)
            admitted.isLeft() shouldBe true
            val direct = Direct.run(ILL_TYPED, Mode.CORE)
            direct.isLeft() shouldBe true
            direct.fold({ e -> e.category shouldBe "TypeMismatch" }, { })
        }
    })
