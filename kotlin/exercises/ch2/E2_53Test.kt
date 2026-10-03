// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.53

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Truth
import sicp.runtime.datumList
import sicp.runtime.renderDatum

public class E2_53Test :
    FunSpec({
        test("Exercise 2.53 predicts canonical native renderings of seven data results").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_53() shouldBe
                listOf(
                    renderDatum(datumList(Symbol("a"), Symbol("b"), Symbol("c"))),
                    renderDatum(datumList(datumList(Symbol("george")))),
                    renderDatum(datumList(datumList(Symbol("y1"), Symbol("y2")))),
                    renderDatum(datumList(Symbol("y1"), Symbol("y2"))),
                    renderDatum(Truth(false)),
                    renderDatum(Truth(false)),
                    renderDatum(datumList(Symbol("red"), Symbol("shoes"), Symbol("blue"), Symbol("socks"))),
                )
        }
    })
