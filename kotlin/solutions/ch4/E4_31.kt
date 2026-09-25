// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.isSinkCall
import sicp.ch4.parseExpr
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.ThunkState
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VSym
import sicp.runtime.VThunk
import sicp.runtime.VThunkNoMemo
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.isTrue
import sicp.runtime.listItems
import sicp.runtime.vlist

// Exercise 4.31: laziness as an upward-compatible extension. The
// declaration syntax `(name lazy)` / `(name lazy-memo)` rides the typed
// parser: [parseAnnotatedExpr] recognizes the procedure-definition shape
// whose parameter list carries annotated entries and lowers it to an
// ordinary lambda plus a modes datum under the reserved constructor
// `*annotated-lambda*`, a name no program text can reach. Evaluating that
// application builds the plain procedure and records its per-parameter
// mode; [AnnotatedParams] binds strict parameters by evaluation, `lazy`
// parameters as unmemoized thunks (the book's call-by-name), and
// `lazy-memo` parameters as memoized thunks (call-by-need), and forces at
// the demand sites the lazy section named: an `if` predicate and any value
// a strict primitive will use. A procedure with no annotations binds and
// applies exactly like the base evaluator.

/** How one parameter of an annotated procedure receives its argument. */
public enum class ParamMode {
    /** The base rule: evaluated at application. */
    Strict,

    /** Delayed, re-run at every demand: call-by-name. */
    Lazy,

    /** Delayed, computed at most once: call-by-need. */
    LazyMemo,
}

/** The reserved constructor the annotation parser lowers to; no reader
 * produced text can name it. */
private const val ANNOTATED_LAMBDA: String = "*annotated-lambda*"

/** Every top-level form as an [Expr], procedure definitions with annotated
 * parameters lowered through [parseAnnotatedExpr]. */
context(r: Raise<SchemeError>)
public fun parseAnnotatedProgram(forms: List<Value>): List<Expr> = forms.map { parseAnnotatedExpr(it) }

/** One datum as an [Expr]; an annotated procedure definition lowers to the
 * reserved constructor over its modes datum and plain lambda, everything
 * else parses by the base grammar. */
context(r: Raise<SchemeError>)
public fun parseAnnotatedExpr(v: Value): Expr {
    if (v is VPair && v.car is VSym && (v.car as VSym).name == "define") {
        val lowered = lowerAnnotatedDefine(v)
        if (lowered != null) return lowered
    }
    return parseExpr(v)
}

/** The lowering, or null when [v] is not a definition whose parameter list
 * carries at least one annotated entry. */
context(r: Raise<SchemeError>)
private fun lowerAnnotatedDefine(v: VPair): Expr? {
    val items = cdrListOf(v) ?: return null
    if (items.isEmpty()) return null
    val signature = items[0] as? VPair ?: return null
    val name = (signature.car as? VSym)?.name ?: return null
    val parsed = parseSignature(signature) ?: return null
    if (parsed.modes.isEmpty()) return null
    val lambda = LambdaE(parsed.names.toPersistentList(), null, parseBodyOf(items))
    val modesDatum = vlist(parsed.modes.entries.map { vlist(listOf(VSym(it.key), VSym(modeName(it.value)))) })
    return DefineE(name, AppE(VarE(ANNOTATED_LAMBDA), persistentListOf(LitE(modesDatum), lambda)))
}

private data class Signature(
    val names: List<String>,
    /** Only the annotated entries: parameter name to its mode. */
    val modes: Map<String, ParamMode>,
)

/** Reads a parameter list whose entries are names or `(name mode)` pairs,
 * or null when the shape is not a proper annotated-capable list. */
private fun parseSignature(signature: VPair): Signature? {
    val names = mutableListOf<String>()
    val modes = mutableMapOf<String, ParamMode>()
    var cursor: Value = signature.cdr
    while (cursor is VPair) {
        when (val entry = cursor.car) {
            is VSym -> {
                names.add(entry.name)
            }

            is VPair -> {
                val parameter = entry.car as? VSym ?: return null
                val mode = annotatedModeOf(entry) ?: return null
                names.add(parameter.name)
                modes[parameter.name] = mode
            }

            else -> {
                return null
            }
        }
        cursor = cursor.cdr
    }
    return if (cursor is VNil) Signature(names, modes) else null
}

/** The mode symbol of one `(name mode)` entry, or null. */
private fun annotatedModeOf(entry: VPair): ParamMode? {
    val rest = entry.cdr as? VPair ?: return null
    if (rest.cdr !is VNil) return null
    return annotatedMode(rest.car)
}

