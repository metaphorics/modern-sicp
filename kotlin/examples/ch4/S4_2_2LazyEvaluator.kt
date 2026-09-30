// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2
// Chapter 4, section 4.2.2, the lazy evaluator: the thunk machinery --
// the memoized cell that fills once and whose every demand observes the
// recorded value. The forcing instrument makes the lesson observable:
// force attempts and distinct computations are counted separately, and
// the thunk body's effects appear exactly once.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyModule
import sicp.guest.RunResult

private val MEMO_SESSION: String =
    """
    fun main() {
        val t = thunk {
            println("computing")
            21L
        }
        println(force(t))
        println(force(t))
    }
    """.trimIndent()

private val DELAYED_PARAMETER_SESSION: String =
    """
    var count: Long = 0L

    fun id(@Strict x: Long): Long {
        count = count + 1L
        return x
    }

    fun twice(x: Long): Long = x + x

    fun main() {
        println(twice(id(10L)))
        println(count)
    }
    """.trimIndent()

private fun lazyRun(source: String): sicp.ch4.LazyRun =
    LazyModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_2_2LazyEvaluatorTest :
    FunSpec({
        test("a memoized thunk computes once however often it is forced") {
            val run = lazyRun(MEMO_SESSION)
            val result: RunResult = run.result
            result.output shouldBe "computing\n21\n21\n"
            result.error shouldBe null
            run.forcing.forceAttempts shouldBe 2L
            run.forcing.computations shouldBe 1L
        }

        test("the thunk body's effects appear exactly once") {
            lazyRun(MEMO_SESSION).forcing.effects.size shouldBe 1
        }

        test("a delayed parameter memoizes across its references") {
            val run = lazyRun(DELAYED_PARAMETER_SESSION)
            // the body of id ran once even though the parameter is read
            // twice: the second read observes the filled cell
            run.result.output shouldBe "20\n1\n"
            run.forcing.forceAttempts shouldBe 2L
            run.forcing.computations shouldBe 1L
        }
    })
