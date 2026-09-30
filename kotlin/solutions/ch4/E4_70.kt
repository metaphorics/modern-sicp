// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.70: the snapshot discipline for assertion.

package sicp.ch4.solutions

import sicp.ch4.QFact
import sicp.ch4.QPattern
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.70: why `add-assertion!` binds the old collection first.
// Appending while a consumer still walks the collection aliases the
// walk to the mutation: the tail the walk has yet to read is the cell
// the assignment just rebound. The `let` snapshots the old facts into
// a materialized list before the new fact lands, so the snapshot keeps
// answering exactly the old items while the data base grows by one.

/** The two-item data base the assertion extends. */
internal fun itemDatabase(): QueryDatabase {
    val db = QueryDatabase()
    db.assertFact(QFact(list(sym("item"), sym("a"))))
    db.assertFact(QFact(list(sym("item"), sym("b"))))
    return db
}

/** Snapshot, assert, and re-read: the snapshot is untouched by the view. */
public fun letPurposeDemo(): List<String> {
    val db = itemDatabase()
    val driver = QueryDriver.streaming(db)
    val query = QPattern(list(sym("item"), v("x")))
    val snapshot = answerLines(driver, query, listOf(v("x")))
    db.assertFact(QFact(list(sym("item"), sym("c"))))
    val after = answerLines(driver, query, listOf(v("x")))
    return snapshot + after + listOf("snapshot unchanged: ${snapshot == listOf("?x = a", "?x = b")}")
}
