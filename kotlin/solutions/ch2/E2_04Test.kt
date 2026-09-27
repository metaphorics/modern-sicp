// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_04Test :
    FunSpec({
        test("carFn and cdrFn recover exactly what consFn closed over") {
            carFn(consFn(3L, 4L)) shouldBe 3L
            cdrFn(consFn(3L, 4L)) shouldBe 4L
        }
        test("consFn works generically, not just over Long") {
            carFn(consFn("a", "b")) shouldBe "a"
            cdrFn(consFn("a", "b")) shouldBe "b"
        }
        test("ex_2_04 matches cdrFn(consFn(3, 4))") {
            ex_2_04() shouldBe 4L
        }
    })
