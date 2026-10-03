// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.5: the generic arithmetic tower

package sicp.ch2.exercises

import arrow.core.raise.Raise
import java.math.BigInteger
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.hypot
import kotlin.math.sin
import kotlin.reflect.KClass

/**
 * The section's numeric tower: a sealed hierarchy whose runtime type plays
 * the book's type tag. The book attaches tags because every Lisp datum is
 * a pair; here the value carries its level in its type, so `attach-tag`
 * and `contents` disappear and the dispatch key is the list of type names
 * the arguments present.
 */
public sealed interface Num

/** The integer level: the host's primitive `Long`, exact while it fits. */
@JvmInline
public value class ZLong(
    public val n: Long,
) : Num

/** Where checked `Long` arithmetic runs past its width, the integer
 * package promotes the result to [BigZ] rather than wrapping around. */
public data class BigZ(
    public val n: BigInteger,
) : Num

/** The exact rational level: `BigInteger` components (a `Long` pair
 * overflows during multiplication before reduction), positive denominator,
 * always gcd-reduced. */
public data class QRat(
    public val num: BigInteger,
    public val den: BigInteger,
) : Num

/** The inexact real level. */
@JvmInline
public value class Real(
    public val d: Double,
) : Num

/** The complex level: the outer tag of Figure 2.24, wrapping the
 * representation the inner dispatch then works on. */
public data class Complex(
    public val rep: ComplexRep,
) : Num

/** The representation level inside a [Complex]: the second layer of the
 * two-level tag system. */
public sealed interface ComplexRep

/** Ben's rectangular representation. */
public data class Rect(
    public val re: Double,
    public val im: Double,
) : ComplexRep

/** Alyssa's polar representation. */
public data class Polar(
    public val mag: Double,
    public val ang: Double,
) : ComplexRep

/** The section's own domain errors, raised through the tower's `Raise`
 * exactly as the book's `(error "No method for these types" ...)` calls. */
public sealed interface GenError {
    /** A lookup missed every package installed for these tags. */
    public data class NoMethod(
        public val op: String,
        public val tags: List<String>,
    ) : GenError {
        public override fun toString(): String = "No method for these types: $op $tags"
    }

    /** A handler met arguments of the wrong number or shape for its slot. */
    public data class BadArgs(
        public val op: String,
        public val detail: String,
    ) : GenError

    /** A divisor was zero. */
    public data class DivideByZero(
        public val op: String,
    ) : GenError

    /** Two polys were combined under different variables. */
    public data class NotSameVariable(
        public val op: String,
        public val first: String,
        public val second: String,
    ) : GenError
}

/** A handler an installer puts into the table: applied to the argument
 * list, it returns one tower value or raises a [GenError]. */
public typealias NumOp = Raise<GenError>.(List<Num>) -> Num

/**
 * The section's operation-and-type table: handlers keyed by operation name
 * and the list of type tags. Dispatch is data, not `when` arms, because
 * this section keeps installing packages that did not exist when the
 * generic front-ends compiled.
 */
public class NumTable {
    private val entries: MutableMap<String, MutableMap<List<String>, NumOp>> = HashMap()

    /** The book's `put`: installs `handler` under `(op, tags)`, overwriting
     * an earlier install of exactly that pair. */
    public fun put(
        op: String,
        tags: List<String>,
        handler: NumOp,
    ) {
        entries.getOrPut(op) { HashMap() }[tags] = handler
    }

    /** The book's `get`: the handler under `(op, tags)`, or null on a
     * miss -- never a false-ish sentinel. */
    public fun get(
        op: String,
        tags: List<String>,
    ): NumOp? = entries[op]?.get(tags)
}

/**
 * The coercion registry of 2.5.2: a second table, keyed by a pair of
 * runtime types, holding the functions that transform one level into
 * another.
 */
public class CoercionTable {
    private val entries: MutableMap<Pair<KClass<*>, KClass<*>>, (Num) -> Num> = HashMap()

    /** The book's `put-coercion`. */
    public fun putCoercion(
        from: KClass<*>,
        to: KClass<*>,
        f: (Num) -> Num,
    ) {
        entries[from to to] = f
    }

    /** The book's `get-coercion`. */
    public fun getCoercion(
        from: KClass<*>,
        to: KClass<*>,
    ): ((Num) -> Num)? = entries[from to to]
}

