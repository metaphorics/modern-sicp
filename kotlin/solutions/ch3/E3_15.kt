// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import sicp.runtime.PairCell
import sicp.runtime.Symbol

/**
 * Replace the first datum in the nested pair reached from [x].
 */
public fun setToWow(x: PairCell): PairCell {
    (x.first as PairCell).first = Symbol("wow")
    return x
}
