// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.VPair

/**
 * Exercise 3.14: `mystery` walks a chain, holding the old `cdr` in a
 * temporary while it repoints each pair's `cdr` at the cells it has
 * already passed. The question is what `mystery` does in general, and
 * what prints as `v` and `w` after `val w = mystery(v)` for
 * `v = (a b c d)`.
 */
public fun mystery(x: VPair): VPair = throw PendingSolution()
