// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch5

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Argument
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CallableReference
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.GuestType
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.Primitives
import sicp.guest.Return
import sicp.guest.RunResult
import sicp.guest.Span
import sicp.guest.Statement
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.TopProperty
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.While
import sicp.guest.checkedLiteralValue
import sicp.guest.renderPrinted
import sicp.guest.valueEquals
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Machine
import sicp.runtime.MachineOp
import sicp.runtime.MachineProgramError
import sicp.runtime.OpCond
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Source.LabelSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.assemble

/** The linkage of section 5.5: how a compiled sequence continues. */
public sealed interface Linkage {
    /** Fall through to the next instruction. */
    public data object Next : Linkage

    /** Return to the label held in `continue`. */
    public data class ReturnTo(
        val label: String,
    ) : Linkage

    /** Branch to a label. */
    public data class BranchTo(
        val label: String,
    ) : Linkage
}

/** Register discipline around compiled sequences (exercise 5.37). */
public enum class PreservingMode {
    /** Save only registers later sequences need (the shipped default). */
    LIVENESS,

    /** Save every register the first sequence modifies. */
    ALWAYS,

    /** Omit optional saves; correctness-required saves remain. */
    NEVER,
}

/** The compiler switches of exercises 5.32-5.44: they select between the
 * code-generation strategies the book compares, never between behaviors of
 * one program. */
public class CompilerOptions(
    /** Save/restore discipline around sequences (5.37). */
    public val preserving: PreservingMode = PreservingMode.LIVENESS,
    /** Argument-list assembly: stack/reverse when true, incremental append
     * when false. Both evaluate operand effects in source order (5.36). */
    public val leftToRightArguments: Boolean = true,
    /** Operators compiled inline instead of through the generic primitive
     * application path (5.38, 5.44). */
    public val openCodedPrimitives: Set<String> = setOf("+", "-", "*", "/", "%", "<", ">", "<=", ">=", "==", "!=", "&&", "||", "!"),
    /** Frame-distance addresses when true; name-only lookup when false
     * (5.40-5.42). */
    public val lexicalAddressing: Boolean = true,
    /** Hoist local function declarations to the front of their block before
     * compiling (5.43). */
    public val scanOutDefines: Boolean = true,
)

/** An instruction sequence with its register needs and modifications — the
 * liveness contract of section 5.5's `preserving` lesson. */
public class InstrSeq(
    /** Registers the sequence assumes are assigned. */
    public val needs: Set<String>,
    /** Registers the sequence writes. */
    public val modifies: Set<String>,
    /** The machine instructions. */
    public val stmts: List<Stmt>,
)

/** `preserving`: save around [first] the registers [second] needs that
 * [first] modifies and does not itself need. */
public fun preserving(
    needed: Set<String>,
    first: InstrSeq,
    second: InstrSeq,
): InstrSeq {
    val mustSave = (first.modifies intersect needed) - first.needs
    val saves = mustSave.map { Save(it) }
    val restores = mustSave.reversed().map { Restore(it) }
    val needs = first.needs + (second.needs - first.modifies)
    return InstrSeq(needs, first.modifies + second.modifies, saves + first.stmts + restores + second.stmts)
}

/** Sequential composition of instruction sequences. */
public fun appendSeq(
    first: InstrSeq,
    second: InstrSeq,
): InstrSeq = InstrSeq(first.needs + (second.needs - first.modifies), first.modifies + second.modifies, first.stmts + second.stmts)

/** Appends raw statements to a sequence without changing its bookkeeping. */
public fun tackOnInstrSeq(
    seq: InstrSeq,
    stmts: List<Stmt>,
): InstrSeq = InstrSeq(seq.needs, seq.modifies, seq.stmts + stmts)

/**
 * The compiler of section 5.5 over the shared checked syntax: core forms
 * compile to instruction sequences with explicit linkage, stack discipline,
 * and lexical addressing from the compile-time frame list
 * (`List<List<String>>`); the remaining grammar forms compile to one
 * machine operation over their checked node — the operation layer — never
 * to silent stand-ins. Compiled procedures enter by label through
 * `goto (reg target)` and return through `continue`, so nested compiled
 * calls are real machine control flow.
 */
public object Compiler {
    /** Compiles a checked program to machine instructions. */
    public fun compile(
        checked: CheckedProgram,
        options: CompilerOptions = CompilerOptions(),
    ): Either<AdmissionError, List<Stmt>> = Either.Right(ProgramCompiler(checked, CompileRuntime(OutputSink()), options).compileUnit())

    /** Runs compiled source; named, checked top-level function declarations
     * may be implemented by interpreted closures for exercise 5.47. */
    public fun compileAndRun(
        checked: CheckedProgram,
        options: CompilerOptions = CompilerOptions(),
        interpretedBindings: Map<String, GValue.VFunction> = emptyMap(),
        interpretedSink: OutputSink? = null,
    ): RunResult {
        val sink = OutputSink()
        val runtime = CompileRuntime(sink, interpretedBindings, interpretedSink)
        val instructions = compileChecked(checked, runtime, options)
        val outcome: Either<GuestError, GValue> =
            either {
                val machine = Machine(REGS, runtime.operations(), instructions)
                runtime.bind(machine)
                machine.run()
                machine.registers["val"]?.content ?: GValue.VUnit
            }
        return outcome.fold(
            { error -> RunResult(sink.contents(), error, null) },
            { value -> RunResult(sink.contents(), null, value) },
        )
    }

    /** The compiled runtime's operation table, so exercises can assemble a
     * monitored compiled machine (5.34, 5.45, 5.46) over [REGS]. */
    public fun operations(
        checked: CheckedProgram,
        sink: OutputSink = OutputSink(),
    ): Map<String, MachineOp> {
        val runtime = CompileRuntime(sink)
        compileChecked(checked, runtime, CompilerOptions())
        return runtime.operations()
    }

    /** The compiled machine a run drives, for the monitoring exercises. */
    public fun machine(
        checked: CheckedProgram,
        options: CompilerOptions = CompilerOptions(),
        sink: OutputSink = OutputSink(),
    ): Machine {
        val runtime = CompileRuntime(sink)
        val instructions = compileChecked(checked, runtime, options)
        val machine = Machine(REGS, runtime.operations(), instructions)
        runtime.bind(machine)
        return machine
    }

    /** Session 5.49: admit, append, execute, and observe one compiled form at
     * a time on the same assembled machine. */
    public fun session(
        mode: Mode = Mode.CORE,
        options: CompilerOptions = CompilerOptions(),
    ): CompilerSession = CompilerSession(mode, options)

    /** Checked syntax has already passed admission. */
    private fun compileChecked(
        checked: CheckedProgram,
        runtime: CompileRuntime,
        options: CompilerOptions,
    ): List<Stmt> = ProgramCompiler(checked, runtime, options).compileUnit()

    /** The register names the compiled machine uses. */
    public val REGS: Set<String> = setOf("val", "env", "proc", "argl", "continue", "target")
}

/** A session distinguishes source rejection, machine assembly, and a stopped
 * guest run; only [RunResult.error] represents an error raised by guest code. */
public sealed interface CompilerSessionError {
    public data class AdmissionRejected(
        val rejection: AdmissionError,
    ) : CompilerSessionError

    public data class EntryMissing(
        val name: String,
    ) : CompilerSessionError

    public data class MachineRejected(
        val program: MachineProgramError,
    ) : CompilerSessionError

    public data class ExecutionStopped(
        val cause: GuestError,
    ) : CompilerSessionError
}

/** A persistent section 5.49 read-compile-execute-print machine. Every
 * submitted source is checked together with previously accepted declarations
 * before any of its effects run. The named entry must be a zero-argument
 * function; only declarations new in this form are appended to the controller.
 * An assembly fault or guest fault stops the session rather than replaying
 * previous effects on a replacement machine. */
public class CompilerSession internal constructor(
    private val mode: Mode,
    private val options: CompilerOptions,
) {
    private val sink = OutputSink()
    private val runtime = CompileRuntime(sink)
    private var sourceText = ""
    private var compiler: ProgramCompiler? = null
    private var activeMachine: Machine? = null
    private var terminalError: CompilerSessionError? = null

    /** The same machine across accepted turns; null before the first turn. */
    public val machine: Machine? get() = activeMachine

    /** The output is only this turn's delta, not previous turns replayed. */
    public fun appendAndRun(
        source: String,
        entryName: String,
    ): Either<CompilerSessionError, RunResult> {
        terminalError?.let { return Either.Left(it) }
        if (activeMachine?.halted() == false) {
            return Either.Left(CompilerSessionError.MachineRejected(MachineProgramError.Running))
        }
        val candidate = if (sourceText.isEmpty()) source else "$sourceText\n$source"
        return Admission.admit(candidate, mode, requireEntryPoint = false).fold(
            { Either.Left(CompilerSessionError.AdmissionRejected(it)) },
            { checked -> compileAndExecute(checked, candidate, entryName) },
        )
    }

    private fun compileAndExecute(
        checked: CheckedProgram,
        candidate: String,
        entryName: String,
    ): Either<CompilerSessionError, RunResult> {
        val entry =
            checked.syntax.declarations
                .filterIsInstance<FunctionDecl>()
                .firstOrNull { it.name == entryName && it.parameters.isEmpty() }
                ?: return Either.Left(CompilerSessionError.EntryMissing(entryName))
        val unit = compiler ?: ProgramCompiler(checked, runtime, options).also { compiler = it }
        val instructions = unit.compileUnit(checked, entry.name)
        val previous = activeMachine
        val assemblyError =
            if (previous == null) {
                assemble(instructions).fold({ it }, { null })
            } else {
                previous.append(instructions).fold({ it }, { null })
            }
        if (assemblyError != null) {
            val stopped = CompilerSessionError.MachineRejected(assemblyError)
            terminalError = stopped
            return Either.Left(stopped)
        }
        val machine =
            previous ?: Machine(Compiler.REGS, runtime.operations(), instructions).also {
                activeMachine = it
                runtime.bind(it)
            }
        sourceText = candidate
        val cursor = sink.mark()
        val outcome: Either<GuestError, GValue> =
            either {
                machine.run()
                machine.registers["val"]?.content ?: GValue.VUnit
            }
        return Either.Right(
            outcome.fold(
                { error ->
                    terminalError = CompilerSessionError.ExecutionStopped(error)
                    RunResult(sink.since(cursor), error, null)
                },
                { value -> RunResult(sink.since(cursor), null, value) },
            ),
        )
    }
}

