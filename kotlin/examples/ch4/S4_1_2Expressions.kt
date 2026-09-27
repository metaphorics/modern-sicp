// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.2, representing expressions: the typed Expr nodes
// the parser produces, `cond->if` as the derived-expression rewrite, and the
// D19 operation table -- the immutable registry keyed by the operation plus
// the ordered tag list, whose `get` answers the absent option on a miss and
// whose `put` overwrites -- that exercise 4.3 builds the data-directed eval
// on.

package sicp.ch4.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.condToIfChain
import sicp.ch4.evalText
import sicp.ch4.parseExpr
import sicp.ch4.printValue
import sicp.ch4.readDatum
import sicp.ch4.setupEnvironment
import sicp.runtime.CondE
import sicp.runtime.Expr
import sicp.runtime.Key
import sicp.runtime.OpTable
import sicp.runtime.VInt
import sicp.runtime.VSym
import sicp.runtime.Value

/** The parse boundary: text to typed node, asserting well-formed input. */
private fun parse(text: String): Expr = either { parseExpr(readDatum(text)) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

/** The eval boundary: one expression text, asserting no fault. */
private fun eval(text: String): Value = either { evalText(text, env) }.fold({ e -> throw AssertionError(e.toString()) }, { it })

private val env = setupEnvironment(OutputSink())

/** The tag-list key the D19 registry orders: `("t1" "t2")` as a chain. */
private fun tagKey(tags: List<String>): Key = tags.foldRight(Key.Nil as Key) { tag, acc -> Key.Pair(Key.Sym(tag), acc) }

public class S4_1_2ExpressionsTest :
    FunSpec({
        test("cond and its cond->if rewrite answer identically") {
            val cond = parse("(cond ((= x 0) 'zero) ((< x 0) 'neg) (else 'pos))") as CondE
            val rewritten = condToIfChain(cond.clauses)
            eval("(define x 0)")
            eval("(cond ((= x 0) 'zero) ((< x 0) 'neg) (else 'pos))") shouldBe VSym("zero")
            either { Evaluator(env).eval(rewritten, env) } shouldBe Either.Right(VSym("zero"))
        }

        test("x drives the cond through every clause") {
            eval("(cond ((= x 0) 'zero) ((< x 0) 'neg) (else 'pos))") shouldBe VSym("zero")
            eval("(set! x -5)")
            eval("(cond ((= x 0) 'zero) ((< x 0) 'neg) (else 'pos))") shouldBe VSym("neg")
            eval("(set! x 5)")
            eval("(cond ((= x 0) 'zero) ((< x 0) 'neg) (else 'pos))") shouldBe VSym("pos")
        }

        test("the D19 registry keys on the operation plus the ordered tag list") {
            val table = OpTable()
            val realPart: sicp.runtime.Op = { args -> args[0] }
            table.put(Key.Sym("real-part"), tagKey(listOf("rectangular")), realPart)
            table.get(Key.Sym("real-part"), tagKey(listOf("rectangular"))) shouldBe realPart
            // get on a missing key is the absent option, never a false-ish sentinel
            table.get(Key.Sym("imag-part"), tagKey(listOf("rectangular"))) shouldBe null
            table.get(Key.Sym("real-part"), tagKey(listOf("polar"))) shouldBe null
            // put on an existing key overwrites
            val replacement: sicp.runtime.Op = { args -> args[1] }
            table.put(Key.Sym("real-part"), tagKey(listOf("rectangular")), replacement)
            table.get(Key.Sym("real-part"), tagKey(listOf("rectangular"))) shouldBe replacement
        }

        test("quoted data evaluates to itself") {
            eval("(car '(a b c))") shouldBe VSym("a")
            printValue(eval("'(a b)")) shouldBe "(a b)"
        }
    })
