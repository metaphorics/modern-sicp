// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.3, evaluator data structures: procedures and
// environments. Frames are the runtime's persistent-map frame behind a
// `var` (the captured-var closure model of the re-cut 3.2): `define`
// rebinds the frame map on the same node, so every holder of the frame
// sees the new binding, and a compound procedure carries its captured
// environment. Names print per the printer contract.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.Env
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VProc

public class S4_1_3FramesTest :
    FunSpec({
        test("define adds to the first frame and set! rebinds where the name lives") {
            val env = setupEnvironment(OutputSink())
            env.define("x", VInt(10))
            val inner = Env.child(env)
            either { inner.lookup("x") } shouldBe either { VInt(10) }
            inner.define("x", VInt(20)) // shadows without touching the outer frame
            either { inner.lookup("x") } shouldBe either { VInt(20) }
            either { env.lookup("x") } shouldBe either { VInt(10) }
            either { inner.set("x", VInt(30)) } // set! rebinds the nearest owning frame
            either { inner.lookup("x") } shouldBe either { VInt(30) }
            either { env.lookup("x") } shouldBe either { VInt(10) }
            val deepest = Env.child(inner)
            either { deepest.set("x", VInt(40)) } // a frame without the binding walks outward
            either { inner.lookup("x") } shouldBe either { VInt(40) }
        }

        test("a lookup that walks off the chain raises the typed unbound fault") {
            val env = setupEnvironment(OutputSink())
            either<SchemeError, sicp.runtime.Value> { env.lookup("no-such-name") }
                .fold({ it shouldBe SchemeError.Unbound("no-such-name") }, { })
        }

        test("extendEnvironment binds parameters over the captured environment") {
            val sink = OutputSink()
            val global = setupEnvironment(sink)
            val plus = either { global.lookup("+") }.fold({ throw AssertionError() }, { it })
            val frame =
                either { Env.extend(listOf("n"), listOf(VInt(5)), global, null) }
                    .fold({ throw AssertionError() }, { it })
            either { frame.lookup("n") } shouldBe either { VInt(5) }
            either { frame.lookup("+") } shouldBe either { plus }
        }

        test("a named compound procedure prints with its name") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (define (square x) (* x x))
                    square
                    (lambda (y) y)
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            // the define prints nothing; the value lines follow the contract
            transcript shouldBe "#[compound-procedure square]\n#[compound-procedure]\n"
        }

        test("procedures capture their defining environment") {
            val sink = OutputSink()
            val global = setupEnvironment(sink)
            val transcript =
                runProgram(
                    """
                    (define (make-adder n) (lambda (x) (+ x n)))
                    (define add2 (make-adder 2))
                    add2
                    (add2 40)
                    """.trimIndent(),
                    global,
                    sink,
                )
            // both defines print nothing; the anonymous lambda prints unnamed
            transcript shouldBe "#[compound-procedure]\n42\n"
            // the compound value is a VProc over one captured parameter name
            val adder = either { global.lookup("add2") }.fold({ throw AssertionError() }, { it }) as VProc
            adder.params.toList() shouldBe listOf("x")
        }
    })
