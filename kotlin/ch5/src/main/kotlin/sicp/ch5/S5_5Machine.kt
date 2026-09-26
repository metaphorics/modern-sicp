// SPDX-License-Identifier: GPL-3.0-only
// Section 5.5.7: interfacing compiled code to the evaluator. The 5.4
// machine's apply-dispatch learns the compiled-procedure case, the
// driver grows the flag-guarded external entry the book's
// `compile-and-go` arms, and the compiled-procedure word of footnote 323
// is the runtime's [VCompiledProc]: an entry label resolved against the
// assembled machine's label table, over an environment word's table
// entry. Compiled code runs beside interpreted code on one machine,
// one global environment, and one operations table: the compiler's
// `(const name)` variable inputs are symbols to the same environment
// operations the interpreted path uses, and `(label entryN)` operation
// inputs are label addresses, the base assembler's own values.
//
// The machine counts executed instructions (the seam exercises 5.15 to
// 5.19 establish), so the measuring exercises read steps and stack
// statistics from the same run.

package sicp.ch5

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch4.readProgram
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Env
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.SchemeError
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VCompiledProc
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.noParams

// ---------------------------------------------------------------------------
// The controller fragments
// ---------------------------------------------------------------------------

/** The apply-dispatch of 5.5.7: the compiled-procedure test joins the
 *  book's dispatch, before the unknown-type stop, and `compiled-apply`
 *  restores `continue` and jumps to the compiled code's entry. */
public val compiledApplyDispatch: List<Stmt> =
    listOf(
        Label("apply-dispatch"),
        Test(sicp.runtime.OpCond("primitive-procedure?", listOf(reg("proc")))),
        Branch("primitive-apply"),
        Test(sicp.runtime.OpCond("compound-procedure?", listOf(reg("proc")))),
        Branch("compound-apply"),
        Test(sicp.runtime.OpCond("compiled-procedure?", listOf(reg("proc")))),
        Branch("compiled-apply"),
        Goto(GotoTarget.Lbl("unknown-procedure-type")),
        Label("compiled-apply"),
        sicp.runtime.Restore("continue"),
        Assign("val", opSrc("compiled-procedure-entry", reg("proc"))),
        Goto(GotoTarget.ByReg("val")),
    )

/** The external entry: reached when the machine starts with `flag` set,
 *  it points `continue` at `print-result` and jumps to the compiled code
 *  in `val`. */
public val externalEntry: List<Stmt> =
    listOf(
        Label("external-entry"),
        sicp.runtime.Perform(sicp.runtime.OpAct("initialize-stack", emptyList())),
        Assign("env", opSrc("get-global-environment")),
        Assign("continue", labelSrc("print-result")),
        Goto(GotoTarget.ByReg("val")),
    )

/** The flag-guarded driver: the book's `;; branches if flag is set`
 *  branch before the plain driver fragment, so `compile-and-go` starts
 *  the machine in the compiled code and later passes never re-enter the
 *  external entry. */
public val driverWithExternalEntry: List<Stmt> =
    listOf(Branch("external-entry")) + evaluatorFragment("driver")

/** The 5.5.7 evaluator's controller: the 5.4 fragments in printed
 *  order, with the flag-guarded driver and the compiled apply-dispatch
 *  in place of their 5.4 counterparts, and the external entry appended.
 *  A variant machine concatenates a different fragment order (5.45's
 *  monitored driver replaces the plain one here, as in 5.4). */
public val ecevalController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "driver" -> driverWithExternalEntry
            "apply-dispatch" -> compiledApplyDispatch
            else -> stmts
        }
    } + externalEntry

/** The monitored 5.5.7 controller: the fragments with the 5.4.4
 *  monitored driver (flag-guarded) and the compiled apply-dispatch; the
 *  stack-measuring exercises (5.45, 5.46) and the book's monitored
 *  5.5.7 session run on it. */
public val monitoredEcevalController: List<Stmt> =
    evaluatorControllerFragments.flatMap { (name, stmts) ->
        when (name) {
            "driver" -> listOf(Branch("external-entry")) + monitoredDriver
            "apply-dispatch" -> compiledApplyDispatch
            else -> stmts
        }
    } + externalEntry

// ---------------------------------------------------------------------------
// The compiled operations
// ---------------------------------------------------------------------------

