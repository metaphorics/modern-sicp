// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.3, finding assertions by pattern matching: the
// book's matcher examples, `extend-if-consistent` matching a stored
// pattern that contains variables, and `find-assertions`/`check-an-
// assertion` over the data base.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Frame
import sicp.ch4.bindingInFrame
import sicp.ch4.patternMatch
import sicp.ch4.patternVar
import sicp.ch4.printValue
import sicp.runtime.take

public class S4_4_4MatcherTest :
    FunSpec({
        val data = readQuery("((a b) c (a b))")

        test("the book's matches of ((a b) c (a b))") {
            val f1 = patternMatch(patQuery("(?x c ?x)"), data, Frame.Empty)
            f1?.let { bindingInFrame(patternVar("x"), it) }?.toString() shouldBe "(a b)"

            val f2 = patternMatch(patQuery("(?x ?y ?z)"), data, Frame.Empty)
            bindingInFrame(patternVar("x"), f2!!).toString() shouldBe "(a b)"
            bindingInFrame(patternVar("y"), f2).toString() shouldBe "c"
            bindingInFrame(patternVar("z"), f2).toString() shouldBe "(a b)"

            val f3 = patternMatch(patQuery("((?x ?y) c (?x ?y))"), data, Frame.Empty)
            bindingInFrame(patternVar("x"), f3!!).toString() shouldBe "a"
            bindingInFrame(patternVar("y"), f3).toString() shouldBe "b"
        }

        test("the book's non-match") {
            patternMatch(patQuery("(?x a ?y)"), data, Frame.Empty) shouldBe null
        }

        test("a stored pattern value is matched recursively") {
            // ?x bound to (f ?y); matching ?x against (f b) binds ?y to b.
            val frame = Frame.Empty.extended(patternVar("x"), patQuery("(f ?y)"))
            val result = patternMatch(patternVar("x"), readQuery("(f b)"), frame)
            bindingInFrame(patternVar("y"), result!!).toString() shouldBe "b"
            bindingInFrame(patternVar("x"), result).toString() shouldBe "(f (? y))"
        }

        test("find-assertions scans the data base through the frame") {
            val system = microshaftSystem()
            val matches = system.findAssertions(patQuery("(supervisor ?x ?x)"), Frame.Empty).take(1)
            matches.size shouldBe 0
            val hits = system.findAssertions(patQuery("(job ?x (computer programmer))"), Frame.Empty).take(9)
            hits.map { printValue(bindingInFrame(patternVar("x"), it)!!) } shouldBe
                listOf("(Hacker Alyssa P)", "(Fect Cy D)")
        }
    })
