// SPDX-License-Identifier: GPL-3.0-only
// Section 5.4: the explicit-control evaluator. The book's register machine
// that runs the metacircular evaluator's algorithm directly, as a controller
// sequence over the committed 5.2 simulator: the registers, the monitored
// stack, the flag, and the assembler are exactly 5.2's, and the controller
// is the `Stmt` list in `S5_4Controller.kt`.
//
// Machine words. The runtime `Value` type is the word type, the 5.3
// precedent, so every word rides in a machine register unchanged: object
// values are themselves, and the evaluator's own data are tagged words no
// object-language value can spell -- the environment word (a handle into
// the evaluator's environment table), the procedure words, the thunk word
// of exercise 5.25, and exercise 5.30's condition codes. Object-language
// expressions are the list structure the reader produced, the book's own
// uniform representation, so the syntax operations are the 4.1.2 list
// procedures the controller names.
//
// Nothing raises out of a correct run: the driver loop ends when the input
// queue runs dry, through the typed [EvaluatorFault] whose detail is
// [INPUT_EXHAUSTED], and every other failure of the evaluator stops the
// machine with a typed fault too.

package sicp.ch5

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch4.readProgram
import sicp.runtime.Env
import sicp.runtime.SchemeError
import sicp.runtime.Stmt
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VReal
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.VTagged
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

/** Every failure of the evaluator travels through one of these: the detail
 *  is the object-level message (`unbound variable: x`, `division by zero`)
 *  the book's `signal-error` reports, and the queue-dry end of the driver
 *  loop is one of them too. */
public data class EvaluatorFault(
    val detail: String,
) : MachineError() {
    public override fun toString(): String = detail
}

/** The message whose [EvaluatorFault] ends the driver loop when the input
 *  queue runs dry: the edition's stop for the book's unbounded
 *  read-eval-print loop. */
public const val INPUT_EXHAUSTED: String = "the evaluator's input queue is empty"

/** The stop note a driver run reports for a typed fault, the book's
 *  `signal-error` line. */
public fun faultNote(e: MachineError): String =
    when (e) {
        is EvaluatorFault -> "operation failed: ${e.detail}"
        else -> e.toString()
    }

/** The reserved tag prefix of the evaluator's own words: no object-language
 *  value carries it, because the object language has no tagged words. */
private const val WORD_TAG: String = "sicp-word:"

// ---------------------------------------------------------------------------
// Machine words
// ---------------------------------------------------------------------------

/** The environment word: a handle into the evaluator's environment table,
 *  what the `env` register holds. */
public fun envWord(handle: Int): Value = VTagged("$WORD_TAG environment", VInt(handle.toLong()))

/** Whether [w] is an environment word. */
public fun isEnvWord(w: Value): Boolean = w is VTagged && w.tag == "$WORD_TAG environment"

/** The compound procedure word: the parameter names, the body, and the
 *  environment word, the book's `make-procedure` product. */
public fun compoundProcedureWord(
    params: List<String>,
    body: List<Value>,
    env: Value,
): Value =
    VTagged(
        "$WORD_TAG compound-procedure",
        vlist(listOf(vlist(params.map(::VSym)), vlist(body), env)),
    )

/** Whether [w] is a compound procedure word. */
public fun isCompoundProcedure(w: Value): Boolean = w is VTagged && w.tag == "$WORD_TAG compound-procedure"

/** The primitive procedure word: the name into the primitive table. */
public fun primitiveWord(name: String): Value = VTagged("$WORD_TAG primitive", VSym(name))

/** Whether [w] is a primitive procedure word. */
public fun isPrimitiveProcedure(w: Value): Boolean = w is VTagged && w.tag == "$WORD_TAG primitive"

/** The name a primitive procedure word carries. */
public fun primitiveName(w: Value): String = ((w as VTagged).data as VSym).name

/** The thunk word of exercise 5.25: the delayed expression and the
 *  environment it delays over. The base evaluator never builds one. */
public fun thunkWord(
    expr: Value,
    env: Value,
): Value = VTagged("$WORD_TAG thunk", vlist(listOf(expr, env)))

/** Whether [w] is a thunk word. */
public fun isThunk(w: Value): Boolean = w is VTagged && w.tag == "$WORD_TAG thunk"

