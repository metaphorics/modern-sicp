// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/** Transitional operation table for the legacy [Value] and [Key] APIs. */
public class OpTable {
    private val entries: MutableMap<Key, MutableMap<Key, Op>> = HashMap()

    public fun put(
        op: Key,
        tag: Key,
        handler: Op,
    ) {
        entries.getOrPut(op) { HashMap() }[tag] = handler
    }

    public fun get(
        op: Key,
        tag: Key,
    ): Op? = entries[op]?.get(tag)
}
