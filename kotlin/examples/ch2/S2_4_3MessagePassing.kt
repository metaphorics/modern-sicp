// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.3, message passing

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

/**
 * The message-passing alternative (2.4.3): instead of intelligent
 * operations that dispatch on data type, a data object is itself a
 * procedure that dispatches on the operation name it receives as a
 * "message." Every message this section's numbers answer names a real
 * number, so the object is a `fun interface` from an operation name to
 * one -- the section's sketch for a single-behavior dispatch object, one
 * level up from a bare returned lambda.
 */
public fun interface MessageObject {
    public fun send(op: String): Double
}

/**
 * The book's `make-from-real-imag` in message-passing style: the value
 * returned is the `dispatch` procedure itself, closing over `x` and `y`.
 */
public fun makeFromRealImagMessagePassing(
    x: Double,
    y: Double,
): MessageObject =
    MessageObject { op ->
        when (op) {
            "real-part" -> x
            "imag-part" -> y
            "magnitude" -> sqrt(x * x + y * y)
            "angle" -> atan2(y, x)
            else -> error("Unknown op: MAKE-FROM-REAL-IMAG $op")
        }
    }

/**
 * The corresponding `apply-generic`: it simply feeds the operation's name
 * to the data object and lets the object do the work.
 */
public fun applyGenericMessagePassing(
    op: String,
    arg: MessageObject,
): Double = arg.send(op)

public class S2_4_3MessagePassingTest :
    FunSpec({
        test("the message object answers every operation the section names") {
            val z = makeFromRealImagMessagePassing(3.0, 4.0)
            applyGenericMessagePassing("real-part", z) shouldBe 3.0
            applyGenericMessagePassing("imag-part", z) shouldBe 4.0
            applyGenericMessagePassing("magnitude", z) shouldBe 5.0
            applyGenericMessagePassing("angle", z) shouldBe 0.9272952180016122
        }

        test("an unanswered message fails the way the book's dispatch does") {
            val z = makeFromRealImagMessagePassing(3.0, 4.0)
            val failure = runCatching { applyGenericMessagePassing("rotate", z) }
            failure.isFailure shouldBe true
            failure.exceptionOrNull()?.message shouldBe "Unknown op: MAKE-FROM-REAL-IMAG rotate"
        }
    })
