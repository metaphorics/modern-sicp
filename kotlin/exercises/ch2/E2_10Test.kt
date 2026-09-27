// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.10

package sicp.ch2.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E2_10Test :
    FunSpec({
        test("Exercise 2.10: dividing [4, 8] by [1, 2], which does not span zero, succeeds").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions.assertEquals(
                Either.Right(Interval(2.0, 8.0)),
                either { divIntervalChecked(makeInterval(4.0, 8.0), makeInterval(1.0, 2.0)) },
            )
        }
    })
