// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.VPair

/**
 * Exercise 3.13: `makeCycle` uses `lastPair` to close the chain `x` back
 * onto its own first pair. The question is what the structure `z` created
 * by `makeCycle` looks like, and what happens when `lastPair(z)` tries to
 * find an end that no longer exists.
 */
public fun makeCycle(x: VPair): VPair = throw PendingSolution()
