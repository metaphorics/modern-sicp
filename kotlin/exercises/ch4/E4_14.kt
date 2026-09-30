// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.14

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.14: Louis Reasoner installs the host `map` as a primitive,
// and his evaluator applies primitives plainly -- no special case -- so
// the primitive's handler can only call its procedure argument as a
// primitive call. The argument is a compound procedure, so the first
// element dies with the kernel's fault convention. Eva Lu Ator defines
// `map` in the object language instead; her `map` is a compound
// procedure, the evaluator applies it normally, and the same call -- map
// the squaring procedure over [1, 2, 3] -- answers `[1, 4, 9]`. Pins:
// Louis's run reads `null`, Eva's reads `[1, 4, 9]`.

/** Louis: the procedure argument is compound, and his map primitive can
 * only call primitives. => "null\n" */
public fun louisTranscript(): String = throw PendingSolution()

/** Eva: map defined in the object language applies the procedure
 * normally. => "[1, 4, 9]\n" */
public fun evaTranscript(): String = throw PendingSolution()
