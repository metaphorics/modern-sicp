// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.56

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.take

public class E3_56Test :
    FunSpec({
        test("Exercise 3.56: merge orders two streams and keeps equal heads once") {
            merge(streamEnumerateInterval(1, 3), streamEnumerateInterval(3, 5)).take(20) shouldBe
                listOf(1L, 2L, 3L, 4L, 5L)
            merge(LStream.Empty, streamEnumerateInterval(1, 3)).take(3) shouldBe
                listOf(1L, 2L, 3L)
            merge(integers, hamming).take(12) shouldBe
                listOf(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L)
        }

        test("Exercise 3.56: hamming enumerates the 2-3-5 numbers in order") {
            hamming.take(15) shouldBe
                listOf(1L, 2L, 3L, 4L, 5L, 6L, 8L, 9L, 10L, 12L, 15L, 16L, 18L, 20L, 24L)
        }

        test("Exercise 3.56: the first 200 hamming numbers are strictly increasing 5-smooth values") {
            val h = hamming.take(200)
            h.zipWithNext { a, b -> a < b }.all { it } shouldBe true
            h.forEach { x ->
                var v = x
                for (p in listOf(2L, 3L, 5L)) {
                    while (v % p == 0L) v /= p
                }
                v shouldBe 1L
            }
        }
    })
