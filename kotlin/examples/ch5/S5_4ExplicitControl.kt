// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.4
// Section 5.4, running the explicit-control evaluator: the book's session
// on the typed controller -- the factorial definition, then the call,
// value 120 -- and the monitored machine's stack statistics, which are
// machine outputs: pushes, maximum depth, and executed instructions are
// read from the run, and the machine run agrees with direct execution.

package sicp.ch5.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.CheckedProgram
import sicp.guest.GuestError
import sicp.guest.Mode
import sicp.guest.RunResult

private val factorialSource: String =
    """
    fun factorial(n: Long): Long = if (n < 2L) 1L else factorial(n - 1L) * n

    fun main() {
        println(factorial(5L))
    }
    """.trimIndent()

private val haltAfterPrintSource: String =
    """
    fun divide(a: Long, b: Long): Long = a / b

    fun main() {
        println(1L)
        println(divide(1L, 0L))
        println(2L)
    }
    """.trimIndent()

private fun checked(source: String): CheckedProgram =
    Admission.admit(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

private fun runMachine(source: String): RunResult =
    ExplicitControl.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

private fun runDirect(source: String): RunResult =
    Direct.run(source, Mode.CORE).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

/** The monitored stack statistics of one recursive factorial run: pushes
 * and the high-water depth, asserted balanced before they leave. */
private fun stackStatistics(n: Long): Pair<Long, Long> {
    val source =
        """
        fun factorial(n: Long): Long = if (n < 2L) 1L else factorial(n - 1L) * n

        fun main() {
            println(factorial(${n}L))
        }
        """.trimIndent()
    val machine = ExplicitControl.machine(checked(source))
    either<GuestError, Unit> { machine.run() }.fold(
        { e -> throw AssertionError("the machine run faulted: ${e.category}") },
        { },
    )
    // a balanced controller halts with every save restored
    machine.stack.depth shouldBe 0
    return machine.stack.pushes to machine.stack.maxDepth
}

public class S5_4ExplicitControlTest :
    FunSpec({
        test("the machine run answers the book's session: value 120") {
            val result = runMachine(factorialSource)
            result.output shouldBe "120\n"
            result.error shouldBe null
        }

        test("the machine run agrees with direct execution") {
            val onMachine = runMachine(factorialSource)
            val direct = runDirect(factorialSource)
            onMachine.output shouldBe direct.output
            onMachine.mainValue shouldBe direct.mainValue
            onMachine.error shouldBe null
            direct.error shouldBe null
        }

        test("the monitored machine reports its stack statistics as machine outputs") {
            val (pushes5, depth5) = stackStatistics(5L)
            val (pushes6, depth6) = stackStatistics(6L)
            // recursive descent costs stack: deeper recursion pushes more
            // and reaches a deeper high-water mark
            (depth6 > depth5) shouldBe true
            (pushes6 > pushes5) shouldBe true
        }

        test("a typed guest error stops the run after its earlier effects") {
            val result = runMachine(haltAfterPrintSource)
            result.output shouldBe "1\n"
            result.error?.category shouldBe "DivisionByZero"
            result.mainValue shouldBe null
        }
    })
