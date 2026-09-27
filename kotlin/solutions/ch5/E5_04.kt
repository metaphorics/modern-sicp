// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.4: controller sequences for the recursive expt
// machine and its iterative reformulation. The recursive controller saves
// only `continue` around the recursive call: the subproblem clobbers `n`
// (decrementing it) but never `b`, so `b` needs no save. The iterative
// controller has no stack and no `continue` at all.

package sicp.ch5.solutions

/** The recursive expt machine: registers `b`, `n`, `val`, `continue`,
 *  one saved register per level. */
public val exptRecursiveController: List<HandInstruction> =
    listOf(
        HAssignLabel("continue", "expt-done"),
        HLabelDef("expt-loop"),
        HTest("=", listOf(argReg("n"), argNum(0))),
        HBranch("base-expt"),
        HSave("continue"),
        HAssignLabel("continue", "after-expt"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(1))),
        HGotoLabel("expt-loop"),
        HLabelDef("after-expt"),
        HRestore("continue"),
        HAssignOp("val", "*", listOf(argReg("b"), argReg("val"))),
        HGotoReg("continue"),
        HLabelDef("base-expt"),
        HAssignConst("val", HNum(1)),
        HGotoReg("continue"),
        HLabelDef("expt-done"),
    )

/** The iterative expt machine: registers `b`, `counter`, `product`, the
 *  state wholly in the registers. */
public val exptIterativeController: List<HandInstruction> =
    listOf(
        HAssignReg("counter", "n"),
        HAssignConst("product", HNum(1)),
        HLabelDef("expt-iter"),
        HTest("=", listOf(argReg("counter"), argNum(0))),
        HBranch("expt-done"),
        HAssignOp("product", "*", listOf(argReg("b"), argReg("product"))),
        HAssignOp("counter", "-", listOf(argReg("counter"), argNum(1))),
        HGotoLabel("expt-iter"),
        HLabelDef("expt-done"),
    )

private val exptRecursiveSim = HandSim(exptRecursiveController, handArithOps)

private val exptIterativeSim = HandSim(exptIterativeController, handArithOps)

/** Run a machine with `b` and `n` as inputs and answer [answerReg]. */
private fun runExpt(
    sim: HandSim,
    b: Long,
    n: Long,
    answerReg: String,
): HandVal {
    val outcome = sim.run(mapOf("b" to HNum(b), "n" to HNum(n)))
    return (outcome as HandHalted).answer(answerReg)
}

/** Recursive expt on (2, 10) and (3, 5), then iterative on the same
 *  inputs: identical answers, different machinery. The recursive machine
 *  answers in `val`, the iterative one in `product`. */
public fun exptMachineRuns(): List<String> =
    listOf(
        runExpt(exptRecursiveSim, b = 2, n = 10, answerReg = "val"),
        runExpt(exptRecursiveSim, b = 3, n = 5, answerReg = "val"),
        runExpt(exptIterativeSim, b = 2, n = 10, answerReg = "product"),
        runExpt(exptIterativeSim, b = 3, n = 5, answerReg = "product"),
    ).map { render(it) }

/** The recursive machine's peak stack depth on (b, n): one saved
 *  `continue` per level, exactly n deep. */
public fun exptRecursiveMaxDepth(
    b: Long,
    n: Long,
): Int = (exptRecursiveSim.run(mapOf("b" to HNum(b), "n" to HNum(n))) as HandHalted).maxDepth
