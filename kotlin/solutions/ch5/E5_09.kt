// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.9: refusing labels as operands of machine
// operations. The book's base assembler accepts a label wherever a
// primitive expression may stand -- including the operands of an operation
// call -- and hands the operation the label's instruction address, a
// nonsense datum for arithmetic. Exercise 5.9's simulator is the standard
// assembler run with strictLabels: the operand check happens once, at
// assembly time, and the typed label-operand error names the label.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.Machine
import sicp.ch5.arithOperations
import sicp.ch5.assemble
import sicp.ch5.constV
import sicp.ch5.getRegisterContents
import sicp.ch5.labelSrc
import sicp.ch5.makeMachine
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.setRegisterContents
import sicp.runtime.Assign
import sicp.runtime.Label
import sicp.runtime.Stmt
import sicp.runtime.VInt

/** A machine operation whose operand is written `(label b)`. */
private val labelAsOperandController: List<Stmt> =
    listOf(Assign("t", opSrc("+", reg("a"), labelSrc("b"))))

/** The strict assembler's answer: the typed label-operand error, raised
 *  while the machine is being built. */
public fun labelOperandOutcome(): String {
    val outcome =
        either {
            val machine = Machine(listOf("a", "t"), arithOperations)
            machine.install(assemble(labelAsOperandController, machine, strictLabels = true))
            machine
        }
    return outcome.fold({ e -> e.toString() }, { "assembled" })
}

/** The book's base assembler accepts the same controller and computes the
 *  label's address (the index of the first instruction), the behavior the
 *  strict variant refuses: `a + address(b)` with a = 5 is 5. */
public fun labelOperandUnderBaseAssembler(): Long =
    machineRun {
        val controller =
            listOf<Stmt>(
                Label("addr"),
                Assign("t", opSrc("+", reg("a"), labelSrc("addr"))),
            )
        val machine = makeMachine(listOf("a", "t"), arithOperations, controller)
        machine.setRegisterContents("a", VInt(5))
        machine.start()
        (machine.getRegisterContents("t") as VInt).n
    }
