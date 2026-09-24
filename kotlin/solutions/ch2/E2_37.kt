// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.37

package sicp.ch2.exercises

/** A matrix: a list of vector rows, the book's representation. */
public typealias Matrix = List<List<Long>>

/** The book's `dot-product`: accumulate `+` over the elementwise products. */
public fun dotProduct(
    v: List<Long>,
    w: List<Long>,
): Long = accumulateList({ x, acc -> x + acc }, 0L, v.zip(w) { a, b -> a * b })

/** The book's `matrix-*-vector`: dot each row of [m] with [v]. */
public fun matrixStarVector(
    m: Matrix,
    v: List<Long>,
): List<Long> = m.map { row -> dotProduct(row, v) }

/** The book's `transpose`: the first element of every row, then transpose the rests. */
public fun transpose(m: Matrix): List<List<Long>> =
    if (m.any { it.isEmpty() }) emptyList() else listOf(m.map { it.first() }) + transpose(m.map { it.drop(1) })

/** The book's `matrix-*-matrix`: every row of [m] dotted with every column of [n]. */
public fun matrixStarMatrix(
    m: Matrix,
    n: Matrix,
): List<List<Long>> = m.map { row -> transpose(n).map { column -> dotProduct(row, column) } }

/** The book's matrix: `((1 2 3 4) (4 5 6 6) (6 7 8 9))`. */
public fun exerciseMatrix(): Matrix = listOf(listOf(1L, 2L, 3L, 4L), listOf(4L, 5L, 6L, 6L), listOf(6L, 7L, 8L, 9L))

/** `matrixStarVector` with the book's matrix and `(1 1 1 1)`, the row sums `(10 21 30)`. */
public fun ex_2_37(): List<Long> = matrixStarVector(exerciseMatrix(), listOf(1L, 1L, 1L, 1L))
