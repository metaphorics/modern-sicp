// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.solutions

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.OutputSink
import sicp.ch4.parseExpr
import sicp.ch4.parseProgram
import sicp.ch4.parseTextEither
import sicp.ch4.readDatum
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value

/** Runs the top-level forms of `program` on `evaluator`, answering the last
 * value or the typed fault. */
private fun runIn(
    evaluator: WithLetDerived,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

/** Parses one datum as an expression; a parse fault is a test defect. */
private fun parseOf(text: String): Expr = either { parseExpr(readDatum(text)) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

public class E46Test :
    FunSpec({
        test("Exercise 4.6: the rewrite is the lambda application") {
            letRewrite(parseOf("(let ((x 3) (y 4)) (+ x y))") as LetE) shouldBe
                AppE(
                    LambdaE(
                        persistentListOf("x", "y"),
                        null,
                        persistentListOf(parseOf("(+ x y)")),
                    ),
                    persistentListOf(LitE(VInt(3)), LitE(VInt(4))),
                )
        }

        test("Exercise 4.6: the derived let computes the body in the new frame") {
            val evaluator = WithLetDerived(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let ((x 3) (y 4)) (+ x y))") shouldBe Either.Right(VInt(7))
        }

        test("Exercise 4.6: the inits evaluate in the outer environment") {
            val evaluator = WithLetDerived(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define x 5) (let ((x 3) (y x)) y)") shouldBe Either.Right(VInt(5))
        }

        test("Exercise 4.6: a malformed binding list fails typed at parse") {
            parseTextEither("(let (x))") shouldBe Either.Left(SchemeError.Parse("bad let form: (let (x))"))
        }

        test("Exercise 4.6: an inner let shadows and leaves the outer binding") {
            val evaluator = WithLetDerived(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define x 5) (let ((x 5)) (let ((x 2)) x))") shouldBe Either.Right(VInt(2))
            runIn(evaluator, "x") shouldBe Either.Right(VInt(5))
        }

        test("Exercise 4.6: lets nest as derived expressions at every depth") {
            val evaluator = WithLetDerived(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let ((a 1)) (let ((b (+ a 1))) (+ a b)))") shouldBe Either.Right(VInt(3))
        }
    })
