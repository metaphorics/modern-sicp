// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.3.1

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The section's running example: no particular number, a method for any number. */
public fun cube(x: Long): Long = x * x * x

/** The common template, its slots turned into formal parameters. */
public fun sum(
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long = if (a > b) 0L else term(a) + sum(term, next(a), next, b)

public fun inc(n: Long): Long = n + 1L

public fun sumCubes(
    a: Long,
    b: Long,
): Long = sum(::cube, a, ::inc, b)

public fun identity(x: Long): Long = x

public fun sumIntegers(
    a: Long,
    b: Long,
): Long = sum(::identity, a, ::inc, b)

/** [piTerm] and [piNext] are unlikely to be useful for any other purpose, so block structure hides them. */
public fun piSum(
    a: Long,
    b: Long,
): Double {
    fun piTerm(x: Long): Double = 1.0 / (x * (x + 2L))

    fun piNext(x: Long): Long = x + 4L
    return if (a > b) 0.0 else piTerm(a) + piSum(piNext(a), b)
}

/** The real-valued overload of [sum], picked by the compiler from the types of its arguments. */
public fun sum(
    term: (Double) -> Double,
    a: Double,
    next: (Double) -> Double,
    b: Double,
): Double = if (a > b) 0.0 else term(a) + sum(term, next(a), next, b)

public fun cube(x: Double): Double = x * x * x

/** The definite integral of [f] from [a] to [b], approximated with rectangles of width [dx]. */
public fun integral(
    f: (Double) -> Double,
    a: Double,
    b: Double,
    dx: Double,
): Double {
    fun addDx(x: Double): Double = x + dx
    return sum(f, a + dx / 2.0, ::addDx, b) * dx
}

public class S1_3_1ProceduresAsArgumentsTest :
    FunSpec({
        test("sum built from cube and identity reproduces the specialized procedures") {
            sumCubes(1L, 10L) shouldBe 3025L
            sumIntegers(1L, 10L) shouldBe 55L
        }
        test("pi-sum approximates pi, slowly") {
            8.0 * piSum(1L, 1000L) shouldBe 3.139592655589783
        }
        test("integral approximates the exact 1/4 more closely as dx shrinks") {
            integral(::cube, 0.0, 1.0, 0.01) shouldBe 0.24998750000000042
            integral(::cube, 0.0, 1.0, 0.001) shouldBe 0.249999875000001
        }
    })
