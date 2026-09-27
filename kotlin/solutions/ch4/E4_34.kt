// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import sicp.ch4.EvalStep
import sicp.ch4.LazyEvaluator
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.isSinkCall
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.ThunkState
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VPrimitive
import sicp.runtime.VTagged
import sicp.runtime.VThunk
import sicp.runtime.Value
import sicp.runtime.cons

// Exercise 4.34: printing lazy pairs. The procedural pairs of the section
// are indistinguishable from any other procedure, so the evaluator cannot
// print what it cannot identify; the representation therefore changes to a
// tagged cell -- [WithLazyPrint] makes `cons` a non-strict primitive that
// packages its two operand expressions as thunks inside a `lazy-pair`
// tag, and `car`/`cdr` stay strict primitives that force their slot. The
// driver prints through [WithLazyPrint.printLazy], whose budget is the
// edition's answer to infinite lists: at most ten elements of a list,
// then ` ...)` -- the prefix plus ellipsis. A finite pair prints whole,
// a dotted tail prints after ` . `, and nested lazy pairs print
// recursively.

/** The evaluator with an identifiable, non-strict pair constructor. */
public class WithLazyPrint(
    global: Env,
) : LazyEvaluator(global) {
    public companion object {
        /** The tag identifying a lazy pair. */
        public const val TAG: String = "lazy-pair"
    }

    context(r: Raise<SchemeError>)
    override fun applyDelaying(
        procedure: Value,
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep =
        when {
            procedure is VPrimitive && procedure.name == "cons" -> {
                lazyCons(operands, env)
            }

            procedure is VPrimitive && (procedure.name == "car" || procedure.name == "cdr") -> {
                lazyAccessor(procedure.name, operands, env)
            }

            else -> {
                super.applyDelaying(procedure, operands, env)
            }
        }

    /** The non-strict constructor: both slots become thunks. */
    context(r: Raise<SchemeError>)
    private fun lazyCons(
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep {
        if (operands.size != 2) r.raise(SchemeError.WrongArity("cons", "2", operands.size))
        val slots = delayOperands(operands, env)
        return EvalStep.Done(VTagged(TAG, cons(slots[0], slots[1])))
    }

    /** The strict accessors: the pair forces, then the demanded slot. */
    context(r: Raise<SchemeError>)
    private fun lazyAccessor(
        name: String,
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep {
        if (operands.size != 1) r.raise(SchemeError.WrongArity(name, "1", operands.size))
        val forced = actualValue(operands.first(), env)
        val tagged = forced as? VTagged
        if (tagged == null || tagged.tag != TAG) {
            r.raise(SchemeError.TypeMismatch("$name of a non-pair: $forced"))
        }
        val slots = tagged.data as? VPair ?: r.raise(SchemeError.TypeMismatch("$name of a malformed lazy pair"))
        val slot = if (name == "car") slots.car else slots.cdr
        return EvalStep.Done(forceValue(slot))
    }

    /** The driver's lazy printing: [budget] elements per list, then an
     * ellipsis; anything that is not a lazy pair prints as itself. */
    context(r: Raise<SchemeError>)
    public fun printLazy(
        v: Value,
        budget: Int = 10,
    ): String {
        val out = StringBuilder()
        printInto(out, forceValue(v), budget)
        return out.toString()
    }

    context(r: Raise<SchemeError>)
    private fun printInto(
        out: StringBuilder,
        forced: Value,
        budget: Int,
    ) {
        val tagged = forced as? VTagged
        if (tagged == null || tagged.tag != TAG) {
            out.append(printValue(forced))
            return
        }
        out.append('(')
        var node: VTagged = tagged
        var count = 0
        while (true) {
            val current = node
            val slots = current.data as VPair
            printInto(out, forceValue(slots.car), budget)
            count++
            val tail = forceValue(slots.cdr)
            val next = tail as? VTagged
            if (next != null && next.tag == TAG && count < budget) {
                out.append(' ')
                node = next
                continue
            }
            if (next != null && next.tag == TAG) {
                out.append(" ...)")
                return
            }
            if (tail !is VNil) {
                out.append(" . ").append(printValue(tail))
            }
            out.append(')')
            return
        }
    }
}

/** Runs [text] under the lazy-printing driver: every top-level answer
 * prints through [WithLazyPrint.printLazy]. */
public fun lazyPrintTranscript(text: String): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = WithLazyPrint(env)
    either {
        for (expr in parseProgram(readProgram(text))) {
            when {
                expr is DefineE -> evaluator.eval(expr, env)

                // a define prints nothing
                expr is AppE && isSinkCall(expr) -> evaluator.eval(expr, env)

                // display/newline: side effect only
                else -> sink.line(evaluator.printLazy(evaluator.eval(expr, env)))
            }
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** A dotted pair and a proper list print in their shapes.
 * => "(1 . 2)\n(1 2)\n" */
public fun lazyPairPrintTranscript(): String =
    lazyPrintTranscript(
        """
        (cons 1 2)
        (cons 1 (cons 2 '()))
        """.trimIndent(),
    )

/** The infinite list prints as the ten-element prefix plus the ellipsis.
 * => "(1 1 1 1 1 1 1 1 1 1 ...)\n" */
public fun onesBudgetPrintTranscript(): String =
    lazyPrintTranscript(
        """
        (define ones (cons 1 ones))
        ones
        """.trimIndent(),
    )

/** Elements force on demand: `car` answers without printing the list.
 * => "1\n" */
public fun carOfOnesTranscript(): String =
    lazyPrintTranscript(
        """
        (define ones (cons 1 ones))
        (car ones)
        """.trimIndent(),
    )

/** Nested lazy pairs print recursively. => "((1) 2)\n" */
public fun nestedLazyPrintTranscript(): String =
    lazyPrintTranscript(
        """
        (cons (cons 1 '()) (cons 2 '()))
        """.trimIndent(),
    )
