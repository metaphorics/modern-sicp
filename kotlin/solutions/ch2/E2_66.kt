// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.66

package sicp.ch2.exercises

/** One data-base record: a key plus a payload value. */
public data class Record(
    val key: Long,
    val value: String,
)

/** A binary tree of [Record]s, ordered by [Record.key]. */
public sealed interface RecordTree {
    public data object Empty : RecordTree

    public data class Node(
        val record: Record,
        val left: RecordTree,
        val right: RecordTree,
    ) : RecordTree
}

/** The book's `lookup`: `Θ(log n)` on a balanced [RecordTree]. */
public fun lookup(
    givenKey: Long,
    tree: RecordTree,
): Record? =
    when (tree) {
        is RecordTree.Empty -> {
            null
        }

        is RecordTree.Node -> {
            when {
                givenKey == tree.record.key -> tree.record
                givenKey < tree.record.key -> lookup(givenKey, tree.left)
                else -> lookup(givenKey, tree.right)
            }
        }
    }

private fun balancedRecordTree(records: List<Record>): RecordTree {
    if (records.isEmpty()) return RecordTree.Empty
    val mid = records.size / 2
    return RecordTree.Node(
        records[mid],
        balancedRecordTree(records.subList(0, mid)),
        balancedRecordTree(records.subList(mid + 1, records.size)),
    )
}

/** A small sample database, ordered by key. */
public val sampleDb: RecordTree =
    balancedRecordTree(
        listOf(Record(1L, "alice"), Record(3L, "bob"), Record(5L, "carol"), Record(7L, "dave"), Record(9L, "erin"))
            .sortedBy { it.key },
    )

/** `lookup` of key `7` in [sampleDb]. */
public fun ex_2_66(): String? = lookup(7L, sampleDb)?.value
