// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.4, running the evaluator as a program: the run's
// observation contract. A program owns its effects -- the writes appear in
// program order with no driver lines around them -- the global names are
// the unit's top-level declarations, the primitive families answer their
// pinned values, and the first typed fault stops the run after its earlier
// effects. The book's sample interaction survives as the append session.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.guest.Mode
import sicp.guest.RunResult

private val APPEND_SESSION: String =
    """
    fun append(xs: List<String>, ys: List<String>): List<String> =
        if (xs.isEmpty()) ys else listOf(xs.get(0)) + append(xs.drop(1), ys)

    fun render(xs: List<String>): String {
        var out = "["
        var i = 0
        while (i < xs.size) {
            if (i > 0) { out = out + ", " }
            out = out + xs.get(i)
            i = i + 1
        }
        return out + "]"
    }

    fun main() {
        println(render(append(listOf("a", "b", "c"), listOf("d", "e", "f"))))
    }
    """.trimIndent()

private val PRIMITIVE_SESSION: String =
    """
    fun main() {
        println(2L + 3L * 4L)
        println(7L / 2L)
        println(7L % 2L)
        println(3L < 4L)
        val xs = listOf(10L, 20L, 30L)
        println(xs.get(2))
        println("a" + "b")
    }
    """.trimIndent()

private val EFFECT_SESSION: String =
    """
    fun main() {
        print("a")
        print("b")
        println("c")
        println(42L)
    }
    """.trimIndent()

private val HALT_SESSION: String =
    """
    fun divide(a: Long, b: Long): Long = a / b

    fun main() {
        println(3L)
        println(divide(1L, 0L))
        println(4L)
    }
    """.trimIndent()

private fun runCore(source: String): RunResult =
    Direct.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_1_4DriverTest :
    FunSpec({
        test("the book's sample interaction: append as data, then applied") {
            runCore(APPEND_SESSION).output shouldBe "[a, b, c, d, e, f]\n"
        }

        test("the primitive families answer their pinned values") {
            runCore(PRIMITIVE_SESSION).output shouldBe "14\n3\n1\ntrue\n30\nab\n"
        }

        test("print and println write their effects in program order, with no driver lines") {
            runCore(EFFECT_SESSION).output shouldBe "abc\n42\n"
        }

        test("a typed fault stops the program after its earlier effects") {
            val result = runCore(HALT_SESSION)
            result.output shouldBe "3\n"
            result.error?.category shouldBe "DivisionByZero"
            result.mainValue shouldBe null
        }
    })
