// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.3.3

package sicp.ch1.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.abs
import kotlin.math.cos
import kotlin.math.sin

private fun closeEnough(
    x: Double,
    y: Double,
): Boolean = abs(x - y) < 0.001

/** Assumes [f] is negative at [negPoint] and positive at [posPoint]; [average] is section 1.1.7's. */
public fun search(
    f: (Double) -> Double,
    negPoint: Double,
    posPoint: Double,
): Double {
    val midpoint = average(negPoint, posPoint)
    if (closeEnough(negPoint, posPoint)) return midpoint
    val testValue = f(midpoint)
    return when {
        testValue > 0.0 -> search(f, negPoint, midpoint)
        testValue < 0.0 -> search(f, midpoint, posPoint)
        else -> midpoint
    }
}

/** The domain's one error: [a] and [b] both land on the same side of the root. */
public sealed interface HalfIntervalError {
    public data class SameSign(
        val a: Double,
        val b: Double,
    ) : HalfIntervalError
}

context(r: Raise<HalfIntervalError>)
public fun halfIntervalMethod(
    f: (Double) -> Double,
    a: Double,
    b: Double,
): Double {
    val aValue = f(a)
    val bValue = f(b)
    return when {
        aValue < 0.0 && bValue > 0.0 -> search(f, a, b)
        bValue < 0.0 && aValue > 0.0 -> search(f, b, a)
        else -> r.raise(HalfIntervalError.SameSign(a, b))
    }
}

private const val TOLERANCE = 0.00001

private fun closeEnoughFixedPoint(
    v1: Double,
    v2: Double,
): Boolean = abs(v1 - v2) < TOLERANCE

public fun fixedPoint(
    f: (Double) -> Double,
    firstGuess: Double,
): Double {
    tailrec fun tryGuess(guess: Double): Double {
        val next = f(guess)
        return if (closeEnoughFixedPoint(guess, next)) next else tryGuess(next)
    }
    return tryGuess(firstGuess)
}

/**
 * The naive fixed point of `y \mapsto x / y`: defined for the record, never called from a
 * test, because the sequence it generates oscillates and never satisfies [closeEnoughFixedPoint].
 */
public fun sqrtByNaiveFixedPoint(x: Double): Double = fixedPoint({ y -> x / y }, 1.0)

public fun averageDamp(f: (Double) -> Double): (Double) -> Double = { x -> average(x, f(x)) }

/** Average damping tames the oscillation [sqrtByNaiveFixedPoint] falls into. */
public fun sqrtByAverageDamp(x: Double): Double = fixedPoint(averageDamp { y -> x / y }, 1.0)

public class S1_3_3GeneralMethodsTest :
    FunSpec({
        test("half-interval-method finds a root of sin between 2 and 4") {
            val result = either<HalfIntervalError, Double> { halfIntervalMethod(::sin, 2.0, 4.0) }
            result shouldBe Either.Right(3.14111328125)
        }
        test("half-interval-method finds a root of a cubic between 1 and 2") {
            val cubic = { x: Double -> x * x * x - 2.0 * x - 3.0 }
            val result = either<HalfIntervalError, Double> { halfIntervalMethod(cubic, 1.0, 2.0) }
            result shouldBe Either.Right(1.89306640625)
        }
        test("half-interval-method signals its one error when the endpoints agree in sign") {
            val result = either<HalfIntervalError, Double> { halfIntervalMethod(::sin, 0.1, 0.2) }
            result shouldBe Either.Left(HalfIntervalError.SameSign(0.1, 0.2))
        }
        test("fixed-point locates a fixed point of cosine") {
            fixedPoint(::cos, 1.0) shouldBe 0.7390822985224024
        }
        test("fixed-point locates a fixed point of y -> sin y + cos y") {
            fixedPoint({ y -> sin(y) + cos(y) }, 1.0) shouldBe 1.2587315962971173
        }
        test("average-damp of square applied to 10 averages 10 and 100") {
            averageDamp(::square)(10.0) shouldBe 55.0
        }
        test("sqrt via average-damped fixed-point search matches the section 1.1.7 procedure") {
            sqrtByAverageDamp(2.0) shouldBe 1.4142135623746899
        }
    })
