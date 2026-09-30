// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.24

package sicp.ch2.exercises

import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum

/** Render the nested datum tree in the canonical native constructor form. */
public fun ex_2_24(): String = renderDatum(datumList(Whole(1L), Whole(2L), datumList(Whole(3L), Whole(4L))))
