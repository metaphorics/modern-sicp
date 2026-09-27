// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_69

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_69Test :
    FunSpec({
        test("Exercise 4.69: the greats queries") {
            greatsQueries() shouldBe
                listOf(
                    "query: ((great grandson) ?g ?ggs)",
                    "((great grandson) Adam Irad)",
                    "((great grandson) Cain Mehujael)",
                    "((great grandson) Enoch Methushael)",
                    "((great grandson) Irad Lamech)",
                    "((great grandson) Mehujael Jabal)",
                    "((great grandson) Mehujael Jubal)",
                    "query: (?relationship Adam Irad) -- first answer:",
                    "((great . grandson) Adam Irad)",
                    "query: ((great great great great great grandson) Adam ?d)",
                    "((great great great great great grandson) Adam Jabal)",
                    "((great great great great great grandson) Adam Jubal)",
                )
        }
    })
