// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.3.4

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private const val DX = 0.00001

/** The derivative of [g], to the precision [DX] allows: a function in, a function out. */
public fun deriv(g: (Double) -> Double): (Double) -> Double = { x -> (g(x + DX) - g(x)) / DX }

public fun newtonTransform(g: (Double) -> Double): (Double) -> Double = { x -> x - g(x) / deriv(g)(x) }

public fun newtonsMethod(
    g: (Double) -> Double,
    guess: Double,
): Double = fixedPoint(newtonTransform(g), guess)

/** Newton's method finds a zero of `y \mapsto y^2 - x`, starting from a guess of 1. */
public fun sqrtByNewtonsMethod(x: Double): Double = newtonsMethod({ y -> square(y) - x }, 1.0)

/** [g] computes a function, [transform] transforms [g]; the result is a fixed point of the transformed function. */
public fun fixedPointOfTransform(
    g: (Double) -> Double,
    transform: ((Double) -> Double) -> (Double) -> Double,
    guess: Double,
): Double = fixedPoint(transform(g), guess)

/** The section's first square-root computation, recast as a fixed point of an average-damped transform. */
public fun sqrtByAverageDampOfTransform(x: Double): Double = fixedPointOfTransform({ y -> x / y }, ::averageDamp, 1.0)

/** The section's second square-root computation, recast as a fixed point of a Newton-transformed function. */
public fun sqrtByNewtonOfTransform(x: Double): Double = fixedPointOfTransform({ y -> square(y) - x }, ::newtonTransform, 1.0)

public class S1_3_4ReturnedValuesTest :
    FunSpec({
        test("deriv approximates the derivative of cube at 5, whose exact value is 75") {
            deriv(::cube)(5.0) shouldBe 75.00014999664018
        }
        test("Newton's method and average damping both find the square root of 2") {
            sqrtByNewtonsMethod(2.0) shouldBe 1.4142135623822438
            sqrtByAverageDamp(2.0) shouldBe 1.4142135623746899
        }
        test("both fixed-point-of-transform computations of sqrt agree with the section's earlier ones") {
            sqrtByAverageDampOfTransform(2.0) shouldBe sqrtByAverageDamp(2.0)
            sqrtByNewtonOfTransform(2.0) shouldBe sqrtByNewtonsMethod(2.0)
        }
    })
