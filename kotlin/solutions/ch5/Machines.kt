// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, sections 5.1 to 5.2: the register-machine machinery the
// exercise solutions share. Every machine of the chapter is the typed
// controller data of the edition's machine substrate (contract section
// 4.4): `Stmt` over `Source`, run by `Machine` with guest values in the
// registers. This file is the sugar that contract allows over that data
// (source builders), the primitive device table the section's machines
// assume (every numeric rule delegated to `Primitives`, so the machines
// share the guest numeric model of section 3.1), and the run and render
// helpers the solutions pin their observable answers with.
//
// Machine words follow the substrate: a label address is the `VInt` that
// `LabelSrc` installs, and a machine's data numbers are `VLong` or
// `VDouble`, so `render` can print a return address as its label name --
// what a hand simulation writes -- while a count prints as a number.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION
import sicp.guest.Primitives
import sicp.guest.renderPrinted
import sicp.runtime.Action
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Cond
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.MachineProgramError
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Source.ConstSrc
import sicp.runtime.Source.LabelSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test

// ---------------------------------------------------------------------------
// Controller data builders
// ---------------------------------------------------------------------------

/** A register operand. */
public fun reg(name: String): Source = RegSrc(name)

/** An integer constant: machine counts and inputs are `VLong`. */
public fun constV(value: Int): Source = ConstSrc(GValue.VLong(value.toLong()))

/** A long constant. */
public fun constV(value: Long): Source = ConstSrc(GValue.VLong(value))

/** A real constant. */
public fun constV(value: Double): Source = ConstSrc(GValue.VDouble(value))

/** A Boolean constant. */
public fun constV(value: Boolean): Source = ConstSrc(GValue.VBool(value))

/** A string constant. */
public fun constV(value: String): Source = ConstSrc(GValue.VString(value))

/** A label address operand: `assign continue (label done)`. */
public fun labelSrc(name: String): Source = LabelSrc(name)

/** An operation source over operands. */
public fun opSrc(
    name: String,
    vararg args: Source,
): Source = OpSrc(name, args.toList())

/** An operation condition for `test`. */
public fun opCond(
    name: String,
    vararg args: Source,
): Cond = OpCond(name, args.toList())

/** An operation action for `perform`. */
public fun opAct(
    name: String,
    vararg args: Source,
): Action = OpAct(name, args.toList())

// ---------------------------------------------------------------------------
// Primitive devices
// ---------------------------------------------------------------------------

private fun binaryDevice(operator: String): MachineOp =
    { args ->
        if (args.size != 2) raise(GuestError.UnassignedRead(NO_POSITION))
        Primitives.binary(operator, args[0], args[1], NO_POSITION)
    }

/** The primitive devices the section's machines assume: the arithmetic of
 *  1.1.7 and the comparisons, each delegated to the guest runtime so the
 *  numeric rules of section 3.1 hold for machine code too. The book's
 *  `=`, `<`, `>` operation names map onto the guest comparison operators. */
public val machineArithmetic: Map<String, MachineOp> =
    mapOf(
        "+" to binaryDevice("+"),
        "-" to binaryDevice("-"),
        "*" to binaryDevice("*"),
        "/" to binaryDevice("/"),
        "%" to binaryDevice("%"),
        "rem" to binaryDevice("%"),
        "=" to binaryDevice("=="),
        "<" to binaryDevice("<"),
        ">" to binaryDevice(">"),
        "<=" to binaryDevice("<="),
        ">=" to binaryDevice(">="),
        "abs" to { args ->
            if (args.size != 1) raise(GuestError.UnassignedRead(NO_POSITION))
            when (val value = args[0]) {
                is GValue.VLong -> GValue.VLong(kotlin.math.abs(value.value))
                is GValue.VDouble -> GValue.VDouble(kotlin.math.abs(value.value))
                else -> raise(GuestError.UnassignedRead(NO_POSITION))
            }
        },
    )

/** A `print` device appending its argument to [out] with the section 3.7
 *  rendering; the driver-loop machines of 5.1.2 observe their result this
 *  way. */
public fun printOp(out: StringBuilder): MachineOp =
    { args ->
        if (args.size != 1) raise(GuestError.UnassignedRead(NO_POSITION))
        out.append(render(args[0]))
        args[0]
    }

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/** One machine word the way a reader writes it: a label address prints as
 *  its label name (the reverse of the substrate's `VInt` index), every
 *  other value follows the section 3.7 rendering. */
public fun render(
    value: GValue,
    labels: Map<String, Int> = emptyMap(),
): String =
    when (value) {
        is GValue.VInt -> labels.entries.firstOrNull { it.value == value.value }?.key ?: value.value.toString()
        is GValue.VLong -> value.value.toString()
        is GValue.VDouble -> value.value.toString()
        is GValue.VBool -> value.value.toString()
        is GValue.VString -> value.value
        is GValue.VUnassigned -> "*unassigned*"
        else -> renderPrinted(value) ?: value.toString()
    }