/** The two halves of a thunk word: the expression, then the environment. */
public fun thunkParts(w: Value): Pair<Value, Value> {
    val parts = ((w as VTagged).data as VPair).toItems()
    return parts[0] to parts[1]
}

/** The condition code of exercise 5.30: a [kind] tag and the detail
 *  `signal-error` reports. The reserved tag keeps it off every user value. */
public fun conditionWord(
    kind: String,
    detail: String,
): Value = VTagged("$WORD_TAG condition $kind", VStr(detail))

/** Whether [w] is a condition code. */
public fun isConditionWord(w: Value): Boolean = w is VTagged && w.tag.startsWith("$WORD_TAG condition ")

/** The detail a condition code carries. */
public fun conditionDetail(w: Value): String = ((w as VTagged).data as VStr).s

/** The kind a condition code carries (`unbound-variable`,
 *  `primitive-failure`, ...). */
public fun conditionKind(w: Value): String = (w as VTagged).tag.removePrefix("$WORD_TAG condition ")

/** Renders a word the way the driver's `user-print` does: procedures,
 *  thunks, and condition codes by name, everything else in surface form. */
public fun renderWord(w: Value): String =
    when {
        isCompoundProcedure(w) -> "#[compound-procedure]"
        isPrimitiveProcedure(w) -> "#[primitive ${primitiveName(w)}]"
        isThunk(w) -> "#[thunk]"
        isConditionWord(w) -> conditionDetail(w)
        else -> w.toString()
    }

// ---------------------------------------------------------------------------
// List-structure helpers, the 4.1.2 syntax procedures' substrate
// ---------------------------------------------------------------------------

/** The items of the proper list [v]; an improper list is a typed fault
 *  naming the operation that asked. */
public fun Raise<MachineError>.listItems(
    op: String,
    v: Value,
): List<Value> {
    val items = ArrayList<Value>()
    var cursor = v
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    if (cursor !is VNil) raise(EvaluatorFault("$op needs a proper list"))
    return items
}

/** The name of the variable expression [w]; anything else is a typed fault
 *  naming the operation that asked. */
public fun Raise<MachineError>.symbolNameOf(w: Value): String =
    when (w) {
        is VSym -> w.name
        else -> raise(EvaluatorFault("expected a variable, found $w"))
    }

private fun VPair.toItems(): List<Value> {
    val items = ArrayList<Value>()
    var cursor: Value = this
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    return items
}

private fun isHead(
    w: Value,
    name: String,
): Boolean = w is VPair && w.car is VSym && (w.car as VSym).name == name

private fun Value.cdrOf(): Value = (this as VPair).cdr

private fun Value.cddrOf(): Value = (this as VPair).cdr.cdrOf()

// ---------------------------------------------------------------------------
// The object-language primitives
// ---------------------------------------------------------------------------

/** One primitive of the global environment: applied to argument values, it
 *  yields one value or a typed [EvaluatorFault]. */
public typealias ObjectPrimitive = Raise<EvaluatorFault>.(List<Value>) -> Value

private fun Raise<EvaluatorFault>.number(
    name: String,
    v: Value,
): Long =
    when (v) {
        is VInt -> v.n
        else -> raise(EvaluatorFault("type error: $name needs a number, got $v"))
    }

private fun arityFault(
    expected: Int,
    given: Int,
): EvaluatorFault = EvaluatorFault("arity mismatch: expected $expected, given $given")

private fun onePrim(
    name: String,
    f: Raise<EvaluatorFault>.(Value) -> Value,
): ObjectPrimitive =
    { args ->
        if (args.size != 1) raise(arityFault(1, args.size))
        f(args[0])
    }

private fun twoPrim(
    name: String,
    f: Raise<EvaluatorFault>.(Value, Value) -> Value,
): ObjectPrimitive =
    { args ->
        if (args.size != 2) raise(arityFault(2, args.size))
        f(args[0], args[1])
    }

/** The n-ary object-language arithmetic: [f] folded left, `(- a)` the
 *  negation, a division by zero the typed failure. */
private fun arith(
    name: String,
    f: (Long, Long) -> Long,
): ObjectPrimitive =
    { args ->
        when {
            args.isEmpty() -> {
                raise(arityFault(2, 0))
            }

            args.size == 1 && name == "-" -> {
                VInt(-number(name, args[0]))
            }

            else -> {
                var acc = number(name, args.first())
                for (v in args.drop(1)) {
                    val word = number(name, v)
                    if (name == "/" && word == 0L) raise(EvaluatorFault("division by zero"))
                    acc = f(acc, word)
                }
                VInt(acc)
            }
        }
    }

