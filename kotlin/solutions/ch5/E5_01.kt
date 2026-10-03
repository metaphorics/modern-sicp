// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program fact-machine in SICP section 5.1.1
//
// Chapter 5, exercise 5.1: design of the iterative factorial machine. The
// statement asks for data-path and controller diagrams (drawn in the book's
// figure notation in solutions/ch5/ex_5_01.md); this file carries the same
// design as typed controller data and computes the machine's answers, so
// the design that is drawn is the design that runs.

package sicp.ch5.solutions

import sicp.guest.GValue
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Stmt
import sicp.runtime.Test

/** The iterative factorial machine's controller: the diamond is the
 *  `(op >)` test, the two buttons are the `p<-c*p` and `c<-c+1` assigns,
 *  and the trailing `fact-done` label is the stop address. The register
 *  `n` is an input: to operate the machine, put the number in `n` and
 *  start the controller at `fact-loop`, the section's convention. */
public val factorialController: List<Stmt> =
    listOf(
        Label("fact-loop"),
        Test(opCond(">", reg("counter"), reg("n"))),
        Branch("fact-done"),
        Assign("product", opSrc("*", reg("counter"), reg("product"))),
        Assign("counter", opSrc("+", reg("counter"), constV(1))),
        Goto(GotoTarget.Lbl("fact-loop")),
        Label("fact-done"),
    )

/** Run the machine on [n]: registers initialized the way the procedure
 *  initializes them, `(iter 1 1)`, with `n` as the input. */
public fun runFactorialMachine(n: Long): GValue {
    val machine =
        freshMachine(
            setOf("n", "product", "counter"),
            machineArithmetic,
            factorialController,
            mapOf("n" to GValue.VLong(n), "product" to GValue.VLong(1), "counter" to GValue.VLong(1)),
        )
    runToHalt(machine)
    return machine.registers.getValue("product").content
}

/** The machine's answers on n = 0, 1, 5, 10. */
public fun factorialMachineRuns(): List<String> = listOf(0L, 1L, 5L, 10L).map { n -> render(runFactorialMachine(n)) }
