// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.7: the machines designed in exercise 5.4 (the
// recursive-exponent machine and its iterative reformulation) running on the
// real 5.2 simulator. The controllers are transcribed line for line from the
// 5.4 solutions into the runtime's controller language; each simulated run
// is paired with the direct host computation of the same exponent as an
// oracle, so a machine run is correct exactly when its answer equals the
// host's. The book's own example machine of 5.2, the gcd machine, is built
// here once and reused by the later exercises.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.arithOperations
import sicp.ch5.constV
import sicp.ch5.getRegisterContents
import sicp.ch5.labelSrc
import sicp.ch5.makeMachine
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.setRegisterContents
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Reg
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VInt
import sicp.runtime.Value

/** A raised [MachineError] escaping [machineRun]: the host's wrapper for
 *  the machine's typed halt, carrying the error intact. */
internal class MachineStuck(
    val error: MachineError,
) : RuntimeException(error.toString())

/** Runs one block in a machine scope; a raised [MachineError] escapes as a
 *  [MachineStuck], so successful pins stay honest and error tests catch the
 *  typed error instead. */
internal fun <A> machineRun(block: Raise<MachineError>.() -> A): A = either(block).fold({ e -> throw MachineStuck(e) }, { it })

/** The book's gcd machine of 5.2: registers `a`, `b`, `t`, the section's
 *  arithmetic operations, and the controller of 5.1.1's reduction step. */
public val gcdController: List<Stmt> =
    listOf(
        Label("test-b"),
        Test(opCond("=", reg("b"), constV(0))),
        Branch("gcd-done"),
        Assign("t", opSrc("rem", reg("a"), reg("b"))),
        Assign("a", reg("b")),
        Assign("b", reg("t")),
        Goto(GotoTarget.Lbl("test-b")),
        Label("gcd-done"),
    )

/** A fresh gcd machine, assembled and ready for inputs. */
public fun gcdMachine(): Machine =
    machineRun {
        makeMachine(listOf("a", "b", "t"), arithOperations, gcdController)
    }

/** The recursive-exponent machine of exercise 5.4: `continue` is saved
 *  before the subproblem and restored after the multiplication, one
 *  saved `continue` per level; the subproblem clobbers `n` but never
 *  `b`. */
public val exptRecursiveStmts: List<Stmt> =
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
        Assign("val", opSrc("*", reg("b"), reg("val"))),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
        Label("base-expt"),
        Assign("val", constV(1)),
        Goto(GotoTarget.ByReg("continue")),
        Label("expt-done"),
    )

/** The iterative-exponent machine of exercise 5.4: `counter` and
 *  `product`, no stack and no `continue`. */
public val exptIterativeStmts: List<Stmt> =
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

/** Runs one expt machine with inputs `b`, `n` and reads the answer from
 *  [answerReg]; every run starts from a fresh machine so no register
 *  carries over. */
private fun runExpt(
    controller: List<Stmt>,
    b: Long,
    n: Long,
    answerReg: Reg,
): Value =
    machineRun {
        val machine = makeMachine(listOf("b", "n", "continue", "val", "counter", "product"), arithOperations, controller)
        machine.setRegisterContents("b", VInt(b))
        machine.setRegisterContents("n", VInt(n))
        machine.start()
        machine.getRegisterContents(answerReg)
    }

/** The host oracle: b to the n by repeated multiplication, the same
 *  reduction the machines execute. */
private fun hostExpt(
    b: Long,
    n: Long,
): Long {
    var product = 1L
    repeat(n.toInt()) { product *= b }
    return product
}

/** Both 5.4 machines on both inputs, each line pairing the machine's
 *  answer with its oracle's answer. The recursive machine answers in
 *  `val`, the iterative one in `product`. */
public fun simulatedExptRuns(): List<String> =
    listOf(
        Triple("recursive", exptRecursiveStmts, "val"),
        Triple("iterative", exptIterativeStmts, "product"),
    ).flatMap { (kind, controller, answerReg) ->
        listOf(2L to 10L, 3L to 5L).map { (b, n) ->
            val answer = runExpt(controller, b, n, answerReg)
            "$kind expt($b, $n) = $answer (host ${hostExpt(b, n)})"
        }
    }
