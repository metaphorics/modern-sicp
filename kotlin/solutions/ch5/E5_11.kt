// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.11: the save and restore disciplines. The book's
// three possibilities for `restore` -- (a) the standard, name-blind pop,
// (b) a pop that checks the register the value was saved from, and (c) a
// stack of its own for every register -- are all implemented here as
// variant assemblers that recompose the standard builders, and run on the
// same machines. Part (a)'s elimination question is answered with the
// name-blind controller: because `(restore n)` in afterfib-n-2 pops the
// same slot `(restore val)` would, the pair `(assign n (reg val))`,
// `(restore val)` collapses to one `(restore n)`, swapping the roles of
// `val` and `n` in the final sum.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch5.CompiledProgram
import sicp.ch5.Exec
import sicp.ch5.Inst
import sicp.ch5.Machine
import sicp.ch5.MachineError
import sicp.ch5.Op
import sicp.ch5.arithOperations
import sicp.ch5.constV
import sicp.ch5.executionProcedure
import sicp.ch5.extractLabels
import sicp.ch5.getRegisterContents
import sicp.ch5.labelSrc
import sicp.ch5.opCond
import sicp.ch5.opSrc
import sicp.ch5.reg
import sicp.ch5.renderStmt
import sicp.ch5.setRegisterContents
import sicp.ch5.summarize
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Reg
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VInt
import sicp.runtime.Value

/** The recursive Fibonacci machine of figure 5.12, transcribed line for
 *  line: each level saves `continue` before the first call, saves `n`
 *  beside it, and restores the pair at `afterfib-n-1`, so the second
 *  call can be set up over them. */
public val fibSimController: List<Stmt> =
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

/** The afterfib-n-2 entry of the figure, trimmed by part (a)'s
 *  elimination: `(restore n)` pops the saved `val` directly, so
 *  `(assign n (reg val))` is gone and the final sum's operands are
 *  swapped. */
public val fibNameBlindController: List<Stmt> =
    fibSimController.takeWhile { it != Label("afterfib-n-2") } +
        listOf(
            Label("afterfib-n-2"),
            Restore("n"),
            Restore("continue"),
            Assign("val", opSrc("+", reg("val"), reg("n"))),
            Goto(GotoTarget.ByReg("continue")),
            Label("immediate-answer"),
            Assign("val", reg("n")),
            Goto(GotoTarget.ByReg("continue")),
            Label("fib-done"),
        )

/** The book's out-of-order sequence: `restore y` after `x`, not `y`, was
 *  saved last. */
public val outOfOrderController: List<Stmt> =
    listOf(
        Assign("y", constV(1)),
        Assign("x", constV(2)),
        Save("y"),
        Save("x"),
        Restore("y"),
        Label("done"),
    )

/** Discipline (b): the stack pairs each value with the register it was
 *  saved from, and a restore of a differently named register is the typed
 *  restore-mismatch error. */
public class TaggedStackMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    /** The saved pairs, most recent at the end. */
    public val saved: ArrayDeque<Pair<Reg, Value>> = ArrayDeque()
}

/** Discipline (c): one stack per register; `restore y` pops from `y`'s own
 *  stack, however many other registers were saved since. */
public class PerRegisterStackMachine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    private val stacks = LinkedHashMap<Reg, ArrayDeque<Value>>()

    /** The stack kept for [name]'s saves and restores. */
    public fun stackFor(name: Reg): ArrayDeque<Value> = stacks.getOrPut(name) { ArrayDeque() }
}

/** The tagged save builder: pairs the register's content with its name. */
context(r: Raise<MachineError>)
private fun taggedSaveProc(
    inst: Save,
    machine: TaggedStackMachine,
): Exec {
    val source = machine.registerFor(inst.reg)
    return { _ ->
        machine.saved.addLast(inst.reg to source.content)
        machine.pc += 1
    }
}

/** The tagged restore builder: pops the most recent pair, refusing a
 *  differently named register. */
context(r: Raise<MachineError>)
private fun taggedRestoreProc(
    inst: Restore,
    machine: TaggedStackMachine,
): Exec {
    val target = machine.registerFor(inst.reg)
    return { r ->
        val entry = machine.saved.removeLastOrNull() ?: r.raise(MachineError.StackUnderflow(inst.reg))
        if (entry.first != inst.reg) {
            r.raise(MachineError.RestoreMismatch(inst.reg, entry.first))
        }
        target.store(entry.second)
        machine.pc += 1
    }
}

/** The per-register save builder: pushes onto the register's own stack. */
context(r: Raise<MachineError>)
private fun perRegisterSaveProc(
    inst: Save,
    machine: PerRegisterStackMachine,
): Exec {
    val source = machine.registerFor(inst.reg)
    return { _ ->
        machine.stackFor(inst.reg).addLast(source.content)
        machine.pc += 1
    }
}

/** The per-register restore builder: pops the register's own stack. */
context(r: Raise<MachineError>)
private fun perRegisterRestoreProc(
    inst: Restore,
    machine: PerRegisterStackMachine,
): Exec {
    val target = machine.registerFor(inst.reg)
    return { r ->
        val value = machine.stackFor(inst.reg).removeLastOrNull() ?: r.raise(MachineError.StackUnderflow(inst.reg))
        target.store(value)
        machine.pc += 1
    }
}

