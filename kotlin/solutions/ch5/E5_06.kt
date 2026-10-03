// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program fib-machine in SICP section 5.1.4
//
// Chapter 5, exercise 5.6: the redundant save/restore pair in the Fibonacci
// controller. Each recursion level needs its caller's return address once:
// pushed before the first call, popped by `afterfib-n-2`'s restore after
// the second. The original pushes it twice, because `afterfib-n-1` first
// restores it (into `continue`, which the second call's setup clobbers)
// and then the second call saves it again. Removing `afterfib-n-1`'s
// `(restore continue)` together with the second call's `(save continue)`
// keeps the value on the stack across the whole level, pushed once and
// popped once.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test

/** The Figure 5.12 controller with the redundant pair removed. */
public val fibModifiedController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("fib-done")),
        Label("fib-loop"),
        Test(opCond("<", reg("n"), constV(2))),
        Branch("immediate-answer"),
        Save("continue"),
        Assign("continue", labelSrc("afterfib-n-1")),
        Save("n"),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-1"),
        Restore("n"),
        Assign("n", opSrc("-", reg("n"), constV(2))),
        Assign("continue", labelSrc("afterfib-n-2")),
        Save("val"),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-2"),
        Assign("n", reg("val")),
        Restore("val"),
        Restore("continue"),
        Assign("val", opSrc("+", reg("val"), reg("n"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("immediate-answer"),
        Assign("val", reg("n")),
        Goto(GotoTarget.ByReg("continue")),
        Label("fib-done"),
    )

internal fun fibAnswer(
    controller: List<Stmt>,
    n: Long,
): String {
    val machine =
        freshMachine(
            setOf("n", "val", "continue"),
            machineArithmetic,
            controller,
            mapOf("n" to GValue.VLong(n), "val" to GValue.VLong(0)),
        )
    runToHalt(machine)
    return render(machine.registers.getValue("val").content)
}

/** One run's counts: instructions executed (labels excluded) and saves. */
private data class FibCounts(
    val steps: Long,
    val saves: Long,
)

private fun fibCounts(
    controller: List<Stmt>,
    n: Long,
): FibCounts {
    val machine =
        freshMachine(
            setOf("n", "val", "continue"),
            machineArithmetic,
            controller,
            mapOf("n" to GValue.VLong(n), "val" to GValue.VLong(0)),
        )
    return FibCounts(runToHalt(machine), machine.stack.pushes)
}

/** The report: both machines answer alike on fib 10, and the run's counts
 *  on fib 6 show each of the tree's internal calls shedding one save and
 *  one restore. */
public fun modifiedFibReport(): List<String> {
    val original = fibCounts(fibController, 6)
    val modified = fibCounts(fibModifiedController, 6)
    return listOf(
        fibAnswer(fibController, 10),
        fibAnswer(fibModifiedController, 10),
        "steps=${original.steps} saves=${original.saves}",
        "steps=${modified.steps} saves=${modified.saves}",
    )
}

/** The tempting wrong pairing: removing `afterfib-n-2`'s restore together
 *  with the second call's save leaves `continue` holding `afterfib-n-2`
 *  at each level's end, whose `(goto (reg continue))` then loops with the
 *  stack shrinking until a restore reaches past its bottom. */
public val fibWrongPairController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("fib-done")),
        Label("fib-loop"),
        Test(opCond("<", reg("n"), constV(2))),
        Branch("immediate-answer"),
        Save("continue"),
        Assign("continue", labelSrc("afterfib-n-1")),
        Save("n"),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-1"),
        Restore("n"),
        Restore("continue"),
        Assign("n", opSrc("-", reg("n"), constV(2))),
        Assign("continue", labelSrc("afterfib-n-2")),
        Save("val"),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-2"),
        Assign("n", reg("val")),
        Restore("val"),
        Assign("val", opSrc("+", reg("val"), reg("n"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("immediate-answer"),
        Assign("val", reg("n")),
        Goto(GotoTarget.ByReg("continue")),
        Label("fib-done"),
    )

/** How the wrong pairing ends on fib [n]: the typed machine fault names
 *  the impossible state, a restore reaching past the bottom of the stack.
 *  The category and the instruction are the observable; the wording is
 *  not. */
public fun fibWrongPairEnding(n: Long): String {
    val machine =
        freshMachine(
            setOf("n", "val", "continue"),
            machineArithmetic,
            fibWrongPairController,
            mapOf("n" to GValue.VLong(n), "val" to GValue.VLong(0)),
        )
    return when (val outcome = runStaged(machine)) {
        is MachineOutcome.Stuck -> stuckLine(outcome.instruction, outcome.error)
        is MachineOutcome.Halted -> "halted after ${outcome.instructions} instructions"
        is MachineOutcome.OutOfBudget -> "not halting: ${outcome.instructions} instructions without a halt"
    }
}
