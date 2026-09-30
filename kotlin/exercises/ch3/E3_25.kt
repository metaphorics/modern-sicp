// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.PendingSolution

/**
 * Exercise 3.25: a table whose lookup and insert take a whole list of
 * keys. Each key but the last names a subtable on the current level's
 * backbone; the last names a record. Values under different numbers of
 * keys live in different branches of the one backbone -- a key names
 * either a value or a subtable, following the book.
 */
public class KeyListTable {
    public fun lookup(keys: List<Datum>): Datum? = throw PendingSolution()

    public fun insert(
        keys: List<Datum>,
        value: Datum,
    ): Unit = throw PendingSolution()
}
