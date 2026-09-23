// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentListOf
import kotlin.math.PI

/** A product type: the data class generates equals, hashCode, and toString. */
public data class Frac(
    val num: Long,
    val den: Long,
)

/** A lightweight wrapper: a value class carries no allocation at runtime. */
@JvmInline
public value class Money(
    val cents: Long,
)

/** A sum type: a [Shape] is one member of a closed set of variants. */
public sealed interface Shape

public data class Circle(
    val r: Double,
) : Shape

public data class Rect(
    val w: Double,
    val h: Double,
) : Shape

/** The `when` over a sealed type must cover every variant, without `else`. */
public fun area(s: Shape): Double =
    when (s) {
        is Circle -> PI * s.r * s.r
        is Rect -> s.w * s.h
    }

public class S0_4DataTest :
    FunSpec({
        test("data class equality is structural, identity is not") {
            Frac(1L, 2L) shouldBe Frac(1L, 2L)
            (Frac(1L, 2L) === Frac(1L, 2L)) shouldBe false
        }
        test("when over a sealed type is exhaustive") {
            area(Circle(1.0)) shouldBe PI
            area(Rect(2.0, 3.0)) shouldBe 6.0
        }
        test("value classes wrap a single value") {
            Money(250L).cents shouldBe 250L
            Money(250L) shouldBe Money(250L)
        }
        test("persistent lists keep the old version intact") {
            val xs = persistentListOf(1L, 2L)
            val ys = xs.adding(3L)
            xs shouldBe persistentListOf(1L, 2L)
            ys shouldBe persistentListOf(1L, 2L, 3L)
        }
    })
