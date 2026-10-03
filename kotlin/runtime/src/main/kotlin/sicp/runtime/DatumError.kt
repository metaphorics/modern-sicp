// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

/** Typed error categories for operations on native lesson data. */
public sealed class DatumError {
    public abstract val message: String

    public abstract val offending: List<Datum>

    public data class TypeMismatch(
        override val message: String,
        override val offending: List<Datum> = emptyList(),
    ) : DatumError()

    public data class BadDatum(
        override val message: String,
        override val offending: List<Datum> = emptyList(),
    ) : DatumError()
}
