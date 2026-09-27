// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.38

package sicp.ch2.exercises

/** The book's `fold-right`: `accumulateList` under its other name. */
public fun <T, R> foldRight(
    op: (T, R) -> R,
    initial: R,
    sequence: List<T>,
): R = accumulateList(op, initial, sequence)

/** The book's `fold-left`: combines left to right, updating the result so far. */
public tailrec fun <T, R> foldLeft(
    op: (R, T) -> R,
    initial: R,
    sequence: List<T>,
): R = if (sequence.isEmpty()) initial else foldLeft(op, op(initial, sequence.first()), sequence.drop(1))

/** Both folds applied to integer division on `(1 2 3)`: `foldRight` gives 3/2, `foldLeft` gives 1/6. */
public fun ex_2_38(): List<Double> {
    val sequence = listOf(1L, 2L, 3L)
    return listOf(
        foldRight({ x: Long, acc: Double -> x / acc }, 1.0, sequence),
        foldLeft({ acc: Double, x: Long -> acc / x }, 1.0, sequence),
    )
}
