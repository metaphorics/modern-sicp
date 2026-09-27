// SPDX-License-Identifier: GPL-3.0-only
// Section 5.2: the register-machine simulator. The controller is the
// runtime's `Stmt` list -- the register-machine language of 5.1.5 as data --
// and the assembler of `S5_2Assembler.kt` compiles it once into instructions,
// each carrying the execution procedure the book's `update-insts!` builds, so
// a run only walks compiled closures. This file is 5.2.1's machine model:
// registers, the stack, and `make-new-machine`. The stack is 5.2.4's
// monitored one (total pushes and maximum depth, the meters the 5.2.4
// exercises read), `pc` is an index into the assembled instruction list, and
// `flag` is an ordinary register that `test` sets and `branch` reads.
//
// Extension seams sit at the book's own procedure boundaries: a subclass
// overrides `newRegister` to choose the register's class (exercise 5.18's
// traced registers) or `execute` to act between instructions (exercises 5.15
// to 5.19), and the assembler's builders are free functions a variant
// assembler recomposes (exercises 5.9 to 5.12).

package sicp.ch5

import arrow.core.raise.Raise
import sicp.runtime.Reg
import sicp.runtime.Source
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.Value

/** The chapter's one typed error for the simulator: every `error` the
 *  book's 5.2 signals raises one of these through `Raise<MachineError>`.
 *  Assembly-time faults (a duplicated or undefined label, an unknown
 *  register or operation, exercise 5.9's label operand) and run-time faults
 *  (an empty stack, exercise 5.11's restore discipline) are machine
 *  behavior, not host exceptions. */
public sealed class MachineError {
    /** Two controller labels name the same place (exercise 5.8). */
    public data class DuplicateLabel(
        val name: String,
    ) : MachineError() {
        public override fun toString(): String = "the label $name is used twice"
    }

    /** A `goto`, `branch`, or `assign` names a label the controller never
     *  defines. */
    public data class UnknownLabel(
        val name: String,
    ) : MachineError() {
        public override fun toString(): String = "undefined label: $name"
    }

    /** An instruction names a register the machine never allocated. */
    public data class UnknownRegister(
        val name: String,
    ) : MachineError() {
        public override fun toString(): String = "no such register: $name"
    }

    /** An instruction applies an operation the operations table lacks. */
    public data class UnknownOperation(
        val name: String,
    ) : MachineError() {
        public override fun toString(): String = "unknown operation: $name"
    }

    /** Exercise 5.9: an operation operand written `(label x)`. */
    public data class LabelOperand(
        val name: String,
    ) : MachineError() {
        public override fun toString(): String = "an operation input is written (reg r) or (const c), not (label $name)"
    }

    /** A `restore` reached past the bottom of the stack. */
    public data class StackUnderflow(
        val reg: String,
    ) : MachineError() {
        public override fun toString(): String = "restore $reg from an empty stack"
    }

    /** Exercise 5.11: a `restore` found a value saved from another
     *  register. */
    public data class RestoreMismatch(
        val wanted: String,
        val found: String,
    ) : MachineError() {
        public override fun toString(): String = "restore $wanted but the stack holds $found"
    }

    /** A `goto (reg r)` whose register holds no label address. */
    public data class BadGotoTarget(
        val reg: String,
        val value: Value,
    ) : MachineError() {
        public override fun toString(): String = "register $reg does not hold a label address: $value"
    }

    /** A `branch` read the flag before any `test` set it. */
    public data object BranchWithoutTest : MachineError() {
        public override fun toString(): String = "branch read the flag before a test set it"
    }
}

/** One operation of a machine's operations table: the host computation that
 *  implements a machine operation (the book pairs each name with a Scheme
 *  procedure; this edition pairs it with a receiver-on-[Raise] function). */
public typealias Op = Raise<MachineError>.(List<Value>) -> Value

/** The contents of a register the machine has not written yet: the book's
 *  `*unassigned*` symbol, also the initial value of `flag`. */
public val unassigned: Value = VSym("*unassigned*")

/** One named register: a cell holding one machine word, initially
 *  [unassigned]. `store` is the single write path, so exercise 5.18's traced
 *  registers override it and every write reports. */
public open class Register(
    public val name: Reg,
) {
    private var cell: Value = unassigned

    /** The register's contents. */
    public val content: Value get() = cell

    /** Writes [value] into the register. */
    public open fun store(value: Value) {
        cell = value
    }
}

