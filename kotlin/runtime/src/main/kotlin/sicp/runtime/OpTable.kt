// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/**
 * The book's operation-and-type dispatch table (2.4.2): `put` installs an
 * [Op] under an `(operation, tag)` pair, `get` retrieves it, and a later
 * install overwrites an earlier one. Dispatch is data here, not `when`
 * arms, because sealed dispatch closes the type set at compile time while
 * 2.73 through 2.97 extend the table at runtime by `install`ing packages
 * that may not exist when the caller compiles.
 */
public class OpTable {
    private val entries: MutableMap<Key, MutableMap<Key, Op>> = HashMap()

    /** The book's `put`: installs `handler` under `(op, tag)`, overwriting
     * an earlier install of exactly that pair. */
    public fun put(
        op: Key,
        tag: Key,
        handler: Op,
    ) {
        entries.getOrPut(op) { HashMap() }[tag] = handler
    }

    /** The book's `get`: the handler under `(op, tag)`, or the absent
     * option on a miss — never a false-ish sentinel. */
    public fun get(
        op: Key,
        tag: Key,
    ): Op? = entries[op]?.get(tag)
}
