// SPDX-License-Identifier: GPL-3.0-only
// Section 4.4 fixtures: the Microshaft data base and the section's prose
// rules as typed query constructors, plus the answer helpers the demos
// share. Query programs are domain data (grammar 4.3); nothing here is
// re-parsed from old source.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QFact
import sicp.ch4.QList
import sicp.ch4.QNot
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QQuery
import sicp.ch4.QRule
import sicp.ch4.QSym
import sicp.ch4.QTerm
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer
import sicp.ch4.renderTerm

/** One symbol term. */
internal fun sym(name: String): QSym = QSym(name)

/** One pattern variable. */
internal fun v(name: String): QVar = QVar(name)

/** One proper list term. */
internal fun list(vararg items: QTerm): QList = QList(items.toList(), null)

/** One improper list term: [items..., tail]. */
internal fun improper(
    items: List<QTerm>,
    tail: QTerm,
): QList = QList(items, tail)

/** The Microshaft data base of 4.4.1, in the book's order. */
internal fun microshaftDatabase(): QueryDatabase {
    val db = QueryDatabase()
    db.assertFact(
        QFact(
            list(sym("address"), list(sym("Bitdiddle"), sym("Ben")), list(sym("Slumerville"), list(sym("Ridge"), sym("Road")), sym("10"))),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Bitdiddle"), sym("Ben")), list(sym("computer"), sym("wizard")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Bitdiddle"), sym("Ben")), sym("60000"))))
    db.assertFact(
        QFact(
            list(
                sym("address"),
                list(sym("Hacker"), sym("Alyssa"), sym("P")),
                list(sym("Cambridge"), list(sym("Mass"), sym("Ave")), sym("78")),
            ),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Hacker"), sym("Alyssa"), sym("P")), list(sym("computer"), sym("programmer")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Hacker"), sym("Alyssa"), sym("P")), sym("40000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Hacker"), sym("Alyssa"), sym("P")), list(sym("Bitdiddle"), sym("Ben")))))
    db.assertFact(
        QFact(
            list(
                sym("address"),
                list(sym("Fect"), sym("Cy"), sym("D")),
                list(sym("Cambridge"), list(sym("Ames"), sym("Street")), sym("3")),
            ),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Fect"), sym("Cy"), sym("D")), list(sym("computer"), sym("programmer")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Fect"), sym("Cy"), sym("D")), sym("35000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Fect"), sym("Cy"), sym("D")), list(sym("Bitdiddle"), sym("Ben")))))
    db.assertFact(
        QFact(
            list(
                sym("address"),
                list(sym("Tweakit"), sym("Lem"), sym("E")),
                list(sym("Boston"), list(sym("Bay"), sym("State"), sym("Road")), sym("22")),
            ),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Tweakit"), sym("Lem"), sym("E")), list(sym("computer"), sym("technician")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Tweakit"), sym("Lem"), sym("E")), sym("25000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Tweakit"), sym("Lem"), sym("E")), list(sym("Bitdiddle"), sym("Ben")))))
    db.assertFact(
        QFact(
            list(
                sym("address"),
                list(sym("Reasoner"), sym("Louis")),
                list(sym("Slumerville"), list(sym("Pine"), sym("Tree"), sym("Road")), sym("80")),
            ),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Reasoner"), sym("Louis")), list(sym("computer"), sym("programmer"), sym("trainee")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Reasoner"), sym("Louis")), sym("30000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Reasoner"), sym("Louis")), list(sym("Hacker"), sym("Alyssa"), sym("P")))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Bitdiddle"), sym("Ben")), list(sym("Warbucks"), sym("Oliver")))))
    db.assertFact(
        QFact(
            list(sym("address"), list(sym("Warbucks"), sym("Oliver")), list(sym("Swellesley"), list(sym("Top"), sym("Heap"), sym("Road")))),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Warbucks"), sym("Oliver")), list(sym("administration"), sym("big"), sym("wheel")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Warbucks"), sym("Oliver")), sym("150000"))))
    db.assertFact(
        QFact(list(sym("address"), list(sym("Scrooge"), sym("Eben")), list(sym("Weston"), list(sym("Shady"), sym("Lane")), sym("10")))),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Scrooge"), sym("Eben")), list(sym("accounting"), sym("chief"), sym("accountant")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Scrooge"), sym("Eben")), sym("75000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Scrooge"), sym("Eben")), list(sym("Warbucks"), sym("Oliver")))))
    db.assertFact(
        QFact(
            list(
                sym("address"),
                list(sym("Cratchet"), sym("Robert")),
                list(sym("Allston"), list(sym("N"), sym("Harvard"), sym("Street")), sym("16")),
            ),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Cratchet"), sym("Robert")), list(sym("accounting"), sym("scrivener")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Cratchet"), sym("Robert")), sym("18000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Cratchet"), sym("Robert")), list(sym("Scrooge"), sym("Eben")))))
    db.assertFact(
        QFact(
            list(sym("address"), list(sym("Aull"), sym("DeWitt")), list(sym("Slumerville"), list(sym("Onion"), sym("Square")), sym("5"))),
        ),
    )
    db.assertFact(QFact(list(sym("job"), list(sym("Aull"), sym("DeWitt")), list(sym("administration"), sym("secretary")))))
    db.assertFact(QFact(list(sym("salary"), list(sym("Aull"), sym("DeWitt")), sym("25000"))))
    db.assertFact(QFact(list(sym("supervisor"), list(sym("Aull"), sym("DeWitt")), list(sym("Warbucks"), sym("Oliver")))))
    db.assertFact(QFact(list(sym("can-do-job"), list(sym("computer"), sym("wizard")), list(sym("computer"), sym("programmer")))))
    db.assertFact(QFact(list(sym("can-do-job"), list(sym("computer"), sym("wizard")), list(sym("computer"), sym("technician")))))
    db.assertFact(
        QFact(list(sym("can-do-job"), list(sym("computer"), sym("programmer")), list(sym("computer"), sym("programmer"), sym("trainee")))),
    )
    db.assertFact(
        QFact(
            list(sym("can-do-job"), list(sym("administration"), sym("secretary")), list(sym("administration"), sym("big"), sym("wheel"))),
        ),
    )
    return db
}