/** Builds an exact rational from `Long` parts, the constructor the
 * section's examples and exercises read naturally. */
public fun qr(
    n: Long,
    d: Long,
): QRat = QRat(n.toBigInteger(), d.toBigInteger())

/** The dispatch tag a value presents: the name of its own level. */
public fun typeTagOf(z: Num): String =
    when (z) {
        is ZLong -> "integer"
        is BigZ -> "bigint"
        is QRat -> "rational"
        is Real -> "real"
        is Complex -> "complex"
        is Poly -> "polynomial"
        is Rat -> "rational-function"
    }

/**
 * The book's `apply-generic` as 2.5.1 presents it: look the operation up
 * under the argument tags and apply the handler to the arguments -- whose
 * tags are their own types, so nothing is stripped on the way down.
 */
context(r: Raise<GenError>)
public fun applyGeneric(
    table: NumTable,
    op: String,
    args: List<Num>,
): Num {
    val tags = args.map { typeTagOf(it) }
    val handler = table.get(op, tags) ?: r.raise(GenError.NoMethod(op, tags))
    return r.handler(args)
}

/**
 * The 2.5.2 revision: on a two-argument miss, try coercing each argument
 * into the other's type and retry. This is the book's version as written,
 * which happily coerces two arguments of the same type to each other;
 * exercise 2.81 asks what that costs.
 */
context(r: Raise<GenError>)
public fun applyGenericWithCoercion(
    table: NumTable,
    coercions: CoercionTable,
    op: String,
    args: List<Num>,
): Num {
    val tags = args.map { typeTagOf(it) }
    table.get(op, tags)?.let { return it.invoke(r, args) }
    if (args.size == 2) {
        val t1toT2 = coercions.getCoercion(args[0]::class, args[1]::class)
        val t2toT1 = coercions.getCoercion(args[1]::class, args[0]::class)
        when {
            t1toT2 != null -> {
                return applyGenericWithCoercion(table, coercions, op, listOf(t1toT2(args[0]), args[1]))
            }

            t2toT1 != null -> {
                return applyGenericWithCoercion(table, coercions, op, listOf(args[0], t2toT1(args[1])))
            }

            else -> {}
        }
    }
    return r.raise(GenError.NoMethod(op, tags))
}

/** The rectangular and polar internals of 2.4.1, presented at the
 * representation level the outer `complex` tag directs to. The set of
 * representations is closed (both predate this section), so the internal
 * procedures dispatch with an exhaustive `when` rather than the table. */
internal fun repRealPart(z: ComplexRep): Double =
    when (z) {
        is Rect -> z.re
        is Polar -> z.mag * cos(z.ang)
    }

internal fun repImagPart(z: ComplexRep): Double =
    when (z) {
        is Rect -> z.im
        is Polar -> z.mag * sin(z.ang)
    }

internal fun repMagnitude(z: ComplexRep): Double =
    when (z) {
        is Rect -> hypot(z.re, z.im)
        is Polar -> z.mag
    }

internal fun repAngle(z: ComplexRep): Double =
    when (z) {
        is Rect -> atan2(z.im, z.re)
        is Polar -> z.ang
    }

internal fun repMakeFromRealImag(
    x: Double,
    y: Double,
): ComplexRep = Rect(x, y)

internal fun repMakeFromMagAng(
    mag: Double,
    ang: Double,
): ComplexRep = Polar(mag, ang)

internal fun Raise<GenError>.twoNums(
    op: String,
    args: List<Num>,
): List<Num> {
    if (args.size != 2) raise(GenError.BadArgs(op, "expected 2 arguments, got ${args.size}"))
    return args
}

private fun Raise<GenError>.twoLongs(args: List<Num>): Pair<Long, Long> {
    val (a, b) = twoNums("integer arithmetic", args)
    if (a !is ZLong || b !is ZLong) raise(GenError.BadArgs("integer arithmetic", "expected two integers"))
    return a.n to b.n
}

/**
 * The representation table inside the complex level: the rectangular and
 * polar packages install here, and the complex level's procedures reach
 * them through it -- the export path Figure 2.24 pictures. Its handlers
 * carry [ComplexRep] payloads, so this second registry sits beside the
 * tower's [NumTable] rather than inside it.
 */
public class RepTable {
    private val entries: MutableMap<String, MutableMap<String, (ComplexRep) -> Num>> = HashMap()

