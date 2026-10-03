// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.9: refusing labels as operands of machine
// operations. The book's base assembler accepts a label wherever a
// primitive expression may stand -- including the operands of an operation
// call -- and hands the operation the label's instruction address, a
// nonsense datum for arithmetic. The edition's assembler is the strict
// variant: the operand check happens once, at assembly time, and the typed
// label-operand error names the label.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Label
import sicp.runtime.Stmt
import sicp.runtime.assemble

/** A machine operation whose operand is written `(label b)`. */
private val labelAsOperandController: List<Stmt> =
    listOf(Assign("t", opSrc("+", reg("a"), labelSrc("b"))))

/** The strict assembler's answer: the typed label-operand error with the
 *  label it names, raised while the machine is being built. The category
 *  and the name are the observable; the wording is not. */
public fun labelOperandOutcome(): String =
    assemble(labelAsOperandController).fold(
        { error -> renderProgramError(error) },
        { "assembled" },
    )

/** The base assembler's behavior, the one the strict variant refuses: a
 *  label standing as a primitive expression is the label's instruction
 *  address. The address is read through the register form the strict rule
 *  still admits, then added to `a` exactly as the permissive operation
 *  would have: `a + address(addr)` with a = 5 is 5. */
public fun labelOperandUnderBaseAssembler(): Long {
    val controller =
        listOf<Stmt>(
            Label("addr"),
            Assign("t", labelSrc("addr")),
        )
    val machine = freshMachine(setOf("a", "t"), machineArithmetic, controller, mapOf("a" to GValue.VLong(5)))
    runToHalt(machine)
    val address = (machine.registers.getValue("t").content as GValue.VInt).value
    val a = (machine.registers.getValue("a").content as GValue.VLong).value
    return a + address
}
