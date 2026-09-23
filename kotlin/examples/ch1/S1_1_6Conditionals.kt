// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.6

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The case analysis spelled clause by clause. Kotlin requires a `when` used
 * as an expression to be exhaustive, so the negation arm carries `else`;
 * the explicit `x < 0` test would be redundant after the first two arms.
 */
public fun abs(x: Long): Long =
    when {
        x > 0L -> x
        x == 0L -> 0L
        else -> -x
    }

/** The `else` form of the text: only the negative case is spelled out. */
public fun absElse(x: Long): Long =
    when {
        x < 0L -> -x
        else -> x
    }

/** The two-case form of the text: an `if` expression. */
public fun absIf(x: Long): Long = if (x < 0L) -x else x

/** The connective reading: greater than, or equal. */
public fun greaterOrEqual(
    x: Long,
    y: Long,
): Boolean = x > y || x == y

/** The negation reading: not less than. */
public fun greaterOrEqualNot(
    x: Long,
    y: Long,
): Boolean = !(x < y)

public class S1_1_6ConditionalsTest :
    FunSpec({
        test("the when spells the case analysis clause by clause") {
            abs(-4L) shouldBe 4L
            abs(0L) shouldBe 0L
            abs(7L) shouldBe 7L
        }
        test("every shape of the case analysis agrees") {
            listOf(-4L, 0L, 7L, -1L).forEach { v ->
                absElse(v) shouldBe abs(v)
                absIf(v) shouldBe abs(v)
            }
        }
        test("compound predicates compose with the connectives") {
            (7L > 5L && 7L < 10L) shouldBe true
            greaterOrEqual(4L, 3L) shouldBe true
            greaterOrEqual(3L, 3L) shouldBe true
            greaterOrEqual(2L, 3L) shouldBe false
            listOf(4L to 3L, 3L to 3L, 2L to 3L).forEach { (x, y) ->
                greaterOrEqualNot(x, y) shouldBe greaterOrEqual(x, y)
            }
        }
    })