    /** Installs `handler` under `(op, tag)`. */
    public fun put(
        op: String,
        tag: String,
        handler: (ComplexRep) -> Num,
    ) {
        entries.getOrPut(op) { HashMap() }[tag] = handler
    }

    /** The handler under `(op, tag)`, or null on a miss. */
    public fun get(
        op: String,
        tag: String,
    ): ((ComplexRep) -> Num)? = entries[op]?.get(tag)
}

/** The dispatch tag of a representation. */
public fun repTagOf(rep: ComplexRep): String =
    when (rep) {
        is Rect -> "rectangular"
        is Polar -> "polar"
    }

/** The book's inner `apply-generic`: one representation-level lookup and
 * application, the dispatch Figure 2.24's inner arrow stands for. The
 * representation packages answer with tower values (reals), the shapes
 * the complex level's own arithmetic consumes. */
context(r: Raise<GenError>)
public fun applyRepGeneric(
    reps: RepTable,
    op: String,
    rep: ComplexRep,
): Num {
    val tag = repTagOf(rep)
    val handler = reps.get(op, tag) ?: r.raise(GenError.NoMethod(op, listOf(tag)))
    return handler(rep)
}

private fun Raise<GenError>.twoComplex(args: List<Num>): Pair<Complex, Complex> {
    val (a, b) = twoNums("complex arithmetic", args)
    if (a !is Complex || b !is Complex) raise(GenError.BadArgs("complex arithmetic", "expected two complex numbers"))
    return a to b
}

/**
 * The complex package of 2.5.1: the internal procedures are the
 * representation-level operations above, exported to this level and, from
 * here, to the outside world through the table.
 */
public fun installComplexPackage(table: NumTable) {
    table.put("add", listOf("complex", "complex")) { args ->
        val (a, b) = twoComplex(args)
        Complex(repMakeFromRealImag(repRealPart(a.rep) + repRealPart(b.rep), repImagPart(a.rep) + repImagPart(b.rep)))
    }
    table.put("sub", listOf("complex", "complex")) { args ->
        val (a, b) = twoComplex(args)
        Complex(repMakeFromRealImag(repRealPart(a.rep) - repRealPart(b.rep), repImagPart(a.rep) - repImagPart(b.rep)))
    }
    table.put("mul", listOf("complex", "complex")) { args ->
        val (a, b) = twoComplex(args)
        Complex(repMakeFromMagAng(repMagnitude(a.rep) * repMagnitude(b.rep), repAngle(a.rep) + repAngle(b.rep)))
    }
    table.put("div", listOf("complex", "complex")) { args ->
        val (a, b) = twoComplex(args)
        Complex(repMakeFromMagAng(repMagnitude(a.rep) / repMagnitude(b.rep), repAngle(a.rep) - repAngle(b.rep)))
    }
    table.put("make-from-real-imag", listOf("complex")) { args ->
        val (x, y) = twoNums("make-from-real-imag", args)
        if (x !is Real || y !is Real) raise(GenError.BadArgs("make-from-real-imag", "expected two reals"))
        Complex(repMakeFromRealImag(x.d, y.d))
    }
    table.put("make-from-mag-ang", listOf("complex")) { args ->
        val (m, a) = twoNums("make-from-mag-ang", args)
        if (m !is Real || a !is Real) raise(GenError.BadArgs("make-from-mag-ang", "expected two reals"))
        Complex(repMakeFromMagAng(m.d, a.d))
    }
}

/**
 * The integer package: the host's own checked arithmetic (`Math.*Exact`),
 * promoted to [BigZ] where the `Long` width would be exceeded.
 */
public fun installIntegerPackage(table: NumTable) {
    table.put("add", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            ZLong(Math.addExact(a, b))
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().add(b.toBigInteger()))
        }
    }
    table.put("sub", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            ZLong(Math.subtractExact(a, b))
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().subtract(b.toBigInteger()))
        }
    }
    table.put("mul", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            ZLong(Math.multiplyExact(a, b))
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().multiply(b.toBigInteger()))
        }
    }
    table.put("div", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        if (b == 0L) raise(GenError.DivideByZero("integer div"))
        try {
            ZLong(Math.divideExact(a, b))
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().divide(b.toBigInteger()))
        }
    }
    table.put("make", listOf("integer")) { args ->
        val (a, _) = twoNums("make", args)
        a
    }
}

