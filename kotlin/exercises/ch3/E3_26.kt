// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PendingSolution

/**
 * Exercise 3.26: a table whose (key, value) records are organized as a
 * binary tree instead of an unordered scan, so lookup and insert take
 * steps proportional to the tree's depth. Define the key ordering your
 * table assumes: the reference solution compares symbols
 * lexicographically by name and integers numerically, and refuses a
 * comparison between the two.
 */
public class TreeTable {
    public fun lookup(key: Datum): Datum? = throw PendingSolution()

    public fun insert(
        key: Datum,
        value: Datum,
    ): Unit = throw PendingSolution()
}