/** The compiled runtime: global environment construction, procedure entry,
 * lexical lookup, and the operation layer for forms outside core codegen. */
internal class CompileRuntime(
    private val sink: OutputSink,
    private val interpretedBindings: Map<String, GValue.VFunction> = emptyMap(),
    private val interpretedSink: OutputSink? = null,
) {
    private val globals = Env.root()
    private val entries = linkedMapOf<String, Pair<String, List<String>>>()
    private val binaryOperators = setOf("to", "+", "-", "*", "/", "%", "<", "<=", ">", ">=", "==", "!=", "===", "!==")
    private var globalsInstalled = false
    private var machine: Machine? = null

    fun bind(machine: Machine) {
        this.machine = machine
    }

    fun registerEntry(
        name: String,
        entry: String,
        params: List<String>,
    ) {
        entries[name] = entry to params
        if (globalsInstalled) globals.define(name, procedureValue(entry, params))
    }

    fun registerClass(
        name: String,
        properties: List<String>,
        isData: Boolean,
        isObject: Boolean,
    ) {
        classes[name] = ClassShape(properties, isData, isObject)
        if (globalsInstalled && isObject) globals.define(name, GValue.VObject(name, isData, mutableMapOf()))
    }

    fun registerMethod(
        className: String,
        methodName: String,
        label: String,
        params: List<String>,
    ) {
        methods.getOrPut(className) { linkedMapOf() }[methodName] = MethodEntry(label, params)
    }

    private val methods = linkedMapOf<String, LinkedHashMap<String, MethodEntry>>()

    private class MethodEntry(
        val label: String,
        val params: List<String>,
    )

    private val classes = linkedMapOf<String, ClassShape>()

    private class ClassShape(
        val properties: List<String>,
        val isData: Boolean,
        val isObject: Boolean,
    )

    fun operations(): Map<String, MachineOp> {
        val ops = linkedMapOf<String, MachineOp>()
        ops["compiled-const"] = { args -> args[0] }
        ops["compiled-globals"] = { _ -> GValue.VEnvVal(installGlobals()) }
        ops["session-root"] = { _ -> GValue.VEnvVal(globals) }
        ops["child-env"] = { args -> GValue.VEnvVal(Env.child(asEnv(args[0]))) }
        ops["compiled-bind"] = { args -> GValue.VEnvVal(bindProcedure(args[0], args[1])) }
        ops["procedure-entry"] = { args -> procedureEntry(args[0]) }
        ops["compiled-lookup"] = { args -> compiledLookup(args) }
        ops["adjoin-arg"] = { args -> GValue.VList((mutableListOf(args[0]) + (args[1] as GValue.VList).items).toMutableList(), false) }
        ops["compiled-empty-args"] = { _ -> GValue.VList(mutableListOf(), false) }
        ops["compiled-binary"] = { args -> Primitives.binary((args[0] as GValue.VString).value, args[1], args[2], NO_POSITION) }
        ops["compiled-unary"] = { args -> Primitives.unary((args[0] as GValue.VString).value, args[1], NO_POSITION) }
        ops["declare-local"] = { args ->
            val env = asEnv(args[2])
            env.define((args[0] as GValue.VString).value, args[1])
            GValue.VUnit
        }
        ops["compiled-assign"] = { args -> compiledAssign(args) }
        ops["compiled-unit"] = { _ -> GValue.VUnit }
        ops["compiled-true"] = { _ -> GValue.VBool(true) }
        ops["is-true"] = { args -> GValue.VBool(Primitives.truth(args[0], NO_POSITION)) }
        ops["compiled-null"] = { _ -> GValue.VNull }
        ops["compiled-is-null"] = { args -> GValue.VBool(args[0] is GValue.VNull) }
        ops["compiled-property"] = { args -> compiledProperty(args[0], (args[1] as GValue.VString).value) }
        ops["compiled-index"] = { args -> compiledIndex(args[0], args[1]) }
        ops["compiled-is"] = { args -> GValue.VBool(isTypeName(args[1], (args[0] as GValue.VString).value)) }
        ops["compiled-equal"] = { args -> GValue.VBool(valueEquals(args[0], args[1])) }
        ops["compiled-render"] = { args -> GValue.VString(renderPrinted(args[0]) ?: shapeFault()) }
        ops["compiled-concat"] = { args -> GValue.VString(textOf(args[0]) + textOf(args[1])) }
        ops["compiled-construct"] = { args -> compiledConstruct(args) }
        ops["compiled-primitive"] = { args ->
            val name = (args[0] as GValue.VString).value
            val operands = (args[1] as GValue.VList).items
            when {
                operands.size == 2 && name in binaryOperators -> {
                    Primitives.binary(name, operands[0], operands[1], NO_POSITION)
                }

                operands.size == 1 && (name == "-" || name == "!") -> {
                    Primitives.unary(name, operands[0], NO_POSITION)
                }

                else -> {
                    Primitives.call(name, operands, sink, NO_POSITION)
                }
            }
        }
        ops["compiled-pair-args"] = { args -> GValue.VList(mutableListOf(args[0], args[1]), false) }
        ops["compiled-singleton"] = { args -> GValue.VList(mutableListOf(args[0]), false) }
        ops["compiled-destructure"] = { args -> destructureInto(args[0], (args[1] as GValue.VInt).value) }
        ops["compiled-write-property"] = { args -> compiledWriteProperty(args) }
        ops["compiled-write-index"] = { args -> compiledWriteIndex(args) }
        ops["compiled-method-proc"] = { args -> methodProc(args) }
        ops["compiled-capture"] = { args -> captureEnv(args) }
        ops["compiled-apply-fn"] = { args -> applyFunctionValue(args) }
        ops["collection-start"] = { args -> collectionStart(args[0], args[1], args[2]) }
        ops["collection-done"] = { args -> GValue.VBool(collectionDone(args[0])) }
        ops["collection-args"] = { args -> collectionArguments(args[0]) }
        ops["collection-callback"] = { args -> collectionFields(args[0]).getValue("callback") }
        ops["collection-step"] = { args -> collectionStep(args[0], args[1]) }
        ops["collection-result"] = { args -> collectionFields(args[0]).getValue("acc") }
        ops["compiled-is-interpreted"] = { args -> GValue.VBool(args[0] is GValue.VFunction) }
        ops["compiled-is-procedure"] = { args ->
            val proc = args[0]
            GValue.VBool(proc is GValue.VObject && proc.className == "compiled-procedure")
        }
        ops["stack-peek"] = { _ -> machine?.stack?.peek() ?: raise(GuestError.UnassignedRead(NO_POSITION)) }
        ops["append-arg"] = { args ->
            val tail = args[1] as? GValue.VList ?: raise(GuestError.ShapeFault(NO_POSITION))
            GValue.VList((tail.items + args[0]).toMutableList(), false)
        }
        ops["error-value"] = { _ -> raise(GuestError.ShapeFault(NO_POSITION)) }
        return ops
    }

    private fun asEnv(value: GValue): Env = (value as GValue.VEnvVal).env

    private fun procedureValue(
        label: String,
        params: List<String>,
    ): GValue =
        GValue.VObject(
            "compiled-procedure",
            false,
            mutableMapOf(
                "entry" to GValue.VString(label),
                "params" to GValue.VList(params.map { GValue.VString(it) }.toMutableList(), false),
                "env" to GValue.VEnvVal(globals),
            ),
        )

    context(r: Raise<GuestError>)
    private fun installGlobals(): Env {
        if (globalsInstalled) return globals
        for ((name, shape) in classes) {
            if (shape.isObject) globals.define(name, GValue.VObject(name, shape.isData, mutableMapOf()))
        }
        for ((name, entry) in entries) globals.define(name, procedureValue(entry.first, entry.second))
        for ((name, implementation) in interpretedBindings) {
            if (name !in entries) return r.raise(GuestError.UnassignedRead(NO_POSITION))
            globals.bindings.getValue(name).value = implementation
        }
        globalsInstalled = true
        return globals
    }

    context(r: Raise<GuestError>)
    private fun bindProcedure(
        proc: GValue,
        argl: GValue,
    ): Env {
        if (proc !is GValue.VObject || proc.className != "compiled-procedure") return r.raise(GuestError.ShapeFault(NO_POSITION))
        val rawParams = (proc.fields["params"] as? GValue.VList)?.items ?: return r.raise(GuestError.ShapeFault(NO_POSITION))
        val params = rawParams.map { (it as? GValue.VString)?.value ?: return r.raise(GuestError.ShapeFault(NO_POSITION)) }
        val arguments = (argl as? GValue.VList)?.items ?: return r.raise(GuestError.ShapeFault(NO_POSITION))
        val values = proc.fields["bound"]?.let { listOf(it) + arguments } ?: arguments
        if (values.size != params.size) return r.raise(GuestError.ShapeFault(NO_POSITION))
        val parent = (proc.fields["env"] as? GValue.VEnvVal)?.env ?: Env.root()
        val env = Env.child(parent)
        for ((index, name) in params.withIndex()) env.define(name, values[index])
        return env
    }

    context(r: Raise<GuestError>)
    private fun procedureEntry(proc: GValue): GValue {
        if (proc !is GValue.VObject || proc.className != "compiled-procedure") return r.raise(GuestError.ShapeFault(NO_POSITION))
        val label = (proc.fields["entry"] as? GValue.VString)?.value ?: return r.raise(GuestError.ShapeFault(NO_POSITION))
        val index = machine?.labels?.get(label) ?: return r.raise(GuestError.ShapeFault(NO_POSITION))
        return GValue.VInt(index)
    }

    context(r: Raise<GuestError>)
    private fun shapeFault(): Nothing = r.raise(GuestError.ShapeFault(NO_POSITION))

    context(r: Raise<GuestError>)
    private fun compiledProperty(
        receiver: GValue,
        name: String,
    ): GValue {
        if (receiver is GValue.VObject) {
            receiver.fields[name]?.let { return it }
            val method = methods[receiver.className]?.get(name)
            if (method != null) {
                return GValue.VObject(
                    "compiled-procedure",
                    false,
                    mutableMapOf(
                        "entry" to GValue.VString(method.label),
                        "params" to GValue.VList(method.params.map { GValue.VString(it) }.toMutableList(), false),
                        "env" to GValue.VEnvVal(globals),
                        "bound" to receiver,
                    ),
                )
            }
        }
        return Primitives.property(receiver, name, NO_POSITION)
    }

    context(r: Raise<GuestError>)
    private fun compiledIndex(
        receiver: GValue,
        index: GValue,
    ): GValue {
        if (receiver is GValue.VList && index is GValue.VInt) {
            if (index.value < 0 || index.value >= receiver.items.size) return r.raise(GuestError.IndexOutOfBounds(NO_POSITION))
            return receiver.items[index.value]
        }
        if (receiver is GValue.VMap) return receiver.entries[index] ?: GValue.VNull
        return shapeFault()
    }

    private fun isTypeName(
        value: GValue,
        name: String,
    ): Boolean {
        if (value is GValue.VNull && name.endsWith("?")) return true
        val core = name.removeSuffix("?")
        return when {
            core == "Int" -> {
                value is GValue.VInt
            }

            core == "Long" -> {
                value is GValue.VLong
            }

            core == "Double" -> {
                value is GValue.VDouble
            }

            core == "Boolean" -> {
                value is GValue.VBool
            }

            core == "String" -> {
                value is GValue.VString
            }

            core == "List" || core == "Collection" -> {
                value is GValue.VList || value is GValue.VLazyList
            }

            core == "MutableList" -> {
                value is GValue.VList && value.mutable
            }

            core == "Set" -> {
                value is GValue.VList && value.asSet
            }

            core == "Map" || core == "MutableMap" -> {
                value is GValue.VMap
            }

            core == "Pair" -> {
                value is GValue.VPair
            }

            core == "Function" -> {
                value is GValue.VFunction ||
                    (value is GValue.VObject && value.className == "compiled-procedure")
            }

            core == "Nothing" -> {
                false
            }

            core == "Unit" -> {
                value is GValue.VUnit
            }

            core == "Null" -> {
                value is GValue.VNull
            }

            else -> {
                value is GValue.VObject && value.className == core
            }
        }
    }

    private fun textOf(value: GValue): String =
        when (value) {
            is GValue.VString -> value.value
            is GValue.VLong -> value.value.toString()
            is GValue.VInt -> value.value.toString()
            is GValue.VDouble -> value.value.toString()
            is GValue.VBool -> value.value.toString()
            else -> ""
        }

    context(r: Raise<GuestError>)
    private fun compiledConstruct(args: List<GValue>): GValue {
        val className = (args[0] as GValue.VString).value
        val propertyNames = (args[1] as GValue.VList).items.map { (it as GValue.VString).value }
        val values = (args[2] as GValue.VList).items
        val argumentNames = (args[3] as GValue.VList).items.map { (it as GValue.VString).value }
        if (propertyNames.size != values.size || argumentNames.size != values.size) return shapeFault()
        val fields = linkedMapOf<String, GValue>()
        for ((index, name) in propertyNames.withIndex()) {
            val source = argumentNames.indexOf(name).takeIf { it >= 0 } ?: index
            fields[name] = values[source]
        }
        val shape = classes[className] ?: return shapeFault()
        return GValue.VObject(className, structural = shape.isData, fields)
    }

    context(r: Raise<GuestError>)
    private fun destructureInto(
        source: GValue,
        count: Int,
    ): GValue {
        val values =
            when (source) {
                is GValue.VObject -> source.fields.values.toList()
                is GValue.VPair -> listOf(source.first, source.second)
                is GValue.VList -> source.items.toList()
                else -> return shapeFault()
            }
        if (values.size < count) return shapeFault()
        return GValue.VList(values.take(count).toMutableList(), false)
    }

    context(r: Raise<GuestError>)
    private fun compiledWriteProperty(args: List<GValue>): GValue {
        val receiver = args[0] as? GValue.VObject ?: return shapeFault()
        val name = (args[1] as? GValue.VString)?.value ?: return shapeFault()
        if (!receiver.fields.containsKey(name)) return shapeFault()
        receiver.fields[name] = args[2]
        return args[2]
    }

    context(r: Raise<GuestError>)
    private fun compiledWriteIndex(args: List<GValue>): GValue {
        Primitives.writeIndex(args[0], args[1], args[2], NO_POSITION)
        return args[2]
    }

    context(r: Raise<GuestError>)
    private fun methodProc(args: List<GValue>): GValue {
        val values = (args[0] as? GValue.VList)?.items ?: return shapeFault()
        val received = values.firstOrNull() ?: return shapeFault()
        val name = (args[1] as? GValue.VString)?.value ?: return shapeFault()
        val entry = (received as? GValue.VObject)?.let { methods[it.className]?.get(name) }
        if (entry != null) {
            return GValue.VObject(
                "compiled-procedure",
                false,
                mutableMapOf(
                    "entry" to GValue.VString(entry.label),
                    "params" to GValue.VList(entry.params.map { GValue.VString(it) }.toMutableList(), false),
                    "env" to GValue.VEnvVal(globals),
                ),
            )
        }
        if (name == "copy" && received is GValue.VObject && received.structural) {
            val names =
                (args[2] as? GValue.VList)?.items?.map {
                    (it as? GValue.VString)?.value ?: return shapeFault()
                } ?: return shapeFault()
            return GValue.VFunction("copy", values.size) { arguments ->
                val original =
                    arguments.firstOrNull() as? GValue.VObject
                        ?: raise(GuestError.ShapeFault(NO_POSITION))
                val fields = LinkedHashMap(original.fields)
                for ((index, property) in names.withIndex()) {
                    if (property !in fields) raise(GuestError.ShapeFault(NO_POSITION))
                    fields[property] = arguments.getOrNull(index + 1)
                        ?: raise(GuestError.ShapeFault(NO_POSITION))
                }
                GValue.VObject(original.className, true, fields)
            }
        }
        val field = (received as? GValue.VObject)?.fields?.get(name)
        if (field is GValue.VFunction) {
            return GValue.VFunction(name, values.size) { arguments -> field.apply(this, arguments.drop(1)) }
        }
        return GValue.VFunction(name, values.size) { arguments ->
            Primitives.member(arguments.first(), name, arguments.drop(1), NO_POSITION)
        }
    }

    context(r: Raise<GuestError>)
    private fun collectionFields(state: GValue): MutableMap<String, GValue> {
        val carrier = state as? GValue.VObject ?: return shapeFault()
        if (carrier.className != "compiled-collection") return shapeFault()
        return carrier.fields
    }

    context(r: Raise<GuestError>)
    private fun collectionStart(
        receiver: GValue,
        kindValue: GValue,
        argumentsValue: GValue,
    ): GValue {
        val items = receiver as? GValue.VList ?: return shapeFault()
        val kind = (kindValue as? GValue.VString)?.value ?: return shapeFault()
        val arguments = (argumentsValue as? GValue.VList)?.items ?: return shapeFault()
        val expected = if (kind == "fold") 2 else 1
        if (arguments.size != expected) return shapeFault()
        val initial: GValue =
            when (kind) {
                "map", "filter" -> GValue.VList(mutableListOf(), false, asSet = kind == "filter" && items.asSet)
                "fold" -> arguments[0]
                "any" -> GValue.VBool(false)
                "all" -> GValue.VBool(true)
                else -> return shapeFault()
            }
        return GValue.VObject(
            "compiled-collection",
            false,
            mutableMapOf(
                "items" to items,
                "kind" to GValue.VString(kind),
                "callback" to arguments.last(),
                "index" to GValue.VInt(0),
                "acc" to initial,
            ),
        )
    }

    context(r: Raise<GuestError>)
    private fun collectionDone(state: GValue): Boolean {
        val fields = collectionFields(state)
        val kind = (fields.getValue("kind") as GValue.VString).value
        val acc = fields.getValue("acc")
        if (kind == "any" && acc is GValue.VBool && acc.value) return true
        if (kind == "all" && acc is GValue.VBool && !acc.value) return true
        val index = (fields.getValue("index") as GValue.VInt).value
        return index >= (fields.getValue("items") as GValue.VList).items.size
    }

    context(r: Raise<GuestError>)
    private fun collectionArguments(state: GValue): GValue {
        val fields = collectionFields(state)
        val index = (fields.getValue("index") as GValue.VInt).value
        val item = (fields.getValue("items") as GValue.VList).items[index]
        val values =
            if ((fields.getValue("kind") as GValue.VString).value == "fold") {
                mutableListOf(fields.getValue("acc"), item)
            } else {
                mutableListOf(item)
            }
        return GValue.VList(values, false)
    }

    context(r: Raise<GuestError>)
    private fun collectionStep(
        state: GValue,
        value: GValue,
    ): GValue {
        val fields = collectionFields(state)
        val index = (fields.getValue("index") as GValue.VInt).value
        val item = (fields.getValue("items") as GValue.VList).items[index]
        when ((fields.getValue("kind") as GValue.VString).value) {
            "map" -> (fields.getValue("acc") as GValue.VList).items.add(value)
            "filter" -> if (Primitives.truth(value, NO_POSITION)) (fields.getValue("acc") as GValue.VList).items.add(item)
            "fold" -> fields["acc"] = value
            "any", "all" -> fields["acc"] = GValue.VBool(Primitives.truth(value, NO_POSITION))
        }
        fields["index"] = GValue.VInt(index + 1)
        return state
    }

    context(r: Raise<GuestError>)
    private fun captureEnv(args: List<GValue>): GValue {
        val proc = args[0] as? GValue.VObject ?: return shapeFault()
        val fields = LinkedHashMap(proc.fields)
        fields["env"] = GValue.VEnvVal(asEnv(args[1]))
        return GValue.VObject(proc.className, proc.structural, fields)
    }

    context(r: Raise<GuestError>)
    private fun applyFunctionValue(args: List<GValue>): GValue {
        val fn = args[0] as? GValue.VFunction ?: return shapeFault()
        val callArgs = (args[1] as? GValue.VList)?.items ?: return shapeFault()
        val external = interpretedSink ?: return fn.apply(r, callArgs)
        val cursor = external.mark()
        return try {
            fn.apply(r, callArgs)
        } finally {
            sink.write(external.since(cursor))
        }
    }

    context(r: Raise<GuestError>)
    private fun compiledLookup(args: List<GValue>): GValue {
        val name = (args[0] as GValue.VString).value
        val address = args[1] as GValue.VList
        val depth = (address.items[0] as GValue.VInt).value
        val displacement = (address.items[1] as GValue.VInt).value
        val origin = asEnv(args[2])
        val cell =
            if (depth < 0) {
                origin.lookup(name)
            } else {
                var frame: Env? = origin
                repeat(depth) { frame = frame?.parent }
                frame
                    ?.bindings
                    ?.entries
                    ?.elementAtOrNull(displacement)
                    ?.takeIf { it.key == name }
                    ?.value
            } ?: return r.raise(GuestError.UnassignedRead(NO_POSITION))
        val value = cell.value
        if (value is GValue.VUnassigned) return r.raise(GuestError.UnassignedRead(NO_POSITION))
        return value
    }

    context(r: Raise<GuestError>)
    private fun compiledAssign(args: List<GValue>): GValue {
        val name = (args[0] as GValue.VString).value
        val value = args[1]
        val cell = asEnv(args[2]).lookup(name) ?: return r.raise(GuestError.UnassignedRead(NO_POSITION))
        cell.value = value
        return value
    }
}