/**
 * The rational package: the code of 2.1.1 carried over unchanged in
 * structure, with `BigInteger` components because `Long` numerators and
 * denominators overflow during multiplication before reduction.
 */
public fun installRationalPackage(table: NumTable) {
    fun Raise<GenError>.makeRat(
        n: BigInteger,
        d: BigInteger,
    ): QRat {
        if (d.signum() == 0) raise(GenError.DivideByZero("make-rational"))
        val g = n.abs().gcd(d.abs())
        val pair = if (d.signum() < 0) n.negate() to d.negate() else n to d
        return QRat(pair.first.divide(g), pair.second.divide(g))
    }

    fun Raise<GenError>.twoRats(args: List<Num>): Pair<QRat, QRat> {
        val (a, b) = twoNums("rational arithmetic", args)
        if (a !is QRat || b !is QRat) raise(GenError.BadArgs("rational arithmetic", "expected two rationals"))
        return a to b
    }
    table.put("add", listOf("rational", "rational")) { args ->
        val (a, b) = twoRats(args)
        makeRat(a.num.multiply(b.den).add(b.num.multiply(a.den)), a.den.multiply(b.den))
    }
    table.put("sub", listOf("rational", "rational")) { args ->
        val (a, b) = twoRats(args)
        makeRat(a.num.multiply(b.den).subtract(b.num.multiply(a.den)), a.den.multiply(b.den))
    }
    table.put("mul", listOf("rational", "rational")) { args ->
        val (a, b) = twoRats(args)
        makeRat(a.num.multiply(b.num), a.den.multiply(b.den))
    }
    table.put("div", listOf("rational", "rational")) { args ->
        val (a, b) = twoRats(args)
        makeRat(a.num.multiply(b.den), a.den.multiply(b.num))
    }
    table.put("make", listOf("rational")) { args ->
        val (n, d) = twoNums("make", args)
        if (n !is ZLong || d !is ZLong) raise(GenError.BadArgs("make", "expected two integers"))
        makeRat(n.n.toBigInteger(), d.n.toBigInteger())
    }
}

/** Install the equality and zero operations expected by the tower lessons. */
public fun installTowerPredicates(table: NumTable) {
    fun bool(b: Boolean): Num = if (b) ZLong(1) else ZLong(0)
    table.put("equ?", listOf("integer", "integer")) { args ->
        val (a, b) = twoNums("equ?", args)
        bool(a is ZLong && b is ZLong && a.n == b.n)
    }
    table.put("equ?", listOf("bigint", "bigint")) { args ->
        val (a, b) = twoNums("equ?", args)
        bool(a is BigZ && b is BigZ && a.n == b.n)
    }
    table.put("equ?", listOf("rational", "rational")) { args ->
        val (a, b) = twoNums("equ?", args)
        bool(a is QRat && b is QRat && a.num == b.num && a.den == b.den)
    }
    table.put("equ?", listOf("real", "real")) { args ->
        val (a, b) = twoNums("equ?", args)
        bool(a is Real && b is Real && a.d == b.d)
    }
    table.put("equ?", listOf("complex", "complex")) { args ->
        val (a, b) = twoNums("equ?", args)
        bool(
            a is Complex && b is Complex &&
                repRealPart(a.rep) == repRealPart(b.rep) && repImagPart(a.rep) == repImagPart(b.rep),
        )
    }
    table.put("=zero?", listOf("integer")) { args ->
        val z = args.single()
        bool(z is ZLong && z.n == 0L)
    }
    table.put("=zero?", listOf("bigint")) { args ->
        val z = args.single()
        bool(z is BigZ && z.n.signum() == 0)
    }
    table.put("=zero?", listOf("rational")) { args ->
        val z = args.single()
        bool(z is QRat && z.num.signum() == 0)
    }
    table.put("=zero?", listOf("real")) { args ->
        val z = args.single()
        bool(z is Real && z.d == 0.0)
    }
    table.put("=zero?", listOf("complex")) { args ->
        val z = args.single()
        bool(z is Complex && repRealPart(z.rep) == 0.0 && repImagPart(z.rep) == 0.0)
    }
}

/** Installs the whole generic arithmetic package: the three number
 * packages plus the predicates of exercises 2.79 and 2.80, which the
 * extended exercises and the polynomial package take as present. */
