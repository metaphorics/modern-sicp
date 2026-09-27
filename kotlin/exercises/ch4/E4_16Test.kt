// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.parseTextEither
import sicp.runtime.AppE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SetE
import sicp.runtime.VInt
import sicp.runtime.VarE

public class E4_16Test :
    FunSpec({
        test("Exercise 4.16: mutual recursion through internal defines runs under the scan-out").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mutualRecursionTranscript() shouldBe "#t\n"
        }

        test("Exercise 4.16: reading a reserved name before its set! fails the typed premature read").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            prematureReadTranscript() shouldBe "Error: type mismatch: a is read before it is assigned\n"
        }

        test("Exercise 4.16: the same program on the base evaluator fails as a plain unbound variable").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            basePrematureTranscript() shouldBe "Error: unbound variable: a\n"
        }

        test("Exercise 4.16: the scan reserves the defined names and assigns in source order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val body = parseTextEither("(define b 1) (define a 2) (+ a b)").getOrNull() ?: emptyList()
            val let = scanOutDefines(body).single() as LetE
            let.bindings.map { it.name } shouldBe listOf("b", "a")
            let.bindings.map { (it.value as LitE).v.toString() } shouldBe
                listOf("*unassigned*", "*unassigned*")
            let.body.dropLast(1) shouldBe listOf(SetE("b", LitE(VInt(1))), SetE("a", LitE(VInt(2))))
            let.body.last() shouldBe AppE(VarE("+"), persistentListOf(VarE("a"), VarE("b")))
        }
    })
