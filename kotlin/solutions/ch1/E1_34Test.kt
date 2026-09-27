// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.34

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_34Test :
    FunSpec({
        test("f(square) is 4 and f of the given lambda is 6") {
            ex_1_34() shouldBe Pair(4L, 6L)
        }
    })
