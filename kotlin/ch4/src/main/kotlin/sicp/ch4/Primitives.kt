// SPDX-License-Identifier: GPL-3.0-only
// The primitive procedure table of 4.1.4 and the arithmetic beneath it: the
// grammar's core set plus the additions programs/core/metacircular.scm
// needs (set-car! and set-cdr! for its environment frames, `length` and
// `map` for its setup, `exact->inexact` and the `c(a|d)+r` compositions its
// syntax procedures call, and `apply` for `apply-in-underlying-scheme`).
// Handlers raise the edition's typed [SchemeError], never a raw value.
// `apply` and `map` are installed as names only; [Evaluator.applyProcedure]
// intercepts them because they must apply procedure values.

package sicp.ch4

import arrow.core.raise.Raise
import sicp.runtime.Op
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VPrimitive
import sicp.runtime.VReal
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.equalv
import sicp.runtime.setCar
import sicp.runtime.setCdr
import sicp.runtime.vlist
import kotlin.math.abs

/** The driver's recording output sink: `display` and `newline` append here,
 * and the driver returns the whole transcript. No `println` anywhere. */
public class OutputSink {
    private val lines = StringBuilder()

    /** Appends one piece of display output, no line ending. */
    public fun emit(s: String) {
        lines.append(s)
    }

    /** Appends one line ending. */
    public fun newline() {
        lines.append('\n')
    }

    /** Appends one complete line; the driver's result lines use it. */
    public fun line(s: String) {
        lines.append(s).append('\n')
    }

    public override fun toString(): String = lines.toString()
}

/** The single argument of a unary primitive. */
private fun Raise<SchemeError>.one(
    name: String,
    args: List<Value>,
): Value {
    if (args.size != 1) raise(SchemeError.WrongArity(name, "1", args.size))
    return args[0]
}

/** The two arguments of a binary primitive. */
private fun Raise<SchemeError>.two(
    name: String,
    args: List<Value>,
): Pair<Value, Value> {
    if (args.size != 2) raise(SchemeError.WrongArity(name, "2", args.size))
    return Pair(args[0], args[1])
}

/** The exact-integer operand of an arithmetic primitive. */
private fun Raise<SchemeError>.intOf(
    name: String,
    v: Value,
): Long =
    when (v) {
        is VInt -> v.n
        else -> raise(SchemeError.TypeMismatch("$name: expected an integer, got $v"))
    }

/** Any numeric operand. */
private fun Raise<SchemeError>.numberOf(
    name: String,
    v: Value,
): Double =
    when (v) {
        is VInt -> v.n.toDouble()
        is VReal -> v.d
        else -> raise(SchemeError.TypeMismatch("$name: expected a number, got $v"))
    }

/** The items of a proper list argument. */
private fun Raise<SchemeError>.itemsOf(v: Value): List<Value> {
    val items = mutableListOf<Value>()
    var cursor = v
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    if (cursor !is VNil) raise(SchemeError.TypeMismatch("not a proper list: $v"))
    return items
}

/** The pair a pair-only primitive demands. */
private fun Raise<SchemeError>.pairOf(
    name: String,
    v: Value,
): VPair = v as? VPair ?: raise(SchemeError.TypeMismatch("$name of a non-pair: $v"))

/** Type-checks every operand as a number; true when all are exact. */
private fun Raise<SchemeError>.exactOperands(
    name: String,
    args: List<Value>,
): Boolean {
    for (a in args) {
        if (a !is VInt && a !is VReal) {
            raise(SchemeError.TypeMismatch("$name: expected a number, got $a"))
        }
    }
    return args.all { it is VInt }
}

/** The exact result of an arithmetic fold, overflow checked. */
private fun Raise<SchemeError>.exactFold(
    name: String,
    args: List<Value>,
    initial: Long,
    step: (Long, Long) -> Long,
): Long =
    args.fold(initial) { acc, a ->
        try {
            step(acc, intOf(name, a))
        } catch (e: ArithmeticException) {
            raise(SchemeError.Overflow)
        }
    }

/** The numbers of an arithmetic argument list. */
private fun Raise<SchemeError>.numbersOf(
    name: String,
    args: List<Value>,
): List<Double> = args.map { numberOf(name, it) }

/** The book's `+`: exact when every operand is exact, checked. */
public val addOp: Op =
    { args ->
        if (exactOperands("+", args)) {
            VInt(exactFold("+", args, 0L) { acc, x -> Math.addExact(acc, x) })
        } else {
            VReal(numbersOf("+", args).sum())
        }
    }

