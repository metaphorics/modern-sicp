// SPDX-License-Identifier: GPL-3.0-only
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

/** The Figure 5.12 controller with the redundant pair removed. */
public val fibModifiedController: List<HandInstruction> =
    listOf(
        HAssignLabel("continue", "fib-done"),
        HLabelDef("fib-loop"),
        HTest("<", listOf(argReg("n"), argNum(2))),
        HBranch("immediate-answer"),
        HSave("continue"),
        HAssignLabel("continue", "afterfib-n-1"),
        HSave("n"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(1))),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-1"),
        HRestore("n"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(2))),
        HAssignLabel("continue", "afterfib-n-2"),
        HSave("val"),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-2"),
        HAssignReg("n", "val"),
        HRestore("val"),
        HRestore("continue"),
        HAssignOp("val", "+", listOf(argReg("val"), argReg("n"))),
        HGotoReg("continue"),
        HLabelDef("immediate-answer"),
        HAssignReg("val", "n"),
        HGotoReg("continue"),
        HLabelDef("fib-done"),
    )

internal val fibModifiedSim = HandSim(fibModifiedController, handArithOps)

internal fun fibAnswer(
    sim: HandSim,
    n: Long,
): String {
    val outcome = sim.run(mapOf("n" to HNum(n), "val" to HNum(0)))
    return render((outcome as HandHalted).answer("val"))
}

/** The report: both machines answer alike on fib 10, and the hand model's
 *  counts on fib 6 show each of the tree's internal calls shedding one
 *  save and one restore. */
public fun modifiedFibReport(): List<String> {
    val original = fibSim.run(mapOf("n" to HNum(6), "val" to HNum(0))) as HandHalted
    val modified = fibModifiedSim.run(mapOf("n" to HNum(6), "val" to HNum(0))) as HandHalted
    return listOf(
        fibAnswer(fibSim, n = 10),
        fibAnswer(fibModifiedSim, n = 10),
        "steps=${original.steps} saves=${original.saves}",
        "steps=${modified.steps} saves=${modified.saves}",
    )
}

/** The tempting wrong pairing: removing `afterfib-n-2`'s restore together
 *  with the second call's save leaves `continue` holding `afterfib-n-2`
 *  at each level's end, whose `(goto (reg continue))` then loops with the
 *  stack growing. The hand model answers that the machine does not halt. */
public val fibWrongPairController: List<HandInstruction> =
    listOf(
        HAssignLabel("continue", "fib-done"),
        HLabelDef("fib-loop"),
        HTest("<", listOf(argReg("n"), argNum(2))),
        HBranch("immediate-answer"),
        HSave("continue"),
        HAssignLabel("continue", "afterfib-n-1"),
        HSave("n"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(1))),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-1"),
        HRestore("n"),
        HRestore("continue"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(2))),
        HAssignLabel("continue", "afterfib-n-2"),
        HSave("val"),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-2"),
        HAssignReg("n", "val"),
        HRestore("val"),
        HAssignOp("val", "+", listOf(argReg("val"), argReg("n"))),
        HGotoReg("continue"),
        HLabelDef("immediate-answer"),
        HAssignReg("val", "n"),
        HGotoReg("continue"),
        HLabelDef("fib-done"),
    )

/** How the wrong pairing ends on fib [n]: the hand model names the
 *  impossible state, a restore reaching past the bottom of the stack. */
public fun fibWrongPairEnding(n: Long): String =
    when (
        val outcome =
            HandSim(fibWrongPairController, handArithOps).run(
                mapOf("n" to HNum(n), "val" to HNum(0)),
                fuel = 100_000,
            )
    ) {
        is HandHalted -> "halted with answer ${render(outcome.answer("val"))}"
        is HandStuck -> "stuck: ${outcome.reason}"
        OutOfFuel -> "no halt within the step budget"
    }
