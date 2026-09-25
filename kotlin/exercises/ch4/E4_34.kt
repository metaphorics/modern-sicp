// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.34

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.34: printing lazy pairs. The representation gains an
 * identifiable tagged cell with a non-strict `cons`, and the driver prints
 * through the budget printer: at most ten elements per list, then
 * ` ...)`, with dotted tails and nested pairs rendered in their shapes.
 *
 * Expected answers: `(cons 1 2)` prints `(1 . 2)`, the proper list prints
 * `(1 2)`, `ones` prints the ten-element prefix plus the ellipsis,
 * `(car ones)` answers 1, and the nested pair prints `((1) 2)`.
 */
public fun lazyPairPrintTranscript(): String = throw PendingSolution()

public fun onesBudgetPrintTranscript(): String = throw PendingSolution()

public fun carOfOnesTranscript(): String = throw PendingSolution()

public fun nestedLazyPrintTranscript(): String = throw PendingSolution()