/** The label address of the entry name [name] a compiled procedure word
 *  carries; the machine is absent only while it is under construction. */
context(r: Raise<MachineError>)
private fun entryAddress(
    state: EvaluatorState,
    op: String,
    name: String,
): Int {
    val machine = requireNotNull(state.machine) { "$op: the machine is not installed" }
    return machine.labels[name]
        ?: r.raise(EvaluatorFault("$op: no label $name in the assembled machine"))
}

context(r: Raise<MachineError>)
private fun compiledWordOf(
    op: String,
    w: Value,
): VCompiledProc =
    when (w) {
        is VCompiledProc -> w
        else -> r.raise(EvaluatorFault("$op needs a compiled procedure"))
    }

/** The frame number and displacement the compiler's one list constant carries. */
context(r: Raise<MachineError>)
private fun lexicalAddressOf(
    op: String,
    address: Value,
): Pair<Int, Int> {
    val items = r.listItems(op, address)
    if (items.size != 2) r.raise(EvaluatorFault("$op needs a two-element address"))
    val frame = items[0] as? VInt ?: r.raise(EvaluatorFault("$op needs integer indexes"))
    val displacement = items[1] as? VInt ?: r.raise(EvaluatorFault("$op needs integer indexes"))
    return frame.n.toInt() to displacement.n.toInt()
}

/** The [frameNumber]-th frame out from [env], resolved through the table. */
context(r: Raise<MachineError>)
private fun lexicalFrameOf(
    state: EvaluatorState,
    op: String,
    env: Value,
    frameNumber: Int,
): Env {
    if (frameNumber < 0) r.raise(EvaluatorFault("$op: lexical frame $frameNumber is out of range"))
    var frame: Env? = state.envOf(op, env)
    repeat(frameNumber) { frame = frame?.parent }
    return frame ?: r.raise(EvaluatorFault("$op: lexical frame $frameNumber is out of range"))
}

/** The operations the compiled code names and the interpreted path never
 *  does: the footnote-323 compiled-procedure family, the book's
 *  `false?`, the argument-list builder `cons`, and `make-compiled-
 *  procedure`'s `(label entryN)` input, a label address the base
 *  assembler delivers as an integer. Exercise 5.42's lexical addressing
 *  reads and writes frame slots by number: the frame is the runtime
 *  environment's own chain, and the displacement indexes the frame's
 *  bindings in their insertion order, the order `extend-environment`
 *  bound the parameters in. */
internal fun compiledOperations(
    state: EvaluatorState,
    runtime: Map<String, ObjectPrimitive>,
): Map<String, Op> =
    mapOf(
        "make-compiled-procedure" to
            compiledTwoArgs("make-compiled-procedure") { entry, env ->
                val address =
                    when (entry) {
                        is VInt -> entry.n.toInt()
                        else -> raise(EvaluatorFault("make-compiled-procedure needs an entry label"))
                    }
                val machine = requireNotNull(state.machine) { "make-compiled-procedure: the machine is not installed" }
                val name =
                    machine.labels.entries
                        .firstOrNull { it.value == address }
                        ?.key
                        ?: raise(EvaluatorFault("make-compiled-procedure: no label at address $address"))
                VCompiledProc(name, noParams, state.envOf("make-compiled-procedure", env))
            },
        "compiled-procedure?" to
            compiledOneArg("compiled-procedure?") { w ->
                VBool(w is VCompiledProc)
            },
        "compiled-procedure-entry" to
            compiledOneArg("compiled-procedure-entry") { w ->
                val word = compiledWordOf("compiled-procedure-entry", w)
                VInt(entryAddress(state, "compiled-procedure-entry", word.entry).toLong())
            },
        "compiled-procedure-env" to
            compiledOneArg("compiled-procedure-env") { w ->
                val word = compiledWordOf("compiled-procedure-env", w)
                state.envs.add(word.env)
                envWord(state.envs.size - 1)
            },
        "false?" to
            compiledOneArg("false?") { w ->
                VBool(w == VBool(false))
            },
        "cons" to
            compiledTwoArgs("cons") { value, argl ->
                sicp.runtime.cons(value, argl)
            },
        "list" to
            compiledOneArg("list") { w ->
                sicp.runtime.vlist(listOf(w))
            },
        "apply-primitive-procedure" to
            compiledTwoArgs("apply-primitive-procedure") { proc, argl ->
                if (!isPrimitiveProcedure(proc)) {
                    raise(EvaluatorFault("apply-primitive-procedure needs a primitive procedure"))
                }
                val name = primitiveName(proc)
                val values = listItems("apply-primitive-procedure", argl)
                val support = runtime[name]
                if (support != null) {
                    support(this, values)
                } else {
                    when (val applied = applyObjectPrimitive(name, values)) {
                        is Either.Left -> raise(applied.value)
                        is Either.Right -> applied.value
                    }
                }
            },
        "lexical-address-lookup" to
            compiledTwoArgs("lexical-address-lookup") { address, env ->
                val (frameNumber, displacement) = lexicalAddressOf("lexical-address-lookup", address)
                val frame = lexicalFrameOf(state, "lexical-address-lookup", env, frameNumber)
                val entry =
                    frame.frame.entries.elementAtOrNull(displacement)
                        ?: raise(EvaluatorFault("lexical-address-lookup: address ($frameNumber $displacement) is out of range"))
                if (entry.value == VSym("*unassigned*")) {
                    raise(EvaluatorFault("lexical-address-lookup: variable ${entry.key} is unassigned"))
                }
                entry.value
            },
        "lexical-address-set!" to
            compiledThreeArgs("lexical-address-set!") { address, value, env ->
                val (frameNumber, displacement) = lexicalAddressOf("lexical-address-set!", address)
                val frame = lexicalFrameOf(state, "lexical-address-set!", env, frameNumber)
                val name =
                    frame.frame.keys.elementAtOrNull(displacement)
                        ?: raise(EvaluatorFault("lexical-address-set!: address ($frameNumber $displacement) is out of range"))
                frame.define(name, value)
                VSym("ok")
            },
    )

