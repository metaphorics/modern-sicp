// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/01-basic-personnel: fact retrieval with unification.
// A query case is domain data (grammar 4.3): the functions below declare the
// data base, the rules, the query, and the variables to report, and the query
// engine answers it; `main` has nothing to run.

fun facts(): List<QFact> =
    listOf(
        QFact(QList(listOf(QSym("address"), QSym("BitdiddleBen"), QSym("RidgeRoad")), null)),
        QFact(QList(listOf(QSym("salary"), QSym("BitdiddleBen"), QSym("60000")), null)),
    )

fun rules(): List<QRule> = emptyList()

fun query(): QQuery = QPattern(QList(listOf(QSym("address"), QSym("BitdiddleBen"), QVar("x")), null))

fun variables(): List<QVar> = listOf(QVar("x"))

fun main() {}
