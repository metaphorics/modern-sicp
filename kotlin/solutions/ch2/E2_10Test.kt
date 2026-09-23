// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.10

package sicp.ch2.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_10Test :
    FunSpec({
        test("dividing by an interval that does not span zero succeeds") {
            either { divIntervalChecked(makeInterval(4.0, 8.0), makeInterval(1.0, 2.0)) } shouldBe
                Either.Right(Interval(2.0, 8.0))
        }
        test("dividing by an interval that spans zero raises SpansZero") {
            val y = makeInterval(-1.0, 1.0)
            either { divIntervalChecked(makeInterval(4.0, 8.0), y) } shouldBe
                Either.Left(IntervalError.SpansZero(y))
        }
        test("an interval with a zero endpoint also spans zero") {
            val y = makeInterval(0.0, 2.0)
            either { divIntervalChecked(makeInterval(4.0, 8.0), y) } shouldBe
                Either.Left(IntervalError.SpansZero(y))
        }
    })
