// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_05

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_05Test :
    FunSpec({
        test("Exercise 5.5: hand-simulating the factorial and Fibonacci machines") {
            handSimulationTraces() shouldBe
                listOf(
                    "(save continue) stack=(fact-done)",
                    "(save n) stack=(3 fact-done)",
                    "(save continue) stack=(after-fact 3 fact-done)",
                    "(save n) stack=(2 after-fact 3 fact-done)",
                    "branch taken to base-case",
                    "return to after-fact",
                    "(restore n) n=2 stack=(after-fact 3 fact-done)",
                    "(restore continue) continue=after-fact stack=(3 fact-done)",
                    "return to after-fact",
                    "(restore n) n=3 stack=(fact-done)",
                    "(restore continue) continue=fact-done stack=()",
                    "return to fact-done",
                    "answer 6",
                    "(save continue) stack=(fib-done)",
                    "(save n) stack=(3 fib-done)",
                    "(save continue) stack=(afterfib-n-1 3 fib-done)",
                    "(save n) stack=(2 afterfib-n-1 3 fib-done)",
                    "branch taken to immediate-answer",
                    "return to afterfib-n-1",
                    "(restore n) n=2 stack=(afterfib-n-1 3 fib-done)",
                    "(restore continue) continue=afterfib-n-1 stack=(3 fib-done)",
                    "(save continue) stack=(afterfib-n-1 3 fib-done)",
                    "(save val) stack=(1 afterfib-n-1 3 fib-done)",
                    "branch taken to immediate-answer",
                    "return to afterfib-n-2",
                    "(restore val) val=1 stack=(afterfib-n-1 3 fib-done)",
                    "(restore continue) continue=afterfib-n-1 stack=(3 fib-done)",
                    "return to afterfib-n-1",
                    "(restore n) n=3 stack=(fib-done)",
                    "(restore continue) continue=fib-done stack=()",
                    "(save continue) stack=(fib-done)",
                    "(save val) stack=(1 fib-done)",
                    "branch taken to immediate-answer",
                    "return to afterfib-n-2",
                    "(restore val) val=1 stack=(fib-done)",
                    "(restore continue) continue=fib-done stack=()",
                    "return to fib-done",
                    "answer 2",
                )
        }
        test("Exercise 5.5: the factorial machine's stack peaks at two frames") {
            factorialMaxDepth(3) shouldBe 4
        }
    })