/** The runtime-support primitives the compiled metacircular evaluator of
 *  5.50 calls at object level: the environment operations behind
 *  environment words (the book's list frames the machine provides
 *  instead), the list accessors the syntax procedures brigade with, the
 *  arithmetic and comparison entries the primitive table names, the
 *  `display`/`newline` sinks, the `apply-in-underlying-scheme` escape,
 *  and `error`. The symbol `the-empty` names a fresh frame, the base of
 *  the global environment the metacircular's `setup-environment`
 *  extends. */
internal fun runtimeSupportPrimitives(state: EvaluatorState): Map<String, ObjectPrimitive> {
    fun Raise<EvaluatorFault>.supportEnv(word: Value): Env =
        when {
            isEnvWord(word) -> {
                state.envOf("extend-environment", word)
            }

            word is VSym && word.name == "the-empty" -> {
                val fresh = Env.global()
                state.envs.add(fresh)
                fresh
            }

            else -> {
                raise(EvaluatorFault("extend-environment needs an environment"))
            }
        }

    fun Raise<EvaluatorFault>.items(v: Value): List<Value> {
        val out = ArrayList<Value>()
        var cursor = v
        while (cursor is VPair) {
            out.add(cursor.car)
            cursor = cursor.cdr
        }
        if (cursor !is VNil) raise(EvaluatorFault("the environment operations need proper lists"))
        return out
    }

    fun Raise<EvaluatorFault>.nameOf(v: Value): String =
        when (v) {
            is VSym -> v.name
            else -> raise(EvaluatorFault("the environment operations need variable names"))
        }

    fun Raise<EvaluatorFault>.carOf(v: Value): Value = (v as? VPair)?.car ?: raise(EvaluatorFault("car needs a pair"))

    fun Raise<EvaluatorFault>.cdrOf(v: Value): Value = (v as? VPair)?.cdr ?: raise(EvaluatorFault("cdr needs a pair"))

    fun Raise<EvaluatorFault>.numOf(v: Value): Long = (v as? VInt)?.n ?: raise(EvaluatorFault("an integer is needed"))

    fun Raise<EvaluatorFault>.extended(
        names: List<String>,
        values: List<Value>,
        base: Env,
    ): Value {
        if (names.size != values.size) {
            raise(EvaluatorFault("arity mismatch: expected ${names.size}, given ${values.size}"))
        }
        val frame = either<SchemeError, Env> { Env.extend(names, values, base) }
        return when (frame) {
            is Either.Left -> {
                raise(EvaluatorFault("extend-environment failed"))
            }

            is Either.Right -> {
                state.envs.add(frame.value)
                envWord(state.envs.size - 1)
            }
        }
    }

    return mapOf(
        "extend-environment" to
            { args ->
                val names = items(args[0]).map { nameOf(it) }
                val values = items(args[1])
                extended(names, values, supportEnv(args[2]))
            },
        "lookup-variable-value" to
            { args ->
                val name = nameOf(args[0])
                val holder = supportEnv(args[1])
                val hit = either<SchemeError, Value> { holder.lookup(name) }
                when (hit) {
                    is Either.Right -> hit.value
                    is Either.Left -> raise(EvaluatorFault("unbound variable: $name"))
                }
            },
        "set-variable-value!" to
            { args ->
                val name = nameOf(args[0])
                val holder = supportEnv(args[2])
                val done = either<SchemeError, Unit> { holder.set(name, args[1]) }
                when (done) {
                    is Either.Right -> VSym("ok")
                    is Either.Left -> raise(EvaluatorFault("unbound variable: $name"))
                }
            },
        "define-variable!" to
            { args ->
                supportEnv(args[2]).define(nameOf(args[0]), args[1])
                VSym("ok")
            },
        "apply-in-underlying-scheme" to
            { args ->
                if (!isPrimitiveProcedure(args[0])) {
                    raise(EvaluatorFault("apply-in-underlying-scheme needs a primitive"))
                }
                val values = items(args[1])
                when (val applied = applyObjectPrimitive(primitiveName(args[0]), values)) {
                    is Either.Left -> raise(applied.value)
                    is Either.Right -> applied.value
                }
            },
        "error" to
            { args ->
                raise(EvaluatorFault(args.joinToString(" ") { renderWord(it) }))
            },
        "cadr" to { args -> carOf(cdrOf(args[0])) },
        "caddr" to { args -> carOf(cdrOf(cdrOf(args[0]))) },
        "cadddr" to { args -> carOf(cdrOf(cdrOf(cdrOf(args[0])))) },
        "caadr" to { args -> carOf(carOf(cdrOf(args[0]))) },
        "cdadr" to { args -> cdrOf(carOf(cdrOf(args[0]))) },
        "cddr" to { args -> cdrOf(cdrOf(args[0])) },
        "cdddr" to { args -> cdrOf(cdrOf(cdrOf(args[0]))) },
        "quotient" to
            { args ->
                val divisor = numOf(args[1])
                if (divisor == 0L) raise(EvaluatorFault("division by zero"))
                VInt(numOf(args[0]) / divisor)
            },
        "abs" to
            { args ->
                VInt(kotlin.math.abs(numOf(args[0])))
            },
        "<=" to { args -> VBool(numOf(args[0]) <= numOf(args[1])) },
        ">=" to { args -> VBool(numOf(args[0]) >= numOf(args[1])) },
        "display" to { args -> args[0] },
        "newline" to { _ -> VSym("newline") },
    )
}

