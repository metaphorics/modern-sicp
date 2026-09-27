// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E0_03Test :
    FunSpec({
        test("a leaf is depth 1") {
            depth(Leaf(1L)) shouldBe 1
        }
        test("a node is one more than its deeper child") {
            depth(Node(Leaf(1L), Leaf(2L))) shouldBe 2
            depth(Node(Node(Leaf(1L), Leaf(2L)), Leaf(3L))) shouldBe 3
            depth(Node(Node(Leaf(1L), Node(Leaf(2L), Leaf(3L))), Leaf(4L))) shouldBe 4
        }
    })