/** The stack of 5.2.1 as 5.2.4 monitors it: the saved words plus the two
 *  counters (`pushes` and `maxDepth`) that `print-stack-statistics` reads
 *  and the measuring exercises of 5.2.4 pin. */
public class MachineStack {
    private val entries = ArrayDeque<Value>()

    /** Total pushes since the last [initialize]. */
    public var pushes: Long = 0
        private set

    /** Deepest the stack has been since the last [initialize]. */
    public var maxDepth: Int = 0
        private set

    /** Push [value], advancing both counters. */
    public fun push(value: Value) {
        entries.addLast(value)
        pushes += 1
        if (entries.size > maxDepth) maxDepth = entries.size
    }

    /** Pops the top word; [reg] names the `restore` that asked, so an empty
     *  stack is the typed underflow naming the register. */
    public fun pop(
        reg: Reg,
        r: Raise<MachineError>,
    ): Value = entries.removeLastOrNull() ?: r.raise(MachineError.StackUnderflow(reg))

    /** The book's `initialize-stack`: empties the stack and zeroes the
     *  counters. */
    public fun initialize() {
        entries.clear()
        pushes = 0
        maxDepth = 0
    }

    /** The statistics line `print-stack-statistics` writes. */
    public fun statistics(): String = "(total-pushes = $pushes maximum-depth = $maxDepth)"
}

/** The machine model: named registers, the monitored stack, the operations
 *  table, and the assembled instruction sequence the execution loop walks.
 *  Construct with [makeMachine], or subclass and `install` a program. With
 *  [deriveRegisters] (exercise 5.13), an instruction naming an
 *  unallocated register allocates it at first sight instead of faulting,
 *  so the controller determines the register set. */
public open class Machine(
    registerNames: List<Reg>,
    userOperations: Map<String, Op>,
    private val deriveRegisters: Boolean = false,
) {
    /** The machine's registers by name, allocated in the given order. */
    public val registers: MutableMap<Reg, Register> = LinkedHashMap()

    /** The monitored stack. */
    public val stack: MachineStack = MachineStack()

    /** Everything the machine's operations print or report appends here. */
    public val transcript: StringBuilder = StringBuilder()

    /** The flag register: `test` writes it, `branch` reads it. */
    public val flag: Register = Register("flag")

    /** The operations table: the two basic-machine operations first, then
     *  the caller's, which may override them. */
    public val operations: Map<String, Op> =
        buildMap {
            put("initialize-stack") { _ ->
                stack.initialize()
                VNil
            }
            put("print-stack-statistics") { _ ->
                transcript.appendLine(stack.statistics())
                VNil
            }
            putAll(userOperations)
        }

    /** The program counter: the index of the next instruction to execute. */
    public var pc: Int = 0

    /** The label table the assembler built: label to instruction index. */
    public var labels: Map<String, Int> = emptyMap()
        private set

    /** The assembled instructions; a subclass's [execute] walks these. */
    protected var insts: List<Inst> = emptyList()

    /** True once [pc] has walked off the end of the sequence: the stop
     *  address, where a trailing label points. */
    public val halted: Boolean get() = pc >= insts.size

    init {
        registerNames.forEach { allocateRegister(it) }
    }

    /** Allocates one register; a duplicated name is a host bug. Subclasses
     *  override [newRegister] to choose the register's class (5.18). */
    public fun allocateRegister(name: Reg): Register {
        check(name !in registers) { "multiply defined register: $name" }
        return newRegister(name).also { registers[name] = it }
    }

    /** Builds the register named [name]. */
    protected open fun newRegister(name: Reg): Register = Register(name)

    /** The register an instruction names; an unallocated name is an
     *  assembly error, unless the machine derives its registers from the
     *  controller (exercise 5.13), in which case first sight allocates. */
    context(r: Raise<MachineError>)
    public fun registerFor(name: Reg): Register {
        registers[name]?.let { return it }
        if (deriveRegisters) {
            return allocateRegister(name)
        }
        return r.raise(MachineError.UnknownRegister(name))
    }

    /** Assembles [controller] and installs it as the instruction sequence.
     *  Assembly faults (unknown registers, labels, or operations) raise
     *  through the context. */
    context(r: Raise<MachineError>)
    public fun install(controller: List<sicp.runtime.Stmt>) {
        install(assemble(controller, this))
    }

    /** Installs an already-assembled program (the seam the variant
     *  assemblers of exercises 5.9 to 5.11 plug). */
    public fun install(program: CompiledProgram) {
        insts = program.insts
        labels = program.labels
    }

    /** Runs the machine from the beginning of the controller sequence and
     *  stops when it reaches the end. Subclasses that keep per-run state
     *  (exercise 5.19's breakpoints) override this to re-arm it. */
    context(r: Raise<MachineError>)
    public open fun start() {
        pc = 0
        execute()
    }

    /** The execution loop: runs the instruction under [pc] and repeats
     *  until the sequence ends. Exercises 5.15 to 5.19 override this one
     *  method to count, trace, or stop between instructions. */
    context(r: Raise<MachineError>)
    protected open fun execute() {
        while (pc < insts.size) {
            insts[pc].exec(r)
        }
    }
}

