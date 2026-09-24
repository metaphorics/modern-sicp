// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.76

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_76Test :
    FunSpec({
        test("ex_2_76 agrees on the same pair across all three strategies") {
            val (explicit, dataDirected, messagePassing) = ex_2_76()
            explicit shouldBe (3.0 to 5.0)
            dataDirected shouldBe explicit
            messagePassing shouldBe explicit
        }

        test("explicit dispatch also serves the polar representation the sealed hierarchy already has") {
            val w = RepZ.Polar(5.0, 0.0)
            edRealPart(w) shouldBe 5.0
            edMagnitude(w) shouldBe 5.0
        }

        test("installing a second package leaves the first package's handler exactly as installed") {
            val table = RepresentationTable()
            installRectRepresentation(table)
            val before = table.get("real-part", "rect")
            installPolarRepresentation(table)
            val after = table.get("real-part", "rect")
            (before === after) shouldBe true
        }
    })