/** One program's compilation: labels, linkage, and the compile-time
 * environment as the lexical address frame list. */
private class ProgramCompiler(
    private var checked: CheckedProgram,
    private val runtime: CompileRuntime,
    private val options: CompilerOptions,
) {
    private var labelCount = 0

    /** Loop exits and continues for the innermost compiled loop. */
    private val loopEnds = ArrayDeque<String>()

    private val loopContinues = ArrayDeque<String>()

    /** Lexical saves active on dynamic early-exit paths, in stack order. */
    private val openSaves = mutableListOf<String>()
    private val loopSaveDepths = ArrayDeque<Int>()

    /** Data-class property order and singleton objects for construction. */
    private val classShapes = linkedMapOf<String, List<String>>()

    /** Compile-time frames, innermost first; each entry is that frame's
     * declared names in declaration order (the lexical address). */
    private val frames: MutableList<MutableList<String>> = mutableListOf()
    private val rootNames = mutableListOf<String>()
    private var compiledDeclarations = 0
    private var loopErrorLabel = ""

    /** Compiles only the declarations appended since this compiler's last
     * segment. Labels and root lexical addresses retain their identities. */
    fun compileUnit(
        next: CheckedProgram = checked,
        entryName: String = "main",
    ): List<Stmt> {
        val all = next.syntax.declarations
        val first = compiledDeclarations == 0
        val declarations = all.subList(compiledDeclarations, all.size)
        checked = next
        compiledDeclarations = all.size
        for (declaration in declarations) {
            when (declaration) {
                is sicp.guest.DataClass -> {
                    classShapes[declaration.name] = declaration.properties.map { it.name }
                    runtime.registerClass(declaration.name, declaration.properties.map { it.name }, isData = true, isObject = false)
                }

                is sicp.guest.DataObject -> {
                    classShapes[declaration.name] = emptyList()
                    runtime.registerClass(declaration.name, emptyList(), isData = true, isObject = true)
                }

                is sicp.guest.PlainClass -> {
                    classShapes[declaration.name] = declaration.properties.map { it.name }
                    runtime.registerClass(declaration.name, declaration.properties.map { it.name }, isData = false, isObject = false)
                }

                else -> {}
            }
        }
        frames.add(rootNames)
        rootNames.addAll(declarations.filterIsInstance<sicp.guest.DataObject>().map { it.name })
        val haltLabel = freshLabel("halt")
        val mainLabel = freshLabel("program-main")
        val endLabel = freshLabel("program-end")
        loopErrorLabel = freshLabel("loop-error")
        val prologue = mutableListOf<Stmt>()
        prologue.add(Assign("env", OpSrc(if (first) "compiled-globals" else "session-root", emptyList())))
        prologue.add(Assign("continue", LabelSrc(haltLabel)))
        prologue.add(Goto(GotoTarget.Lbl(mainLabel)))
        for (declaration in declarations) {
            if (declaration !is FunctionDecl) continue
            val label = freshLabel("${declaration.name}-entry")
            entries[declaration.name] = label
            runtime.registerEntry(declaration.name, label, declaration.parameters.map { it.name })
            rootNames.add(declaration.name)
        }
        rootNames.addAll(declarations.filterIsInstance<TopProperty>().map { it.property.name })
        val body = mutableListOf<Stmt>()
        for (declaration in declarations) {
            if (declaration is sicp.guest.PlainClass) {
                for (method in declaration.methods) {
                    val label = freshLabel("${declaration.name}-${method.name}")
                    runtime.registerMethod(declaration.name, method.name, label, listOf("this") + method.parameters.map { it.name })
                    body.addAll(compileMethodBody(method, label))
                }
            }
        }
        for (declaration in declarations) {
            if (declaration is FunctionDecl) body.addAll(compileFunction(declaration))
        }
        body.add(Label(mainLabel))
        for (declaration in declarations) {
            if (declaration is TopProperty) body.addAll(compileTopProperty(declaration))
        }
        body.addAll(compileEntryCall(entryName, haltLabel))
        body.add(Label(haltLabel))
        body.add(Goto(GotoTarget.Lbl(endLabel)))
        body.add(Label(loopErrorLabel))
        body.add(Assign("val", OpSrc("error-value", emptyList())))
        body.add(Goto(GotoTarget.ByReg("continue")))
        body.add(Label(endLabel))
        frames.removeAt(frames.lastIndex)
        return prologue + body
    }

    private fun compileMethodBody(
        method: FunctionDecl,
        label: String,
    ): List<Stmt> {
        val instructions = mutableListOf<Stmt>()
        instructions.add(Goto(GotoTarget.Lbl("$label-skip")))
        instructions.add(Label(label))
        instructions.add(Assign("env", OpSrc("compiled-bind", listOf(RegSrc("proc"), RegSrc("argl")))))
        frames.add((listOf("this") + method.parameters.map { it.name }).toMutableList())
        instructions.addAll(compileExpression(method.body, Linkage.Next).stmts)
        frames.removeAt(frames.size - 1)
        instructions.add(Goto(GotoTarget.ByReg("continue")))
        instructions.add(Label("$label-skip"))
        return instructions
    }

    private fun compileTopProperty(declaration: TopProperty): List<Stmt> {
        val initializer = compileExpression(declaration.initializer, Linkage.Next)
        val bind =
            Assign(
                "val",
                OpSrc(
                    "declare-local",
                    listOf(sicp.runtime.Source.ConstSrc(GValue.VString(declaration.property.name)), RegSrc("val"), RegSrc("env")),
                ),
            )
        return initializer.stmts + bind
    }

    private fun entryLabel(name: String): String {
        labelCount++
        return "$name-entry-$labelCount"
    }

    private fun freshLabel(prefix: String): String {
        labelCount++
        return "$prefix-$labelCount"
    }

    private fun compileFunction(declaration: FunctionDecl): List<Stmt> {
        val entry = entries[declaration.name] ?: freshLabel("${declaration.name}-entry")
        val instructions = mutableListOf<Stmt>()
        instructions.add(Label(entry))
        instructions.add(Assign("env", OpSrc("compiled-bind", listOf(RegSrc("proc"), RegSrc("argl")))))
        frames.add(declaration.parameters.map { it.name }.toMutableList())
        instructions.addAll(compileExpression(declaration.body, Linkage.Next).stmts)
        frames.removeAt(frames.size - 1)
        instructions.add(Goto(GotoTarget.ByReg("continue")))
        return instructions
    }

    private val entries: MutableMap<String, String> = linkedMapOf()

    private fun compileEntryCall(
        name: String,
        haltLabel: String,
    ): List<Stmt> {
        val call = Call(Name(name, NO_POSITION), emptyList(), emptyList(), NO_POSITION)
        return compileExpression(call, Linkage.ReturnTo(haltLabel)).stmts
    }

    private fun compileExpression(
        expression: Expression,
        linkage: Linkage,
    ): InstrSeq =
        when (expression) {
            is Literal -> simple(literalStmt(expression), setOf("val"))
            is Name -> simple(lookupStmts(expression), setOf("val"))
            is Lambda -> simple(lambdaStmts(expression), setOf("val"))
            is Binary -> compileBinary(expression)
            is Unary -> compileUnary(expression)
            is If -> compileIf(expression, linkage)
            is Call -> compileCall(expression, linkage)
            is StringTemplate -> compileTemplate(expression)
            is Block -> compileBlock(expression)
            is Return -> compileReturn(expression)
            is When -> compileWhen(expression)
            is Member -> compileMember(expression)
            is Index -> compileIndex(expression)
            is Elvis -> compileElvis(expression)
            is Is -> simple(isStmts(expression), setOf("val"))
            is This -> simple(lookupStmts(Name("this", expression.span)), setOf("val"))
            is CallableReference -> simple(lookupStmts(Name(expression.name, expression.span)), setOf("val"))
        }

    private fun compileStatement(statement: Statement): InstrSeq =
        when (statement) {
            is ExpressionStatement -> compileExpression(statement.expression, Linkage.Next)
            is LocalProperty -> compileLocal(statement)
            is Assignment -> compileAssignment(statement)
            is Return -> compileReturn(statement)
            is Break -> simple(breakStmts(statement.span), setOf("val"))
            is Continue -> simple(continueStmts(statement.span), setOf("val"))
            is While -> compileWhile(statement)
            is For -> compileFor(statement)
            is Destructure -> compileDestructure(statement)
            is FunctionDecl -> compileLocalFunction(statement)
        }

    private fun simple(
        stmts: List<Stmt>,
        modifies: Set<String>,
    ): InstrSeq = InstrSeq(emptySet(), modifies, stmts)

    private fun literalStmt(literal: Literal): List<Stmt> =
        listOf(
            Assign(
                "val",
                OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(checkedLiteralValue(literal, checked.types[literal])))),
            ),
        )

    private fun lookupStmts(name: Name): List<Stmt> {
        val found = frames.indexOfLast { name.text in it }
        val depth = if (found < 0) -1 else frames.size - 1 - found
        val addressDepth = if (!options.lexicalAddressing || depth < 0) -1 else depth
        val displacement = if (depth < 0) -1 else frames[found].indexOf(name.text)
        val address = GValue.VList(mutableListOf(GValue.VInt(addressDepth), GValue.VInt(displacement)), false)
        return listOf(
            Assign(
                "val",
                OpSrc(
                    "compiled-lookup",
                    listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name.text)), sicp.runtime.Source.ConstSrc(address), RegSrc("env")),
                ),
            ),
        )
    }

    private fun lambdaStmts(lambda: Lambda): List<Stmt> = lambdaLikeStmts(null, lambda.parameters.map { it.name }, lambda.body)

    private fun lambdaLikeStmts(
        name: String?,
        params: List<String>,
        body: Expression,
    ): List<Stmt> {
        val entry = freshLabel("lambda")
        val exit = freshLabel("lambda-exit")
        val instructions = mutableListOf<Stmt>()
        instructions.add(Goto(GotoTarget.Lbl(exit)))
        instructions.add(Label(entry))
        instructions.add(Assign("env", OpSrc("compiled-bind", listOf(RegSrc("proc"), RegSrc("argl")))))
        frames.add(params.toMutableList())
        val enclosingSaves = openSaves.toList()
        openSaves.clear()
        instructions.addAll((if (body is Block) compileBlockBody(body) else compileExpression(body, Linkage.Next)).stmts)
        openSaves.addAll(enclosingSaves)
        frames.removeAt(frames.size - 1)
        instructions.add(Goto(GotoTarget.ByReg("continue")))
        instructions.add(Label(exit))
        instructions.add(
            Assign(
                "val",
                OpSrc(
                    "compiled-const",
                    listOf(
                        sicp.runtime.Source.ConstSrc(
                            GValue.VObject(
                                "compiled-procedure",
                                false,
                                mutableMapOf(
                                    "entry" to GValue.VString(entry),
                                    "params" to GValue.VList(params.map { GValue.VString(it) }.toMutableList(), false),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
        instructions.add(Assign("val", OpSrc("compiled-capture", listOf(RegSrc("val"), RegSrc("env")))))
        return instructions
    }

    private fun compileShortCircuit(expression: Binary): InstrSeq {
        val left = compileExpression(expression.left, Linkage.Next)
        val right = compileExpression(expression.right, Linkage.Next)
        val trueLabel = freshLabel("boolean-true")
        val end = freshLabel("boolean-end")
        val stmts = mutableListOf<Stmt>()
        stmts.addAll(left.stmts)
        stmts.add(Test(OpCond("is-true", listOf(RegSrc("val")))))
        stmts.add(Branch(trueLabel))
        if (expression.operator == "&&") {
            stmts.add(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(GValue.VBool(false))))))
            stmts.add(Goto(GotoTarget.Lbl(end)))
            stmts.add(Label(trueLabel))
            stmts.addAll(right.stmts)
        } else {
            stmts.addAll(right.stmts)
            stmts.add(Goto(GotoTarget.Lbl(end)))
            stmts.add(Label(trueLabel))
            stmts.add(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(GValue.VBool(true))))))
        }
        stmts.add(Label(end))
        return InstrSeq(left.needs + right.needs, setOf("val"), stmts)
    }

    private fun compileBinary(expression: Binary): InstrSeq {
        if (expression.operator == "&&" || expression.operator == "||") return compileShortCircuit(expression)
        val left = compileExpression(expression.left, Linkage.Next)
        openSaves.add("val")
        val right = compileExpression(expression.right, Linkage.Next)
        openSaves.removeAt(openSaves.lastIndex)
        val combine: List<Stmt> =
            if (expression.operator in options.openCodedPrimitives) {
                listOf(
                    Assign(
                        "val",
                        OpSrc(
                            "compiled-binary",
                            listOf(sicp.runtime.Source.ConstSrc(GValue.VString(expression.operator)), RegSrc("val"), RegSrc("argl")),
                        ),
                    ),
                )
            } else {
                primitiveApplyStmts(expression.operator, 2)
            }
        // Every right operand writes val. Even NEVER must retain its left
        // operand; only optional liveness saves may be omitted.
        val stmts =
            left.stmts + Save("val") + right.stmts + Assign("argl", RegSrc("val")) + Restore("val") + combine
        return InstrSeq(left.needs + right.needs, setOf("val", "argl"), stmts)
    }

    private fun compileUnary(expression: Unary): InstrSeq {
        val operand = compileExpression(expression.operand, Linkage.Next)
        val combine =
            Assign("val", OpSrc("compiled-unary", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(expression.operator)), RegSrc("val"))))
        return InstrSeq(operand.needs, setOf("val"), operand.stmts + combine)
    }

    private fun compileIf(
        expression: If,
        linkage: Linkage,
    ): InstrSeq {
        val trueLabel = freshLabel("if-true")
        val afterLabel = freshLabel("if-after")
        val condition = compileExpression(expression.condition, Linkage.Next)
        val consequent = compileExpression(expression.yes, afterLinkage(linkage, afterLabel))
        val alternative =
            expression.no?.let { compileExpression(it, afterLinkage(linkage, afterLabel)) }
                ?: simple(listOf(Assign("val", OpSrc("compiled-unit", emptyList()))), setOf("val"))
        val stmts =
            condition.stmts +
                Test(OpCond("is-true", listOf(RegSrc("val")))) +
                Branch(trueLabel) +
                alternative.stmts +
                Goto(GotoTarget.Lbl(afterLabel)) +
                Label(trueLabel) +
                consequent.stmts +
                (if (linkage is Linkage.Next) listOf(Label(afterLabel)) else listOf(Label(afterLabel)))
        return InstrSeq(condition.needs + consequent.needs + alternative.needs, setOf("val"), stmts)
    }

    private fun afterLinkage(
        linkage: Linkage,
        afterLabel: String,
    ): Linkage = if (linkage is Linkage.Next) Linkage.Next else linkage

    private fun compileCall(
        expression: Call,
        linkage: Linkage,
        unwind: List<String> = emptyList(),
    ): InstrSeq {
        val calleeName = expression.callee as? Name
        if (calleeName != null && calleeName.text in classShapes) return constructStmts(expression, calleeName.text)
        if (calleeName != null && !resolvableName(calleeName.text)) return primitiveCallStmts(expression, calleeName.text)
        val memberCallee = expression.callee as? Member
        if (memberCallee != null) {
            val staticType = checked.types[memberCallee.receiver]
            val receiverType = (if (staticType is GuestType.Nullable) staticType.base else staticType) as? GuestType.Named
            if (receiverType?.name in setOf("List", "MutableList", "Set", "Collection") &&
                memberCallee.name in setOf("map", "filter", "fold", "any", "all")
            ) {
                return collectionCallStmts(expression, memberCallee)
            }
            return methodCallStmts(expression, memberCallee)
        }
        openSaves.add("continue")
        openSaves.add("env")
        val callee = compileExpression(expression.callee, Linkage.Next)
        openSaves.add("val")
        val argumentSeqs = compileArguments(expression.arguments)
        repeat(3) { openSaves.removeAt(openSaves.lastIndex) }
        val stmts = mutableListOf<Stmt>()
        stmts.add(Save("continue"))
        stmts.add(Save("env"))
        stmts.addAll(callee.stmts)
        stmts.add(Save("val"))
        argumentSave(argumentSeqs, stmts)
        stmts.add(Restore("proc"))
        val interpretedLabel = freshLabel("call-interpreted")
        val returnLabel = freshLabel("return")
        if (linkage is Linkage.ReturnTo) {
            // Preserve the caller's continuation through argument evaluation.
            stmts.add(Restore("env"))
            stmts.add(Restore("continue"))
            for (register in unwind.asReversed()) stmts.add(Restore(register))
        } else {
            stmts.add(Assign("continue", LabelSrc(returnLabel)))
        }
        stmts.add(Test(OpCond("compiled-is-interpreted", listOf(RegSrc("proc")))))
        stmts.add(Branch(interpretedLabel))
        stmts.add(Assign("target", OpSrc("procedure-entry", listOf(RegSrc("proc")))))
        stmts.add(Goto(GotoTarget.ByReg("target")))
        stmts.add(Label(interpretedLabel))
        stmts.add(Assign("val", OpSrc("compiled-apply-fn", listOf(RegSrc("proc"), RegSrc("argl")))))
        if (linkage is Linkage.ReturnTo) {
            stmts.add(Goto(GotoTarget.ByReg("continue")))
        } else {
            stmts.add(Label(returnLabel))
            stmts.add(Restore("env"))
            stmts.add(Restore("continue"))
            if (linkage is Linkage.BranchTo) stmts.add(Goto(GotoTarget.Lbl(linkage.label)))
        }
        val needs = callee.needs + argumentSeqs.flatMap { it.needs }.toSet()
        return InstrSeq(needs, setOf("val", "argl", "proc", "continue", "target", "env"), stmts)
    }

    private fun compileTemplate(template: StringTemplate): InstrSeq {
        if (template.fragments.isEmpty()) {
            return simple(
                listOf(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(GValue.VString("")))))),
                setOf("val"),
            )
        }
        val first = compileExpression(template.fragments.first(), Linkage.Next)
        var stmts = first.stmts + Assign("val", OpSrc("compiled-render", listOf(RegSrc("val"))))
        var needs = first.needs
        for (fragment in template.fragments.drop(1)) {
            openSaves.add("val")
            val next = compileExpression(fragment, Linkage.Next)
            openSaves.removeAt(openSaves.lastIndex)
            needs = needs + next.needs
            stmts = stmts +
                Save("val") +
                next.stmts +
                Assign("val", OpSrc("compiled-render", listOf(RegSrc("val")))) +
                Assign("argl", RegSrc("val")) +
                Restore("val") +
                Assign("val", OpSrc("compiled-concat", listOf(RegSrc("val"), RegSrc("argl"))))
        }
        return InstrSeq(needs, setOf("val", "argl"), stmts)
    }

    private fun resolvableName(name: String): Boolean = entries.containsKey(name) || frames.any { name in it }

    private fun compileArguments(arguments: List<Argument>): List<InstrSeq> {
        val sequences = mutableListOf<InstrSeq>()
        for (argument in arguments) {
            if (!options.leftToRightArguments) openSaves.add("argl")
            sequences.add(compileExpression(argument.value, Linkage.Next))
            if (options.leftToRightArguments) {
                openSaves.add("val")
            } else {
                openSaves.removeAt(openSaves.lastIndex)
            }
        }
        if (options.leftToRightArguments) repeat(arguments.size) { openSaves.removeAt(openSaves.lastIndex) }
        return sequences
    }

    private fun argumentSave(
        argumentSeqs: List<InstrSeq>,
        stmts: MutableList<Stmt>,
    ) {
        if (!options.leftToRightArguments) {
            stmts.add(Assign("argl", OpSrc("compiled-empty-args", emptyList())))
            for (argument in argumentSeqs) {
                stmts.add(Save("argl"))
                stmts.addAll(argument.stmts)
                stmts.add(Restore("argl"))
                stmts.add(Assign("argl", OpSrc("append-arg", listOf(RegSrc("val"), RegSrc("argl")))))
            }
            return
        }
        for (argument in argumentSeqs) {
            stmts.addAll(argument.stmts)
            stmts.add(Save("val"))
        }
        stmts.add(Assign("argl", OpSrc("compiled-empty-args", emptyList())))
        repeat(argumentSeqs.size) {
            stmts.add(Restore("val"))
            stmts.add(Assign("argl", OpSrc("adjoin-arg", listOf(RegSrc("val"), RegSrc("argl")))))
        }
    }

    /** Iterates a collection in the machine. Each callback enters its
     * compiled procedure label with the same captured environment as any
     * ordinary compiled call; the collection state survives in `target`. */
    private fun collectionCallStmts(
        expression: Call,
        callee: Member,
    ): InstrSeq {
        openSaves.add("continue")
        openSaves.add("env")
        val receiver = compileExpression(callee.receiver, Linkage.Next)
        openSaves.add("val")
        val arguments = compileArguments(expression.arguments)
        repeat(3) { openSaves.removeAt(openSaves.lastIndex) }
        val loop = freshLabel("collection-loop")
        val done = freshLabel("collection-done")
        val resumed = freshLabel("collection-resume")
        val interpreted = freshLabel("collection-interpreted")
        val nullCase = if (callee.safe) freshLabel("collection-null") else null
        val end = if (nullCase != null) freshLabel("collection-end") else null
        val stmts = mutableListOf<Stmt>()
        stmts.add(Save("continue"))
        stmts.add(Save("env"))
        stmts.addAll(receiver.stmts)
        if (nullCase != null) {
            stmts.add(Test(OpCond("compiled-is-null", listOf(RegSrc("val")))))
            stmts.add(Branch(nullCase))
        }
        stmts.add(Save("val"))
        argumentSave(arguments, stmts)
        stmts.add(Restore("val"))
        stmts.add(
            Assign(
                "target",
                OpSrc(
                    "collection-start",
                    listOf(
                        RegSrc("val"),
                        sicp.runtime.Source.ConstSrc(GValue.VString(callee.name)),
                        RegSrc("argl"),
                    ),
                ),
            ),
        )
        stmts.add(Label(loop))
        stmts.add(Test(OpCond("collection-done", listOf(RegSrc("target")))))
        stmts.add(Branch(done))
        stmts.add(Save("target"))
        stmts.add(Save("env"))
        stmts.add(Assign("argl", OpSrc("collection-args", listOf(RegSrc("target")))))
        stmts.add(Assign("proc", OpSrc("collection-callback", listOf(RegSrc("target")))))
        stmts.add(Test(OpCond("compiled-is-interpreted", listOf(RegSrc("proc")))))
        stmts.add(Branch(interpreted))
        stmts.add(Assign("continue", LabelSrc(resumed)))
        stmts.add(Assign("target", OpSrc("procedure-entry", listOf(RegSrc("proc")))))
        stmts.add(Goto(GotoTarget.ByReg("target")))
        stmts.add(Label(interpreted))
        stmts.add(Assign("val", OpSrc("compiled-apply-fn", listOf(RegSrc("proc"), RegSrc("argl")))))
        stmts.add(Label(resumed))
        stmts.add(Restore("env"))
        stmts.add(Restore("target"))
        stmts.add(Assign("target", OpSrc("collection-step", listOf(RegSrc("target"), RegSrc("val")))))
        stmts.add(Goto(GotoTarget.Lbl(loop)))
        stmts.add(Label(done))
        stmts.add(Assign("val", OpSrc("collection-result", listOf(RegSrc("target")))))
        stmts.add(Restore("env"))
        stmts.add(Restore("continue"))
        if (nullCase != null && end != null) {
            stmts.add(Goto(GotoTarget.Lbl(end)))
            stmts.add(Label(nullCase))
            stmts.add(Restore("env"))
            stmts.add(Restore("continue"))
            stmts.add(Assign("val", OpSrc("compiled-null", emptyList())))
            stmts.add(Label(end))
        }
        val needs = receiver.needs + arguments.flatMap { it.needs }.toSet()
        return InstrSeq(needs, setOf("val", "argl", "proc", "target", "env", "continue"), stmts)
    }

    private fun methodCallStmts(
        expression: Call,
        callee: Member,
    ): InstrSeq {
        openSaves.add("continue")
        openSaves.add("env")
        val receiver = compileExpression(callee.receiver, Linkage.Next)
        openSaves.add("val")
        val argumentSeqs = compileArguments(expression.arguments)
        repeat(3) { openSaves.removeAt(openSaves.lastIndex) }
        val stmts = mutableListOf<Stmt>()
        stmts.add(Save("continue"))
        stmts.add(Save("env"))
        stmts.addAll(receiver.stmts)
        val nullLabel = if (callee.safe) freshLabel("safe-null") else null
        if (nullLabel != null) {
            stmts.add(Test(OpCond("compiled-is-null", listOf(RegSrc("val")))))
            stmts.add(Branch(nullLabel))
        }
        stmts.add(Save("val"))
        argumentSave(argumentSeqs, stmts)
        stmts.add(Restore("val"))
        stmts.add(Assign("argl", OpSrc("adjoin-arg", listOf(RegSrc("val"), RegSrc("argl")))))
        stmts.add(
            Assign(
                "proc",
                OpSrc(
                    "compiled-method-proc",
                    listOf(
                        RegSrc("argl"),
                        sicp.runtime.Source.ConstSrc(GValue.VString(callee.name)),
                        sicp.runtime.Source.ConstSrc(
                            GValue.VList(expression.arguments.map { GValue.VString(it.name ?: "") }.toMutableList(), false),
                        ),
                    ),
                ),
            ),
        )
        val returnLabel = freshLabel("return")
        val interpretedLabel = freshLabel("method-interpreted")
        stmts.add(Assign("continue", LabelSrc(returnLabel)))
        stmts.add(Test(OpCond("compiled-is-interpreted", listOf(RegSrc("proc")))))
        stmts.add(Branch(interpretedLabel))
        stmts.add(Assign("target", OpSrc("procedure-entry", listOf(RegSrc("proc")))))
        stmts.add(Goto(GotoTarget.ByReg("target")))
        stmts.add(Label(interpretedLabel))
        stmts.add(Assign("val", OpSrc("compiled-apply-fn", listOf(RegSrc("proc"), RegSrc("argl")))))
        stmts.add(Label(returnLabel))
        stmts.add(Restore("env"))
        stmts.add(Restore("continue"))
        if (nullLabel != null) {
            val afterLabel = freshLabel("safe-after")
            stmts.add(Goto(GotoTarget.Lbl(afterLabel)))
            stmts.add(Label(nullLabel))
            stmts.add(Restore("env"))
            stmts.add(Restore("continue"))
            stmts.add(Assign("val", OpSrc("compiled-null", emptyList())))
            stmts.add(Label(afterLabel))
        }
        val needs = receiver.needs + argumentSeqs.flatMap { it.needs }.toSet()
        return InstrSeq(needs, setOf("val", "argl", "proc", "continue", "target", "env"), stmts)
    }

    private fun constructStmts(
        expression: Call,
        name: String,
    ): InstrSeq {
        val properties = classShapes[name] ?: emptyList()
        val callArguments = expression.arguments
        val argumentSeqs = compileArguments(callArguments)
        val stmts = mutableListOf<Stmt>()
        argumentSave(argumentSeqs, stmts)
        stmts.add(
            Assign(
                "val",
                OpSrc(
                    "compiled-construct",
                    listOf(
                        sicp.runtime.Source.ConstSrc(GValue.VString(name)),
                        sicp.runtime.Source.ConstSrc(GValue.VList(properties.map { GValue.VString(it) }.toMutableList(), false)),
                        RegSrc("argl"),
                        sicp.runtime.Source.ConstSrc(
                            GValue.VList(callArguments.map { GValue.VString(it.name ?: "") }.toMutableList(), false),
                        ),
                    ),
                ),
            ),
        )
        val needs = argumentSeqs.flatMap { it.needs }.toSet()
        return InstrSeq(needs, setOf("val", "argl"), stmts)
    }

    private fun primitiveCallStmts(
        expression: Call,
        name: String,
    ): InstrSeq {
        val argumentSeqs = compileArguments(expression.arguments)
        val stmts = mutableListOf<Stmt>()
        argumentSave(argumentSeqs, stmts)
        stmts.add(Assign("val", OpSrc("compiled-primitive", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name)), RegSrc("argl")))))
        val needs = argumentSeqs.flatMap { it.needs }.toSet()
        return InstrSeq(needs, setOf("val", "argl"), stmts)
    }

    private fun primitiveApplyStmts(
        name: String,
        arity: Int,
    ): List<Stmt> {
        if (arity !=
            2
        ) {
            return listOf(
                Assign("val", OpSrc("compiled-primitive", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name)), RegSrc("argl")))),
            )
        }
        return listOf(
            Assign("argl", OpSrc("compiled-pair-args", listOf(RegSrc("val"), RegSrc("argl")))),
            Assign("val", OpSrc("compiled-primitive", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name)), RegSrc("argl")))),
        )
    }

    private fun compileWhen(expression: When): InstrSeq {
        val stmts = mutableListOf<Stmt>()
        val endLabel = freshLabel("when-end")
        var needs = emptySet<String>()
        val hasSubject = expression.subject != null
        val subjectExpression = expression.subject
        if (subjectExpression != null) {
            val subject = compileExpression(subjectExpression, Linkage.Next)
            needs = needs + subject.needs
            stmts.addAll(subject.stmts)
            stmts.add(Assign("target", RegSrc("val")))
            stmts.add(Save("target"))
            openSaves.add("target")
        }
        for (branch in expression.branches) {
            val caseLabel = freshLabel("when-case")
            val nextLabel = freshLabel("when-next")
            val typePattern = branch.typePattern
            val branchPattern = branch.pattern
            if (typePattern != null) {
                stmts.add(Assign("target", OpSrc("stack-peek", emptyList())))
                stmts.add(
                    Test(
                        OpCond(
                            "compiled-is",
                            listOf(sicp.runtime.Source.ConstSrc(GValue.VString(guestTypeName(typePattern))), RegSrc("target")),
                        ),
                    ),
                )
                stmts.add(Branch(caseLabel))
            } else if (branchPattern != null && subjectExpression != null) {
                val pattern = compileExpression(branchPattern, Linkage.Next)
                needs = needs + pattern.needs
                stmts.addAll(pattern.stmts)
                stmts.add(Assign("target", OpSrc("stack-peek", emptyList())))
                stmts.add(Test(OpCond("compiled-equal", listOf(RegSrc("target"), RegSrc("val")))))
                stmts.add(Branch(caseLabel))
            } else if (branchPattern != null) {
                val pattern = compileExpression(branchPattern, Linkage.Next)
                needs = needs + pattern.needs
                stmts.addAll(pattern.stmts)
                stmts.add(Test(OpCond("is-true", listOf(RegSrc("val")))))
                stmts.add(Branch(caseLabel))
            }
            stmts.add(Goto(GotoTarget.Lbl(nextLabel)))
            stmts.add(Label(caseLabel))
            val body = compileExpression(branch.body, Linkage.Next)
            needs = needs + body.needs
            stmts.addAll(body.stmts)
            stmts.add(Goto(GotoTarget.Lbl(endLabel)))
            stmts.add(Label(nextLabel))
        }
        val otherwise = expression.otherwise
        if (otherwise != null) {
            val fallback = compileExpression(otherwise, Linkage.Next)
            needs = needs + fallback.needs
            stmts.addAll(fallback.stmts)
        } else {
            stmts.add(Assign("val", OpSrc("compiled-unit", emptyList())))
        }
        stmts.add(Label(endLabel))
        if (hasSubject) {
            stmts.add(Restore("target"))
            openSaves.removeAt(openSaves.lastIndex)
        }
        return InstrSeq(needs, setOf("val", "target"), stmts)
    }

    private fun compileMember(expression: Member): InstrSeq {
        val receiver = compileExpression(expression.receiver, Linkage.Next)
        val read =
            Assign("val", OpSrc("compiled-property", listOf(RegSrc("val"), sicp.runtime.Source.ConstSrc(GValue.VString(expression.name)))))
        if (!expression.safe) {
            return InstrSeq(receiver.needs, setOf("val"), receiver.stmts + read)
        }
        val nullLabel = freshLabel("safe-null")
        val endLabel = freshLabel("safe-end")
        val stmts =
            receiver.stmts +
                Test(OpCond("compiled-is-null", listOf(RegSrc("val")))) +
                Branch(nullLabel) +
                read +
                Goto(GotoTarget.Lbl(endLabel)) +
                Label(nullLabel) +
                Assign("val", OpSrc("compiled-null", emptyList())) +
                Label(endLabel)
        return InstrSeq(receiver.needs, setOf("val"), stmts)
    }

    private fun compileIndex(expression: Index): InstrSeq {
        val receiver = compileExpression(expression.receiver, Linkage.Next)
        openSaves.add("val")
        val index = compileExpression(expression.index, Linkage.Next)
        openSaves.removeAt(openSaves.lastIndex)
        val stmts =
            receiver.stmts +
                Save("val") +
                index.stmts +
                Assign("argl", RegSrc("val")) +
                Restore("val") +
                Assign("val", OpSrc("compiled-index", listOf(RegSrc("val"), RegSrc("argl"))))
        return InstrSeq(receiver.needs + index.needs, setOf("val", "argl"), stmts)
    }

    private fun compileElvis(expression: Elvis): InstrSeq {
        val left = compileExpression(expression.left, Linkage.Next)
        val right = compileExpression(expression.right, Linkage.Next)
        val rightLabel = freshLabel("elvis-right")
        val endLabel = freshLabel("elvis-end")
        val stmts =
            left.stmts +
                Test(OpCond("compiled-is-null", listOf(RegSrc("val")))) +
                Branch(rightLabel) +
                Goto(GotoTarget.Lbl(endLabel)) +
                Label(rightLabel) +
                right.stmts +
                Label(endLabel)
        return InstrSeq(left.needs + right.needs, setOf("val"), stmts)
    }

    private fun isStmts(expression: Is): List<Stmt> {
        val operand = compileExpression(expression.value, Linkage.Next)
        val test =
            Assign(
                "val",
                OpSrc("compiled-is", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(guestTypeName(expression.type))), RegSrc("val"))),
            )
        if (!expression.negated) return operand.stmts + test
        return operand.stmts + test +
            Assign("val", OpSrc("compiled-unary", listOf(sicp.runtime.Source.ConstSrc(GValue.VString("!")), RegSrc("val"))))
    }

    private fun guestTypeName(type: GuestType): String =
        when (type) {
            is GuestType.Named -> type.name
            is GuestType.Nullable -> guestTypeName(type.base) + "?"
            is GuestType.Function -> "Function"
            GuestType.Nothing -> "Nothing"
            GuestType.Null -> "Null"
        }

    private fun compileWhile(statement: While): InstrSeq {
        val continueLabel = freshLabel("while")
        val bodyLabel = freshLabel("while-body")
        val endLabel = freshLabel("while-end")
        loopContinues.addLast(continueLabel)
        loopEnds.addLast(endLabel)
        loopSaveDepths.addLast(openSaves.size)
        val condition = compileExpression(statement.condition, Linkage.Next)
        val body = compileBlock(statement.body)
        loopContinues.removeLast()
        loopEnds.removeLast()
        loopSaveDepths.removeLast()
        val stmts =
            listOf(Label(continueLabel)) +
                condition.stmts +
                Test(OpCond("is-true", listOf(RegSrc("val")))) +
                Branch(bodyLabel) +
                Goto(GotoTarget.Lbl(endLabel)) +
                Label(bodyLabel) +
                body.stmts +
                Goto(GotoTarget.Lbl(continueLabel)) +
                Label(endLabel) +
                Assign("val", OpSrc("compiled-unit", emptyList()))
        return InstrSeq(condition.needs + body.needs, setOf("val"), stmts)
    }

    /** A `for` over a list or a range. A range keeps only its running index
     * and its limit, so `break` never pays for the rest of it and a limit of
     * `Long.MAX_VALUE` ends by comparison instead of wrapping; `continue`
     * jumps to the advance step, not past it. */
    private fun compileFor(statement: For): InstrSeq {
        val itemsName = "\$forItems${labelCount + 1}"
        val indexName = "\$forIndex${labelCount + 1}"
        val limitName = "\$forLimit${labelCount + 1}"
        val testLabel = freshLabel("for")
        val bodyLabel = freshLabel("for-body")
        val nextLabel = freshLabel("for-next")
        val endLabel = freshLabel("for-end")
        val stmts = mutableListOf<Stmt>()
        var needs = emptySet<String>()
        val endExpression = statement.end

        fun declare(
            name: String,
            from: String,
        ) = Assign("val", OpSrc("declare-local", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name)), RegSrc(from), RegSrc("env"))))

        fun binary(operator: String) =
            Assign(
                "val",
                OpSrc("compiled-binary", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(operator)), RegSrc("val"), RegSrc("argl"))),
            )

        fun compare(operator: String): List<Stmt> =
            lookupStmts(Name(limitName, statement.span)) + Assign("argl", RegSrc("val")) +
                lookupStmts(Name(indexName, statement.span)) + binary(operator)

        val step: GValue
        if (endExpression != null) {
            val start = compileExpression(statement.iterable, Linkage.Next)
            openSaves.add("val")
            val end = compileExpression(endExpression, Linkage.Next)
            openSaves.removeAt(openSaves.lastIndex)
            needs = needs + start.needs + end.needs
            stmts.addAll(start.stmts)
            stmts.add(Save("val"))
            stmts.addAll(end.stmts)
            stmts.add(Assign("argl", RegSrc("val")))
            stmts.add(Restore("val"))
            stmts.add(declare(indexName, "val"))
            stmts.add(declare(limitName, "argl"))
            frames[frames.size - 1].add(indexName)
            frames[frames.size - 1].add(limitName)
            val isLong = (checked.types[statement.iterable] as? GuestType.Named)?.name == "Long"
            step = if (isLong) GValue.VLong(1L) else GValue.VInt(1)
        } else {
            val items = compileExpression(statement.iterable, Linkage.Next)
            needs = needs + items.needs
            stmts.addAll(items.stmts)
            stmts.add(declare(itemsName, "val"))
            stmts.add(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(GValue.VInt(0))))))
            stmts.add(declare(indexName, "val"))
            frames[frames.size - 1].add(itemsName)
            frames[frames.size - 1].add(indexName)
            step = GValue.VInt(1)
        }
        loopContinues.addLast(nextLabel)
        loopEnds.addLast(endLabel)
        loopSaveDepths.addLast(openSaves.size)
        val bodyStatements = mutableListOf<Stmt>()
        if (endExpression != null) {
            bodyStatements.addAll(lookupStmts(Name(indexName, statement.span)))
        } else {
            bodyStatements.addAll(lookupStmts(Name(itemsName, statement.span)))
            bodyStatements.add(Save("val"))
            bodyStatements.addAll(lookupStmts(Name(indexName, statement.span)))
            bodyStatements.add(Assign("argl", RegSrc("val")))
            bodyStatements.add(Restore("val"))
            bodyStatements.add(Assign("val", OpSrc("compiled-index", listOf(RegSrc("val"), RegSrc("argl")))))
        }
        bodyStatements.add(Save("env"))
        bodyStatements.add(Assign("env", OpSrc("child-env", listOf(RegSrc("env")))))
        frames.add(mutableListOf(statement.name))
        bodyStatements.add(declare(statement.name, "val"))
        openSaves.add("env")
        bodyStatements.addAll(compileBlock(statement.body).stmts)
        openSaves.removeAt(openSaves.lastIndex)
        frames.removeAt(frames.size - 1)
        bodyStatements.add(Restore("env"))
        val advance = mutableListOf<Stmt>()
        if (endExpression != null) {
            advance.addAll(compare("=="))
            advance.add(Test(OpCond("is-true", listOf(RegSrc("val")))))
            advance.add(Branch(endLabel))
        }
        advance.addAll(lookupStmts(Name(indexName, statement.span)))
        advance.add(Assign("argl", RegSrc("val")))
        advance.add(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(step)))))
        advance.add(binary("+"))
        advance.add(
            Assign(
                "val",
                OpSrc("compiled-assign", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(indexName)), RegSrc("val"), RegSrc("env"))),
            ),
        )
        val test =
            if (endExpression != null) {
                compare("<=")
            } else {
                lookupStmts(Name(itemsName, statement.span)) +
                    Assign("val", OpSrc("compiled-property", listOf(RegSrc("val"), sicp.runtime.Source.ConstSrc(GValue.VString("size"))))) +
                    Assign("argl", RegSrc("val")) +
                    lookupStmts(Name(indexName, statement.span)) + binary("<")
            }
        loopContinues.removeLast()
        loopEnds.removeLast()
        loopSaveDepths.removeLast()
        stmts.add(Label(testLabel))
        stmts.addAll(test)
        stmts.add(Test(OpCond("is-true", listOf(RegSrc("val")))))
        stmts.add(Branch(bodyLabel))
        stmts.add(Goto(GotoTarget.Lbl(endLabel)))
        stmts.add(Label(bodyLabel))
        stmts.addAll(bodyStatements)
        stmts.add(Label(nextLabel))
        stmts.addAll(advance)
        stmts.add(Goto(GotoTarget.Lbl(testLabel)))
        stmts.add(Label(endLabel))
        stmts.add(Assign("val", OpSrc("compiled-unit", emptyList())))
        return InstrSeq(needs, setOf("val", "argl", "target"), stmts)
    }

    private fun compileDestructure(statement: Destructure): InstrSeq {
        val initializer = compileExpression(statement.initializer, Linkage.Next)
        val stmts = mutableListOf<Stmt>()
        stmts.addAll(initializer.stmts)
        stmts.add(
            Assign(
                "argl",
                OpSrc("compiled-destructure", listOf(RegSrc("val"), sicp.runtime.Source.ConstSrc(GValue.VInt(statement.names.size)))),
            ),
        )
        for ((index, name) in statement.names.withIndex()) {
            stmts.add(Assign("val", OpSrc("compiled-const", listOf(sicp.runtime.Source.ConstSrc(GValue.VInt(index))))))
            stmts.add(Assign("val", OpSrc("compiled-index", listOf(RegSrc("argl"), RegSrc("val")))))
            stmts.add(
                Assign(
                    "val",
                    OpSrc("declare-local", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name)), RegSrc("val"), RegSrc("env"))),
                ),
            )
            frames[frames.size - 1].add(name)
        }
        return InstrSeq(initializer.needs, setOf("val", "argl"), stmts)
    }

    private fun compileLocalFunction(statement: FunctionDecl): InstrSeq {
        val closure = lambdaLikeStmts(statement.name, statement.parameters.map { it.name }, statement.body)
        val bind =
            Assign(
                "val",
                OpSrc("declare-local", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(statement.name)), RegSrc("val"), RegSrc("env"))),
            )
        return InstrSeq(emptySet(), setOf("val"), closure + bind)
    }

    private fun breakStmts(span: Span): List<Stmt> = unwindLoop() + Goto(GotoTarget.Lbl(loopEnds.lastOrNull() ?: loopErrorLabel))

    private fun continueStmts(span: Span): List<Stmt> = unwindLoop() + Goto(GotoTarget.Lbl(loopContinues.lastOrNull() ?: loopErrorLabel))

    private fun unwindLoop(): List<Stmt> =
        openSaves.subList(loopSaveDepths.lastOrNull() ?: openSaves.size, openSaves.size).asReversed().map(::Restore)

    private fun compileBlock(block: Block): InstrSeq =
        if (block in checked.lambdaCoercions) {
            simple(lambdaLikeStmts(null, emptyList(), block), setOf("val"))
        } else {
            compileBlockBody(block)
        }

    private fun compileBlockBody(block: Block): InstrSeq {
        val ordered =
            if (options.scanOutDefines) {
                block.statements.filterIsInstance<FunctionDecl>() + block.statements.filter { it !is FunctionDecl }
            } else {
                block.statements
            }
        val instructions = mutableListOf<Stmt>()
        instructions.add(Save("env"))
        instructions.add(Assign("env", OpSrc("child-env", listOf(RegSrc("env")))))
        frames.add(mutableListOf())
        openSaves.add("env")
        var needs = emptySet<String>()
        for (statement in ordered) {
            if (statement is LocalProperty || statement is FunctionDecl) frames[frames.size - 1].add(declaredName(statement))
            val sequence = compileStatement(statement)
            needs = needs + sequence.needs
            instructions.addAll(sequence.stmts)
        }
        openSaves.removeAt(openSaves.lastIndex)
        frames.removeAt(frames.size - 1)
        instructions.add(Restore("env"))
        return InstrSeq(needs, setOf("val", "env"), instructions)
    }

    private fun declaredName(statement: Statement): String =
        when (statement) {
            is LocalProperty -> statement.name
            is FunctionDecl -> statement.name
            else -> ""
        }

    private fun compileLocal(statement: LocalProperty): InstrSeq {
        val initializer = compileExpression(statement.initializer, Linkage.Next)
        val define =
            Assign(
                "val",
                OpSrc("declare-local", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(statement.name)), RegSrc("val"), RegSrc("env"))),
            )
        return InstrSeq(initializer.needs, setOf("val"), initializer.stmts + define)
    }

    private fun compileAssignment(statement: Assignment): InstrSeq {
        val target = statement.target
        if (target is Member) return compileFieldAssignment(statement, target)
        if (target is Index) return compileIndexAssignment(statement, target)
        val name = target as Name
        val value = compileExpression(statement.value, Linkage.Next)
        val assign =
            Assign(
                "val",
                OpSrc("compiled-assign", listOf(sicp.runtime.Source.ConstSrc(GValue.VString(name.text)), RegSrc("val"), RegSrc("env"))),
            )
        return InstrSeq(value.needs, setOf("val"), value.stmts + assign)
    }

    private fun compileFieldAssignment(
        statement: Assignment,
        target: Member,
    ): InstrSeq {
        val receiver = compileExpression(target.receiver, Linkage.Next)
        openSaves.add("val")
        val value = compileExpression(statement.value, Linkage.Next)
        openSaves.removeAt(openSaves.lastIndex)
        val stmts =
            receiver.stmts +
                Save("val") +
                value.stmts +
                Assign("argl", RegSrc("val")) +
                Restore("val") +
                Assign(
                    "val",
                    OpSrc(
                        "compiled-write-property",
                        listOf(RegSrc("val"), sicp.runtime.Source.ConstSrc(GValue.VString(target.name)), RegSrc("argl")),
                    ),
                )
        return InstrSeq(receiver.needs + value.needs, setOf("val", "argl"), stmts)
    }

    private fun compileIndexAssignment(
        statement: Assignment,
        target: Index,
    ): InstrSeq {
        val receiver = compileExpression(target.receiver, Linkage.Next)
        openSaves.add("val")
        val index = compileExpression(target.index, Linkage.Next)
        openSaves.add("val")
        val value = compileExpression(statement.value, Linkage.Next)
        repeat(2) { openSaves.removeAt(openSaves.lastIndex) }
        val stmts =
            receiver.stmts +
                Save("val") +
                index.stmts +
                Save("val") +
                value.stmts +
                Assign("argl", RegSrc("val")) +
                Restore("val") +
                Assign("target", RegSrc("val")) +
                Restore("val") +
                Assign("val", OpSrc("compiled-write-index", listOf(RegSrc("val"), RegSrc("target"), RegSrc("argl"))))
        return InstrSeq(receiver.needs + index.needs + value.needs, setOf("val", "argl", "target"), stmts)
    }

    private fun compileReturn(statement: Return): InstrSeq {
        val call = statement.value as? Call
        val callee = call?.callee
        if (call != null && callee !is Member && (callee !is Name || resolvableName(callee.text)) &&
            (callee !is Name || callee.text !in classShapes)
        ) {
            return compileCall(call, Linkage.ReturnTo(""), openSaves.toList())
        }
        val value =
            statement.value?.let { compileExpression(it, Linkage.Next) }
                ?: simple(listOf(Assign("val", OpSrc("compiled-unit", emptyList()))), setOf("val"))
        val unwind = openSaves.asReversed().map(::Restore)
        return InstrSeq(value.needs, setOf("val"), value.stmts + unwind + Goto(GotoTarget.ByReg("continue")))
    }
}