public fun installGenericArithmetic(table: NumTable) {
    installIntegerPackage(table)
    installRationalPackage(table)
    installComplexPackage(table)
    installTowerPredicates(table)
}

/** The book's generic `add`. */
context(r: Raise<GenError>)
public fun add(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGeneric(table, "add", listOf(x, y))

/** The book's generic `sub`. */
context(r: Raise<GenError>)
public fun sub(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGeneric(table, "sub", listOf(x, y))

/** The book's generic `mul`. */
context(r: Raise<GenError>)
public fun mul(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGeneric(table, "mul", listOf(x, y))

/** The book's generic `div`. */
context(r: Raise<GenError>)
public fun div(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGeneric(table, "div", listOf(x, y))

/** Generic numeric equality through the operation table (exercise 2.79). */
context(r: Raise<GenError>)
public fun equv(
    table: NumTable,
    x: Num,
    y: Num,
): Boolean = applyGeneric(table, "equ?", listOf(x, y)) == ZLong(1)

/** A generic numeric zero test through the operation table (exercise 2.80). */
context(r: Raise<GenError>)
public fun isZeroG(
    table: NumTable,
    x: Num,
): Boolean = applyGeneric(table, "=zero?", listOf(x)) == ZLong(1)

/** Construct an integer in the numeric tower through the installed factory. */
context(r: Raise<GenError>)
public fun makeInteger(
    table: NumTable,
    n: Long,
): Num {
    val make = table.get("make", listOf("integer")) ?: r.raise(GenError.NoMethod("make", listOf("integer")))
    return make.invoke(r, listOf(ZLong(n)))
}

/** Construct a rational number in the numeric tower through its installed factory. */
context(r: Raise<GenError>)
public fun makeRational(
    table: NumTable,
    n: Long,
    d: Long,
): Num {
    val make = table.get("make", listOf("rational")) ?: r.raise(GenError.NoMethod("make", listOf("rational")))
    return make.invoke(r, listOf(ZLong(n), ZLong(d)))
}

/** Constructs a complex number from real and imaginary parts through the
 * table, as the book's `make-complex-from-real-imag` does. */
context(r: Raise<GenError>)
public fun makeComplexFromRealImag(
    table: NumTable,
    x: Double,
    y: Double,
): Num {
    val make =
        table.get("make-from-real-imag", listOf("complex"))
            ?: r.raise(GenError.NoMethod("make-from-real-imag", listOf("complex")))
    return make.invoke(r, listOf(Real(x), Real(y)))
}

/** Constructs a complex number from magnitude and angle through the
 * table, as the book's `make-complex-from-mag-ang` does. */
context(r: Raise<GenError>)
public fun makeComplexFromMagAng(
    table: NumTable,
    mag: Double,
    ang: Double,
): Num {
    val make =
        table.get("make-from-mag-ang", listOf("complex"))
            ?: r.raise(GenError.NoMethod("make-from-mag-ang", listOf("complex")))
    return make.invoke(r, listOf(Real(mag), Real(ang)))
}

/** The tower's printed form, the shape the section's interactions show. */
public fun show(z: Num): String =
    when (z) {
        is ZLong -> z.n.toString()
        is BigZ -> z.n.toString()
        is QRat -> "${z.num}/${z.den}"
        is Real -> z.d.toString()
        is Complex -> complexShow(z.rep)
        is Poly -> polyShow(z)
        is Rat -> "(${show(z.num)})/(${show(z.den)})"
    }

private fun complexShow(rep: ComplexRep): String =
    when (rep) {
        is Rect -> "${rep.re}+${rep.im}i"
        is Polar -> "${rep.mag}@${rep.ang}"
    }

private fun polyShow(p: Poly): String =
    if (p.terms.isEmpty()) {
        "0"
    } else {
        p.terms.mapIndexed { i, t -> termShow(t, p.v, i == 0) }.joinToString(" ") + " in ${p.v}"
    }

private fun termShow(
    t: Term,
    v: String,
    first: Boolean,
): String {
    val sign = if (first) "" else "+ "
    val raw = show(t.coeff)
    val coef = if (t.coeff is ZLong && t.coeff.n < 0) "(${t.coeff.n})" else raw
    return when {
        t.order == 0 -> sign + coef
        t.order == 1 -> "$sign$coef*$v"
        else -> "$sign$coef*$v^${t.order}"
    }
}
