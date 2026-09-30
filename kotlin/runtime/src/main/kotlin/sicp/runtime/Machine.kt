// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.Either
import arrow.core.raise.Raise
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.NO_POSITION

/**
 * The register-machine substrate of section 4.4: controller instructions as
 * data, a monitored stack, named registers, and the dispatch loop. The 5.1
 * DSL, the 5.4 explicit-control evaluator, and the 5.5 compiler all emit the
 * same [Stmt] list, so code-as-data survives every stage. Register contents
 * are guest values; a read before any write raises `UnassignedRegister`.
 */
public typealias Reg = String

/** One controller instruction. */
public sealed interface Stmt

/** A label definition; assembly indexes it and execution skips it. */
public data class Label(
    val name: String,
) : Stmt

/** `assign`: write a register from a source. */
public data class Assign(
    val reg: Reg,
    val src: Source,
) : Stmt

/** `test`: set the machine's test flag from a condition. */
public data class Test(
    val cond: Cond,
) : Stmt

/** `branch`: jump to a label when the test flag is set. */
public data class Branch(
    val label: String,
) : Stmt

/** `goto`: jump unconditionally. */
public data class Goto(
    val to: GotoTarget,
) : Stmt

/** `save`: push a register's content. */
public data class Save(
    val reg: Reg,
) : Stmt

/** `restore`: pop into a register. */
public data class Restore(
    val reg: Reg,
) : Stmt

/** `perform`: run an operation for its side effect. */
public data class Perform(
    val act: Action,
) : Stmt

/** The right-hand side of an [Assign]. */
public sealed interface Source {
    /** Another register's content. */
    public data class RegSrc(
        val reg: Reg,
    ) : Source

    /** A literal value. */
    public data class ConstSrc(
        val v: GValue,
    ) : Source

    /** A label's index, for `assign continue (label done)`. */
    public data class LabelSrc(
        val name: String,
    ) : Source

    /** An applied operation: `op` over operand sources. */
    public data class OpSrc(
        val name: String,
        val args: List<Source>,
    ) : Source
}

/** The condition of a [Test]. */
public sealed interface Cond

/** The operand of a [Test]: an operation whose result sets the flag. */
public data class OpCond(
    val name: String,
    val args: List<Source>,
) : Cond

/** The side-effecting operation of a [Perform]. */
public sealed interface Action

/** The operand of a [Perform]: an operation run for effect. */
public data class OpAct(
    val name: String,
    val args: List<Source>,
) : Action

/** Where a [Goto] jumps. */
public sealed interface GotoTarget {
    /** A label name. */
    public data class Lbl(
        val name: String,
    ) : GotoTarget

    /** The label index held in a register: `goto (reg continue)`. */
    public data class ByReg(
        val reg: Reg,
    ) : GotoTarget
}

/** The operation shape of section 4.4: host functions over guest values. */
public typealias MachineOp = Raise<GuestError>.(List<GValue>) -> GValue

/** Typed machine-program errors of section 4.4 (the lesson of 5.9). */
public sealed class MachineProgramError(
    public val category: String,
) {
    public data class DuplicateLabel(
        val name: String,
    ) : MachineProgramError("DuplicateLabel")

    public data class UnknownLabel(
        val name: String,
    ) : MachineProgramError("UnknownLabel")

    public data class LabelOperand(
        val name: String,
    ) : MachineProgramError("LabelOperand")

    /** Appending instructions to a machine that is still executing would
     * shift its control path underfoot. */
    public data object Running : MachineProgramError("Running")
}

/** The assembly census of section 7's 5.2.3 family. */
public class AssemblySummary(
    public val labels: Map<String, Int>,
    public val instructions: Long,
)

/** Checks a controller before it runs: labels resolve once, jumps target
 * labels, and a label never stands where an operation operand belongs. */
