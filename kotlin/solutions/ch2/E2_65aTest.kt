// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65a (addition)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.arbitrary.set
import io.kotest.property.checkAll

public class E2_65aTest :
    FunSpec({
        test("ex_2_65a agrees with java.util.TreeSet on the worked example") {
            ex_2_65a() shouldBe true
        }
        test("unionSetTree agrees with TreeSet.addAll for every generated pair of sets") {
            checkAll(Arb.set(Arb.long(-500L..500L), 0..40), Arb.set(Arb.long(-500L..500L), 0..40)) { a, b ->
                val ours = treeToList1(unionSetTree(treeOf(a), treeOf(b))).toSet()
                val jdk = java.util.TreeSet(a).apply { addAll(b) }
                ours shouldBe jdk
            }
        }
        test("intersectionSetTree agrees with TreeSet.retainAll for every generated pair of sets") {
            checkAll(Arb.set(Arb.long(-500L..500L), 0..40), Arb.set(Arb.long(-500L..500L), 0..40)) { a, b ->
                val ours = treeToList1(intersectionSetTree(treeOf(a), treeOf(b))).toSet()
                val jdk = java.util.TreeSet(a).apply { retainAll(b) }
                ours shouldBe jdk
            }
        }
        test("treeOf an empty set is the empty tree") {
            treeOf(emptySet()) shouldBe SetTree.Empty
        }
    })
