// SPDX-License-Identifier: GPL-3.0-only
// Section 4.2.3 shared fixtures: an explicit thunk spine for lazy lists.
// The guest List surface materializes lazy values on access, so head-only
// walks use this spine and force exactly the tails they demand.

package sicp.ch4.solutions

/** The section's lazy-list data: atoms, end, and cells with delayed tails. */
internal val LAZY_DATA_SOURCE: String =
    """
sealed interface LazyData

data class LazyAtom(val text: String) : LazyData

data object LazyEnd : LazyData
data class LazyCell(val head: LazyData, val tail: Thunk<LazyData>) : LazyData

fun renderAtom(node: LazyData): String = if (node is LazyAtom) node.text else "()"

fun explode(): LazyData {
    1L / 0L
    return LazyEnd
}
    """.trimIndent()