private fun compiledOneArg(
    name: String,
    f: Raise<MachineError>.(Value) -> Value,
): Op =
    { args ->
        if (args.size != 1) raise(EvaluatorFault("$name needs one argument"))
        f(args[0])
    }

private fun compiledTwoArgs(
    name: String,
    f: Raise<MachineError>.(Value, Value) -> Value,
): Op =
    { args ->
        if (args.size != 2) raise(EvaluatorFault("$name needs two arguments"))
        f(args[0], args[1])
    }

private fun compiledThreeArgs(
    name: String,
    f: Raise<MachineError>.(Value, Value, Value) -> Value,
): Op =
    { args ->
        if (args.size != 3) raise(EvaluatorFault("$name needs three arguments"))
        f(args[0], args[1], args[2])
    }

// ---------------------------------------------------------------------------
// The machine
// ---------------------------------------------------------------------------

/** The step-counting machine: every executed instruction advances the
 *  counter, transfers included, the seam 5.15's monitor established. */
public class StepCountingMachine(
    registerNames: List<String>,
    userOperations: Map<String, Op>,
) : Machine(registerNames, userOperations) {
    /** Executed instructions since the machine was built. */
    public var steps: Long = 0

    context(r: Raise<MachineError>)
    override fun execute() {
        while (pc < insts.size) {
            steps += 1
            insts[pc].exec(r)
        }
    }
}

