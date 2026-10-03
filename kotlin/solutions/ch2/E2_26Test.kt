// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.26

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum

public class E2_26Test :
    FunSpec({
        test("append, pair construction, and list construction render distinct shapes") {
            val x = datumList(Whole(1L), Whole(2L), Whole(3L))
            val y = datumList(Whole(4L), Whole(5L), Whole(6L))
            val expected =
                listOf(
                    renderDatum(datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L), Whole(5L), Whole(6L))),
                    renderDatum(pair(x, y)),
                    renderDatum(datumList(x, y)),
                )
            ex_2_26() shouldBe expected
        }
    })