/** The mode one annotation symbol names, or null. */
private fun annotatedMode(v: Value): ParamMode? =
    when {
        v is VSym && v.name == "lazy" -> ParamMode.Lazy
        v is VSym && v.name == "lazy-memo" -> ParamMode.LazyMemo
        else -> null
    }

private fun modeName(mode: ParamMode): String =
    when (mode) {
        ParamMode.Lazy -> "lazy"
        ParamMode.LazyMemo -> "lazy-memo"
        ParamMode.Strict -> "strict"
    }

private fun cdrListOf(v: VPair): List<Value>? =
    run {
        val items = mutableListOf<Value>()
        var cursor = v.cdr
        while (cursor is VPair) {
            items.add(cursor.car)
            cursor = cursor.cdr
        }
        if (cursor is VNil) items else null
    }

context(r: Raise<SchemeError>)
private fun parseBodyOf(items: List<Value>): PersistentList<Expr> {
    if (items.size < 2) r.raise(SchemeError.Parse("empty annotated define body"))
    return items.drop(1).map { parseAnnotatedExpr(it) }.toPersistentList()
}

/** The upward-compatible evaluator: strict by default, per-parameter delay
 * where the declaration says so. */
public class AnnotatedParams(
    global: Env,
) : Evaluator(global) {
    /** The mode table, keyed by procedure identity. */
    private val modes = HashMap<VProc, Map<String, ParamMode>>()

    /** The book's `actual-value` for this evaluator: force what `eval`
     * returned. */
    context(r: Raise<SchemeError>)
    public fun actualValue(
        expr: Expr,
        env: Env,
    ): Value = forceIfThunk(eval(expr, env))

    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when {
            expr is AppE && isConstructor(expr) -> registerAnnotated(expr, env)
            expr is IfE -> forcedIf(expr, env)
            else -> super.step(expr, env)
        }

    private fun isConstructor(expr: AppE): Boolean = (expr.operator as? VarE)?.name == ANNOTATED_LAMBDA && expr.operands.size == 2

    /** Builds the plain procedure and records its modes; the value of the
     * reserved application is the procedure itself. */
    context(r: Raise<SchemeError>)
    private fun registerAnnotated(
        expr: AppE,
        env: Env,
    ): EvalStep {
        val lambda = expr.operands[1] as? LambdaE ?: r.raise(SchemeError.Parse("bad annotated lambda"))
        val declared = declaredModes((expr.operands[0] as LitE).v)
        val procedure = makeProcedure(lambda.params, lambda.rest, lambda.body, env, null) as VProc
        modes[procedure] = declared
        return EvalStep.Done(procedure)
    }

    /** The modes datum as a name-to-mode map. */
    private fun declaredModes(datum: Value): Map<String, ParamMode> {
        val declared = mutableMapOf<String, ParamMode>()
        var cursor: Value = datum
        while (cursor is VPair) {
            val entry = cursor.car as VPair
            val name = (entry.car as VSym).name
            val mode = annotatedModeOf(entry) ?: ParamMode.Strict
            declared[name] = mode
            cursor = cursor.cdr
        }
        return declared
    }

    context(r: Raise<SchemeError>)
    private fun forcedIf(
        expr: IfE,
        env: Env,
    ): EvalStep {
        val predicate = forceIfThunk(eval(expr.predicate, env))
        return EvalStep.Continue(if (isTrue(predicate)) expr.consequent else expr.alternative, env)
    }

    /** Forces the thunk shapes; anything else is already a value. */
    context(r: Raise<SchemeError>)
    public fun forceIfThunk(v: Value): Value =
        when (v) {
            is VThunk -> {
                when (val state = v.state) {
                    is ThunkState.Forced -> {
                        state.v
                    }

                    is ThunkState.Delayed -> {
                        val answer = actualValue(state.expr, state.env)
                        v.state = ThunkState.Forced(answer)
                        answer
                    }
                }
            }

            is VThunkNoMemo -> {
                actualValue(v.expr, v.env)
            }

            else -> {
                v
            }
        }

    /** Binds annotated procedures per parameter mode; primitives demand
     * actual values (forcing any thunk an argument reads); everything else
     * keeps the base discipline. */
    context(r: Raise<SchemeError>)
    override fun evalApplication(
        expr: AppE,
        env: Env,
    ): EvalStep {
        val procedure = eval(expr.operator, env)
        val declared = modes[procedure]
        val arguments =
            when {
                procedure is VProc && declared != null -> {
                    annotatedArguments(procedure, declared, expr.operands, env)
                }

                procedure is VPrimitive -> {
                    expr.operands.map { forceIfThunk(eval(it, env)) }
                }

                else -> {
                    expr.operands.map { eval(it, env) }
                }
            }
        return applyProcedure(procedure, arguments)
    }

    /** One value per operand: strict parameters evaluated, `lazy` as
     * unmemoized thunks, `lazy-memo` as memoized ones. */
    context(r: Raise<SchemeError>)
    private fun annotatedArguments(
        procedure: VProc,
        declared: Map<String, ParamMode>,
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> {
        if (procedure.rest == null && procedure.params.size != operands.size) {
            r.raise(SchemeError.WrongArity(procedure.name ?: "procedure", procedure.params.size.toString(), operands.size))
        }
        return operands.mapIndexed { index, operand ->
            val mode = declared[procedure.params[index]] ?: ParamMode.Strict
            when (mode) {
                ParamMode.Strict -> eval(operand, env)
                ParamMode.Lazy -> VThunkNoMemo(operand, env)
                ParamMode.LazyMemo -> VThunk(ThunkState.Delayed(operand, env))
            }
        }
    }

    /** `apply` re-binds an applied annotated procedure per its modes, from
     * already-evaluated values. */
    context(r: Raise<SchemeError>)
    override fun applyProcedure(
        procedure: Value,
        arguments: List<Value>,
    ): EvalStep =
        when {
            procedure is VPrimitive && procedure.name == "apply" && arguments.size == 2 && modes.containsKey(arguments[0]) -> {
                val target = arguments[0] as VProc
                val declared = modes.getValue(target)
                applyProcedure(target, annotatedValues(target, declared, listItems(arguments[1])))
            }

            else -> {
                super.applyProcedure(procedure, arguments)
            }
        }

    /** Binds evaluated values to an annotated procedure's parameters,
     * packaging each delayed one as a self-evaluating thunk. */
    context(r: Raise<SchemeError>)
    private fun annotatedValues(
        procedure: VProc,
        declared: Map<String, ParamMode>,
        values: List<Value>,
    ): List<Value> =
        values.mapIndexed { index, value ->
            val mode = declared[procedure.params[index]] ?: ParamMode.Strict
            when (mode) {
                ParamMode.Strict -> value
                ParamMode.Lazy -> VThunkNoMemo(LitE(value), procedure.env)
                ParamMode.LazyMemo -> VThunk(ThunkState.Delayed(LitE(value), procedure.env))
            }
        }
}

/** Runs [text] under the annotated evaluator, forcing every top-level
 * answer, on the printer contract. */
public fun annotatedTranscript(text: String): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = AnnotatedParams(env)
    either {
        for (expr in parseAnnotatedProgram(readProgram(text))) {
            when {
                expr is DefineE -> evaluator.eval(expr, env)

                // a define prints nothing
                expr is AppE && isSinkCall(expr) -> evaluator.eval(expr, env)

                // display/newline: side effect only
                else -> sink.line(printValue(evaluator.actualValue(expr, env)))
            }
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** The counting session's definitions and the annotated call. */
private val MIXED_PROGRAM: String =
    """
    (define count 0)
    (define (id x) (set! count (+ count 1)) x)
    (define (f a (b lazy) c (d lazy-memo)) (list a b b c d d))
    (f (id 1) (id (+ 2 3)) (id 4) (id (* 5 6)))
    count
    """.trimIndent()

/** The declared discipline: `a` and `c` strict, `b` call-by-name, `d`
 * call-by-need. The strict pair counts 2 at the call, each `b` use
 * recomputes, `d` computes once: count 5. => "(1 5 5 4 30 30)\n5\n" */
public fun annotatedMixedTranscript(): String = annotatedTranscript(MIXED_PROGRAM)

/** All-lazy-memo declaration: every delayed parameter computes at most
 * once, so the count lands on 4. => "(1 5 5 4 30 30)\n4\n" */
public fun annotatedAllMemoTranscript(): String =
    annotatedTranscript(
        MIXED_PROGRAM
            .replace("(b lazy)", "(b lazy-memo)"),
    )

/** A `lazy` parameter is never evaluated at all unless the body demands
 * it: the dangerous argument passes harmlessly. => "7\n" */
public fun lazyParamSkipsTranscript(): String =
    annotatedTranscript(
        """
        (define (g (x lazy)) 7)
        (g (/ 1 0))
        """.trimIndent(),
    )

/** The strict counterfactual: an ordinary parameter is evaluated at the
 * call. => "Error: division by zero\n" */
public fun strictParamEagerTranscript(): String =
    annotatedTranscript(
        """
        (define (g x) 7)
        (g (/ 1 0))
        """.trimIndent(),
    )
