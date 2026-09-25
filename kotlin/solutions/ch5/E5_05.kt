// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.5: hand simulations of the recursive factorial
// machine of Figure 5.11 and the Fibonacci machine of Figure 5.12. The
// two controllers are transcribed line for line from the figures; the
// hand model of Handsim.kt steps them the way a reader does, so every
// trace value is the hand model's computation.

package sicp.ch5.solutions

/** Figure 5.11: the recursive factorial machine. Registers `n`, `val`,
 *  `continue`; `n` and `continue` saved before each recursive call and
 *  restored on return; `val` never saved. */
public val recursiveFactorialController: List<HandInstruction> =
    listOf(
        HAssignLabel("continue", "fact-done"),
        HLabelDef("fact-loop"),
        HTest("=", listOf(argReg("n"), argNum(1))),
        HBranch("base-case"),
        HSave("continue"),
        HSave("n"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(1))),
        HAssignLabel("continue", "after-fact"),
        HGotoLabel("fact-loop"),
        HLabelDef("after-fact"),
        HRestore("n"),
        HRestore("continue"),
        HAssignOp("val", "*", listOf(argReg("n"), argReg("val"))),
        HGotoReg("continue"),
        HLabelDef("base-case"),
        HAssignConst("val", HNum(1)),
        HGotoReg("continue"),
        HLabelDef("fact-done"),
    )

/** Figure 5.12: the Fibonacci machine. Two recursive calls per level, so
 *  `val` joins the saved registers around the second call. */
public val fibController: List<HandInstruction> =
    listOf(
        HAssignLabel("continue", "fib-done"),
        HLabelDef("fib-loop"),
        HTest("<", listOf(argReg("n"), argNum(2))),
        HBranch("immediate-answer"),
        HSave("continue"),
        HAssignLabel("continue", "afterfib-n-1"),
        HSave("n"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(1))),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-1"),
        HRestore("n"),
        HRestore("continue"),
        HAssignOp("n", "-", listOf(argReg("n"), argNum(2))),
        HSave("continue"),
        HAssignLabel("continue", "afterfib-n-2"),
        HSave("val"),
        HGotoLabel("fib-loop"),
        HLabelDef("afterfib-n-2"),
        HAssignReg("n", "val"),
        HRestore("val"),
        HRestore("continue"),
        HAssignOp("val", "+", listOf(argReg("val"), argReg("n"))),
        HGotoReg("continue"),
        HLabelDef("immediate-answer"),
        HAssignReg("val", "n"),
        HGotoReg("continue"),
        HLabelDef("fib-done"),
    )

internal val recursiveFactorialSim = HandSim(recursiveFactorialController, handArithOps)

internal val fibSim = HandSim(fibController, handArithOps)

/** The full trace of one hand simulation: one line per significant point
 *  (a save, a restore, a taken branch, a return through `continue`),
 *  then the answer line. */
public fun handTrace(
    sim: HandSim,
    initialRegisters: Map<String, HandVal>,
    answerRegister: String,
): List<String> {
    val outcome = sim.run(initialRegisters)
    return (outcome as HandHalted).let { halted -> halted.events + "answer ${render(halted.answer(answerRegister))}" }
}

/** Hand-simulate the factorial machine on n = 3 and the Fibonacci machine
 *  on n = 3: each descends past at least one recursive call, and the
 *  Fibonacci machine branches at both call sites. */
public fun handSimulationTraces(): List<String> =
    handTrace(recursiveFactorialSim, mapOf("n" to HNum(3), "val" to HNum(0)), "val") +
        handTrace(fibSim, mapOf("n" to HNum(3), "val" to HNum(0)), "val")

/** The factorial machine's peak stack depth on n = 3: two frames, each
 *  holding `continue` and `n`. */
public fun factorialMaxDepth(n: Long): Int = (recursiveFactorialSim.run(mapOf("n" to HNum(n), "val" to HNum(0))) as HandHalted).maxDepth
