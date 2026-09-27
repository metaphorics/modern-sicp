// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65a (addition)

package sicp.ch2.exercises

/** Builds this section's balanced [SetTree] from an arbitrary (unordered, duplicate-free) set of elements. */
public fun treeOf(elements: Set<Long>): SetTree = listToTree(elements.sorted())

/**
 * Cross-checks [unionSetTree]/[intersectionSetTree] (exercise 2.65)
 * against `java.util.TreeSet`, the JDK's own balanced (red-black) tree
 * set, for one worked example: `{1, 3, 5, 7, 9}` and `{3, 5, 7, 9, 11}`.
 * `E2_65aTest.kt` extends this single check into a Kotest property test
 * over many generated pairs of sets.
 */
public fun ex_2_65a(): Boolean {
    val a = setOf(1L, 3L, 5L, 7L, 9L)
    val b = setOf(3L, 5L, 7L, 9L, 11L)
    val ourUnion = treeToList1(unionSetTree(treeOf(a), treeOf(b))).toSet()
    val ourIntersection = treeToList1(intersectionSetTree(treeOf(a), treeOf(b))).toSet()
    val jdkUnion = java.util.TreeSet(a).apply { addAll(b) }
    val jdkIntersection = java.util.TreeSet(a).apply { retainAll(b) }
    return ourUnion == jdkUnion && ourIntersection == jdkIntersection
}
