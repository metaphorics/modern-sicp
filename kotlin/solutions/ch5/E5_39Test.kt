// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_39

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.guest.GValue

public class E5_39Test :
    FunSpec({
        test("Exercise 5.39: the lexical lookup answers the bound value and the unassigned word otherwise") {
            val values = listOf(listOf(GValue.VLong(1)), listOf(GValue.VLong(2), GValue.VLong(3)))
            lexicalAddressLookup(LexicalAddress(0, 0), values) shouldBe GValue.VLong(1)
            lexicalAddressLookup(LexicalAddress(1, 1), values) shouldBe GValue.VLong(3)
            lexicalAddressLookup(LexicalAddress(2, 0), values) shouldBe GValue.VUnassigned
        }
    })
