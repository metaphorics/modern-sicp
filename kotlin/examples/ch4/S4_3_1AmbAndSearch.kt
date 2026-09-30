// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3
// Chapter 4, section 4.3.1, amb and search: choice commits left to right,
// a failed demand resumes the most recent untried alternative, and one run
// explores to exhaustion -- so every attempt's effect lands in the answer
// stream in search order and the first success is the run's value. The
// unbounded `anIntegerStartingFrom` generator of the section cannot be
// observed through the published run (it explores to exhaustion and has
// no incremental or bounded answer view), so the example keeps the finite
// search coverage and the report names the missing contract.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.SearchModule
import sicp.ch4.SearchRun
import sicp.guest.GValue

private val PAIR_CHOICES: String =
    """
    fun main() {
        val a = choose(1L, 2L, 3L)
        val b = choose("a", "b")
        println("[${'$'}{a}, ${'$'}{b}]")
    }
    """.trimIndent()

private val PRIME_SUM_PAIR: String =
    """
    fun isPrime(n: Long): Boolean {
        if (n < 2L) return false
        var d = 2L
        while (d * d <= n) {
            if (n % d == 0L) return false
            d = d + 1L
        }
        return true
    }

    fun main() {
        val a = choose(1L, 3L, 5L, 8L)
        val b = choose(20L, 35L, 110L)
        demand(isPrime(a + b))
        println("[${'$'}{a}, ${'$'}{b}]")
    }
    """.trimIndent()

private val FIRST_SUCCESS: String =
    """
    fun main() {
        val x = choose(1L, 2L, 3L)
        demand(x > 1L)
        println(x)
    }
    """.trimIndent()

private fun searchRun(source: String): SearchRun =
    SearchModule.run(source).fold(
        { e -> throw AssertionError("admission rejected the unit: ${e.category}: ${e.message}") },
        { it },
    )

public class S4_3_1AmbAndSearchTest :
    FunSpec({
        test("a pair of choices has six possible values") {
            val run = searchRun(PAIR_CHOICES)
            run.result.output shouldBe "[1, a]\n[1, b]\n[2, a]\n[2, b]\n[3, a]\n[3, b]\n"
            run.result.error shouldBe null
            // three alternatives at the first choice, two at each of their
            // continuations
            run.choices shouldBe 9L
        }

        test("the prime-sum-pair answers [3, 20], then [3, 110], then [8, 35]") {
            val run = searchRun(PRIME_SUM_PAIR)
            run.result.output shouldBe "[3, 20]\n[3, 110]\n[8, 35]\n"
            run.result.error shouldBe null
        }

        test("the run reports its first success and counts each entered alternative") {
            val run = searchRun(FIRST_SUCCESS)
            run.result.output shouldBe "2\n3\n"
            run.result.mainValue shouldBe GValue.VUnit
            // all three alternatives entered; the first success remains 2
            run.choices shouldBe 3L
        }

        test("bounded consumption does not enter the recursive alternative") {
            val source =
                """
                fun natural(n: Long): Long = choose(n, natural(n + 1L))
                fun main() {
                    println(natural(1L))
                }
                """.trimIndent()
            val run =
                SearchModule.run(source, maxAnswers = 1).fold(
                    { e -> throw AssertionError("admission rejected ${e.category}: ${e.message}") },
                    { it },
                )
            run.result.output shouldBe "1\n"
            run.result.error shouldBe null
            run.choices shouldBe 1L
        }

        test("choice horizon stops before entering the next alternative") {
            val source =
                """
                fun main() {
                    val x = choose(1L, 2L, 3L)
                    println(x)
                }
                """.trimIndent()
            val run =
                SearchModule.run(source, maxAnswers = 3, maxChoices = 2).fold(
                    { e -> throw AssertionError("admission rejected ${e.category}: ${e.message}") },
                    { it },
                )
            run.result.output shouldBe "1\n2\n"
            run.result.error shouldBe null
            run.choices shouldBe 2L
        }
    })
