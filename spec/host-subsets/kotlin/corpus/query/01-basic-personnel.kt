// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/01-basic-personnel: fact retrieval with unification
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

fun main() {
    val facts = listOf(
        QList(listOf(QSym("address"), QSym("BitdiddleBen"), QSym("RidgeRoad")), null),
        QList(listOf(QSym("salary"), QSym("BitdiddleBen"), QSym("60000")), null),
    )
    val answer: QTerm = find(facts, QList(listOf(QSym("address"), QSym("BitdiddleBen"), QVar("x")), null)) ?: QSym("none")
    println("?x = " + render(answer))
}
