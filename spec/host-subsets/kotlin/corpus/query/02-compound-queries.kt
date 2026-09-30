// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/02-compound-queries: conjunction and disjunction of queries
fun unify(a: QTerm, b: QTerm): QTerm? =
    when {
        a is QVar -> b
        b is QVar -> a
        a is QSym && b is QSym -> if (a.name == b.name) a else null
        a is QList && b is QList -> unifyList(a, b)
        else -> null
    }

fun unifyList(a: QList, b: QList): QTerm? {
    if (a.items.size != b.items.size) return null
    var result: QTerm? = null
    var index = 0
    while (index < a.items.size) {
        result = unify(a.items.get(index), b.items.get(index)) ?: return null
        index = index + 1
    }
    return result
}

fun render(t: QTerm): String =
    when {
        t is QSym -> t.name
        t is QVar -> "?" + t.name
        t is QList -> renderList(t)
        else -> "?"
    }

fun renderList(xs: QList): String {
    var out = "["
    var first = true
    for (item in xs.items) {
        if (first) {
            first = false
        } else {
            out = out + ", "
        }
        out = out + render(item)
    }
    return out + "]"
}

fun find(facts: List<QTerm>, pattern: QTerm): QTerm? {
    for (fact in facts) {
        val bound = unify(fact, pattern)
        if (bound != null) return bound
    }
    return null
}

fun runQuery(query: QQuery, facts: List<QTerm>): QTerm? =
    when {
        query is QPattern -> find(facts, query.term)
        query is QAnd -> runAnd(query, facts)
        query is QOr -> runOr(query, facts)
        else -> null
    }

fun runAnd(query: QAnd, facts: List<QTerm>): QTerm? {
    var answer: QTerm? = null
    for (part in query.parts) {
        answer = runQuery(part, facts) ?: return null
    }
    return answer
}

fun runOr(query: QOr, facts: List<QTerm>): QTerm? {
    for (part in query.parts) {
        val answer = runQuery(part, facts)
        if (answer != null) return answer
    }
    return null
}

fun main() {
    val facts = listOf(
        QList(listOf(QSym("job"), QSym("BitdiddleBen"), QSym("computer")), null),
        QList(listOf(QSym("job"), QSym("AlyssaP"), QSym("programmer")), null),
    )
    val compound: QQuery = QAnd(
        listOf(
            QPattern(QList(listOf(QSym("job"), QSym("BitdiddleBen"), QVar("x")), null)),
            QPattern(QList(listOf(QSym("job"), QSym("AlyssaP"), QVar("y")), null)),
        ),
    )
    val answer: QTerm = runQuery(compound, facts) ?: QSym("none")
    println("?y = " + render(answer))
}
