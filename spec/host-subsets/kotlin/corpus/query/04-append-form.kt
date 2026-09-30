// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/04-append-form: rules with recursive structure.
// A query case is domain data (grammar 4.3): the functions below declare the
// data base, the rules, the query, and the variables to report, and the query
// engine answers it; `main` has nothing to run. The append relation is two
// rules: appending to the empty list gives the second list, and appending a
// longer list peels one element and recurs; the query asks for the whole.

fun facts(): List<QFact> = emptyList()

fun rules(): List<QRule> =
    listOf(
        QRule(
            QList(
                listOf(QSym("append"), QList(emptyList(), null), QVar("y"), QVar("y")),
                null,
            ),
            QAnd(emptyList()),
        ),
        QRule(
            QList(
                listOf(
                    QSym("append"),
                    QList(listOf(QVar("u")), QVar("v")),
                    QVar("y"),
                    QList(listOf(QVar("u")), QVar("z")),
                ),
                null,
            ),
            QPattern(QList(listOf(QSym("append"), QVar("v"), QVar("y"), QVar("z")), null)),
        ),
    )

fun query(): QQuery =
    QPattern(
        QList(
            listOf(
                QSym("append"),
                QList(listOf(QSym("a"), QSym("b")), null),
                QList(listOf(QSym("c")), null),
                QVar("z"),
            ),
            null,
        ),
    )

fun variables(): List<QVar> = listOf(QVar("z"))

fun main() {}
