// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.raise.Raise

/** Typed errors raised by the legacy [Value] data helpers. */
public sealed class DataError {
    public abstract val message: String

    public abstract val offending: List<Value>

    public data class TypeMismatch(
        override val message: String,
        override val offending: List<Value> = emptyList(),
    ) : DataError()

    public data class BadDatum(
        override val message: String,
        override val offending: List<Value> = emptyList(),
    ) : DataError()
}

/** Legacy fallible operation shape for [Value] and [OpTable] callers. */
public typealias Op = Raise<DataError>.(List<Value>) -> Value
