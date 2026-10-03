// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.28

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.28: why the operator forces. Application dispatches on the
// procedure VALUE in the operator position, so what sits there must be an
// actual procedure, not the thunk an argument would be. In `id(::add)(2, 3)`
// the operator expression `id(::add)` evaluates to `id`'s parameter, which
// the application delayed; forcing it yields the `add` function and the
// call answers 5. The lazy experiment makes that forcing an invariant
// (the operator is forced before application), and the counterfactual --
// applying the thunk itself without forcing -- cannot survive source
// typing at all: the checker rejects the call before any guest effect,
// which is exactly the failed-admission-before-effect boundary.

/** The operator forced: `id(::add)(2, 3)` dispatches on the `add`
 * function. => "5\n" */
internal val FORCED_OPERATOR_PROGRAM: String =
    """
fun add(a: Long, b: Long): Long = a + b

fun id(x: (Long, Long) -> Long): (Long, Long) -> Long = x

fun main() {
    println(id(::add)(2L, 3L))
}
    """.trimIndent()

/** The counterfactual: the delayed operator applied without forcing. The
 * checker rejects the call before the program runs, so the effect before
 * it never happens. */
internal val UNFORCED_OPERATOR_PROGRAM: String =
    """
fun add(a: Long, b: Long): Long = a + b

fun main() {
    println("before")
    val op = thunk { ::add }
    println(op(2L, 3L))
}
    """.trimIndent()

/** The operator forced: the call dispatches on the `add` function.
 * => "5\n" */
public fun forcedOperatorTranscript(): String = outcomeText(LazyModule.run(FORCED_OPERATOR_PROGRAM).map { it.result })

/** The operator unforced: the call is rejected at admission, before the
 * `before` effect can run. => "Error: <rejection category>\n" */
public fun unforcedOperatorTranscript(): String = outcomeText(LazyModule.run(UNFORCED_OPERATOR_PROGRAM).map { it.result })
