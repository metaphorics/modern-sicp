// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.16: scan out internal definitions. Alyssa P. Hacker proposes
 * `scanOutDefines`: a procedure body containing definition statements is
 * rewritten into one binding form that reserves every defined name with
 * the unassigned marker, followed by assignments in source order and the
 * rest of the body; a body without internal definitions is unchanged.
 * Part (b): installing the scan at procedure creation scans once per
 * procedure, while installing it at application would re-scan the body on
 * every call -- creation is the better place. Part (c): reading a name
 * still holding the marker raises the typed premature-read fault (this
 * edition's `UnassignedRead`), while the unscanned kernel answers its
 * null for the same read.
 *
 * Expected answers: the mutual even?/odd? recursion answers `true` on 10
 * under the scan-out; the premature read answers `UnassignedRead`; the
 * unscanned probe answers `null`.
 */
public fun mutualRecursionTranscript(): String = throw PendingSolution()

/** The scanned premature read raises the typed fault. => "UnassignedRead" */
public fun prematureReadCategory(): String = throw PendingSolution()

/** The unscanned kernel answers its own fault convention. => "null" */
public fun basePrematureCategory(): String = throw PendingSolution()
