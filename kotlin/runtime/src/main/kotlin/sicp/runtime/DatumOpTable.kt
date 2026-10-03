// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.raise.Raise

/** A fallible host computation over native lesson data. */
public typealias DatumOp = Raise<DatumError>.(List<Datum>) -> Datum

/** Runtime-extensible dispatch over native datum keys. */
public class DatumOpTable {
    private val entries: MutableMap<DatumKey, MutableMap<DatumKey, DatumOp>> = HashMap()

    /** Install or replace one handler for an operation and tag. */
    public fun put(
        op: DatumKey,
        tag: DatumKey,
        handler: DatumOp,
    ) {
        entries.getOrPut(op) { HashMap() }[tag] = handler
    }

    /** Return the handler for the operation and tag, or null when absent. */
    public fun get(
        op: DatumKey,
        tag: DatumKey,
    ): DatumOp? = entries[op]?.get(tag)
}
