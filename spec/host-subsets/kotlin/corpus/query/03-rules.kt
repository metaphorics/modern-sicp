// SPDX-License-Identifier: GPL-3.0-only
// Corpus case query/03-rules: rule application

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

fun runBody(body: QQuery, facts: List<QTerm>, bound: QTerm): QTerm? =
    when {
        body is QPattern -> find(facts, body.term)
        body is QAnd -> {
            var answer: QTerm? = bound
            for (part in body.parts) {
                answer = runBody(part, facts, answer ?: bound) ?: return null
            }
            answer
        }
        else -> bound
    }

fun applyRule(rule: QRule, facts: List<QTerm>, pattern: QTerm): QTerm? {
    val bound = unify(rule.conclusion, pattern) ?: return null
    return runBody(rule.body, facts, bound)
}

fun main() {
    val facts = listOf(QList(listOf(QSym("son"), QSym("Adam"), QSym("Cain")), null))
    val rules = listOf(
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
    val pattern = QList(listOf(QSym("grandson"), QSym("Adam"), QVar("x")), null)
    var answer: QTerm? = null
    for (rule in rules) {
        val bound = applyRule(rule, facts, pattern)
        if (bound != null) answer = bound
    }
    println("?x = " + render(answer ?: QSym("none")))
}
