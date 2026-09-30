// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.31: laziness as an upward-compatible extension. The lazy
// experiment's declarations are exactly the per-parameter discipline the
// book asks for: `@Strict` parameters (and every primitive argument)
// evaluate at the call, `@Delayed` or unannotated parameters arrive as
// memoized transparent thunks, and the book's third mode -- `lazy`, a
// re-run-at-every-demand call-by-name parameter -- is expressed with a
// function-value parameter the body invokes at each demand, because the
// module's thunks compute at most once by contract. A procedure with no
// annotations keeps the delayed default; the counting session pins all
// three disciplines at once.

/** The counting session's `id`. */
internal val ANNOTATED_PRELUDE: String =
    """
var count: Long = 0L

fun id(x: Long): Long {
    count = count + 1L
    return x
}

fun showLong(n: Long): String = "${'$'}{n}"

fun showList(xs: List<Long>): String {
    var out = "["
    var index = 0
    while (index < xs.size) {
        if (index > 0) {
            out = out + ", "
        }
        out = out + showLong(xs.get(index))
        index = index + 1
    }
    return out + "]"
}
    """.trimIndent()

/** The mixed discipline: `a` and `c` strict, `b` call-by-name through a
 * function value, `d` call-by-need. The strict pair counts 2 at the call,
 * each `b` demand re-invokes, `d` computes once: count 5.
 * => "[1, 5, 5, 4, 30, 30]\n5\n" */
internal val MIXED_PROGRAM: String =
    ANNOTATED_PRELUDE + "\n" +
        """
fun f(@Strict a: Long, b: () -> Long, @Strict c: Long, d: Long): List<Long> = listOf(a, b(), b(), c, d, d)

fun main() {
    println(showList(f(id(1L), { id(5L) }, id(4L), id(30L))))
    println(count)
}
        """.trimIndent()

/** All-memo declaration: every delayed parameter computes at most once, so
 * the count lands on 4. => "[1, 5, 5, 4, 30, 30]\n4\n" */
internal val ALL_MEMO_PROGRAM: String =
    ANNOTATED_PRELUDE + "\n" +
        """
fun g(@Strict a: Long, b: Long, @Strict c: Long, d: Long): List<Long> = listOf(a, b, b, c, d, d)

fun main() {
    println(showList(g(id(1L), id(5L), id(4L), id(30L))))
    println(count)
}
        """.trimIndent()

/** The declared discipline: `a` and `c` strict, `b` call-by-name, `d`
 * call-by-need. => "[1, 5, 5, 4, 30, 30]\n5\n" */
public fun annotatedMixedTranscript(): String = outcomeText(LazyModule.run(MIXED_PROGRAM).map { it.result })

/** All-lazy-memo declaration: every delayed parameter computes at most
 * once, so the count lands on 4. => "[1, 5, 5, 4, 30, 30]\n4\n" */
public fun annotatedAllMemoTranscript(): String = outcomeText(LazyModule.run(ALL_MEMO_PROGRAM).map { it.result })

/** A delayed parameter is never evaluated at all unless the body demands
 * it: the dangerous argument passes harmlessly. => "7\n" */
public fun lazyParamSkipsTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                """
fun g(get: () -> Long): Long = 7

fun main() {
    println(g { 1L / 0L })
}
                """.trimIndent(),
            ).map { it.result },
    )

/** The strict counterfactual: a `@Strict` parameter is evaluated at the
 * call. => "Error: DivisionByZero\n" */
public fun strictParamEagerTranscript(): String =
    outcomeText(
        LazyModule
            .run(
                """
fun g(@Strict x: Long): Long = 7

fun main() {
    println(g(1L / 0L))
}
                """.trimIndent(),
            ).map { it.result },
    )
