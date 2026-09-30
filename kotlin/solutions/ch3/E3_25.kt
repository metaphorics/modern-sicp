// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

/**
 * A multi-key table backed by nested host-data pair chains. Each record's
 * first field stores a key; its second field stores a value or a subtable.
 */
public class KeyListTable {
    private val header: PairCell = pair(Empty, Empty)

    public fun lookup(keys: List<Datum>): Datum? = lookupIn(keys, header.second)

    public fun insert(
        keys: List<Datum>,
        value: Datum,
    ) {
        insertIn(keys, value, header)
    }

    private fun lookupIn(
        keys: List<Datum>,
        records: Datum,
    ): Datum? {
        val record = findRecord(keys.first(), records) ?: return null
        return if (keys.size == 1) {
            record.second
        } else {
            lookupIn(keys.subList(1, keys.size), record.second)
        }
    }

    private fun insertIn(
        keys: List<Datum>,
        value: Datum,
        level: PairCell,
    ) {
        val record = findRecord(keys.first(), level.second)
        if (keys.size == 1) {
            if (record == null) {
                level.second = pair(pair(keys.first(), value), level.second)
                return
            }
            record.second = value
            return
        }
        val rest = keys.subList(1, keys.size)
        if (record != null) {
            insertIn(rest, value, record)
            return
        }
        val subtable = pair(keys.first(), Empty)
        level.second = pair(subtable, level.second)
        insertIn(rest, value, subtable)
    }

    private fun findRecord(
        key: Datum,
        records: Datum,
    ): PairCell? {
        if (records !is PairCell) {
            return null
        }
        val record = records.first
        return if (record is PairCell && structurallyEqual(key, record.first)) {
            record
        } else {
            findRecord(key, records.second)
        }
    }
}
