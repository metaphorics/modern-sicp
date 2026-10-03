// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.78

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Recognize both the tower's numeric variants and Kotlin's unwrapped
 * `Long` and `Double` values. Native inputs keep their own host types rather
 * than passing through an artificial wrapper datum.
 */
public fun typeTagOfAny(v: Any): String? =
    when (v) {
        is Num -> typeTagOf(v)
        is Long -> "integer"
        is Double -> "real"
        else -> null
    }

/** A bare operation's handler: applied to raw host values, it returns one. */
public typealias BareOp = Raise<GenError>.(List<Any>) -> Any

/** The operation table the bare numbers dispatch through. */
public class BareTable {
    private val entries: MutableMap<String, MutableMap<List<String>, BareOp>> = HashMap()

    /** Installs `handler` under `(op, tags)`. */
    public fun put(
        op: String,
        tags: List<String>,
        handler: BareOp,
    ) {
        entries.getOrPut(op) { HashMap() }[tags] = handler
    }

    /** The handler under `(op, tags)`, or null on a miss. */
    public fun get(
        op: String,
        tags: List<String>,
    ): BareOp? = entries[op]?.get(tags)
}

/** The revised `apply-generic` over bare data: tags read off the values
 * themselves, nothing attached, nothing stripped. */
context(r: Raise<GenError>)
public fun applyGenericBare(
    table: BareTable,
    op: String,
    args: List<Any>,
): Any {
    val tags = args.map { v -> typeTagOfAny(v) ?: r.raise(GenError.BadArgs(op, "no level for $v")) }
    val handler = table.get(op, tags) ?: r.raise(GenError.NoMethod(op, tags))
    return r.handler(args)
}

/** The bare integer package: ordinary numbers are the host's own `Long`s,
 * a `ZLong` operand is accepted for compatibility, and an overflow still
 * promotes to [BigZ]. */
public fun installBareIntegerPackage(table: BareTable) {
    fun Raise<GenError>.twoLongs(args: List<Any>): Pair<Long, Long> {
        if (args.size != 2) raise(GenError.BadArgs("integer arithmetic", "expected 2 arguments"))
        val ok = args.all { it is Long || it is ZLong }
        if (!ok) raise(GenError.BadArgs("integer arithmetic", "expected two integers"))
        val bareLong = { v: Any -> if (v is Long) v else (v as ZLong).n }
        return bareLong(args[0]) to bareLong(args[1])
    }
    table.put("add", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            Math.addExact(a, b)
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().add(b.toBigInteger()))
        }
    }
    table.put("sub", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            Math.subtractExact(a, b)
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().subtract(b.toBigInteger()))
        }
    }
    table.put("mul", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        try {
            Math.multiplyExact(a, b)
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().multiply(b.toBigInteger()))
        }
    }
    table.put("div", listOf("integer", "integer")) { args ->
        val (a, b) = twoLongs(args)
        if (b == 0L) raise(GenError.DivideByZero("integer div"))
        try {
            Math.divideExact(a, b)
        } catch (e: ArithmeticException) {
            BigZ(a.toBigInteger().divide(b.toBigInteger()))
        }
    }
}

/** Runs the book's checks over bare numbers: arithmetic on bare `Long`s
 * stays bare, mixed bare and wrapped operands compute, and an overflow
 * promotes. Returns whether all three hold. */
public fun ex_2_78(): Boolean {
    val table = BareTable()
    installBareIntegerPackage(table)
    val sum = arrow.core.raise.either { applyGenericBare(table, "add", listOf(3L, 4L)) }
    val mixed = arrow.core.raise.either { applyGenericBare(table, "mul", listOf(3L, ZLong(4))) }
    val big =
        arrow.core.raise.either {
            applyGenericBare(table, "add", listOf(Long.MAX_VALUE, 1L))
        }
    val promoted =
        java.math.BigInteger
            .valueOf(Long.MAX_VALUE)
            .add(java.math.BigInteger.ONE)
    return sum.getOrNull() == 7L && mixed.getOrNull() == 12L && big.getOrNull() == BigZ(promoted)
}
