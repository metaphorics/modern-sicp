// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Call
import sicp.guest.CheckedProgram
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ForceCounters
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.LambdaParameter
import sicp.guest.Mode
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.Parameter
import sicp.guest.Primitive
import sicp.guest.Primitives
import sicp.guest.RunResult
import sicp.guest.ThunkState
import sicp.guest.forceThunk

/** The finite forcing instrument of section 4.1's reference model: every
 * force request, every distinct computation, and the effect lines thunk
 * bodies produced, in order. */
public class ForcingInstrument(
    public val forceAttempts: Long,
    public val computations: Long,
    public val effects: List<String>,
)

/** One lazy run: the ordinary observation plus the forcing instrument. */
public class LazyRun(
    public val result: RunResult,
    public val forcing: ForcingInstrument,
)

/** The lazy experiment of sections 4.2 and 4.2.3, mode `Lazy`. */
public object LazyModule {
    /** Admits in [Mode.LAZY] and runs; admission failures execute nothing. */
    public fun run(source: String): Either<AdmissionError, LazyRun> = either { execute(Admission.admitOrRaise(source, Mode.LAZY)) }

    private fun execute(checked: CheckedProgram): LazyRun {
        val sink = OutputSink()
        val counters = ForceCounters()
        val effects = mutableListOf<String>()
        val outcome: Either<GuestError, GValue> = either { LazyEvaluator(checked, sink, counters, effects).runMain() }
        val result =
            outcome.fold(
                { error -> RunResult(sink.contents(), error, null) },
                { value -> RunResult(sink.contents(), null, value) },
            )
        return LazyRun(result, ForcingInstrument(counters.attempts, counters.computations, effects.toList()))
    }
}

/** The direct evaluator with lazy-module semantics: unannotated and
 * `@Delayed` parameters delay their arguments in memoized transparent
 * thunks; `@Strict` parameters and primitive arguments stay eager. */
internal class LazyEvaluator(
    checked: CheckedProgram,
    sink: OutputSink,
    private val counters: ForceCounters,
    private val effects: MutableList<String>,
) : Evaluator(checked, sink) {
    override fun delaysFor(parameters: List<Parameter>): List<Boolean> = parameters.map { it.annotation != "Strict" }

    override fun delaysForLambda(parameters: List<LambdaParameter>): List<Boolean> = parameters.map { true }

    context(r: Raise<GuestError>)
    override fun evaluateArguments(
        target: GValue?,
        expression: Call,
        env: Env,
    ): List<GValue> {
        val delays = (target as? GValue.VFunction)?.delays ?: emptyList()
        return expression.arguments.mapIndexed { index, argument ->
            if (delays.getOrElse(index) { false }) delayedArgument(argument.value, env) else eval(argument.value, env)
        }
    }

    context(r: Raise<GuestError>)
    private fun delayedArgument(
        value: Expression,
        env: Env,
    ): GValue = GValue.VThunk(ThunkState.Delayed(tracked { eval(value, env) }), transparent = true, counters = counters)

    /** Wraps a computation so its output effects land in the instrument no
     * matter which force path runs it. */
    private fun tracked(compute: Primitive): Primitive =
        { arguments ->
            val before = sink.contents().length
            val value = compute(this, arguments)
            recordEffects(sink.contents().substring(before))
            value
        }

    context(r: Raise<GuestError>)
    override fun exposeValue(
        value: GValue,
        at: Expression,
    ): GValue = if (value is GValue.VThunk && value.transparent) forceThunk(value) else value

    override fun surfaceValue(name: String): GValue? = if (name == "lazyEnd") GValue.VList(mutableListOf(), mutable = false) else null

    context(r: Raise<GuestError>)
    override fun namedCall(
        callee: Name,
        expression: Call,
        env: Env,
    ): GValue =
        when (callee.text) {
            "thunk" -> thunkCall(expression, env)
            "force" -> forceCall(expression, env)
            "lazyPair" -> lazyPairCall(expression, env)
            "lazyEnd" -> GValue.VList(mutableListOf(), mutable = false)
            else -> super.namedCall(callee, expression, env)
        }

    context(r: Raise<GuestError>)
    private fun thunkCall(
        expression: Call,
        env: Env,
    ): GValue {
        val body = evaluateArguments(null, expression, env).single()
        return GValue.VThunk(
            ThunkState.Delayed(tracked { Primitives.invoke(body, emptyList(), expression.span) }),
            transparent = false,
            counters = counters,
        )
    }

    context(r: Raise<GuestError>)
    private fun forceCall(
        expression: Call,
        env: Env,
    ): GValue {
        val thunk =
            evaluateArguments(null, expression, env).single() as? GValue.VThunk
                ?: r.raise(GuestError.UnassignedRead(expression.span))
        return forceThunk(thunk)
    }

    context(r: Raise<GuestError>)
    private fun lazyPairCall(
        expression: Call,
        env: Env,
    ): GValue {
        val arguments = evaluateArguments(null, expression, env)
        val tail = arguments[1] as? GValue.VThunk ?: r.raise(GuestError.UnassignedRead(expression.span))
        return GValue.VLazyList(arguments[0], tail)
    }

    private fun recordEffects(delta: String) {
        for (line in delta.split('\n')) {
            if (line.isNotEmpty()) effects.add(line)
        }
    }
}
