// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_19Test :
    FunSpec({
        test("the 3.13 cycle is caught in constant space") {
            val cycle = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)

            containsCycleConstantSpace(cycle) shouldBe true
        }

        test("a one-cell cycle is caught too") {
            val selfLoop = pair(Symbol("a"), Empty)
            selfLoop.second = selfLoop

            containsCycleConstantSpace(selfLoop) shouldBe true
        }

        test("even and odd finite chains are reported acyclic") {
            containsCycleConstantSpace(datumList(Symbol("a"), Symbol("b"))) shouldBe false
            containsCycleConstantSpace(datumList(Symbol("a"), Symbol("b"), Symbol("c"))) shouldBe false
            containsCycleConstantSpace(datumList(Symbol("a"))) shouldBe false
        }

        test("constant-space and remembered-pair detectors agree for shared and cyclic data") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val shared = pair(x, x)
            val cycle = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)

            (containsCycleConstantSpace(shared) == containsCycle(shared)) shouldBe true
            (containsCycleConstantSpace(cycle) == containsCycle(cycle)) shouldBe true
        }
    })
