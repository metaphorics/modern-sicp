// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.49

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * A named shared record whose contents are locked for the duration of a
 * piece of work.
 */
public class SharedRecord(
    public val name: String,
) {
    /** Run `work` holding this record's lock. */
    public suspend fun <A> withRecord(work: suspend () -> A): A = throw PendingSolution()
}

/**
 * The directory of records. A process must hold the directory while it
 * discovers which record it needs next, which is exactly the situation
 * where acquisition order cannot be fixed in advance.
 */
public class ResourceDirectory {
    /** Install a record so later lookups can find it. */
    public fun install(record: SharedRecord): Unit = throw PendingSolution()

    /** Run `work` holding the directory, the one lock every lookup needs. */
    public suspend fun <A> withDirectory(work: suspend () -> A): A = throw PendingSolution()

    /** Look up a record by name; the caller must hold the directory. */
    public fun find(name: String): SharedRecord? = throw PendingSolution()
}

/**
 * The scenario of the exercise: one process holds the directory and
 * waits for a record; the other holds that record and waits for the
 * directory to name its next resource. Answers what became of them.
 */
public suspend fun directoryDeadlockDemo(): String = throw PendingSolution()