/** The variant assembler for one of the disciplines: [variant] supplies
 *  the save and restore execution procedures; everything else is the
 *  standard builders. */
context(r: Raise<MachineError>)
public fun assembleWith(
    controller: List<Stmt>,
    machine: Machine,
    variant: (Stmt) -> Exec?,
): CompiledProgram {
    val (instStmts, labels) = extractLabels(controller)
    val insts =
        instStmts.map { inst ->
            val exec: Exec = variant(inst) ?: executionProcedure(inst, machine, labels)
            Inst(renderStmt(inst), exec)
        }
    return CompiledProgram(insts, labels, summarize(instStmts, labels))
}

/** The checking discipline (b) over a controller and machine. */
context(r: Raise<MachineError>)
public fun assembleTagged(
    controller: List<Stmt>,
    machine: TaggedStackMachine,
): CompiledProgram =
    assembleWith(controller, machine) { inst ->
        when (inst) {
            is Save -> taggedSaveProc(inst, machine)
            is Restore -> taggedRestoreProc(inst, machine)
            else -> null
        }
    }

/** The per-register discipline (c) over a controller and machine. */
context(r: Raise<MachineError>)
public fun assemblePerRegister(
    controller: List<Stmt>,
    machine: PerRegisterStackMachine,
): CompiledProgram =
    assembleWith(controller, machine) { inst ->
        when (inst) {
            is Save -> perRegisterSaveProc(inst, machine)
            is Restore -> perRegisterRestoreProc(inst, machine)
            else -> null
        }
    }

/** Runs [controller] under [discipline] (a, b, or c) with input register
 *  [input] set to [n], answering the named result register. */
private fun runFib(
    controller: List<Stmt>,
    discipline: String,
    n: Long,
): Value =
    machineRun {
        val machine: Machine =
            when (discipline) {
                "a" -> Machine(listOf("n", "continue", "val"), arithOperations)
                "b" -> TaggedStackMachine(listOf("n", "continue", "val"), arithOperations)
                else -> PerRegisterStackMachine(listOf("n", "continue", "val"), arithOperations)
            }
        when (discipline) {
            "a" -> machine.install(controller)
            "b" -> machine.install(assembleTagged(controller, machine as TaggedStackMachine))
            else -> machine.install(assemblePerRegister(controller, machine as PerRegisterStackMachine))
        }
        machine.setRegisterContents("n", VInt(n))
        machine.start()
        machine.getRegisterContents("val")
    }

/** The book's out-of-order sequence under one discipline letter, answering
 *  `y`'s content or the typed error's text. */
private fun outOfOrderOutcome(discipline: String): String =
    either {
        val machine: Machine =
            when (discipline) {
                "a" -> Machine(listOf("x", "y", "continue"), arithOperations)
                "b" -> TaggedStackMachine(listOf("x", "y", "continue"), arithOperations)
                else -> PerRegisterStackMachine(listOf("x", "y", "continue"), arithOperations)
            }
        when (discipline) {
            "a" -> machine.install(outOfOrderController)
            "b" -> machine.install(assembleTagged(outOfOrderController, machine as TaggedStackMachine))
            else -> machine.install(assemblePerRegister(outOfOrderController, machine as PerRegisterStackMachine))
        }
        machine.start()
        machine.getRegisterContents("y").toString()
    }.fold({ e -> e.toString() }, { it })

/** The three disciplines on the same out-of-order sequence and on the same
 *  fib run, then part (a)'s elimination running on the name-blind
 *  assembler and failing the checking one. */
public fun restoreDisciplineRuns(): List<String> =
    listOf(
        "out-of-order restore under (a) name-blind: y = ${outOfOrderOutcome("a")}",
        "out-of-order restore under (b) checking: ${outOfOrderOutcome("b")}",
        "out-of-order restore under (c) per-register: y = ${outOfOrderOutcome("c")}",
        "fib(3) under (a): val = ${runFib(fibSimController, "a", 3)}",
        "fib(3) under (b): val = ${runFib(fibSimController, "b", 3)}",
        "fib(3) under (c): val = ${runFib(fibSimController, "c", 3)}",
        "fib(3) with the eliminated assign, discipline (a): val = ${runFib(fibNameBlindController, "a", 3)}",
        "fib(5) with the eliminated assign, discipline (a): val = ${runFib(fibNameBlindController, "a", 5)}",
        "the eliminated controller under (b): ${runEliminatedUnderChecking()}",
    )

/** The eliminated controller under the checking discipline: `(restore n)`
 *  finds a value saved from `val`, the typed mismatch. */
private fun runEliminatedUnderChecking(): String =
    either {
        val machine = TaggedStackMachine(listOf("n", "continue", "val"), arithOperations)
        machine.install(assembleTagged(fibNameBlindController, machine))
        machine.setRegisterContents("n", VInt(3))
        machine.start()
        machine.getRegisterContents("val").toString()
    }.fold({ e -> e.toString() }, { it })
