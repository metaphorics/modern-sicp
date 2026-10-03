// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_63

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_63Test :
    FunSpec({
        test("Exercise 4.63: the Genesis queries") {
            genesisQueries() shouldBe
                listOf(
                    "?x = Irad",
                    "?x = Jabal",
                    "?x = Jubal",
                    "?x = Irad",
                )
        }
    })
