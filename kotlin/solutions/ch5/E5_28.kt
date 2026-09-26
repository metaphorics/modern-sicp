// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.28: the evaluator with its tail recursion removed.
// The naive sequence evaluation of the 5.4.2 footnote replaces the book's
// `ev-sequence`: every expression of a sequence is evaluated across a
// saved `continue`, so no expression is in tail position and a tail call
// pushes. Rerunning the 5.26 and 5.27 experiments shows both rows of the
// book's demonstration: the iterative factorial's maximum depth, constant
// at 10 under the book's evaluator, now grows with n, and the recursive
// factorial's depth keeps growing.

package sicp.ch5.solutions

import sicp.ch5.evaluatorControllerFragments
import sicp.ch5.labelSrc
import sicp.ch5.monitoredDriver
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test

/** The naive sequence evaluation of the 5.4.2 footnote: no expression is
 *  in tail position, so a tail call pushes. */
private val naiveEvSequence: List<Stmt> =
    listOf(
        Label("ev-sequence"),
        Test(sicp.runtime.OpCond("no-more-exps?", listOf(reg("unev")))),
        Branch("ev-sequence-end"),
        Assign("exp", opSrc("first-exp", reg("unev"))),
        Save("unev"),
        Save("env"),
        Assign("continue", labelSrc("ev-sequence-continue")),
        Goto(GotoTarget.Lbl("eval-dispatch")),
        Label("ev-sequence-continue"),
        Restore("env"),
        Restore("unev"),
        Assign("unev", opSrc("rest-exps", reg("unev"))),
        Goto(GotoTarget.Lbl("ev-sequence")),
        Label("ev-sequence-end"),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
    )

/** The monitored controller with the naive sequence evaluation. */
private val nonTailController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "driver" -> monitoredDriver
            "ev-sequence" -> naiveEvSequence
            else -> stmts
        }
    }

/** One measured factorial call on the non-tail-recursive evaluator. */
private fun measureNonTail(
    source: String,
    n: Int,
): Stats {
    val counters = statsOf(runMonitoredOn(nonTailController, "$source\n(factorial $n)"))
    return counters.lastOrNull() ?: error("the call printed no stack statistics")
}

/** The exercise's reruns for n = 1 to 5: both tables, then the growth
 *  answer. */
public fun nonTailRecursiveMeasurements(): List<String> {
    val ns = listOf(1, 2, 3, 4, 5)
    val iterative = ns.map { measureNonTail(iterativeFactorialSource, it) }
    val recursive = ns.map { measureNonTail(recursiveFactorialSource, it) }
    val table =
        ns.zip(iterative) { n, s -> renderStats("non-tail iterative factorial", n, s) } +
            ns.zip(recursive) { n, s -> renderStats("non-tail recursive factorial", n, s) }
    val depths = iterative.map { it.depth }
    val grows = depths.zipWithNext().all { (a, b) -> b > a }
    return table + listOf("iterative maximum depth now grows with n: $grows")
}
