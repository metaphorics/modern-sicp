// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.85

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.85: design `drop`, which lowers a tower value to its
 * simplest representation. The test for lowerable: `project` the value,
 * raise the projection back, and compare with the generic `equ?` -- if
 * the round trip reproduces the value, it can be dropped. `1.5 + 0i`
 * drops as far as `real`, `1 + 0i` as far as `integer`, `2 + 3i` not at
 * all. Finally `applyGenericDropping` rewrites the raising dispatch of
 * exercise 2.84 so every answer leaves the tower simplified.
 */
public fun ex_2_85(): List<String> = throw PendingSolution()
