// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.Either
import io.kotest.assertions.fail
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class RandomTest :
    FunSpec({
        test("from seed 1, random(1000) yields the decision 0001 vector") {
            val vector: List<Long> =
                Random.seeded(1UL).fold(
                    { fail("seeded(1UL) must be Right") },
                    { random -> List(5) { random.random(1000) } },
                )
            vector shouldBe listOf(165L, 517L, 103L, 413L, 928L)
        }

        test("a zero seed is rejected with InvalidSeed") {
            Random.seeded(0UL) shouldBe Either.Left(RandomError.InvalidSeed)
        }
    })
