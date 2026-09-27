// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 86

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_86Test :
    FunSpec({
        test("generic parts add, magnify, and rebuild through the table") {
            ex_2_86() shouldBe true
        }

        test("the transcendentals answer reals for exact levels") {
            val table = NumTable()
            installGenericArithmetic(table)
            installRaise(table)
            installRealPackage(table)
            installTranscendentals(table)
            val sine = arrow.core.raise.either { applyGeneric(table, "sine", listOf(ZLong(0))) }
            sine.getOrNull() shouldBe Real(0.0)
        }
    })
