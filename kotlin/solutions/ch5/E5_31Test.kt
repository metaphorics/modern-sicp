// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.31

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_31Test :
    FunSpec({
        test("the four combinations report which evaluator saves survive preserving") {
            superfluousSaves() shouldBe
                listOf(
                    "(f 'x 'y): ",
                    "((f) 'x 'y): (save continue); (restore continue)",
                    "(f (g 'x) y): (save continue); (save proc); (save argl); (restore argl); (restore proc); (restore continue)",
                    "(f (g 'x) 'y): (save continue); (save proc); (save argl); (restore argl); (restore proc); (restore continue)",
                )
        }
    })
