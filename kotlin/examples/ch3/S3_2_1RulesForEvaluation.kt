// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.2 (head) and 3.2.1, the rules for evaluation

package sicp.ch3.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.BindingFrame
import sicp.runtime.Datum
import sicp.runtime.Whole

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
    public val frameI: BindingFrame = BindingFrame.root()

    public val frameII: BindingFrame = BindingFrame.child(frameI)

    public val frameIII: BindingFrame = BindingFrame.child(frameI)

    init {
        frameI.bind("x", Whole(3))
        frameI.bind("y", Whole(5))
        frameII.bind("z", Whole(6))
        frameII.bind("x", Whole(7))
        frameIII.bind("m", Whole(1))
        frameIII.bind("y", Whole(2))
    }

    public val a: BindingFrame = frameII

    public val b: BindingFrame = frameIII

    public val c: BindingFrame = frameI

    public val d: BindingFrame = frameI

    public fun lookup(
        env: BindingFrame,
        name: String,
    ): Either<sicp.runtime.DatumError, Datum> = either { env.read(name) }
}

public class S3_2_1RulesForEvaluationTest :
    FunSpec({
        test("the value of x with respect to environment D is 3: frame I binds it") {
            Figure31.lookup(Figure31.d, "x") shouldBe Either.Right(Whole(3))
        }

        test("the value of x with respect to environment B is 3: III defers to I") {
            Figure31.lookup(Figure31.b, "x") shouldBe Either.Right(Whole(3))
        }

        test("with respect to environment A, II's x: 7 shadows I's x: 3") {
            Figure31.lookup(Figure31.a, "x") shouldBe Either.Right(Whole(7))
        }

        test("the fun declaration and the function value compute the same square") {
            square(5L) shouldBe 25L
            squareFn(5L) shouldBe 25L
        }
    })