private fun comparison(
    name: String,
    test: (Long, Long) -> Boolean,
): ObjectPrimitive =
    { args ->
        if (args.size != 2) raise(arityFault(2, args.size))
        VBool(test(number(name, args[0]), number(name, args[1])))
    }

/** The primitive procedures of the global environment, the book's eceval
 *  list trimmed to the subset the section's sessions use. */
public val objectPrimitives: Map<String, ObjectPrimitive> =
    mapOf(
        "cons" to twoPrim("cons") { a, b -> cons(a, b) },
        "car" to
            onePrim("car") { v ->
                when (v) {
                    is VPair -> v.car
                    else -> raise(EvaluatorFault("type error: car of $v"))
                }
            },
        "cdr" to
            onePrim("cdr") { v ->
                when (v) {
                    is VPair -> v.cdr
                    else -> raise(EvaluatorFault("type error: cdr of $v"))
                }
            },
        "null?" to onePrim("null?") { v -> VBool(v is VNil) },
        "pair?" to onePrim("pair?") { v -> VBool(v is VPair) },
        "symbol?" to onePrim("symbol?") { v -> VBool(v is VSym) },
        "number?" to onePrim("number?") { v -> VBool(v is VInt || v is VReal) },
        "string?" to onePrim("string?") { v -> VBool(v is VStr) },
        "not" to onePrim("not") { v -> VBool(v == VBool(false)) },
        "eq?" to twoPrim("eq?") { a, b -> VBool(a == b) },
        "equal?" to twoPrim("equal?") { a, b -> VBool(sicp.runtime.equalv(a, b)) },
        "list" to { args -> vlist(args) },
        "+" to arith("+") { a, b -> a + b },
        "-" to arith("-") { a, b -> a - b },
        "*" to arith("*") { a, b -> a * b },
        "/" to arith("/") { a, b -> a / b },
        "=" to comparison("=") { a, b -> a == b },
        "<" to comparison("<") { a, b -> a < b },
        ">" to comparison(">") { a, b -> a > b },
        "remainder" to
            twoPrim("remainder") { a, b ->
                val divisor = number("remainder", b)
                if (divisor == 0L) raise(EvaluatorFault("division by zero"))
                VInt(number("remainder", a) % divisor)
            },
    )

/** Applies the object-language primitive [name] to the values [args]: the
 *  evaluator's `apply-primitive-procedure` calls this with `proc`'s name,
 *  and exercise 5.30 wraps its failures in condition-code words. An unknown
 *  name is the typed fault. */
public fun applyObjectPrimitive(
    name: String,
    args: List<Value>,
): Either<EvaluatorFault, Value> {
    val primitive =
        objectPrimitives[name]
            ?: return Either.Left(EvaluatorFault("unknown primitive procedure: $name"))
    return either { primitive(args) }
}

// ---------------------------------------------------------------------------
// The operations table
// ---------------------------------------------------------------------------

private fun oneArg(
    name: String,
    f: Raise<MachineError>.(Value) -> Value,
): Op =
    { args ->
        if (args.size != 1) raise(EvaluatorFault("$name needs one argument"))
        f(args[0])
    }

private fun twoArgs(
    name: String,
    f: Raise<MachineError>.(Value, Value) -> Value,
): Op =
    { args ->
        if (args.size != 2) raise(EvaluatorFault("$name needs two arguments"))
        f(args[0], args[1])
    }

private fun threeArgs(
    name: String,
    f: Raise<MachineError>.(Value, Value, Value) -> Value,
): Op =
    { args ->
        if (args.size != 3) raise(EvaluatorFault("$name needs three arguments"))
        f(args[0], args[1], args[2])
    }

private fun selfEvaluating(w: Value): Boolean = w is VInt || w is VReal || w is VBool || w is VStr

private fun Raise<MachineError>.secondOf(
    op: String,
    w: Value,
): Value {
    val items = listItems(op, w)
    if (items.size < 2) raise(EvaluatorFault("$op needs a well-formed form"))
    return items[1]
}

private fun Raise<MachineError>.thirdOf(
    op: String,
    w: Value,
): Value {
    val items = listItems(op, w)
    if (items.size < 3) raise(EvaluatorFault("$op needs a well-formed form"))
    return items[2]
}

