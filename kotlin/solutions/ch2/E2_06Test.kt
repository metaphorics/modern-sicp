// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.6

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_06Test :
    FunSpec({
        test("churchToLong decodes churchZero, churchOne, and churchTwo") {
            churchToLong(churchZero()) shouldBe 0L
            churchToLong(churchOne()) shouldBe 1L
            churchToLong(churchTwo()) shouldBe 2L
        }
        test("churchOne is churchAddOne(churchZero()) under decoding") {
            churchToLong(churchOne()) shouldBe churchToLong(churchAddOne(churchZero()))
        }
        test("plusChurch adds by decoded value, for every pair up to four") {
            for (a in 0..4) {
                for (b in 0..4) {
                    var na = churchZero<Long>()
                    repeat(a) { na = churchAddOne(na) }
                    var nb = churchZero<Long>()
                    repeat(b) { nb = churchAddOne(nb) }
                    churchToLong(plusChurch(na, nb)) shouldBe (a + b).toLong()
                }
            }
        }
        test("ex_2_06 is one plus two, three") {
            ex_2_06() shouldBe 3L
        }
    })
