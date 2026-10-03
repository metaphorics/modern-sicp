// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.67

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_67Test :
    FunSpec({
        test("Exercise 4.67: the loop detector") {
            loopDetectorDemos() shouldBe
                listOf(
                    "married Mickey ?who under the detector: loop bounded at depth 8, terminates",
                    "distinct bindings: [?who = Minnie]",
                    "wheel identical to stock as sets: true",
                    "?boss = [Warbucks, Oliver]",
                )
        }
    })