/** The 5.5.7 evaluator: the assembled machine over the [StepCounting-
 *  Machine] register set (the evaluator's seven registers plus 5.38's
 *  `arg1` and `arg2`), the environment table, the input queue, and the
 *  transcript, exactly the 5.4 [Evaluator]'s surfaces plus the step
 *  counter. */
public class CompiledEvaluator internal constructor(
    public val machine: StepCountingMachine,
    private val evaluator: Evaluator,
) {
    /** Everything the driver and the machine printed, in order. */
    public val transcript: List<String>
        get() = evaluator.transcript

    /** Executed instructions, the runs summed. */
    public val steps: Long
        get() = machine.steps

    /** Runs the controller to the driver's end; the queue-dry end
     *  answers null, a real stop the fault note. */
    public fun drive(): String? = evaluator.drive()
}

/** The section's interface procedure: reads the object-language
 *  [source] into the input queue, sets up the global environment, and
 *  builds the 5.5.7 machine over [controller] and [operations] (the
 *  base tables, then [operations] last so an exercise's entries
 *  override on a name collision). [extraPrimitives] binds additional
 *  object-level procedures in the global environment; with
 *  [runtimeSupport], the object-level environment operations and
 *  `apply-in-underlying-scheme` the compiled metacircular evaluator of
 *  5.50 calls are bound too. The flag starts false, so the plain
 *  driver path runs. */
context(r: Raise<MachineError>)
public fun makeCompiledEvaluator(
    source: String,
    controller: List<Stmt> = ecevalController,
    operations: Map<String, Op> = emptyMap(),
    extraPrimitives: Map<String, ObjectPrimitive> = emptyMap(),
    runtimeSupport: Boolean = false,
): CompiledEvaluator {
    val state = EvaluatorState()
    when (val read = either { readProgram(source) }) {
        is Either.Left -> r.raise(EvaluatorFault(read.value.toString()))
        is Either.Right -> state.queue.addAll(read.value)
    }
    val global = sicp.runtime.Env.global()
    global.define("true", VBool(true))
    global.define("false", VBool(false))
    val runtime =
        buildMap<String, ObjectPrimitive> {
            if (runtimeSupport) putAll(runtimeSupportPrimitives(state))
            putAll(extraPrimitives)
        }
    for (name in objectPrimitives.keys) global.define(name, primitiveWord(name))
    for (name in runtime.keys) global.define(name, primitiveWord(name))
    state.envs.add(global)
    val machine =
        StepCountingMachine(
            evaluatorRegisters + listOf("arg1", "arg2"),
            evaluatorOperations + arithOperations + environmentOperations(state) + compiledOperations(state, runtime) + operations,
        )
    // The flag register starts false so the plain driver path runs: the
    // guard branch must find a boolean, never the unassigned word.
    machine.flag.store(VBool(false))
    state.machine = machine
    machine.install(controller)
    return CompiledEvaluator(machine, Evaluator(machine, state))
}

// ---------------------------------------------------------------------------
// Compile and go
// ---------------------------------------------------------------------------

/** The book's `compile-and-go`: the compiled [block] is appended to the
 *  machine's [controller], `val` is set to the [entry]'s address, the
 *  flag arms the external entry, and the machine runs the compiled code,
 *  prints the value, and enters the driver loop, whose inputs are
 *  [source]. */
context(r: Raise<MachineError>)
public fun compileAndGo(
    entry: String,
    block: List<Stmt>,
    source: String,
    controller: List<Stmt> = ecevalController,
    extraPrimitives: Map<String, ObjectPrimitive> = emptyMap(),
    runtimeSupport: Boolean = false,
): CompiledEvaluator {
    val evaluator =
        makeCompiledEvaluator(
            source,
            controller + block,
            runtimeSupport = runtimeSupport,
            extraPrimitives = extraPrimitives,
        )
    val address =
        evaluator.machine.labels[entry]
            ?: r.raise(EvaluatorFault("compile-and-go: the entry $entry is not in the assembled machine"))
    evaluator.machine.registers
        .getValue("val")
        .store(VInt(address.toLong()))
    evaluator.machine.flag.store(VBool(true))
    return evaluator
}
