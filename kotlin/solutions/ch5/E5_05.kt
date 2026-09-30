// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs fact-machine and fib-machine in SICP
// section 5.1.4
//
// Chapter 5, exercise 5.5: hand simulations of the recursive factorial
// machine of Figure 5.11 and the Fibonacci machine of Figure 5.12. The
// two controllers are transcribed line for line from the figures; the
// hand simulation steps them the way a reader does, keeping its own view
// of the stack, so every trace line is the reader's annotation of the
// running machine.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test

/** Figure 5.11: the recursive factorial machine. Registers `n`, `val`,
 *  `continue`; `n` and `continue` saved before each recursive call and
 *  restored on return; `val` never saved. */
public val recursiveFactorialController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("fact-done")),
        Label("fact-loop"),
        Test(opCond("=", reg("n"), constV(1))),
        Branch("base-case"),
        Save("continue"),
        Save("n"),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Assign("continue", labelSrc("after-fact")),
        Goto(GotoTarget.Lbl("fact-loop")),
        Label("after-fact"),
        Restore("n"),
        Restore("continue"),
        Assign("val", opSrc("*", reg("n"), reg("val"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("base-case"),
        Assign("val", constV(1)),
        Goto(GotoTarget.ByReg("continue")),
        Label("fact-done"),
    )

/** Figure 5.12: the Fibonacci machine. Two recursive calls per level, so
 *  `val` joins the saved registers around the second call. */
public val fibController: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("fib-done")),
        Label("fib-loop"),
        Test(opCond("<", reg("n"), constV(2))),
        Branch("immediate-answer"),
        Save("continue"),
        Assign("continue", labelSrc("afterfib-n-1")),
        Save("n"),
        Assign("n", opSrc("-", reg("n"), constV(1))),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-1"),
        Restore("n"),
        Restore("continue"),
        Assign("n", opSrc("-", reg("n"), constV(2))),
        Save("continue"),
        Assign("continue", labelSrc("afterfib-n-2")),
        Save("val"),
        Goto(GotoTarget.Lbl("fib-loop")),
        Label("afterfib-n-2"),
        Assign("n", reg("val")),
        Restore("val"),
        Restore("continue"),
        Assign("val", opSrc("+", reg("val"), reg("n"))),
        Goto(GotoTarget.ByReg("continue")),
        Label("immediate-answer"),
        Assign("val", reg("n")),
        Goto(GotoTarget.ByReg("continue")),
        Label("fib-done"),
    )

/** The full trace of one hand simulation: one line per significant point
 *  (a save, a restore, a taken branch, a return through `continue`),
 *  then the answer line. The reader keeps its own stack picture, top
 *  first, the way the book's simulations write it. */
public fun handTrace(
    controller: List<Stmt>,
    initial: Map<String, GValue>,
    answerRegister: String,
): List<String> {
    val machine = freshMachine(setOf("n", "val", "continue"), machineArithmetic, controller, initial)
    val labels = machine.labels
    val stack = ArrayDeque<GValue>()
    val events = mutableListOf<String>()
    while (!machine.halted()) {
        val instruction = machine.controller[machine.pc]
        val outcome = either<GuestError, Unit> { machine.step() }
        val failure = outcome.fold({ it }, { null })
        if (failure != null) return events + stuckLine(instruction, failure)
        when (instruction) {
            is Save -> {
                stack.addLast(machine.registers.getValue(instruction.reg).content)
                events.add("(${renderInstruction(instruction)}) stack=${renderWords(stack.reversed(), labels)}")
            }

            is Restore -> {
                if (stack.isNotEmpty()) stack.removeLast()
                val restored = machine.registers.getValue(instruction.reg).content
                events.add(
                    "(${renderInstruction(
                        instruction,
                    )}) ${instruction.reg}=${render(restored, labels)} stack=${renderWords(stack.reversed(), labels)}",
                )
            }

            is Branch -> {
                if (machine.testFlag) events.add("branch taken to ${instruction.label}")
            }

            is Goto -> {
                if (instruction.to is GotoTarget.ByReg) {
                    val target = machine.pc
                    val name = labels.entries.firstOrNull { it.value == target }?.key ?: "$target"
                    events.add("return to $name")
                }
            }

            else -> {}
        }
    }
    return events + "answer ${render(machine.registers.getValue(answerRegister).content, labels)}"
}

/** Hand-simulate the factorial machine on n = 3 and the Fibonacci machine
 *  on n = 3: each descends past at least one recursive call, and the
 *  Fibonacci machine branches at both call sites. */
public fun handSimulationTraces(): List<String> {
    val input = mapOf("n" to GValue.VLong(3), "val" to GValue.VLong(0))
    return handTrace(recursiveFactorialController, input, "val") + handTrace(fibController, input, "val")
}

/** The factorial machine's peak stack depth on n = 3: two frames, each
 *  holding `continue` and `n`. */
public fun factorialMaxDepth(n: Long): Int {
    val machine =
        freshMachine(
            setOf("n", "val", "continue"),
            machineArithmetic,
            recursiveFactorialController,
            mapOf("n" to GValue.VLong(n), "val" to GValue.VLong(0)),
        )
    runToHalt(machine)
    return machine.stack.maxDepth.toInt()
}
