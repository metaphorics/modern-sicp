// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme program fib-machine in SICP section 5.1.4
//
// Chapter 5, exercise 5.11: the save and restore disciplines. The book's
// three possibilities for `restore` -- (a) the standard, name-blind pop,
// (b) a pop that checks the register the value was saved from, and (c) a
// stack of its own for every register -- are all implemented here over the
// same typed instruction data and run on the same machines. Part (a)'s
// elimination question is answered with the name-blind controller: because
// `(restore n)` in afterfib-n-2 pops the same slot `(restore val)` would,
// the pair `(assign n (reg val))`, `(restore val)` collapses to one
// `(restore n)`, swapping the roles of `val` and `n` in the final sum.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.MachineOp
import sicp.runtime.OpAct
import sicp.runtime.OpCond
import sicp.runtime.Perform
import sicp.runtime.Reg
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.assemble

/** The afterfib-n-2 entry of the figure, trimmed by part (a)'s
 *  elimination: `(restore n)` pops the saved `val` directly, so
 *  `(assign n (reg val))` is gone and the final sum's operands are
 *  swapped. */
public val fibNameBlindController: List<Stmt> =
    fibController.takeWhile { it != Label("afterfib-n-2") } +
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

/** The save and restore discipline a variant run uses. */
private enum class Discipline {
    /** (b): each saved value keeps the register name it was saved from. */
    TAGGED,

    /** (c): one stack per register. */
    PER_REGISTER,
}

/** How a disciplined run ends. */
private sealed interface DisciplineOutcome {
    data class Answer(
        val value: GValue,
    ) : DisciplineOutcome

    data class Mismatch(
        val restoring: Reg,
        val saved: Reg,
    ) : DisciplineOutcome

    data class Fault(
        val error: GuestError,
    ) : DisciplineOutcome
}

/** One controller run under discipline (b) or (c): the same typed
 *  instruction data the substrate machine executes, with the exercise's
 *  save and restore discipline in place of the standard stack. The
 *  substrate machine is closed to variant execution procedures, so the
 *  modified discipline -- this exercise's own answer -- steps the shared
 *  data here. */
