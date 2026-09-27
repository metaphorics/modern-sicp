// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.53

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_53Test :
    FunSpec({
        test("Exercise 3.53: the doubling stream counts by powers of two") {
            doubledStream().take(12) shouldBe
                listOf(1L, 2L, 4L, 8L, 16L, 32L, 64L, 128L, 256L, 512L, 1024L, 2048L)
        }

        test("Exercise 3.53: element k is 2^k far into the stream") {
            val s = doubledStream()
            streamRef(s, 30) shouldBe (1L shl 30)
            streamRef(s, 40) shouldBe (1L shl 40)
            streamRef(doubledStream(), 20) shouldBe (1L shl 20)
        }
    })
