// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.2.2, the interpreter with lazy evaluation: the
// driver loop over `actual-value`, and the thunk machinery of
// "Representing thunks" -- the memoized cell that fills once and the
// unmemoized variant that re-runs at every demand.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyEvaluator
import sicp.ch4.OutputSink
import sicp.ch4.lazyTranscriptOn
import sicp.ch4.parseExpr
import sicp.ch4.readDatum
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.ThunkState
import sicp.runtime.VInt
import sicp.runtime.VThunk
import sicp.runtime.VThunkNoMemo

private val MACHINE_SESSION: String =
    """
    (define count 0)
    (define (id x) (set! count (+ count 1)) x)
    (define w (id (id 10)))
    """.trimIndent()

public class S4_2_2LazyEvaluatorTest :
    FunSpec({
        test("the driver loop forces the answer before printing") {
            lazyTranscriptOn(
                ::LazyEvaluator,
                """
                (define (try a b)
                  (if (= a 0) 1 b))
                (try 0 (/ 1 0))
                """.trimIndent(),
            ) shouldBe "1\n"
        }

        test("a delayed argument is a thunk cell that fills once") {
            val sink = OutputSink()
            val env = setupEnvironment(sink)
            val evaluator = LazyEvaluator(env)
            either {
                for (form in readProgram(MACHINE_SESSION)) {
                    evaluator.eval(parseExpr(form), env)
                }
                val thunk = evaluator.eval(parseExpr(readDatum("(id (id 10))")), env) as VThunk
                (thunk.state is ThunkState.Delayed) shouldBe true
                evaluator.forceValue(thunk) shouldBe VInt(10)
                (thunk.state is ThunkState.Forced) shouldBe true
                // the memoized cell answers again without re-running `id`
                evaluator.forceValue(thunk) shouldBe VInt(10)
            }
            // the tested thunk is the inner application's value; its one
            // force ran `id` once, after the two calls the session made
            either { evaluator.eval(parseExpr(readDatum("count")), env) }.getOrNull() shouldBe VInt(3)
        }

        test("the unmemoized variant re-runs the expression at every demand") {
            val sink = OutputSink()
            val env = setupEnvironment(sink)
            val evaluator = LazyEvaluator(env)
            either {
                for (form in readProgram(MACHINE_SESSION)) {
                    evaluator.eval(parseExpr(form), env)
                }
                val delayed = evaluator.eval(parseExpr(readDatum("(id (id 10))")), env) as VThunk
                val body = (delayed.state as ThunkState.Delayed).expr
                val noMemo = VThunkNoMemo(body, env)
                evaluator.forceValue(noMemo) shouldBe VInt(10)
                evaluator.forceValue(noMemo) shouldBe VInt(10)
            }
            // two demands of the inner application's thunk: `id` re-runs
            // at every force, after the two calls the session made
            either { evaluator.eval(parseExpr(readDatum("count")), env) }.getOrNull() shouldBe VInt(4)
        }
    })
