// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program gcd-machine in SICP section 5.1.1
//
// Chapter 5, exercise 5.7: the machines designed in exercise 5.4 (the
// recursive-exponent machine and its iterative reformulation) running on the
// real 5.2 simulator. The controllers are the typed instruction data of the
// 5.4 solutions; each simulated run is paired with the direct host
// computation of the same exponent as an oracle, so a machine run is correct
// exactly when its answer equals the host's. The book's own example machine
// of 5.2, the gcd machine, is built here once and reused by the later
// exercises.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.Stmt
import sicp.runtime.Test

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
public fun gcdMachine(): Machine = freshMachine(setOf("a", "b", "t"), machineArithmetic, gcdController)

/** Operate the gcd machine on (a, b) and answer the result left in `a`. */
public fun runGcd(
    a: Long,
    b: Long,
): Long {
    val machine = gcdMachine()
    machine.registers.getValue("a").content = GValue.VLong(a)
    machine.registers.getValue("b").content = GValue.VLong(b)
    runToHalt(machine)
    return (machine.registers.getValue("a").content as GValue.VLong).value
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
        Triple("recursive", exptRecursiveController, "val"),
        Triple("iterative", exptIterativeController, "product"),
    ).flatMap { (kind, controller, answerReg) ->
        listOf(2L to 10L, 3L to 5L).map { (b, n) ->
            val answer = render(runExpt(controller, b, n, answerReg))
            "$kind expt($b, $n) = $answer (host ${hostExpt(b, n)})"
        }
    }
