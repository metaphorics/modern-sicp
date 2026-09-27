// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_70

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_70Test :
    FunSpec({
        test("Exercise 4.70: the let-binding discipline") {
            letPurposeDemo() shouldBe
                listOf(
                    "ones model: take(4) = [1, 1, 1, 1]",
                    "broken add-assertion! on (a b): add c => take(4) = [c, c, c, c]",
                    "let-bound add-assertion! on (a b): add c => take(4) = [c, a, b]",
                    "the book's hazard: the memoized tail reads THE-ASSERTIONS at force time, after set! has rebound the name",
                    "the edition's addAssertion binds the old collection before appending; the hazard cannot arise",
                )
        }
    })
