// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.8, frames and bindings: the immutable
// variable-to-term map, the unmarked-inhabitant rule -- failure is the
// absent option, never a sentinel binding -- and the extend step that
// answers a new frame without touching the base.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QFrame
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4FramesTest :
    FunSpec({
        val x = variable("x")
        val y = variable("y")

        test("the empty frame binds nothing") {
            QFrame(emptyMap()).bindings[x] shouldBe null
        }

        test("extend adds one binding without touching the base") {
            val base = QFrame(emptyMap())
            val f1 = QFrame(base.bindings + (x to terms(sym("a"), sym("b"))))
            val f2 = QFrame(f1.bindings + (y to sym("c")))
            f2.bindings[y] shouldBe sym("c")
            f2.bindings[x] shouldBe terms(sym("a"), sym("b"))
            // the base frame still binds nothing: frames are persistent
            base.bindings[x] shouldBe null
            f1.bindings[y] shouldBe null
        }

        test("a re-extend replaces, per the persistent map") {
            val f = QFrame(mapOf(x to sym("1")))
            val g = QFrame(f.bindings + (x to sym("2")))
            g.bindings[x] shouldBe sym("2")
            f.bindings[x] shouldBe sym("1")
        }

        test("frames with equal bindings are equal") {
            val a: QFrame = QFrame(mapOf(x to terms(sym("a"), sym("b"))))
            val b: QFrame = QFrame(mapOf(QVar("x") to terms(sym("a"), sym("b"))))
            a shouldBe b
        }

        test("a failed match answers no frame, never a marker value") {
            val database = QueryDatabase()
            database.assertFact(fact(terms(sym("job"), sym("Ben"), terms(sym("computer"), sym("wizard")))))
            val driver = QueryDriver.streaming(database)
            answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), sym("programmer")))), listOf(x)) shouldBe
                emptyList()
        }
    })