/** The definition's variable: the name itself, or the head of the procedure
 *  sugar's signature. */
private fun Raise<MachineError>.definitionVariable(w: Value): Value {
    val target = secondOf("definition-variable", w)
    return when (target) {
        is VPair -> target.car
        else -> target
    }
}

/** The definition's value: the plain value, or the lambda the procedure
 *  sugar's signature and body spell. */
private fun Raise<MachineError>.definitionValue(w: Value): Value {
    val items = listItems("definition-value", w)
    val target = items[1]
    when (target) {
        is VPair -> {
            val params = target.toItems().drop(1)
            return cons(VSym("lambda"), cons(vlist(params), vlist(items.drop(2))))
        }

        else -> {
            if (items.size < 3) raise(EvaluatorFault("definition-value needs a well-formed define"))
            return items[2]
        }
    }
}

private fun Raise<MachineError>.procedurePart(
    op: String,
    w: Value,
    part: Int,
): Value {
    if (!isCompoundProcedure(w)) raise(EvaluatorFault("$op needs a compound procedure"))
    return ((w as VTagged).data as VPair).toItems()[part]
}

/** The base operations the controller names: the syntax procedures of
 *  4.1.2 over the reader's list structure, the sequence and argument-list
 *  selectors, the procedure-word operations, and `apply-primitive-
 *  procedure`. The environment-table operations and the driver operations
 *  are bound per evaluator in [makeEvaluator]. */
public val evaluatorOperations: Map<String, Op> =
    mapOf(
        "self-evaluating?" to oneArg("self-evaluating?") { w -> VBool(selfEvaluating(w)) },
        "variable?" to oneArg("variable?") { w -> VBool(w is VSym) },
        "quoted?" to oneArg("quoted?") { w -> VBool(isHead(w, "quote")) },
        "assignment?" to oneArg("assignment?") { w -> VBool(isHead(w, "set!")) },
        "definition?" to oneArg("definition?") { w -> VBool(isHead(w, "define")) },
        "if?" to oneArg("if?") { w -> VBool(isHead(w, "if")) },
        "lambda?" to oneArg("lambda?") { w -> VBool(isHead(w, "lambda")) },
        "begin?" to oneArg("begin?") { w -> VBool(isHead(w, "begin")) },
        "application?" to oneArg("application?") { w -> VBool(w is VPair) },
        "text-of-quotation" to oneArg("text-of-quotation") { w -> secondOf("text-of-quotation", w) },
        "if-predicate" to oneArg("if-predicate") { w -> secondOf("if-predicate", w) },
        "if-consequent" to oneArg("if-consequent") { w -> thirdOf("if-consequent", w) },
        "if-alternative" to
            oneArg("if-alternative") { w ->
                val items = listItems("if-alternative", w)
                if (items.size >= 4) items[3] else VSym("false")
            },
        "begin-actions" to oneArg("begin-actions") { w -> w.cdrOf() },
        "lambda-parameters" to oneArg("lambda-parameters") { w -> secondOf("lambda-parameters", w) },
        "lambda-body" to oneArg("lambda-body") { w -> w.cddrOf() },
        "operator" to oneArg("operator") { w -> (w as VPair).car },
        "operands" to oneArg("operands") { w -> w.cdrOf() },
        "assignment-variable" to oneArg("assignment-variable") { w -> secondOf("assignment-variable", w) },
        "assignment-value" to oneArg("assignment-value") { w -> thirdOf("assignment-value", w) },
        "definition-variable" to oneArg("definition-variable") { w -> definitionVariable(w) },
        "definition-value" to oneArg("definition-value") { w -> definitionValue(w) },
        "first-exp" to oneArg("first-exp") { w -> (w as VPair).car },
        "rest-exps" to oneArg("rest-exps") { w -> w.cdrOf() },
        "last-exp?" to oneArg("last-exp?") { w -> VBool(w.cdrOf() is VNil) },
        "no-more-exps?" to oneArg("no-more-exps?") { w -> VBool(w is VNil) },
        "no-operands?" to oneArg("no-operands?") { w -> VBool(w is VNil) },
        "first-operand" to oneArg("first-operand") { w -> (w as VPair).car },
        "rest-operands" to oneArg("rest-operands") { w -> w.cdrOf() },
        "last-operand?" to oneArg("last-operand?") { w -> VBool(w.cdrOf() is VNil) },
        "empty-arglist" to { _ -> VNil },
        "adjoin-arg" to
            twoArgs("adjoin-arg") { value, argl ->
                vlist(listItems("adjoin-arg", argl) + value)
            },
        "no-args?" to oneArg("no-args?") { w -> VBool(w is VNil) },
        "first-arg" to oneArg("first-arg") { w -> (w as VPair).car },
        "rest-args" to oneArg("rest-args") { w -> w.cdrOf() },
        "primitive-procedure?" to oneArg("primitive-procedure?") { w -> VBool(isPrimitiveProcedure(w)) },
        "compound-procedure?" to oneArg("compound-procedure?") { w -> VBool(isCompoundProcedure(w)) },
        "apply-primitive-procedure" to
            twoArgs("apply-primitive-procedure") { proc, argl ->
                if (!isPrimitiveProcedure(proc)) {
                    raise(EvaluatorFault("apply-primitive-procedure needs a primitive procedure"))
                }
                val values = listItems("apply-primitive-procedure", argl)
                when (val applied = applyObjectPrimitive(primitiveName(proc), values)) {
                    is Either.Left -> raise(applied.value)
                    is Either.Right -> applied.value
                }
            },
        "procedure-parameters" to
            oneArg("procedure-parameters") { w -> procedurePart("procedure-parameters", w, 0) },
        "procedure-body" to oneArg("procedure-body") { w -> procedurePart("procedure-body", w, 1) },
        "procedure-environment" to
            oneArg("procedure-environment") { w -> procedurePart("procedure-environment", w, 2) },
        "make-procedure" to
            threeArgs("make-procedure") { params, body, env ->
                val names = listItems("make-procedure", params).map { symbolNameOf(it) }
                compoundProcedureWord(names, listItems("make-procedure", body), env)
            },
        "true?" to oneArg("true?") { w -> VBool(w != VBool(false)) },
    )

