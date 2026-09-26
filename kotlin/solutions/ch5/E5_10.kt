// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.10: a new instruction syntax behind isolated
// syntax procedures. The new surface keeps the book's instructions and
// adds three register-to-register forms -- `(cpy a b)` for `(assign a
// (reg b))`, `(inc r)` and `(dec r)` for the add-one and subtract-one
// assigns -- each expanded by one procedure in this file and nowhere else.
// The simulator proper never sees the new forms: `syntaxExpand` runs
// before assembly, so changing the surface syntax touches exactly these
// four procedures.

package sicp.ch5.solutions

import sicp.ch5.Machine
import sicp.ch5.arithOperations
import sicp.ch5.assemble
import sicp.ch5.constV
import sicp.ch5.getRegisterContents
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
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VInt

/** The new surface: the book's instructions wrapped in [NBook], plus the
 *  three new register-to-register forms. */
public sealed interface NStmt

/** `(cpy a b)`: the register-to-register assign. */
public data class NCpy(
    val to: Reg,
    val from: Reg,
) : NStmt

/** `(inc r)`: add one to a register in place. */
public data class NInc(
    val reg: Reg,
) : NStmt

/** `(dec r)`: subtract one from a register in place. */
public data class NDec(
    val reg: Reg,
) : NStmt

/** One of the book's instructions, unchanged. */
public data class NBook(
    val stmt: Stmt,
) : NStmt

/** The isolated syntax procedures: every new form expands to exactly one
 *  book instruction, at assembly time, before labels are scanned. */
public fun syntaxExpand(controller: List<NStmt>): List<Stmt> =
    controller.map { stmt ->
        when (stmt) {
            is NCpy -> Assign(stmt.to, reg(stmt.from))
            is NInc -> Assign(stmt.reg, opSrc("+", reg(stmt.reg), constV(1)))
            is NDec -> Assign(stmt.reg, opSrc("-", reg(stmt.reg), constV(1)))
            is NBook -> stmt.stmt
        }
    }

/** The book's gcd machine written in the new syntax: the two register
 *  shuffles become `cpy` lines. */
private val gcdControllerNewSyntax: List<NStmt> =
    listOf(
        NBook(Label("test-b")),
        NBook(Test(opCond("=", reg("b"), constV(0)))),
        NBook(Branch("gcd-done")),
        NBook(Assign("t", opSrc("rem", reg("a"), reg("b")))),
        NCpy("a", "b"),
        NCpy("b", "t"),
        NBook(Goto(GotoTarget.Lbl("test-b"))),
        NBook(Label("gcd-done")),
    )

/** A countdown that exercises `dec` and `inc`: while `count` is nonzero,
 *  decrement it and increment `sum`, which starts at zero. */
private val countdownController: List<NStmt> =
    listOf(
        NBook(Assign("count", reg("n"))),
        NBook(Assign("sum", constV(0))),
        NBook(Label("loop")),
        NBook(Test(opCond("=", reg("count"), constV(0)))),
        NBook(Branch("done")),
        NDec("count"),
        NInc("sum"),
        NBook(Goto(GotoTarget.Lbl("loop"))),
        NBook(Label("done")),
    )

/** Both machines in the new syntax: the gcd answers its usual 2, the
 *  countdown sums 1 for each of its three decrements. */
public fun newSyntaxRuns(): List<String> {
    val gcd =
        machineRun {
            val machine = Machine(listOf("a", "b", "t"), arithOperations)
            machine.install(assemble(syntaxExpand(gcdControllerNewSyntax), machine))
            machine.setRegisterContents("a", VInt(206))
            machine.setRegisterContents("b", VInt(40))
            machine.start()
            machine.getRegisterContents("a")
        }
    val countdown =
        machineRun {
            val machine = Machine(listOf("n", "count", "sum"), arithOperations)
            machine.install(assemble(syntaxExpand(countdownController), machine))
            machine.setRegisterContents("n", VInt(3))
            machine.start()
            machine.getRegisterContents("sum")
        }
    return listOf(
        "gcd(206, 40) in the new syntax = $gcd",
        "countdown(3) sum = $countdown",
    )
}
