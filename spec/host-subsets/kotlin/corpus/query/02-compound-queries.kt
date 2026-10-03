// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/02-compound-queries: and, or, and not over one data base.
// A query case is domain data (grammar 4.3): the functions below declare the
// data base, the rules, the query, and the variables to report, and the query
// engine answers it; `main` has nothing to run. The disjunction merges its
// branches one answer at a time, so the computer answer arrives between the
// programmers, and the negation drops the retired one.

fun facts(): List<QFact> =
    listOf(
        QFact(QList(listOf(QSym("job"), QSym("AlyssaP"), QSym("programmer")), null)),
        QFact(QList(listOf(QSym("job"), QSym("CyD"), QSym("programmer")), null)),
        QFact(QList(listOf(QSym("job"), QSym("LemE"), QSym("programmer")), null)),
        QFact(QList(listOf(QSym("job"), QSym("BitdiddleBen"), QSym("computer")), null)),
        QFact(QList(listOf(QSym("retired"), QSym("CyD")), null)),
    )

fun rules(): List<QRule> = emptyList()

fun query(): QQuery =
    QAnd(
        listOf(
            QOr(
                listOf(
                    QPattern(QList(listOf(QSym("job"), QVar("p"), QSym("programmer")), null)),
                    QPattern(QList(listOf(QSym("job"), QVar("p"), QSym("computer")), null)),
                ),
            ),
            QNot(QPattern(QList(listOf(QSym("retired"), QVar("p")), null))),
        ),
    )

fun variables(): List<QVar> = listOf(QVar("p"))

fun main() {}
