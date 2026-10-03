// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2
// Chapter 4, section 4.2.1, normal order and applicative order: the
// section's `try` and `unless` under delayed arguments, with the strict
// core run as the applicative-order contrast -- under delay the armed
// argument is never evaluated, under strictness it faults first.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.ch4.LazyModule
import sicp.guest.Mode
import sicp.guest.RunResult

private val TRY_SESSION: String =
    """
    fun tryCall(a: Long, b: Long): Long = if (a == 0L) 1L else b

    fun main() {
        println(tryCall(0L, 1L / 0L))
    }
    """.trimIndent()

private val UNLESS_SESSION: String =
    """
    var a: Long = 12L
    var b: Long = 0L

    fun unless(condition: Boolean, usual: Long, exceptional: Long): Long =
        if (condition) exceptional else usual

    fun exceptional(): Long {
        println("exception: returning 0")
        return 0L
    }

    fun main() {
        println(unless(b == 0L, a / b, exceptional()))
    }
    """.trimIndent()

private fun runLazy(source: String): RunResult =
    LazyModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { run -> run.result },
    )

private fun runStrict(source: String): RunResult =
    Direct.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_2_1NormalOrderTest :
    FunSpec({
        test("the try session: the armed argument is never evaluated") {
            val result = runLazy(TRY_SESSION)
            result.output shouldBe "1\n"
            result.error shouldBe null
        }

        test("the same call under applicative order faults first") {
            val result = runStrict(TRY_SESSION)
            result.output shouldBe ""
            result.error?.category shouldBe "DivisionByZero"
        }

        test("unless does useful work past an argument that would fault") {
            val result = runLazy(UNLESS_SESSION)
            result.output shouldBe "exception: returning 0\n0\n"
            result.error shouldBe null
        }

        test("the same unless under applicative order evaluates the faulting arm") {
            val result = runStrict(UNLESS_SESSION)
            result.output shouldBe ""
            result.error?.category shouldBe "DivisionByZero"
        }
    })
