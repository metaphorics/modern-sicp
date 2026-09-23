// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentMap
import kotlinx.collections.immutable.persistentMapOf

/**
 * The register-machine substrate of chapter 5: controller instructions as
 * data, a monitored stack, named registers, and the dispatch loop. The 5.1
 * DSL, the 5.4 explicit-control evaluator, and the 5.5 compiler all emit
 * the same [Stmt] list, so code-as-data survives every stage.
 *
 * Register names are strings — the book's `a`, `b`, `val`, `continue` —
 * not a closed enum, because the general simulator must accept whatever
 * registers a machine description declares.
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

/** `perform`: run an operation for its side effect. */
public data class Perform(
    val act: Action,
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

/** The right-hand side of an [Assign]. */
public sealed interface Source {
    /** Another register's content. */
    public data class RegSrc(
        val reg: Reg,
    ) : Source

    /** A literal value. */
    public data class ConstSrc(
        val v: Value,
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

/** The operand of a [Test]: an operation whose result sets the flag. */
public data class OpCond(
    val name: String,
    val args: List<Source>,
) : Cond

/** The condition of a [Test]. */
public sealed interface Cond

/** The operand of a [Perform]: an operation run for effect. */
public data class OpAct(
    val name: String,
    val args: List<Source>,
) : Action

/** The side-effecting operation of a [Perform]. */
public sealed interface Action

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

/** The monitored stack of 5.2.4: pushes, current depth, and the high-water
 * mark the stack-statistics exercises read. */
public class Stack {
    private val data = ArrayDeque<Value>()

    /** Total pushes since the last [initialize]. */
    public var pushes: Long = 0
        private set

    /** Deepest the stack has been since the last [initialize]. */
    public var maxDepth: Long = 0
        private set

    /** The current depth. */
    public val depth: Int get() = data.size

    /** Push `v`, updating the statistics. */
    public fun push(v: Value) {
        data.addLast(v)
        pushes++
        if (data.size.toLong() > maxDepth) maxDepth = data.size.toLong()
    }

    /** Pop the top; the book's `restore` on an empty stack is an error. */
    context(r: Raise<SchemeError>)
    public fun pop(): Value = data.removeLastOrNull() ?: r.raise(SchemeError.StackUnderflow)

    /** The book's `initialize-stack`: empties the stack and zeroes the
     * statistics. */
    public fun initialize() {
        data.clear()
        pushes = 0
        maxDepth = 0
    }
}

/** One named register. */
public class Register(
    /** The register's name. */
    public val name: Reg,
) {
    /** The register's content; [VNil] until assigned. */
    public var content: Value = VNil
}

/**
 * The assembled machine: registers, a monitored stack, an operation
 * registry, and the controller as an indexed [Stmt] list with a
 * label-to-index map. [run] executes to a halt; [step] executes one
 * instruction so the breakpoint exercises can park the machine.
 */
public class Machine(
    regs: Set<Reg>,
    /** The operation registry: name to [Op]. */
    public val ops: PersistentMap<String, Op>,
    /** The controller instructions, labels included. */
    public val controller: List<Stmt>,
) {
    /** The registers by name. */
    public val registers: Map<Reg, Register> = regs.associateWith(::Register)

    /** The monitored stack. */
    public val stack: Stack = Stack()

    /** The label-to-index map built at assembly. */
    public val labels: Map<String, Int> =
        controller.mapIndexedNotNull { i, s -> (s as? Label)?.name?.let { it to i } }.toMap()

    /** The instruction pointer. */
    public var pc: Int = 0
        private set

    /** The flag [Test] sets and [Branch] reads. */
    public var testFlag: Boolean = false
        private set

    /** The register named [name]; an undeclared name is a machine fault. */
    context(r: Raise<SchemeError>)
    public fun reg(name: Reg): Register = registers[name] ?: r.raise(SchemeError.MachineFault("no such register: $name"))

    /** True once `pc` has walked off the end of the controller. */
    public fun halted(): Boolean = pc >= controller.size

    /** Executes to a halt: [pc] past the last instruction. */
    context(r: Raise<SchemeError>)
    public fun run() {
        while (!halted()) step()
    }

    /** Executes the instruction under [pc]. */
    context(r: Raise<SchemeError>)
    public fun step() {
        if (halted()) return
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
                stack.push(reg(s.reg).content)
                pc++
            }

            is Restore -> {
                reg(s.reg).content = stack.pop()
                pc++
            }
        }
    }

    context(r: Raise<SchemeError>)
    private fun labelPc(name: String): Int = labels[name] ?: r.raise(SchemeError.UnknownLabel(name))

    context(r: Raise<SchemeError>)
    private fun regToPc(reg: Reg): Int =
        when (val v = this.reg(reg).content) {
            is VSym -> labelPc(v.name)
            is VInt -> v.n.toInt()
            else -> r.raise(SchemeError.MachineFault("register $reg does not hold a label: $v"))
        }

    context(r: Raise<SchemeError>)
    private fun evalSource(src: Source): Value =
        when (src) {
            is Source.RegSrc -> reg(src.reg).content
            is Source.ConstSrc -> src.v
            is Source.LabelSrc -> VInt(labelPc(src.name).toLong())
            is Source.OpSrc -> evalOp(src.name, src.args)
        }

    context(r: Raise<SchemeError>)
    private fun evalOp(
        name: String,
        args: List<Source>,
    ): Value {
        val op = ops[name] ?: r.raise(SchemeError.MachineFault("unknown operation: $name"))
        return op(r, args.map { evalSource(it) })
    }

    context(r: Raise<SchemeError>)
    private fun evalAction(act: Action): Value =
        when (act) {
            is OpAct -> evalOp(act.name, act.args)
        }

    public companion object {
        /** An empty operation registry. */
        public val noOps: PersistentMap<String, Op> = persistentMapOf()
    }
}
