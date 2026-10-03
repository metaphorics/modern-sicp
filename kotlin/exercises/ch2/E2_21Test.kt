// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum

public class E2_21Test :
    FunSpec({
        test("Exercise 2.21 renders the squared datum sequence").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_21() shouldBe renderDatum(datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L)))
        }
    })
