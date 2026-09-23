// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.8

package sicp.ch1.exercises

import kotlin.math.abs

private fun cube(guess: Double): Double = guess * guess * guess

/** Newton's improvement for cube roots: (x/y^2 + 2y) / 3. */
private fun improveCube(
    guess: Double,
    x: Double,
): Double = (x / (guess * guess) + 2.0 * guess) / 3.0

private fun goodEnoughCube(
    guess: Double,
    x: Double,
): Boolean = abs(cube(guess) - x) < 0.001

public tailrec fun cubeRootIter(
    guess: Double,
    x: Double,
): Double = if (goodEnoughCube(guess, x)) guess else cubeRootIter(improveCube(guess, x), x)

/** The cube-root function, analogous to the square-root function of 1.1.7. */
public fun ex_1_08(x: Double): Double = cubeRootIter(1.0, x)
