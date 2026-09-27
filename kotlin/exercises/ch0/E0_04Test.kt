// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E0_04Test :
    FunSpec({
        test("Exercise 0.4: parseAmount raises, does not throw").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Either.Right(42L), either { parseAmount("42") })
            org.junit.jupiter.api.Assertions.assertEquals(
                Either.Left(ParseError.NotDigits("1x")),
                either { parseAmount("1x") },
            )
        }
    })
