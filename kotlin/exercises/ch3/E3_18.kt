// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 3.18: `containsCycle` examines a list and reports whether
 * taking successive cdrs would loop forever, that is, whether some cdr
 * chain leads back to a pair already visited. As in exercise 3.17, the
 * standard library has no identity-keyed set, so the visited-pairs
 * structure must be scanned with `===`.
 */
public fun containsCycle(x: Value): Boolean = throw PendingSolution()
