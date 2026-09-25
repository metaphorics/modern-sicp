// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.6, stream operations: `stream-append-delayed`
// and `interleave-delayed` keep their second argument a thunk,
// `flatten-stream` and `stream-flatmap` interleave, and the plain
// 3.5.3 `append`/`interleave` stand in for the 4.71/4.72 probes.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Frame
import sicp.ch4.bindingInFrame
import sicp.ch4.flattenStream
import sicp.ch4.interleave
import sicp.ch4.interleaveDelayed
import sicp.ch4.listStream
import sicp.ch4.patternVar
import sicp.ch4.singletonStream
import sicp.ch4.streamAppend
import sicp.ch4.streamAppendDelayed
import sicp.ch4.streamFlatmap
import sicp.runtime.LStream
import sicp.runtime.VSym
import sicp.runtime.consStream
import sicp.runtime.take

private fun str(args: List<String>): LStream<String> = args.foldRight(LStream.Empty as LStream<String>) { v, acc -> consStream(v) { acc } }

private fun tagged(tag: String): Frame = Frame.Empty.extended(patternVar("t"), VSym(tag))

private fun onesStream(): LStream<Frame> = consStream(Frame.Empty) { onesStream() }

public class S4_4_4StreamsTest :
    FunSpec({
        test("stream-append-delayed does not force the second stream early") {
            var forced = false

            fun late(): LStream<String> {
                forced = true
                return str(listOf("b"))
            }
            val combined = streamAppendDelayed(str(listOf("a1", "a2"))) { late() }
            forced shouldBe false
            combined.take(1) shouldBe listOf("a1")
            forced shouldBe false
            combined.take(3) shouldBe listOf("a1", "a2", "b")
            forced shouldBe true
        }

        test("interleave-delayed alternates and keeps the second delayed") {
            var forced = false

            fun late(): LStream<String> {
                forced = true
                return str(listOf("b1", "b2"))
            }
            val mixed = interleaveDelayed(str(listOf("a1", "a2", "a3"))) { late() }
            forced shouldBe false
            mixed.take(1) shouldBe listOf("a1")
            forced shouldBe true
            mixed.take(5) shouldBe listOf("a1", "b1", "a2", "b2", "a3")
        }

        test("an empty first stream forces the delayed second") {
            interleaveDelayed(LStream.Empty) { str(listOf("only")) }.take(2) shouldBe listOf("only")
            streamAppendDelayed(LStream.Empty) { str(listOf("only")) }.take(2) shouldBe listOf("only")
        }

        test("stream-flatmap interleaves the mapped frame substreams") {
            val input = listStream(listOf(tagged("one"), tagged("two")))
            val mapped =
                streamFlatmap(
                    { frame ->
                        val name = (bindingInFrame(patternVar("t"), frame) as VSym).name
                        if (name ==
                            "one"
                        ) {
                            listStream(listOf(tagged("1a"), tagged("1b")))
                        } else {
                            listStream(listOf(tagged("2a"), tagged("2b")))
                        }
                    },
                    input,
                )
            mapped
                .take(4)
                .map { (bindingInFrame(patternVar("t"), it) as VSym).name } shouldBe
                listOf("1a", "2a", "1b", "2b")
        }

        test("an infinite first substream does not starve the rest") {
            val flattened = flattenStream(consStream(onesStream()) { LStream.Empty })
            flattened.take(3).size shouldBe 3
            interleave(str(listOf("1", "2", "3", "4"))) { str(listOf("a", "b")) }.take(6) shouldBe
                listOf("1", "a", "2", "b", "3", "4")
            streamAppend(str(listOf("1", "2"))) { str(listOf("a", "b")) }.take(4) shouldBe
                listOf("1", "2", "a", "b")
            singletonStream(Frame.Empty).take(2).size shouldBe 1
        }
    })
