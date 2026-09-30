// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.24

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_24Test :
    FunSpec({
        test("ex_2_24 predicts the nested native datum rendering") {
            ex_2_24() shouldBe
                "PairCell(first=Whole(value=1), second=PairCell(first=Whole(value=2), second=PairCell(first=PairCell(first=Whole(value=3), second=PairCell(first=Whole(value=4), second=Empty)), second=Empty)))"
        }
    })