/** The book's `-`: negation with one argument, subtraction with more. */
public val subOp: Op =
    { args ->
        if (args.isEmpty()) raise(SchemeError.WrongArity("-", "at least 1", 0))
        if (exactOperands("-", args)) {
            val head = intOf("-", args[0])
            val rest = args.drop(1)
            val acc =
                if (args.size == 1) {
                    try {
                        Math.negateExact(head)
                    } catch (e: ArithmeticException) {
                        raise(SchemeError.Overflow)
                    }
                } else {
                    exactFold("-", rest, head) { a, x -> Math.subtractExact(a, x) }
                }
            VInt(acc)
        } else {
            val ds = numbersOf("-", args)
            VReal(if (ds.size == 1) -ds[0] else ds.drop(1).fold(ds[0]) { a, x -> a - x })
        }
    }

/** The book's `*`: exact when every operand is exact, checked. */
public val mulOp: Op =
    { args ->
        if (exactOperands("*", args)) {
            VInt(exactFold("*", args, 1L) { acc, x -> Math.multiplyExact(acc, x) })
        } else {
            VReal(numbersOf("*", args).fold(1.0) { a, x -> a * x })
        }
    }

/** The grammar's `/`: floats always; an exact zero denominator is a fault. */
public val divOp: Op =
    { args ->
        if (args.isEmpty()) raise(SchemeError.WrongArity("/", "at least 1", args.size))
        if (args.size == 1) {
            val d = numberOf("/", args[0])
            if (d == 0.0) raise(SchemeError.DivisionByZero)
            VReal(1.0 / d)
        } else {
            var acc = numberOf("/", args[0])
            for (i in 1 until args.size) {
                val d = numberOf("/", args[i])
                if (d == 0.0) raise(SchemeError.DivisionByZero)
                acc /= d
            }
            VReal(acc)
        }
    }

/** Builds one numeric comparison over mixed exact and real operands. */
private fun comparison(
    name: String,
    ok: (Double, Double) -> Boolean,
): Op =
    { args ->
        if (args.size < 2) raise(SchemeError.WrongArity(name, "at least 2", args.size))
        var previous = numberOf(name, args[0])
        var result = true
        for (i in 1 until args.size) {
            val next = numberOf(name, args[i])
            if (!ok(previous, next)) {
                result = false
                break
            }
            previous = next
        }
        VBool(result)
    }

/** The book's `remainder`: truncated, sign of the dividend. */
public val remainderOp: Op =
    { args ->
        val (a, b) = two("remainder", args)
        val x = intOf("remainder", a)
        val y = intOf("remainder", b)
        if (y == 0L) raise(SchemeError.DivisionByZero)
        VInt(x % y)
    }

/** The book's `quotient`: truncated division. */
public val quotientOp: Op =
    { args ->
        val (a, b) = two("quotient", args)
        val x = intOf("quotient", a)
        val y = intOf("quotient", b)
        if (y == 0L) raise(SchemeError.DivisionByZero)
        VInt(x / y)
    }

/** The book's `abs` over exact and real numbers. */
public val absOp: Op =
    { args ->
        val v = one("abs", args)
        when (v) {
            is VInt -> {
                if (v.n == Long.MIN_VALUE) {
                    raise(SchemeError.Overflow)
                } else {
                    VInt(abs(v.n))
                }
            }

            else -> {
                VReal(abs(numberOf("abs", v)))
            }
        }
    }

/** `display`: appends the display form to the sink, answers the unspecified value. */
public fun displayOp(out: OutputSink): Op =
    { args ->
        out.emit(displayValue(one("display", args)))
        VNil
    }

/** `newline`: appends a line ending to the sink. */
public fun newlineOp(out: OutputSink): Op =
    { args ->
        if (args.isNotEmpty()) raise(SchemeError.WrongArity("newline", "0", args.size))
        out.newline()
        VNil
    }

/** The book's `(error message irritant ...)`: the typed user-raised fault. */
public val errorOp: Op =
    { args ->
        if (args.isEmpty()) raise(SchemeError.UserRaised("error", emptyList()))
        val message =
            when (val m = args[0]) {
                is VStr -> m.s
                else -> displayValue(m)
            }
        raise(SchemeError.UserRaised(message, args.drop(1)))
    }

/** `length` over a proper list. */
public val lengthOp: Op =
    { args ->
        VInt(itemsOf(one("length", args)).size.toLong())
    }

/** `exact->inexact`: floats pass through, exact integers widen. */
public val exactToInexactOp: Op =
    { args ->
        when (val v = one("exact->inexact", args)) {
            is VInt -> VReal(v.n.toDouble())
            else -> VReal(numberOf("exact->inexact", v))
        }
    }

