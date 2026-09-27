// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.81

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** Builds a finite request stream, first request at the head. */
private fun requests(vararg rs: RandRequest): LStream<RandRequest> =
    rs.toList().foldRight(LStream.Empty as LStream<RandRequest>) { r, acc -> consStream(r) { acc } }

public class E3_81Test :
    FunSpec({
        test("Exercise 3.81: generate advances the word, reset restarts it").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            randStream(
                requests(RandRequest.Generate, RandRequest.Generate, RandRequest.Reset(7UL), RandRequest.Generate),
            ).take(4) shouldBe
                listOf(6255019084209693600UL, 15601610542105701163UL, 7UL, 15130880334998875822UL)
        }

        test("Exercise 3.81: the same request list builds the same output twice").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val rs =
                listOf(
                    RandRequest.Generate,
                    RandRequest.Generate,
                    RandRequest.Reset(7UL),
                    RandRequest.Generate,
                    RandRequest.Generate,
                )
            val expected =
                listOf(
                    6255019084209693600UL,
                    15601610542105701163UL,
                    7UL,
                    15130880334998875822UL,
                    18178034093014037351UL,
                )
            randStream(requests(*rs.toTypedArray())).take(5) shouldBe expected
            randStream(requests(*rs.toTypedArray())).take(5) shouldBe expected
        }

        test("Exercise 3.81: after a reset the sequence replays from the chosen word").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val fromReset =
                randStream(
                    requests(
                        RandRequest.Generate,
                        RandRequest.Generate,
                        RandRequest.Reset(7UL),
                        RandRequest.Generate,
                        RandRequest.Generate,
                    ),
                ).take(5).drop(3)
            val freshFromSeven =
                randStream(
                    requests(RandRequest.Reset(7UL), RandRequest.Generate, RandRequest.Generate),
                ).take(3).drop(1)
            fromReset shouldBe freshFromSeven
        }

        test("Exercise 3.81: resetting to the initial word matches the prelude random stream").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            randStream(requests(RandRequest.Reset(RANDOM_INIT), RandRequest.Generate)).take(2) shouldBe
                randomNumbers.take(2)
        }
    })