/** The section's prose rules, in the book's order. */
internal fun addProseRules(db: QueryDatabase) {
    db.addRule(
        QRule(
            list(sym("lives-near"), v("person-1"), v("person-2")),
            QAnd(
                listOf(
                    QPattern(list(sym("address"), v("person-1"), improper(listOf(v("town")), v("rest-1")))),
                    QPattern(list(sym("address"), v("person-2"), improper(listOf(v("town")), v("rest-2")))),
                    QNot(QPattern(list(sym("same"), v("person-1"), v("person-2")))),
                ),
            ),
        ),
    )
    db.addRule(QRule(list(sym("same"), v("x"), v("x")), QAnd(emptyList())))
    db.addRule(
        QRule(
            list(sym("wheel"), v("person")),
            QAnd(
                listOf(
                    QPattern(list(sym("supervisor"), v("middle-manager"), v("person"))),
                    QPattern(list(sym("supervisor"), v("x"), v("middle-manager"))),
                ),
            ),
        ),
    )
    db.addRule(
        QRule(
            list(sym("outranked-by"), v("staff-person"), v("boss")),
            QOr(
                listOf(
                    QPattern(list(sym("supervisor"), v("staff-person"), v("boss"))),
                    QAnd(
                        listOf(
                            QPattern(list(sym("supervisor"), v("staff-person"), v("middle-manager"))),
                            QPattern(list(sym("outranked-by"), v("middle-manager"), v("boss"))),
                        ),
                    ),
                ),
            ),
        ),
    )
    db.addRule(QRule(list(sym("append-to-form"), list(), v("y"), v("y")), QAnd(emptyList())))
    db.addRule(
        QRule(
            list(sym("append-to-form"), improper(listOf(v("u")), v("v")), v("y"), improper(listOf(v("u")), v("z"))),
            QPattern(list(sym("append-to-form"), v("v"), v("y"), v("z"))),
        ),
    )
}

/** A fresh data base with Microshaft and the prose rules loaded. */
internal fun microshaftSystem(): QueryDatabase {
    val db = microshaftDatabase()
    addProseRules(db)
    return db
}

/** The numeric reading of a symbolic term (salaries render as their digits). */
internal fun termLong(term: QTerm): Long = (term as? QSym)?.name?.toLong() ?: 0L

/** The plain name of a person term (its symbols joined by spaces). */
internal fun personKey(term: QTerm): String =
    if (term is QList) {
        term.items.joinToString(" ") { renderTerm(it) }
    } else {
        renderTerm(term)
    }

/** The answer lines of one query under [driver], in stream order. */
internal fun answerLines(
    driver: QueryDriver,
    query: QQuery,
    variables: List<QVar>,
): List<String> = driver.run(query, variables).flatMap { renderAnswer(it, variables) }.toList()
