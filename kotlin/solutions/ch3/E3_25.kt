// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.equalv
import sicp.runtime.setCdr

/**
 * The general table of exercise 3.25: lookup and insert take a whole
 * list of keys. Each level is the section's headed list -- a header
 * pair whose car names the level (*table* at the top, the subtable's
 * key below) and whose cdr is the chain of records. A key names either
 * a value or a subtable, following the book.
 */
public class KeyListTable {
    private val header: VPair = VPair(VSym("*table*"), VNil)

    public fun lookup(keys: List<Value>): Value? = lookupIn(keys, header.cdr)

    public fun insert(
        keys: List<Value>,
        value: Value,
    ) {
        insertIn(keys, value, header)
    }

    private fun lookupIn(
        keys: List<Value>,
        records: Value,
    ): Value? {
        val record = assoc(keys.first(), records) ?: return null
        return if (keys.size == 1) {
            record.cdr
        } else {
            lookupIn(keys.subList(1, keys.size), record.cdr)
        }
    }

    private fun insertIn(
        keys: List<Value>,
        value: Value,
        level: VPair,
    ) {
        val record = assoc(keys.first(), level.cdr)
        if (keys.size == 1) {
            if (record == null) {
                level.setCdr(cons(VPair(keys.first(), value), level.cdr))
                return
            }
            record.setCdr(value)
            return
        }
        val rest = keys.subList(1, keys.size)
        if (record != null) {
            insertIn(rest, value, record)
            return
        }
        val sub = VPair(keys.first(), VNil)
        level.setCdr(cons(sub, level.cdr))
        insertIn(rest, value, sub)
    }

    private fun assoc(
        key: Value,
        records: Value,
    ): VPair? {
        if (records !is VPair) {
            return null
        }
        val record = records.car
        return if (record is VPair && equalv(key, record.car)) {
            record
        } else {
            assoc(key, records.cdr)
        }
    }
}
