// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.24

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_24Test :
    FunSpec({
        test("the nested vlist prints (1 (2 (3 4)))") {
            ex_2_24() shouldBe "(1 (2 (3 4)))"
        }
    })
