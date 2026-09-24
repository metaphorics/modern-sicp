// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 85

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_85Test :
    FunSpec({
        test("drop lowers 1.5+0i to real, 1+0i to integer, and keeps 2+3i") {
            ex_2_85() shouldBe listOf("1.5", "1", "2.0+3.0i", "6")
        }

        test("an unprojectable integer stays itself") {
            val table = NumTable()
            installGenericArithmetic(table)
            installRaise(table)
            installProject(table)
            val result = arrow.core.raise.either { dropNum(table, ZLong(5)) }
            result.getOrNull() shouldBe ZLong(5)
        }
    })
