// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.13: deriving the register list from the controller
// text. Instead of naming the registers by hand, the machine's register
// list is the assembler's own scan of the controller (the scan the 5.12
// summary uses for its register line): every assign target, every register
// source and operand, every goto, save, and restore register. The machine
// then assembles without a hand-written register list.

package sicp.ch5.solutions

import sicp.ch5.arithOperations
import sicp.ch5.controllerRegisters
import sicp.ch5.getRegisterContents
import sicp.ch5.makeMachine
import sicp.ch5.setRegisterContents
import sicp.runtime.VInt

/** The gcd machine with its registers derived from the controller, then
 *  run: same answer as the hand-listed machine, and the derived register
 *  set names exactly the controller's registers plus the flag. */
public fun derivedRegisterRuns(): List<String> =
    machineRun {
        val derived = controllerRegisters(gcdController).sorted()
        val machine = makeMachine(derived, arithOperations, gcdController)
        machine.setRegisterContents("a", VInt(206))
        machine.setRegisterContents("b", VInt(40))
        machine.start()
        val registers =
            machine.registers.keys
                .sorted()
                .joinToString(" ")
        listOf(
            "derived registers: ${derived.joinToString(" ")}",
            "gcd(206, 40) = ${machine.getRegisterContents("a")}",
            "allocated registers: $registers",
        )
    }