/** The section's interface procedure: constructs the machine model with the
 *  given registers, operations, and controller. */
context(r: Raise<MachineError>)
public fun makeMachine(
    registerNames: List<Reg>,
    operations: Map<String, Op>,
    controller: List<sicp.runtime.Stmt>,
): Machine = Machine(registerNames, operations).apply { install(controller) }

/** Exercise 5.13's interface: no register list. The registers are
 *  allocated one at a time as the assembler first sees them. */
context(r: Raise<MachineError>)
public fun makeMachineDerivingRegisters(
    operations: Map<String, Op>,
    controller: List<sicp.runtime.Stmt>,
): Machine = Machine(emptyList(), operations, deriveRegisters = true).apply { install(controller) }

/** The book's alternate interface: reads a register's contents. */
context(r: Raise<MachineError>)
public fun Machine.getRegisterContents(name: Reg): Value = registerFor(name).content

/** The book's alternate interface: stores a value into a register. */
context(r: Raise<MachineError>)
public fun Machine.setRegisterContents(
    name: Reg,
    value: Value,
) {
    registerFor(name).store(value)
}

/** Operand and source builders, the constructors of the controller
 *  language: `(reg n)`, `(const 0)`, `(label x)`, and an operation call. */
public fun reg(name: Reg): Source.RegSrc = Source.RegSrc(name)

public fun constV(value: Long): Source.ConstSrc = Source.ConstSrc(sicp.runtime.VInt(value))

public fun constV(value: Double): Source.ConstSrc = Source.ConstSrc(sicp.runtime.VReal(value))

public fun labelSrc(name: String): Source.LabelSrc = Source.LabelSrc(name)

public fun opSrc(
    name: String,
    vararg args: Source,
): Source.OpSrc = Source.OpSrc(name, args.toList())

/** The condition of a `test`: an operation call setting the flag. */
public fun opCond(
    name: String,
    vararg args: Source,
): sicp.runtime.OpCond = sicp.runtime.OpCond(name, args.toList())

/** The section's shared arithmetic, the operations the chapter's machines
 *  assume: the exact arithmetic on integers, the real fallback on mixed or
 *  real words, and the comparisons. */
public val arithOperations: Map<String, Op> =
    mapOf(
        "+" to arith2("+", { a, b -> a + b }) { a, b -> a + b },
        "-" to arith2("-", { a, b -> a - b }) { a, b -> a - b },
        "*" to arith2("*", { a, b -> a * b }) { a, b -> a * b },
        "rem" to arith2("rem", { a, b -> a % b }) { a, b -> a % b },
        "=" to compare2 { a, b -> a == b },
        "<" to compare2 { a, b -> a < b },
        ">" to compare2 { a, b -> a > b },
    )

private fun arith2(
    name: String,
    exact: (Long, Long) -> Long,
    inexact: (Double, Double) -> Double,
): Op =
    { args ->
        val (a, b) = args
        if (a is sicp.runtime.VInt && b is sicp.runtime.VInt) {
            sicp.runtime.VInt(exact(a.n, b.n))
        } else {
            sicp.runtime.VReal(inexact(asReal(name, a), asReal(name, b)))
        }
    }

private fun compare2(op: (Double, Double) -> Boolean): Op =
    { args ->
        val (a, b) = args
        sicp.runtime.VBool(op(asReal("", a), asReal("", b)))
    }

private fun asReal(
    name: String,
    v: Value,
): Double =
    when (v) {
        is sicp.runtime.VInt -> v.n.toDouble()
        is sicp.runtime.VReal -> v.d
        else -> throw IllegalArgumentException("$name: not a number: $v")
    }

/** A `print` operation appending the rendered value to [sink], the way the
 *  driver machines observe a result. */
public fun printOp(sink: StringBuilder): Op =
    { args ->
        sink.appendLine(args.first().toString())
        args.first()
    }