// ---------------------------------------------------------------------------
// The evaluator
// ---------------------------------------------------------------------------

/** The per-evaluator state the machine-bound operations close over: the
 *  environment table the env words index, the driver's input queue, and
 *  the machine, absent while the machine itself is under construction. */
internal class EvaluatorState internal constructor() {
    val envs = ArrayList<Env>()
    val queue = ArrayDeque<Value>()
    var machine: Machine? = null

    /** The environment [word] indexes; a malformed word is a machine bug,
     *  because only the table's own operations build env words. */
    fun envOf(
        op: String,
        word: Value,
    ): Env {
        val handle =
            if (isEnvWord(word)) {
                (word as VTagged).data as? VInt
            } else {
                null
            }
        requireNotNull(handle) { "$op: the register holds no environment word" }
        return envs[handle.n.toInt()]
    }

    /** One transcript line, the driver's output path. */
    fun machinePrompt(line: String) {
        requireNotNull(machine).transcript.appendLine(line)
    }
}

/** The environment-table operations the controller names, bound per
 *  evaluator state: shared with the 5.5 compiled evaluator, which runs
 *  compiled and interpreted code on the same table. */
internal fun environmentOperations(state: EvaluatorState): Map<String, Op> =
    mapOf(
        "get-global-environment" to { _ -> envWord(0) },
        "extend-environment" to
            threeArgs("extend-environment") { params, argl, base ->
                val names = listItems("extend-environment", params).map { symbolNameOf(it) }
                val values = listItems("extend-environment", argl)
                if (names.size != values.size) {
                    raise(EvaluatorFault("arity mismatch: expected ${names.size}, given ${values.size}"))
                }
                val parent = state.envOf("extend-environment", base)
                val frame = either { Env.extend(names, values, parent) }
                when (frame) {
                    is Either.Left -> {
                        raise(EvaluatorFault("arity mismatch: expected ${names.size}, given ${values.size}"))
                    }

                    is Either.Right -> {
                        state.envs.add(frame.value)
                        envWord(state.envs.size - 1)
                    }
                }
            },
        "lookup-variable-value" to
            twoArgs("lookup-variable-value") { exp, env ->
                val name = symbolNameOf(exp)
                val holder = state.envOf("lookup-variable-value", env)
                when (val hit = either { holder.lookup(name) }) {
                    is Either.Right -> hit.value
                    is Either.Left -> raise(EvaluatorFault("unbound variable: $name"))
                }
            },
        "set-variable-value!" to
            threeArgs("set-variable-value!") { exp, value, env ->
                val name = symbolNameOf(exp)
                val holder = state.envOf("set-variable-value!", env)
                when (either { holder.set(name, value) }) {
                    is Either.Left -> raise(EvaluatorFault("unbound variable: $name"))
                    is Either.Right -> VNil
                }
            },
        "define-variable!" to
            threeArgs("define-variable!") { exp, value, env ->
                state.envOf("define-variable!", env).define(symbolNameOf(exp), value)
                VNil
            },
        "read" to { _ ->
            state.queue.removeFirstOrNull() ?: raise(EvaluatorFault(INPUT_EXHAUSTED))
        },
        "prompt-for-input" to { _ ->
            state.machinePrompt(";;; EC-Eval input:")
            VNil
        },
        "announce-output" to { _ ->
            state.machinePrompt(";;; EC-Eval value:")
            VNil
        },
        "user-print" to
            oneArg("user-print") { w ->
                state.machinePrompt(renderWord(w))
                w
            },
        "signal-error" to
            oneArg("signal-error") { w -> raise(EvaluatorFault("signal-error: ${renderWord(w)}")) },
    )

