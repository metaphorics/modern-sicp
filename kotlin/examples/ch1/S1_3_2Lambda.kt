// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.3.2

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** [sum]'s real-valued overload, from section 1.3.1: [piSum] here, without naming [piTerm] and [piNext] first. */
public fun piSumWithLambdas(
    a: Double,
    b: Double,
): Double = sum({ x -> 1.0 / (x * (x + 2.0)) }, a, { x -> x + 4.0 }, b)

/** [integral] of section 1.3.1, without naming `addDx` first. */
public fun integralWithLambda(
    f: (Double) -> Double,
    a: Double,
    b: Double,
    dx: Double,
): Double = sum(f, a + dx / 2.0, { x -> x + dx }, b) * dx

/** Naming a lambda with [val] and writing a [fun] both bind the same kind of value. */
public val plus4Val: (Long) -> Long = { x -> x + 4L }

public fun plus4Fun(x: Long): Long = x + 4L

/** A lambda literal is itself callable, in operator position, without ever being named. */
public fun iifeSum(): Long = ({ x: Long, y: Long, z: Long -> x + y + z * z })(1L, 2L, 3L)

/**
 * The auxiliary-procedure way to bind local variables: a local [fun] used once, right below
 * its only caller.
 */
public fun fWithHelper(
    x: Double,
    y: Double,
): Double {
    fun fHelper(
        a: Double,
        b: Double,
    ): Double = x * a * a + y * b + a * b
    return fHelper(1.0 + x * y, 1.0 - y)
}

/** Local [val] bindings play the role of the book's `let`: they scope to the rest of the block. */
public fun f(
    x: Double,
    y: Double,
): Double {
    val a = 1.0 + x * y
    val b = 1.0 - y
    return x * a * a + y * b + a * b
}

/**
 * `let` binds as locally as possible to where a name is used: with the outer `x` at 5, `run`
 * opens a fresh scope in which `x` shadows the outer binding only for the block's own body.
 */
public fun letScopesLocally(): Long {
    val x = 5L
    return run {
        val x = 3L
        x + x * 10L
    } + x
}

/**
 * `let`'s initializers are computed outside the new scope: with the outer `x` at 2, the `y`
 * binding sees the *outer* `x`, computed before the inner `x` is declared.
 */
public fun letInitializersSeeTheOuterScope(): Long {
    val x = 2L
    val y = x + 2L
    return run {
        val x = 3L
        x * y
    }
}

public class S1_3_2LambdaTest :
    FunSpec({
        test("pi-sum written with lambdas matches the version with named auxiliaries") {
            8.0 * piSumWithLambdas(1.0, 1000.0) shouldBe 3.139592655589783
        }
        test("integral written with a lambda matches the version with a named add-dx") {
            integralWithLambda({ x -> x * x * x }, 0.0, 1.0, 0.01) shouldBe 0.24998750000000042
        }
        test("plus4 as val and as fun compute the same thing") {
            plus4Val(3L) shouldBe 7L
            plus4Fun(3L) shouldBe 7L
        }
        test("a lambda literal can be the operator of a call") {
            iifeSum() shouldBe 12L
        }
        test("the helper-procedure and let-shaped versions of f agree") {
            fWithHelper(2.0, 3.0) shouldBe f(2.0, 3.0)
        }
        test("let allows binding as locally as possible to where a name is used") {
            letScopesLocally() shouldBe 38L
        }
        test("let's initializers are computed outside the new bindings") {
            letInitializersSeeTheOuterScope() shouldBe 12L
        }
    })