private class DisciplineMachine(
    private val controller: List<Stmt>,
    registerNames: Set<String>,
    private val ops: Map<String, MachineOp>,
    private val discipline: Discipline,
    private val initial: Map<Reg, GValue> = emptyMap(),
) {
    private val contents: MutableMap<Reg, GValue> = registerNames.associateWith { initial[it] ?: GValue.VUnassigned }.toMutableMap()
    private val labels: Map<String, Int> = assemble(controller).fold({ emptyMap() }, { it.labels })
    private val tagged = ArrayDeque<Pair<Reg, GValue>>()
    private val stacks = mutableMapOf<Reg, ArrayDeque<GValue>>()
    private var pc = 0
    private var flag = false

    fun run(answerReg: Reg): DisciplineOutcome {
        val outcome =
            either<GuestError, DisciplineOutcome> {
                while (pc < controller.size) {
                    val terminal = step(controller[pc])
                    if (terminal != null) return@either terminal
                }
                DisciplineOutcome.Answer(contentOf(answerReg))
            }
        return outcome.fold({ DisciplineOutcome.Fault(it) }, { it })
    }

    context(r: Raise<GuestError>)
    private fun step(instruction: Stmt): DisciplineOutcome? {
        when (instruction) {
            is Label -> {
                pc++
            }

            is Assign -> {
                contents[instruction.reg] = sourceValue(instruction.src)
                pc++
            }

            is Test -> {
                val condition = instruction.cond as? OpCond ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
                flag = truth(opValue(condition.name, condition.args))
                pc++
            }

            is Branch -> {
                pc = if (flag) labelPc(instruction.label) else pc + 1
            }

            is Goto -> {
                pc =
                    when (val to = instruction.to) {
                        is GotoTarget.Lbl -> labelPc(to.name)
                        is GotoTarget.ByReg -> regToPc(to.reg)
                    }
            }

            is Save -> {
                val word = contentOf(instruction.reg)
                when (discipline) {
                    Discipline.TAGGED -> tagged.addLast(instruction.reg to word)
                    Discipline.PER_REGISTER -> stacks.getOrPut(instruction.reg) { ArrayDeque() }.addLast(word)
                }
                pc++
            }

            is Restore -> {
                val terminal = restoreInstruction(instruction.reg)
                if (terminal != null) return terminal
            }

            is Perform -> {
                val action = instruction.act as? OpAct ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
                opValue(action.name, action.args)
                pc++
            }
        }
        return null
    }

    /** Pops under this discipline; a non-null answer ends the run. */
    context(r: Raise<GuestError>)
    private fun restoreInstruction(target: Reg): DisciplineOutcome? {
        when (discipline) {
            Discipline.TAGGED -> {
                val entry = tagged.removeLastOrNull() ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
                if (entry.first != target) return DisciplineOutcome.Mismatch(target, entry.first)
                contents[target] = entry.second
            }

            Discipline.PER_REGISTER -> {
                val word = stacks[target]?.removeLastOrNull() ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
                contents[target] = word
            }
        }
        pc++
        return null
    }

    context(r: Raise<GuestError>)
    private fun sourceValue(src: Source): GValue =
        when (src) {
            is Source.RegSrc -> contentOf(src.reg)
            is Source.ConstSrc -> src.v
            is Source.LabelSrc -> GValue.VInt(labelPc(src.name))
            is Source.OpSrc -> opValue(src.name, src.args)
        }

    context(r: Raise<GuestError>)
    private fun opValue(
        name: String,
        args: List<Source>,
    ): GValue {
        val op = ops[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
        return op(r, args.map { sourceValue(it) })
    }

    context(r: Raise<GuestError>)
    private fun contentOf(name: Reg): GValue {
        val value = contents[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
        if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRegister(NO_POSITION))
        return value
    }

    context(r: Raise<GuestError>)
    private fun labelPc(name: String): Int = labels[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

    context(r: Raise<GuestError>)
    private fun regToPc(name: Reg): Int =
        when (val value = contentOf(name)) {
            is GValue.VString -> labelPc(value.value)
            is GValue.VInt -> value.value
            is GValue.VLong -> value.value.toInt()
            else -> r.raise(GuestError.UnassignedRegister(NO_POSITION))
        }

    context(r: Raise<GuestError>)
    private fun truth(value: GValue): Boolean =
        when (value) {
            is GValue.VBool -> value.value
            else -> r.raise(GuestError.UnassignedRegister(NO_POSITION))
        }
}

/** The mismatch message this exercise's discipline (b) speaks: it names
 *  both registers and is part of the exercise's semantics, not engine
 *  diagnostic wording. */
private fun mismatchText(mismatch: DisciplineOutcome.Mismatch): String =
    "restore ${mismatch.restoring} but the stack holds ${mismatch.saved}"

/** The out-of-order sequence under the standard, name-blind discipline:
 *  `restore y` pops `x`'s value. */
private fun outOfOrderStandard(): String {
    val machine = freshMachine(setOf("x", "y", "continue"), machineArithmetic, outOfOrderController)
    runToHalt(machine)
    return render(machine.registers.getValue("y").content)
}

/** The out-of-order sequence under one variant discipline: `y`'s rendered
 *  content, or the typed mismatch's text. */
private fun outOfOrderUnder(discipline: Discipline): String {
    val machine = DisciplineMachine(outOfOrderController, setOf("x", "y", "continue"), machineArithmetic, discipline)
    return when (val outcome = machine.run("y")) {
        is DisciplineOutcome.Answer -> render(outcome.value)
        is DisciplineOutcome.Mismatch -> mismatchText(outcome)
        is DisciplineOutcome.Fault -> "stuck: ${outcome.error.category}"
    }
}

/** One fib run under the standard discipline: the rendered `val`. */
private fun fibUnderStandard(
    controller: List<Stmt>,
    n: Long,
): String {
    val machine =
        freshMachine(
            setOf("n", "continue", "val"),
            machineArithmetic,
            controller,
            mapOf("n" to GValue.VLong(n)),
        )
    runToHalt(machine)
    return render(machine.registers.getValue("val").content)
}

/** One fib run under a variant discipline: the rendered `val`, or the
 *  typed mismatch's text. */
private fun fibUnder(
    controller: List<Stmt>,
    discipline: Discipline,
    n: Long,
): String {
    val machine =
        DisciplineMachine(
            controller,
            setOf("n", "continue", "val"),
            machineArithmetic,
            discipline,
            mapOf("n" to GValue.VLong(n)),
        )
    return when (val outcome = machine.run("val")) {
        is DisciplineOutcome.Answer -> render(outcome.value)
        is DisciplineOutcome.Mismatch -> mismatchText(outcome)
        is DisciplineOutcome.Fault -> "stuck: ${outcome.error.category}"
    }
}

/** The eliminated controller under the checking discipline: `(restore n)`
 *  finds a value saved from `val`, the typed mismatch. */
private fun eliminatedUnderChecking(): String = fibUnder(fibNameBlindController, Discipline.TAGGED, 3)

/** The three disciplines on the same out-of-order sequence and on the same
 *  fib run, then part (a)'s elimination running on the name-blind machine
 *  and failing the checking one. */
public fun restoreDisciplineRuns(): List<String> =
    listOf(
        "out-of-order restore under (a) name-blind: y = ${outOfOrderStandard()}",
        "out-of-order restore under (b) checking: ${outOfOrderUnder(Discipline.TAGGED)}",
        "out-of-order restore under (c) per-register: y = ${outOfOrderUnder(Discipline.PER_REGISTER)}",
        "fib(3) under (a): val = ${fibUnderStandard(fibController, 3)}",
        "fib(3) under (b): val = ${fibUnder(fibController, Discipline.TAGGED, 3)}",
        "fib(3) under (c): val = ${fibUnder(fibController, Discipline.PER_REGISTER, 3)}",
        "fib(3) with the eliminated assign, discipline (a): val = ${fibUnderStandard(fibNameBlindController, 3)}",
        "fib(5) with the eliminated assign, discipline (a): val = ${fibUnderStandard(fibNameBlindController, 5)}",
        "the eliminated controller under (b): ${eliminatedUnderChecking()}",
    )