public fun assemble(controller: List<Stmt>): Either<MachineProgramError, AssemblySummary> {
    val labels = linkedMapOf<String, Int>()
    for ((index, statement) in controller.withIndex()) {
        if (statement is Label) {
            if (labels.containsKey(statement.name)) return Either.Left(MachineProgramError.DuplicateLabel(statement.name))
            labels[statement.name] = index
        }
    }
    for (statement in controller) {
        val target =
            when (statement) {
                is Branch -> statement.label
                is Goto -> (statement.to as? GotoTarget.Lbl)?.name
                is Assign -> (statement.src as? Source.LabelSrc)?.name
                else -> null
            }
        if (target != null && !labels.containsKey(target)) return Either.Left(MachineProgramError.UnknownLabel(target))
        if (statement is Assign && statement.src is Source.OpSrc) {
            for (argument in statement.src.args) {
                if (argument is Source.LabelSrc) return Either.Left(MachineProgramError.LabelOperand(argument.name))
            }
        }
        if (statement is Test && statement.cond is OpCond) {
            for (argument in statement.cond.args) {
                if (argument is Source.LabelSrc) return Either.Left(MachineProgramError.LabelOperand(argument.name))
            }
        }
        if (statement is Perform && statement.act is OpAct) {
            for (argument in statement.act.args) {
                if (argument is Source.LabelSrc) return Either.Left(MachineProgramError.LabelOperand(argument.name))
            }
        }
    }
    return Either.Right(AssemblySummary(labels, controller.size.toLong()))
}

/** The pinned trace rendering of section 4.4: one line per instruction,
 * `label: <instruction text>` over the constructor fields. */
public fun traceLine(instruction: Stmt): String =
    when (instruction) {
        is Label -> "${instruction.name}:"
        is Assign -> "assign ${instruction.reg} <- ${sourceText(instruction.src)}"
        is Test -> "test ${condText(instruction.cond)}"
        is Branch -> "branch ${instruction.label}"
        is Goto -> "goto ${targetText(instruction.to)}"
        is Save -> "save ${instruction.reg}"
        is Restore -> "restore ${instruction.reg}"
        is Perform -> "perform ${actionText(instruction.act)}"
    }

private fun sourceText(src: Source): String =
    when (src) {
        is Source.RegSrc -> src.reg
        is Source.ConstSrc -> "const"
        is Source.LabelSrc -> "label ${src.name}"
        is Source.OpSrc -> "(op ${src.name} ${src.args.joinToString(" ") { sourceText(it) }})"
    }

private fun condText(cond: Cond): String =
    when (cond) {
        is OpCond -> "(op ${cond.name} ${cond.args.joinToString(" ") { sourceText(it) }})"
    }

private fun actionText(act: Action): String =
    when (act) {
        is OpAct -> "(op ${act.name} ${act.args.joinToString(" ") { sourceText(it) }})"
    }

private fun targetText(target: GotoTarget): String =
    when (target) {
        is GotoTarget.Lbl -> target.name
        is GotoTarget.ByReg -> "(reg ${target.reg})"
    }

/** The monitored stack of 5.2.4: pushes, current depth, and the high-water
 * mark the stack-statistics exercises read. */
public class Stack {
    private val data = ArrayDeque<GValue>()

    /** Total pushes since the last [initialize]. */
    public var pushes: Long = 0
        private set

    /** Deepest the stack has been since the last [initialize]. */
    public var maxDepth: Long = 0
        private set

    /** The current depth. */
    public val depth: Int get() = data.size

    /** Push `v`, updating the statistics. */
    public fun push(v: GValue) {
        data.addLast(v)
        pushes++
        if (data.size.toLong() > maxDepth) maxDepth = data.size.toLong()
    }

    /** Pop the top; restore on an empty stack is a machine fault. */
    context(r: Raise<GuestError>)
    public fun pop(): GValue = data.removeLastOrNull() ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

    /** The top of the stack without removing it (the monitoring exercises
     * read the current argument frame); an empty stack is a machine fault. */
    context(r: Raise<GuestError>)
    public fun peek(): GValue = data.lastOrNull() ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

    /** The book's `initialize-stack`: empties the stack and zeroes the
     * statistics. */
    public fun initialize() {
        data.clear()
        pushes = 0
        maxDepth = 0
    }
}

/** One named register; content is unassigned until the first write. */
public class Register(
    /** The register's name. */
    public val name: Reg,
) {
    /** The register's content; [GValue.VUnassigned] until assigned. */
    public var content: GValue = GValue.VUnassigned
}

/**
 * The assembled machine: registers, a monitored stack, an operation
 * registry, and the controller as an indexed [Stmt] list with a
 * label-to-index map. [run] executes to a halt; [step] executes one
 * instruction so the breakpoint exercises can park the machine.
 */
