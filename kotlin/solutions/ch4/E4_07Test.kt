// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.solutions

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.OutputSink
import sicp.ch4.parseExpr
import sicp.ch4.parseProgram
import sicp.ch4.readDatum
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.Expr
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value

/** Runs the top-level forms of `program` on `evaluator`, answering the last
 * value or the typed fault. */
private fun runIn(
    evaluator: WithLetStar,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

/** Parses one datum as an expression; a parse fault is a test defect. */
private fun parseOf(text: String): Expr = either { parseExpr(readDatum(text)) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

public class E47Test :
    FunSpec({
        test("Exercise 4.7: the rewrite folds the bindings into nested lets") {
            either { letStarToNestedLets(parseOf("(let* ((x 3) (y (+ x 1))) (* x y))") as AppE) } shouldBe
                Either.Right(
                    LetE(
                        persistentListOf(LetBinding("x", LitE(VInt(3)))),
                        persistentListOf(
                            LetE(
                                persistentListOf(LetBinding("y", parseOf("(+ x 1)"))),
                                persistentListOf(parseOf("(* x y)")),
                            ),
                        ),
                    ),
                )
        }

        test("Exercise 4.7: the book's example computes through sequential bindings") {
            val evaluator = WithLetStar(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let* ((x 3) (y (+ x 1))) (* x y))") shouldBe Either.Right(VInt(12))
        }

        test("Exercise 4.7: each init sees the earlier bindings of the same let*") {
            val evaluator = WithLetStar(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let* ((x 2) (y (* x x)) (z (* y x))) z)") shouldBe Either.Right(VInt(8))
        }

        test("Exercise 4.7: let* shadows from its first binding on") {
            val evaluator = WithLetStar(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define x 9) (let* ((x 1) (y x)) y)") shouldBe Either.Right(VInt(1))
        }

        test("Exercise 4.7: a let* nested in another's body re-enters the clause") {
            val evaluator = WithLetStar(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let* ((x 1)) (let* ((y (+ x 1))) y))") shouldBe Either.Right(VInt(2))
        }
    })
