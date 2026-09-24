// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.2

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.sqrt

public class E3_02Test :
    FunSpec({
        test("the book's session: a monitored sqrt(100.0) is 10.0, then one call is counted") {
            val s = makeMonitored<Double, Double> { x -> sqrt(x) }
            s.call(100.0) shouldBe 10.0
            s.howManyCalls() shouldBe 1
        }

        test("resetCount zeroes the counter without touching f's behavior") {
            val s = makeMonitored<Double, Double> { x -> sqrt(x) }
            s.call(4.0) shouldBe 2.0
            s.call(9.0) shouldBe 3.0
            s.howManyCalls() shouldBe 2
            s.resetCount()
            s.howManyCalls() shouldBe 0
            s.call(16.0) shouldBe 4.0
            s.howManyCalls() shouldBe 1
        }

        test("two monitored wrappers of the same function count independently") {
            val s1 = makeMonitored<Double, Double> { x -> sqrt(x) }
            val s2 = makeMonitored<Double, Double> { x -> sqrt(x) }
            s1.call(1.0)
            s1.call(1.0)
            s2.call(1.0)
            s1.howManyCalls() shouldBe 2
            s2.howManyCalls() shouldBe 1
        }
    })