/** The registers of the evaluator machine description: the book's seven.
 *  `flag` is the simulator's own. */
public val evaluatorRegisters: List<String> =
    listOf("exp", "env", "val", "continue", "proc", "argl", "unev")

/** The section's evaluator: the assembled machine over the controller, the
 *  environment table, the input queue, and the transcript. */
public class Evaluator internal constructor(
    public val machine: Machine,
    private val state: EvaluatorState,
) {
    /** Everything the driver and the machine printed, in order. */
    public val transcript: List<String>
        get() = machine.transcript.lines().filter { it.isNotEmpty() }

    /** The environment the word [word] indexes, for the exercise
     *  operations that extend the environment discipline (5.25, 5.30); a
     *  malformed word is a typed fault naming [op]. */
    context(r: Raise<MachineError>)
    public fun environmentOf(
        op: String,
        word: Value,
    ): Env {
        val index =
            if (isEnvWord(word)) {
                ((word as VTagged).data as? VInt)?.n?.toInt()
            } else {
                null
            }
        if (index == null) r.raise(EvaluatorFault("$op needs an environment word"))
        return state.envs[index]
    }

    /** Registers [env] in the environment table and answers its word, the
     *  writing half of [environmentOf]. */
    public fun internEnvironment(env: Env): Value {
        state.envs.add(env)
        return envWord(state.envs.size - 1)
    }

    /** Runs the controller to the driver's end. The queue-dry end answers
     *  null; a real stop answers the fault note ([faultNote]). */
    public fun drive(): String? {
        val stopped = either { machine.start() }
        return when (stopped) {
            is Either.Left -> {
                when (val fault = stopped.value) {
                    is EvaluatorFault -> {
                        if (fault.detail == INPUT_EXHAUSTED) null else faultNote(fault)
                    }

                    else -> {
                        faultNote(fault)
                    }
                }
            }

            is Either.Right -> {
                null
            }
        }
    }
}

/** The section's interface procedure: reads the object-language [source],
 *  sets up the global environment (`true`, `false`, and the primitives),
 *  builds the machine over [controller] and the operations (the base
 *  table, the environment and driver operations, then [operations] last so
 *  an exercise's entries override on a name collision), and answers the
 *  evaluator ready to [Evaluator.drive]. */
context(r: Raise<MachineError>)
public fun makeEvaluator(
    source: String,
    controller: List<Stmt> = baseEvaluatorController,
    operations: Map<String, Op> = emptyMap(),
): Evaluator {
    val state = EvaluatorState()
    when (val read = either { readProgram(source) }) {
        is Either.Left -> r.raise(EvaluatorFault(read.value.toString()))
        is Either.Right -> state.queue.addAll(read.value)
    }
    val global = Env.global()
    global.define("true", VBool(true))
    global.define("false", VBool(false))
    for (name in objectPrimitives.keys) global.define(name, primitiveWord(name))
    state.envs.add(global)
    val machine =
        Machine(
            evaluatorRegisters,
            evaluatorOperations + environmentOperations(state) + operations,
        )
    state.machine = machine
    machine.install(controller)
    return Evaluator(machine, state)
}
