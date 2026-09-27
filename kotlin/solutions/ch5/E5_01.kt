// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.1: design of the iterative factorial machine. The
// statement asks for data-path and controller diagrams (drawn in the book's
// figure notation in solutions/ch5/ex_5_01.md); this file carries the same
// design as a hand transcription and computes the machine's answers, so the
// design that is drawn is the design that runs.

package sicp.ch5.solutions

/** The iterative factorial machine's controller: the diamond is the
 *  `(op >)` test, the two buttons are the `p<-c*p` and `c<-c+1` assigns,
 *  and the trailing `fact-done` label is the stop address. The register
 *  `n` is an input: to operate the machine, put the number in `n` and
 *  start the controller at `fact-loop`, the section's convention. */
public val factorialController: List<HandInstruction> =
    listOf(
        HLabelDef("fact-loop"),
        HTest(">", listOf(argReg("counter"), argReg("n"))),
        HBranch("fact-done"),
        HAssignOp("product", "*", listOf(argReg("counter"), argReg("product"))),
        HAssignOp("counter", "+", listOf(argReg("counter"), argNum(1))),
        HGotoLabel("fact-loop"),
        HLabelDef("fact-done"),
    )

private val factorialSim = HandSim(factorialController, handArithOps)

/** Run the machine on [n]: registers initialized the way the procedure
 *  initializes them, `(iter 1 1)`, with `n` as the input. */
public fun runFactorialMachine(n: Long): HandVal {
    val outcome = factorialSim.run(mapOf("n" to HNum(n), "product" to HNum(1), "counter" to HNum(1)))
    return (outcome as HandHalted).answer("product")
}

/** The machine's answers on n = 0, 1, 5, 10. */
public fun factorialMachineRuns(): List<String> = listOf(0L, 1L, 5L, 10L).map { n -> render(runFactorialMachine(n)) }