/** Builds one `c(a|d)+r` composition accessor over `path` outside-in. */
internal fun cxrOp(
    name: String,
    path: String,
): Op =
    { args ->
        var cursor = one(name, args)
        for (step in path.reversed()) {
            cursor =
                when (val p = cursor as? VPair) {
                    null -> raise(SchemeError.TypeMismatch("$name of a non-pair: $cursor"))
                    else -> if (step == 'a') p.car else p.cdr
                }
        }
        cursor
    }

/** The `c(a|d)+r` compositions the metacircular corpus program calls. */
internal val cxrNames: List<String> =
    run {
        val names = mutableListOf<String>()

        fun expand(
            prefix: String,
            depth: Int,
        ) {
            if (depth == 0) {
                names.add("c${prefix}r")
                return
            }
            for (s in listOf("a", "d")) expand(prefix + s, depth - 1)
        }
        for (depth in 1..4) expand("", depth)
        names
    }

/** One entry of the book's `primitive-procedures` list. */
public data class PrimitiveEntry(
    val name: String,
    val op: Op,
) {
    /** The value the global environment binds the name to. */
    public fun toValue(): Value = VPrimitive(name, op)
}

/** The book's `primitive-procedures` list, with the metacircular additions. */
public fun defaultPrimitives(out: OutputSink): List<PrimitiveEntry> =
    listOf(
        PrimitiveEntry("car") { args -> pairOf("car", one("car", args)).car },
        PrimitiveEntry("cdr") { args -> pairOf("cdr", one("cdr", args)).cdr },
        PrimitiveEntry("cons") { args ->
            val (a, b) = two("cons", args)
            cons(a, b)
        },
        PrimitiveEntry("list") { args -> vlist(args) },
        PrimitiveEntry("null?") { args -> VBool(one("null?", args) is VNil) },
        PrimitiveEntry("pair?") { args -> VBool(one("pair?", args) is VPair) },
        PrimitiveEntry("eq?") { args ->
            val (a, b) = two("eq?", args)
            VBool(a == b)
        },
        PrimitiveEntry("equal?") { args ->
            val (a, b) = two("equal?", args)
            VBool(equalv(a, b))
        },
        PrimitiveEntry("+", addOp),
        PrimitiveEntry("-", subOp),
        PrimitiveEntry("*", mulOp),
        PrimitiveEntry("/", divOp),
        PrimitiveEntry("=", comparison("=") { a, b -> a == b }),
        PrimitiveEntry("<", comparison("<") { a, b -> a < b }),
        PrimitiveEntry(">", comparison(">") { a, b -> a > b }),
        PrimitiveEntry("<=", comparison("<=") { a, b -> a <= b }),
        PrimitiveEntry(">=", comparison(">=") { a, b -> a >= b }),
        PrimitiveEntry("remainder", remainderOp),
        PrimitiveEntry("quotient", quotientOp),
        PrimitiveEntry("abs", absOp),
        PrimitiveEntry("not") { args -> VBool(one("not", args) == VBool(false)) },
        PrimitiveEntry("display", displayOp(out)),
        PrimitiveEntry("newline", newlineOp(out)),
        PrimitiveEntry("error", errorOp),
        PrimitiveEntry("number?") { args ->
            val a = one("number?", args)
            VBool(a is VInt || a is VReal)
        },
        PrimitiveEntry("symbol?") { args -> VBool(one("symbol?", args) is VSym) },
        PrimitiveEntry("string?") { args -> VBool(one("string?", args) is VStr) },
        PrimitiveEntry("set-car!") { args ->
            val (pair, v) = two("set-car!", args)
            pairOf("set-car!", pair).setCar(v)
            VNil
        },
        PrimitiveEntry("set-cdr!") { args ->
            val (pair, v) = two("set-cdr!", args)
            pairOf("set-cdr!", pair).setCdr(v)
            VNil
        },
        // The names the driver and metacircular.scm look up; these handlers
        // are unreachable -- applyProcedure intercepts both before dispatch.
        PrimitiveEntry("apply") { args -> raise(SchemeError.NotApplicable(VSym("apply"))) },
        PrimitiveEntry("map") { args -> raise(SchemeError.NotApplicable(VSym("map"))) },
        PrimitiveEntry("length", lengthOp),
        PrimitiveEntry("exact->inexact", exactToInexactOp),
    ) + cxrNames.map { name -> PrimitiveEntry(name, cxrOp(name, name.drop(1).dropLast(1))) }
