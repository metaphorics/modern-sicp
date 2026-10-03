// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentMap
import kotlinx.collections.immutable.persistentMapOf

/** Transitional legacy environment over [Value]. Use [BindingFrame] for host data. */
public class Env(
    public var frame: PersistentMap<String, Value>,
    public val parent: Env?,
) {
    public companion object {
        public fun global(): Env = Env(persistentMapOf(), null)

        public fun child(parent: Env): Env = Env(persistentMapOf(), parent)

        context(r: Raise<DataError>)
        public fun extend(
            names: List<String>,
            values: List<Value>,
            parent: Env,
            rest: String? = null,
        ): Env {
            if (rest == null && names.size != values.size) {
                r.raise(DataError.BadDatum("extend: expected ${names.size} arguments, got ${values.size}"))
            }
            if (rest != null && values.size < names.size) {
                r.raise(DataError.BadDatum("extend: expected at least ${names.size} arguments, got ${values.size}"))
            }
            val bound = names.zip(values).toMap(persistentMapOf<String, Value>().builder())
            val env = Env(bound.build(), parent)
            if (rest != null) env.frame = env.frame.putting(rest, vlist(values.drop(names.size)))
            return env
        }
    }

    public fun define(
        name: String,
        value: Value,
    ) {
        frame = frame.putting(name, value)
    }

    context(r: Raise<DataError>)
    public fun lookup(name: String): Value {
        var cursor: Env? = this
        while (cursor != null) {
            val value = cursor.frame[name]
            if (value != null) return value
            cursor = cursor.parent
        }
        r.raise(DataError.BadDatum(name))
    }

    context(r: Raise<DataError>)
    public fun set(
        name: String,
        value: Value,
    ) {
        var cursor: Env? = this
        while (cursor != null) {
            if (cursor.frame.containsKey(name)) {
                cursor.frame = cursor.frame.putting(name, value)
                return
            }
            cursor = cursor.parent
        }
        r.raise(DataError.BadDatum(name))
    }

    public fun snapshot(): Env = Env(frame, parent?.snapshot())
}
