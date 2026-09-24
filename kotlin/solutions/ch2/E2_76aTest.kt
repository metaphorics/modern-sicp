// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.76a

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.cos

public class E2_76aTest :
    FunSpec({
        test("ex_2_76a confirms the additive install and the new package's answer") {
            ex_2_76a() shouldBe true
        }

        test("installPolarDegRepresentation answers real-part and magnitude for a known angle") {
            val table = RepresentationTable()
            installPolarDegRepresentation(table)
            val realPart = table.get("real-part", "polar-deg")?.invoke(2.0, 0.0)
            val magnitude = table.get("magnitude", "polar-deg")?.invoke(2.0, 90.0)
            realPart shouldBe 2.0 * cos(0.0)
            magnitude shouldBe 2.0
        }

        test("the polar package installed alongside the degrees package is unaffected") {
            val table = RepresentationTable()
            installPolarRepresentation(table)
            val beforeDeg = table.get("real-part", "polar")
            installPolarDegRepresentation(table)
            val afterDeg = table.get("real-part", "polar")
            (beforeDeg === afterDeg) shouldBe true
        }
    })
