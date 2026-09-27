// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.2 (head) and 3.2.1, the rules for evaluation

package sicp.ch3.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Env
import sicp.runtime.VInt
import sicp.runtime.Value

/**
 * The section's `square` twice over. The `fun` declaration is the everyday
 * spelling; the `val` below binds the same body as a function value, the
 * book's underlying lambda, under a fresh name because Kotlin refuses a
 * second `square` in one scope.
 */
public fun square(x: Long): Long = x * x

public val squareFn: (Long) -> Long = { x -> x * x }

/**
 * Figure 3.1 as real environments: frame I is global, frames II and III
 * extend it, and A, B, C, D name environments into that structure. A
 * lookup walks outwards until the name is bound, so `x` found from A is
 * II's 7 and from B is I's 3.
 */
public object Figure31 {
    public val frameI: Env = Env.global()

    public val frameII: Env = Env.child(frameI)

    public val frameIII: Env = Env.child(frameI)

    init {
        frameI.define("x", VInt(3))
        frameI.define("y", VInt(5))
        frameII.define("z", VInt(6))
        frameII.define("x", VInt(7))
        frameIII.define("m", VInt(1))
        frameIII.define("y", VInt(2))
    }

    public val a: Env = frameII

    public val b: Env = frameIII

    public val c: Env = frameI

    public val d: Env = frameI

    public fun lookup(
        env: Env,
        name: String,
    ): Either<sicp.runtime.SchemeError, Value> = either { env.lookup(name) }
}

public class S3_2_1RulesForEvaluationTest :
    FunSpec({
        test("the value of x with respect to environment D is 3: frame I binds it") {
            Figure31.lookup(Figure31.d, "x") shouldBe Either.Right(VInt(3))
        }

        test("the value of x with respect to environment B is 3: III defers to I") {
            Figure31.lookup(Figure31.b, "x") shouldBe Either.Right(VInt(3))
        }

        test("with respect to environment A, II's x: 7 shadows I's x: 3") {
            Figure31.lookup(Figure31.a, "x") shouldBe Either.Right(VInt(7))
        }

        test("the fun declaration and the function value compute the same square") {
            square(5L) shouldBe 25L
            squareFn(5L) shouldBe 25L
        }
    })
