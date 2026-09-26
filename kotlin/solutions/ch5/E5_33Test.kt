// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.33

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_33Test :
    FunSpec({
        test("both factorial procedures compile and return the same result") {
            val comparison = altFactorialComparison()
            comparison
                .filter { it.startsWith("factorial:") || it.startsWith("factorial-alt:") }
                .all { it.contains("answers 120") } shouldBe true
            comparison[0] shouldBe
                "factorial saves: (save continue); (save env); (restore env); (restore continue); " +
                "(save continue); (save proc); (save argl); (save proc); (restore proc); " +
                "(restore argl); (restore proc); (restore continue)"
            comparison[1] shouldBe
                "factorial-alt saves: (save continue); (save env); (restore env); (restore continue); " +
                "(save continue); (save proc); (save env); (save proc); (restore proc); " +
                "(restore env); (restore proc); (restore continue)"
            comparison.last() shouldBe
                "both compile to 79 statements with 12 saves/restores; " +
                "the order swaps the preserved register from argl to env, neither runs faster"
        }
    })
