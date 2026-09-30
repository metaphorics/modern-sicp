// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.10: a new instruction syntax behind isolated
// syntax procedures. The new surface keeps the book's instructions and
// adds three register-to-register forms -- `cpy a b` for `assign a (reg
// b)`, `inc r` and `dec r` for the add-one and subtract-one assigns --
// each expanded by one procedure in this file and nowhere else. The
// simulator proper never sees the new forms: `syntaxExpand` runs before
// assembly, so changing the surface syntax touches exactly these syntax
// procedures.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Reg
import sicp.runtime.Stmt
import sicp.runtime.Test

/** The new surface: the book's instructions wrapped in [NBook], plus the
 *  three new register-to-register forms. */
public sealed interface NStmt

/** `cpy a b`: the register-to-register assign. */
public data class NCpy(
    val to: Reg,
    val from: Reg,
) : NStmt

/** `inc r`: add one to a register in place. */
public data class NInc(
    val reg: Reg,
) : NStmt

/** `dec r`: subtract one from a register in place. */
public data class NDec(
    val reg: Reg,
) : NStmt

/** One of the book's instructions, unchanged. */
public data class NBook(
    val stmt: Stmt,
) : NStmt

/** The isolated syntax procedures: every new form expands to exactly one
 *  book instruction, before labels are scanned. */
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
        freshMachine(
            setOf("a", "b", "t"),
            machineArithmetic,
            syntaxExpand(gcdControllerNewSyntax),
            mapOf("a" to GValue.VLong(206), "b" to GValue.VLong(40)),
        )
    runToHalt(gcd)
    val countdown =
        freshMachine(
            setOf("n", "count", "sum"),
            machineArithmetic,
            syntaxExpand(countdownController),
            mapOf("n" to GValue.VLong(3)),
        )
    runToHalt(countdown)
    return listOf(
        "gcd(206, 40) in the new syntax = ${render(gcd.registers.getValue("a").content)}",
        "countdown(3) sum = ${render(countdown.registers.getValue("sum").content)}",
    )
}
