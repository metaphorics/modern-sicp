// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3
// Chapter 4, section 4.3.3, implementing the amb evaluator: the engine's
// observable contract -- the undo trail rolls an ordinary write back when
// its branch dies while a `setPermanent` write survives, resumption
// advances the deepest pending choice first, and a typed fault aborts the
// whole search rather than failing one attempt.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.SearchModule
import sicp.ch4.SearchRun

private val ROLLBACK: String =
    """
    var traced: Long = 0L

    fun main() {
        val x = choose(1L, 2L)
        if (x == 1L) {
            traced = 999L
        }
        demand(x == 2L)
        println(traced)
    }
    """.trimIndent()

private val DEEPEST_FIRST: String =
    """
    fun main() {
        val x = choose(1L, 2L)
        val y = choose("a", "b")
        println("[${'$'}{x}, ${'$'}{y}]")
    }
    """.trimIndent()

private val ABORT: String =
    """
    fun main() {
        val x = choose(1L, 2L)
        println(x)
        val boom = 1L / (x - 1L)
        println(boom)
    }
    """.trimIndent()

private val PERMANENT: String =
    """
    var ordinary: Long = 0L
    var permanent: Long = 0L

    fun main() {
        val x = choose(1L, 2L)
        if (x == 1L) {
            setPermanent {
                permanent = 999L
            }
            ordinary = 111L
        }
        demand(x == 2L)
        println("[${'$'}{ordinary}, ${'$'}{permanent}]")
    }
    """.trimIndent()

private fun searchRun(source: String): SearchRun =
    SearchModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_3_3EngineTest :
    FunSpec({
        test("a write inside a dying branch is rolled back by the undo trail") {
            val run = searchRun(ROLLBACK)
            run.result.output shouldBe "0\n"
            run.result.error shouldBe null
        }

        test("resumption advances the deepest pending choice, not the outermost") {
            searchRun(DEEPEST_FIRST).result.output shouldBe "[1, a]\n[1, b]\n[2, a]\n[2, b]\n"
        }

        test("a typed fault aborts the search rather than failing one attempt") {
            val run = searchRun(ABORT)
            run.result.output shouldBe "1\n"
            run.result.error?.category shouldBe "DivisionByZero"
            run.result.mainValue shouldBe null
        }

        test("a permanent write survives the rollback an ordinary write obeys") {
            val run = searchRun(PERMANENT)
            run.result.output shouldBe "[0, 999]\n"
            run.result.error shouldBe null
        }
    })
