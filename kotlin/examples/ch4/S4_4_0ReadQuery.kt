// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4, the shared query plane for the section's listing
// tests: the typed constructors that build query domain data (the edition's
// `read` step is construction, never a parser over old source) and the
// pinned answer rendering of section 4.4.4.1 -- one line per variable,
// `?name = <rendered term>`.

package sicp.ch4.examples

import sicp.ch4.QFact
import sicp.ch4.QFrame
import sicp.ch4.QList
import sicp.ch4.QQuery
import sicp.ch4.QRule
import sicp.ch4.QSym
import sicp.ch4.QTerm
import sicp.ch4.QVar
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer

/** A symbol term. */
public fun sym(name: String): QSym = QSym(name)

/** A pattern variable. */
public fun variable(name: String): QVar = QVar(name)

/** A proper list term. */
public fun terms(vararg items: QTerm): QList = QList(items.toList(), null)

/** A person name: the query language carries names as symbol lists. */
public fun person(vararg names: String): QList = QList(names.map { sym(it) }, null)

/** An improper list term: the dotted shape `(a b . rest)`. */
public fun dotted(
    items: List<QTerm>,
    tail: QTerm,
): QList = QList(items, tail)

/** One fact. */
public fun fact(term: QTerm): QFact = QFact(term)

/** A pattern query over one term. */
public fun pattern(term: QTerm): QQuery = sicp.ch4.QPattern(term)

/** A rule with a conclusion and body. */
public fun rule(
    conclusion: QTerm,
    body: QQuery,
): QRule = QRule(conclusion, body)

/** The answer lines of one query: the pinned rendering per answer frame. */
public fun answerLines(
    driver: QueryDriver,
    query: QQuery,
    variables: List<QVar>,
): List<String> =
    driver
        .run(query, variables)
        .flatMap { frame: QFrame -> renderAnswer(frame, variables) }
        .toList()