/** A stack picture, top first, as a parenthesized list of words. */
public fun renderWords(
    words: List<GValue>,
    labels: Map<String, Int> = emptyMap(),
): String = words.joinToString(separator = " ", prefix = "(", postfix = ")") { render(it, labels) }

/** One instruction's controller text, for traces and fault reports. */
public fun renderInstruction(instruction: Stmt): String =
    when (instruction) {
        is Label -> {
            "${instruction.name}:"
        }

        is Assign -> {
            "assign ${instruction.reg}"
        }

        is Test -> {
            "test"
        }

        is Branch -> {
            "branch ${instruction.label}"
        }

        is Goto -> {
            when (val to = instruction.to) {
                is GotoTarget.Lbl -> "goto ${to.name}"
                is GotoTarget.ByReg -> "goto (reg ${to.reg})"
            }
        }

        is Save -> {
            "save ${instruction.reg}"
        }

        is Restore -> {
            "restore ${instruction.reg}"
        }

        is Perform -> {
            "perform"
        }
    }

/** The typed fault line the error exercises pin: the category and the
 *  faulting instruction are the observable; the wording is not. */
public fun stuckLine(
    instruction: Stmt,
    error: GuestError,
): String = "stuck: ${error.category} at ${renderInstruction(instruction)}"

/** A typed machine-program error the way the assembly exercises pin it:
 *  the category and the name it names. */
public fun renderProgramError(error: MachineProgramError): String =
    when (error) {
        is MachineProgramError.DuplicateLabel -> "${error.category}: ${error.name}"
        is MachineProgramError.UnknownLabel -> "${error.category}: ${error.name}"
        is MachineProgramError.LabelOperand -> "${error.category}: ${error.name}"
        MachineProgramError.Running -> error.category
    }

// ---------------------------------------------------------------------------
// Running
// ---------------------------------------------------------------------------

/** How a machine run ends. */
public sealed interface MachineOutcome {
    /** `pc` walked off the end of the controller. */
    public data class Halted(
        /** Instructions executed, labels excluded. */
        val instructions: Long,
    ) : MachineOutcome

    /** A typed machine fault at [instruction]: the observable of the
     *  error-signaling exercises is the category and the instruction. */
    public data class Stuck(
        val instruction: Stmt,
        val error: GuestError,
    ) : MachineOutcome

    /** The step budget expired: a controller that does not halt. */
    public data class OutOfBudget(
        val instructions: Long,
    ) : MachineOutcome
}

/** Runs [machine] to its halt or its first fault, answering the book's
 *  instruction count (a label is not an instruction). [budget] bounds a
 *  controller that loops, so a machine run can never hang its caller. */
public fun runStaged(
    machine: Machine,
    budget: Long = 1_000_000,
): MachineOutcome {
    var instructions = 0L
    while (instructions < budget) {
        if (machine.halted()) return MachineOutcome.Halted(instructions)
        val instruction = machine.controller[machine.pc]
        val outcome = either<GuestError, Unit> { machine.step() }
        val failure = outcome.fold({ it }, { null })
        if (failure != null) return MachineOutcome.Stuck(instruction, failure)
        if (instruction !is Label) instructions++
    }
    return MachineOutcome.OutOfBudget(instructions)
}

/** A fresh machine with [registers] named and [initial] contents; every
 *  exercise run owns a fresh machine so no register carries over. */
public fun freshMachine(
    registers: Set<String>,
    ops: Map<String, MachineOp>,
    controller: List<Stmt>,
    initial: Map<String, GValue> = emptyMap(),
): Machine {
    val machine = Machine(registers, ops, controller)
    for ((name, value) in initial) machine.registers.getValue(name).content = value
    return machine
}

/** Runs [machine] to its halt and answers its instructions; a fault is a
 *  solution bug and fails the run with its category. */
public fun runToHalt(
    machine: Machine,
    budget: Long = 1_000_000,
): Long =
    when (val outcome = runStaged(machine, budget)) {
        is MachineOutcome.Halted -> outcome.instructions
        is MachineOutcome.Stuck -> error("the machine faulted: ${stuckLine(outcome.instruction, outcome.error)}")
        is MachineOutcome.OutOfBudget -> error("the machine did not halt within $budget instructions")
    }

/** Steps [machine] over [instruction]; a typed fault is a solution bug and
 *  fails the run with its category. */
internal fun stepOrFail(
    machine: Machine,
    instruction: Stmt,
) {
    val outcome = either<GuestError, Unit> { machine.step() }
    outcome.fold({ error -> error("the machine faulted: ${stuckLine(instruction, error)}") }, { })
}

/** The monitored stack statistics line of section 5.2.4: the cumulative
 *  counters the stack exercises read. */
public fun statisticsLine(machine: Machine): String = "(total-pushes = ${machine.stack.pushes} maximum-depth = ${machine.stack.maxDepth})"

/** Runs [block] in a machine scope; a typed fault is a solution bug and
 *  fails the run with its category. */
internal fun <A> machineScope(block: Raise<GuestError>.() -> A): A =
    either(block).fold({ error("the machine faulted: ${it.category}") }, { it })
