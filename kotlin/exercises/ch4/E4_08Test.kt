// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.OutputSink
import sicp.ch4.parseExpr
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readDatum
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE

/** Runs the top-level forms of `program` on `evaluator` after the named-let
 * expansion, answering the last value or the typed fault. */
private fun runIn(
    evaluator: WithNamedLet,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        val expanded = readProgram(program).map { namedLetExpansion(it) }
        for (expr in parseProgram(expanded)) last = evaluator.eval(expr, evaluator.global)
        last
    }

/** Reads one datum; a parse fault is a test defect. */
private fun datumOf(text: String): Value = either { readDatum(text) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

/** Parses one datum as an expression; a parse fault is a test defect. */
private fun parseOf(text: String): Expr = either { parseExpr(readDatum(text)) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

public class E48Test :
    FunSpec({
        test("Exercise 4.8: the named shape survives the parser as named-let data").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            printValue(namedLetExpansion(datumOf("(let fib ((n 2)) n)"))) shouldBe
                "(named-let fib ((n 2)) n)"
        }

        test("Exercise 4.8: quoted let-shaped data is left alone").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            printValue(namedLetExpansion(datumOf("'(let fib ((n 2)) n)"))) shouldBe
                "(quote (let fib ((n 2)) n))"
        }

        test("Exercise 4.8: the rewrite is the set!-based wrapper").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            either { namedLetToCombination(parseOf("(named-let fib ((n 2)) n)") as AppE) } shouldBe
                Either.Right(
                    AppE(
                        LambdaE(
                            persistentListOf("fib"),
                            null,
                            persistentListOf(
                                SetE("fib", LambdaE(persistentListOf("n"), null, persistentListOf(VarE("n")))),
                                AppE(VarE("fib"), persistentListOf(LitE(VInt(2)))),
                            ),
                        ),
                        persistentListOf(QuoteE(VSym("*named-let*"))),
                    ),
                )
        }

        test("Exercise 4.8: the book's named-let fibonacci runs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithNamedLet(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                "(let fib ((n 10)) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))",
            ) shouldBe Either.Right(VInt(55))
        }

        test("Exercise 4.8: the loop name stays local to the wrapper frame").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithNamedLet(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define fib 7) (let fib ((n 2)) (if (< n 2) n (fib (- n 1))))") shouldBe
                Either.Right(VInt(1))
            runIn(evaluator, "fib") shouldBe Either.Right(VInt(7))
        }

        test("Exercise 4.8: plain let still evaluates").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithNamedLet(setupEnvironment(OutputSink()))
            runIn(evaluator, "(let ((x 3)) x)") shouldBe Either.Right(VInt(3))
        }
    })
