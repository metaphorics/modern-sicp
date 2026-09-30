// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/03-rules: rules as derived assertions.
// A query case is domain data (grammar 4.3): the functions below declare the
// data base, the rules, the query, and the variables to report, and the query
// engine answers it; `main` has nothing to run. The grandson rule holds no
// facts of its own: each answer is derived from two son assertions.

fun facts(): List<QFact> =
    listOf(
        QFact(QList(listOf(QSym("son"), QSym("Adam"), QSym("Cain")), null)),
        QFact(QList(listOf(QSym("son"), QSym("Cain"), QSym("Enoch")), null)),
        QFact(QList(listOf(QSym("son"), QSym("Enoch"), QSym("Irad")), null)),
    )

fun rules(): List<QRule> =
    listOf(
        QRule(
            QList(listOf(QSym("grandson"), QVar("g"), QVar("s")), null),
            QAnd(
                listOf(
                    QPattern(QList(listOf(QSym("son"), QVar("g"), QVar("f")), null)),
                    QPattern(QList(listOf(QSym("son"), QVar("f"), QVar("s")), null)),
                ),
            ),
        ),
    )

fun query(): QQuery = QPattern(QList(listOf(QSym("grandson"), QSym("Adam"), QVar("x")), null))

fun variables(): List<QVar> = listOf(QVar("x"))

fun main() {}
