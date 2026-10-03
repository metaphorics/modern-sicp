// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentMap
import kotlinx.collections.immutable.persistentMapOf

/** A lexical frame whose bindings contain host data and preserve datum identity. */
public class BindingFrame private constructor(
    private var bindings: PersistentMap<String, Datum>,
    private val parent: BindingFrame?,
) {
    public companion object {
        /** Create an empty outermost frame. */
        public fun root(): BindingFrame = BindingFrame(persistentMapOf(), null)

        /** Create an empty frame that searches [parent] after its own bindings. */
        public fun child(parent: BindingFrame): BindingFrame = BindingFrame(persistentMapOf(), parent)
    }

    /** Bind or rebind [name] in this frame only. */
    public fun bind(
        name: String,
        value: Datum,
    ) {
        bindings = bindings.putting(name, value)
    }

    /** Read the nearest binding, raising [DatumError.BadDatum] when absent. */
    context(r: Raise<DatumError>)
    public fun read(name: String): Datum {
        var cursor: BindingFrame? = this
        while (cursor != null) {
            val value = cursor.bindings[name]
            if (value != null) return value
            cursor = cursor.parent
        }
        r.raise(DatumError.BadDatum(name))
    }

    /** Rebind the nearest existing binding without creating a new one. */
    context(r: Raise<DatumError>)
    public fun assign(
        name: String,
        value: Datum,
    ) {
        var cursor: BindingFrame? = this
        while (cursor != null) {
            if (cursor.bindings.containsKey(name)) {
                cursor.bindings = cursor.bindings.putting(name, value)
                return
            }
            cursor = cursor.parent
        }
        r.raise(DatumError.BadDatum(name))
    }

    /** Copy the frame chain while retaining every bound datum reference. */
    public fun snapshot(): BindingFrame = BindingFrame(bindings, parent?.snapshot())
}
