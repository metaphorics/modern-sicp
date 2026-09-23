// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentMap
import kotlinx.collections.immutable.persistentMapOf

/**
 * One frame plus the frame it extends. Names bind [Value]s; the frame is a
 * persistent map behind a `var`, so `define` rebinds this node's map and
 * every holder of the node sees it — the book's `define` and `set!` — while
 * [snapshot] gives 4.3's backtracking evaluator an isolated copy.
 */
public class Env(
    /** This frame's bindings; `define` rebinds it, `set!` rebinds the map
     * of the frame that owns the name. */
    public var frame: PersistentMap<String, Value>,
    /** The frame this one extends, if any. */
    public val parent: Env?,
) {
    public companion object {
        /** A fresh frame with no outer frame: the global environment of 4.1. */
        public fun global(): Env = Env(persistentMapOf(), null)

        /** A fresh frame extending `parent`: the book's `extend-environment`. */
        public fun child(parent: Env): Env = Env(persistentMapOf(), parent)

        /** A fresh frame binding `names` to `values` over `parent`; a
         * non-null `rest` binds the leftover values as a list, the book's
         * dotted parameter. */
        context(r: Raise<SchemeError>)
        public fun extend(
            names: List<String>,
            values: List<Value>,
            parent: Env,
            rest: String? = null,
        ): Env {
            if (rest == null && names.size != values.size) {
                r.raise(SchemeError.WrongArity("extend", names.size.toString(), values.size))
            }
            if (rest != null && values.size < names.size) {
                r.raise(SchemeError.WrongArity("extend", "at least ${names.size}", values.size))
            }
            val bound = names.zip(values).toMap(persistentMapOf<String, Value>().builder())
            val env = Env(bound.build(), parent)
            if (rest != null) {
                env.frame = env.frame.putting(rest, vlist(values.drop(names.size)))
            }
            return env
        }
    }

    /** The book's `define-variable!`: binds (or rebinds) `name` in this
     * frame, shadowing any outer binding without touching it. */
    public fun define(
        name: String,
        value: Value,
    ) {
        frame = frame.putting(name, value)
    }

    /** The book's `lookup-variable-value`: walks the chain outwards. */
    context(r: Raise<SchemeError>)
    public fun lookup(name: String): Value {
        var cursor: Env? = this
        while (cursor != null) {
            val hit = cursor.frame[name]
            if (hit != null) return hit
            cursor = cursor.parent
        }
        r.raise(SchemeError.Unbound(name))
    }

    /** The book's `set-variable-value!`: rebinds the nearest existing
     * binding of `name` on the chain without creating one. */
    context(r: Raise<SchemeError>)
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
        r.raise(SchemeError.Unbound(name))
    }

    /** A deep copy of the chain: new nodes sharing the same persistent
     * maps, so `define` and `set!` on the copy never reach the original.
     * The nondeterministic evaluator snapshots at each `amb` choice so an
     * abandoned branch's assignments die with it. */
    public fun snapshot(): Env = Env(frame, parent?.snapshot())
}
