// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.66

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_66Test :
    FunSpec({
        test("lookup finds a record present in the tree") {
            lookup(7L, sampleDb) shouldBe Record(7L, "dave")
            lookup(1L, sampleDb) shouldBe Record(1L, "alice")
            lookup(9L, sampleDb) shouldBe Record(9L, "erin")
        }
        test("lookup returns null for a key absent from the tree") {
            lookup(100L, sampleDb) shouldBe null
        }
        test("lookup on an empty tree is always null") {
            lookup(1L, RecordTree.Empty) shouldBe null
        }
        test("ex_2_66 looks up key 7 in the sample database") {
            ex_2_66() shouldBe "dave"
        }
    })
