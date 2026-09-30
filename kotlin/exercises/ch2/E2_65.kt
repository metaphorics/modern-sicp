// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65

package sicp.ch2.exercises

/**
 * Exercise 2.65: use the results of exercises 2.63 (`treeToList1`) and
 * 2.64 (`listToTree`) to give `Θ(n)` implementations of `unionSetTree` and
 * `intersectionSetTree` for sets implemented as balanced binary trees,
 * reusing `unionSetOrdered` (2.62) and `intersectionSetOrdered`
 * (`Sets23.kt`) on the flattened lists.
 *
 * The scaffold returns the union and intersection, as sorted lists, of
 * the tree sets built from `[1, 3, 5, 7, 9]` and `[3, 5, 7, 9, 11]`.
 */
public fun unionSetTree(
    t1: SetTree,
    t2: SetTree,
): SetTree = throw PendingExercise()

public fun intersectionSetTree(
    t1: SetTree,
    t2: SetTree,
): SetTree = throw PendingExercise()

public fun ex_2_65(): Pair<List<Long>, List<Long>> = throw PendingExercise()
