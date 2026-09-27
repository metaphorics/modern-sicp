// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.8

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_08Test :
    FunSpec({
        test("cube roots land within the stated tolerance of the true roots") {
            ex_1_08(27.0) shouldBe 3.0000005410641766
            ex_1_08(64.0) shouldBe 4.000017449510739
            ex_1_08(1000.0) shouldBe 10.000000145265767
            ex_1_08(1e12) shouldBe 10_000.0
        }
    })