public class Machine(
    regs: Set<Reg>,
    /** The operation registry: name to [MachineOp]. */
    public val ops: Map<String, MachineOp>,
    /** The controller instructions, labels included. */
    instructions: List<Stmt>,
) {
    public val controller: List<Stmt> get() = program

    /** The registers by name. */
    public val registers: Map<Reg, Register> = regs.associateWith(::Register)

    /** The monitored stack. */
    public val stack: Stack = Stack()

    /** Labels are rebuilt only after a new segment passes assembly. */
    public val labels: Map<String, Int> get() = labelTable

    private val program: MutableList<Stmt> = instructions.toMutableList()
    private var labelTable: Map<String, Int> = assemble(program).fold({ emptyMap() }, { it.labels })

    /** The instruction pointer. */
    public var pc: Int = 0
        private set

    /** The flag [Test] sets and [Branch] reads. */
    public var testFlag: Boolean = false
        private set

    /** Instructions executed since the last read; the monitoring exercises'
     * instruction counter. */
    public var instructions: Long = 0
        private set

    /** Appends an admitted controller segment to a halted machine without
     * replacing registers, stack, output operations, or the program counter.
     * The next `run` starts exactly at the former end of the controller. */
    public fun append(segment: List<Stmt>): Either<MachineProgramError, Unit> {
        if (!halted()) return Either.Left(MachineProgramError.Running)
        val joined =
            object : AbstractList<Stmt>() {
                override val size: Int get() = program.size + segment.size

                override fun get(index: Int): Stmt = if (index < program.size) program[index] else segment[index - program.size]
            }
        return assemble(joined).map { summary ->
            program.addAll(segment)
            labelTable = summary.labels
        }
    }

    /** The register named [name]; an undeclared name is a machine fault. */
    context(r: Raise<GuestError>)
    public fun reg(name: Reg): Register = registers[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

    /** True once `pc` has walked off the end of the controller. */
    public fun halted(): Boolean = pc >= controller.size

    /** Executes to a halt: [pc] past the last instruction. */
    context(r: Raise<GuestError>)
    public fun run() {
        while (!halted()) step()
    }

    /** Executes the instruction under [pc]. */
    context(r: Raise<GuestError>)
    public fun step() {
        if (halted()) return
        instructions++
        when (val s = controller[pc]) {
            is Label -> {
                pc++
            }

            is Assign -> {
                reg(s.reg).content = evalSource(s.src)
                pc++
            }

            is Perform -> {
                evalAction(s.act)
                pc++
            }

            is Test -> {
                testFlag =
                    when (val c = s.cond) {
                        is OpCond -> isTrue(evalOp(c.name, c.args))
                    }
                pc++
            }

            is Branch -> {
                pc = if (testFlag) labelPc(s.label) else pc + 1
            }

            is Goto -> {
                pc =
                    when (val t = s.to) {
                        is GotoTarget.Lbl -> labelPc(t.name)
                        is GotoTarget.ByReg -> regToPc(t.reg)
                    }
            }

            is Save -> {
                stack.push(regContent(s.reg))
                pc++
            }

            is Restore -> {
                reg(s.reg).content = stack.pop()
                pc++
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun regContent(name: Reg): GValue {
        val register = reg(name)
        val value = register.content
        if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRegister(NO_POSITION))
        return value
    }

    context(r: Raise<GuestError>)
    private fun labelPc(name: String): Int = labels[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))

    context(r: Raise<GuestError>)
    private fun regToPc(reg: Reg): Int =
        when (val v = regContent(reg)) {
            is GValue.VString -> labelPc(v.value)
            is GValue.VInt -> v.value
            is GValue.VLong -> v.value.toInt()
            else -> r.raise(GuestError.ShapeFault(NO_POSITION))
        }

    context(r: Raise<GuestError>)
    private fun evalSource(src: Source): GValue =
        when (src) {
            is Source.RegSrc -> regContent(src.reg)
            is Source.ConstSrc -> src.v
            is Source.LabelSrc -> GValue.VInt(labelPc(src.name))
            is Source.OpSrc -> evalOp(src.name, src.args)
        }

    context(r: Raise<GuestError>)
    private fun evalOp(
        name: String,
        args: List<Source>,
    ): GValue {
        val op = ops[name] ?: r.raise(GuestError.UnassignedRegister(NO_POSITION))
        return op(r, args.map { evalSource(it) })
    }

    context(r: Raise<GuestError>)
    private fun evalAction(act: Action): GValue =
        when (act) {
            is OpAct -> evalOp(act.name, act.args)
        }

    context(r: Raise<GuestError>)
    private fun isTrue(value: GValue): Boolean =
        when (value) {
            is GValue.VBool -> value.value
            else -> r.raise(GuestError.ShapeFault(NO_POSITION))
        }

    public companion object {
        /** An empty operation registry. */
        public val noOps: Map<String, MachineOp> = emptyMap()
    }
}
