// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.24

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.vlist

/** The book's expression, evaluated: `toString` prints the nested form `(1 (2 (3 4)))`. */
public fun ex_2_24(): String = vlist(VInt(1L), vlist(VInt(2L), vlist(VInt(3L), VInt(4L)))).toString()
