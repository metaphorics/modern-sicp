// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program expt-machine in SICP section 5.1.3
//
// Chapter 5, exercise 5.4: controller sequences for the recursive expt
// machine and its iterative reformulation. The recursive controller saves
// only `continue` around the recursive call: the subproblem clobbers `n`
// (decrementing it) but never `b`, so `b` needs no save. The iterative
// controller has no stack and no `continue` at all.

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

/** The recursive expt machine: registers `b`, `n`, `val`, `continue`,
 *  one saved register per level. */
public val exptRecursiveController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("expt-done")),
        Label("expt-loop"),
        Test(opCond("=", reg("n"), constV(0))),
        Branch("base-expt"),
        Save("continue"),
        Assign("continue", labelSrc("after-expt")),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Goto(GotoTarget.Lbl("expt-loop")),
        Label("after-expt"),
        Restore("continue"),
        Assign("val", opSrc("*", reg("b"), reg("val"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("base-expt"),
        Assign("val", constV(1)),
        Goto(GotoTarget.ByReg("continue")),
        Label("expt-done"),
    )

/** The iterative expt machine: registers `b`, `counter`, `product`, the
 *  state wholly in the registers. */
public val exptIterativeController: List<Stmt> =
    listOf(
        Assign("counter", reg("n")),
        Assign("product", constV(1)),
        Label("expt-iter"),
        Test(opCond("=", reg("counter"), constV(0))),
        Branch("expt-done"),
        Assign("product", opSrc("*", reg("b"), reg("product"))),
        Assign("counter", opSrc("-", reg("counter"), constV(1))),
        Goto(GotoTarget.Lbl("expt-iter")),
        Label("expt-done"),
    )

/** Run one expt machine with inputs `b`, `n` and answer [answerReg]; every
 *  run starts from a fresh machine so no register carries over. The 5.7
 *  simulator runs reuse this runner. */
internal fun runExpt(
    controller: List<Stmt>,
    b: Long,
    n: Long,
    answerReg: String,
): GValue {
    val machine =
        freshMachine(
            setOf("b", "n", "continue", "val", "counter", "product"),
            machineArithmetic,
            controller,
            mapOf("b" to GValue.VLong(b), "n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return machine.registers.getValue(answerReg).content
}

/** Recursive expt on (2, 10) and (3, 5), then iterative on the same
 *  inputs: identical answers, different machinery. The recursive machine
 *  answers in `val`, the iterative one in `product`. */
public fun exptMachineRuns(): List<String> =
    listOf(
        runExpt(exptRecursiveController, 2, 10, "val"),
        runExpt(exptRecursiveController, 3, 5, "val"),
        runExpt(exptIterativeController, 2, 10, "product"),
        runExpt(exptIterativeController, 3, 5, "product"),
    ).map { render(it) }

/** The recursive machine's peak stack depth on (b, n): one saved
 *  `continue` per level, exactly n deep. */
public fun exptRecursiveMaxDepth(
    b: Long,
    n: Long,
): Int {
    val machine =
        freshMachine(
            setOf("b", "n", "continue", "val", "counter", "product"),
            machineArithmetic,
            exptRecursiveController,
            mapOf("b" to GValue.VLong(b), "n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return machine.stack.maxDepth.toInt()
}
