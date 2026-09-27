// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65

package sicp.ch2.exercises

/** Θ(n): flatten both trees (2.63), merge the sorted lists (2.62), rebuild balanced (2.64). */
public fun unionSetTree(
    t1: SetTree,
    t2: SetTree,
): SetTree = listToTree(unionSetOrdered(treeToList1(t1), treeToList1(t2)))

/** Θ(n): flatten both trees (2.63), intersect the sorted lists (given in `Sets23.kt`), rebuild balanced (2.64). */
public fun intersectionSetTree(
    t1: SetTree,
    t2: SetTree,
): SetTree = listToTree(intersectionSetOrdered(treeToList1(t1), treeToList1(t2)))

/** The union and intersection, as sorted lists, of the tree sets built from `[1,3,5,7,9]` and `[3,5,7,9,11]`. */
public fun ex_2_65(): Pair<List<Long>, List<Long>> {
    val t1 = listToTree(listOf(1L, 3L, 5L, 7L, 9L))
    val t2 = listToTree(listOf(3L, 5L, 7L, 9L, 11L))
    return treeToList1(unionSetTree(t1, t2)) to treeToList1(intersectionSetTree(t1, t2))
}
