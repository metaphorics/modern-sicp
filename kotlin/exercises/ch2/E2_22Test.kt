// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum

public class E2_22Test :
    FunSpec({
        test("Exercise 2.22 renders the reverse-order result of the first attempt").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_22() shouldBe renderDatum(datumList(Whole(16L), Whole(9L), Whole(4L), Whole(1L)))
        }
    })
