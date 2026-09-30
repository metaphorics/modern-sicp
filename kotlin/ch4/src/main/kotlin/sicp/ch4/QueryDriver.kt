// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

/** The query DSL vocabulary of grammar section 4.3, declared verbatim as
 * host domain data. Guest programs build the same shapes through admitted
 * constructor calls; this plane serves host exercise programs and tests. */
public sealed interface QTerm

public data class QSym(
    val name: String,
) : QTerm

public data class QVar(
    val name: String,
) : QTerm

public data class QList(
    val items: List<QTerm>,
    val tail: QTerm?,
) : QTerm

public sealed interface QQuery

public data class QPattern(
    val term: QTerm,
) : QQuery

public data class QAnd(
    val parts: List<QQuery>,
) : QQuery

public data class QOr(
    val parts: List<QQuery>,
) : QQuery

public data class QNot(
    val part: QQuery,
) : QQuery

public data class QGuard(
    val predicate: (List<QTerm>) -> Boolean,
    val args: List<QTerm>,
) : QQuery

public data class QUnique(
    val part: QQuery,
) : QQuery

public data class QRule(
    val conclusion: QTerm,
    val body: QQuery,
)

public data class QFact(
    val term: QTerm,
)

public data class QFrame(
    val bindings: Map<QVar, QTerm>,
)

/** The query database: chronological insertion, facts indexed by head. */
public class QueryDatabase {
    private val facts = mutableListOf<QFact>()
    private val rules = mutableListOf<QRule>()

    public fun assertFact(fact: QFact) {
        facts.add(fact)
    }

    public fun addRule(rule: QRule) {
        rules.add(rule)
    }

    internal fun factsWithHead(head: String): List<QFact> = if (head == "?") facts.toList() else facts.filter { headOf(it.term) == head }

    internal fun rulesWithHead(head: String): List<QRule> =
        if (head == "?") rules.toList() else rules.filter { headOf(it.conclusion) == head }
}

private fun headOf(term: QTerm): String =
    when (term) {
        is QSym -> term.name
        is QVar -> "?"
        is QList -> term.items.firstOrNull()?.let(::headOf) ?: "?"
    }

/** Query results are streams, not a list disguised as a stream. Rule-local
 * variables are renamed on each application so distinct invocations cannot
 * capture one another's bindings. */
public class QueryDriver private constructor(
    private val db: QueryDatabase,
    private val deduplicating: Boolean,
    private val maxDepth: Int,
    private val postponing: Boolean = false,
) {
    private var nextRuleInstance = 0L

    public companion object {
        /** Every answer in database and rule order. */
        public fun streaming(db: QueryDatabase): QueryDriver = QueryDriver(db, deduplicating = false, maxDepth = Int.MAX_VALUE)

        /** Suppress duplicate rendered answers. */
        public fun deduplicating(db: QueryDatabase): QueryDriver = QueryDriver(db, deduplicating = true, maxDepth = Int.MAX_VALUE)

        /** Bound rule expansion and suppress repeated answers. */
        public fun loopDetecting(
            db: QueryDatabase,
            maxDepth: Int,
        ): QueryDriver = QueryDriver(db, deduplicating = true, maxDepth = maxDepth)

        /** Every answer in database and rule order, with `QNot` and `QGuard`
         * postponed: a filter that meets a variable still unbound waits on
         * its frame and runs the moment the last of its variables is bound
         * (exercise 4.77). A filter whose variables are never bound runs
         * when the query ends, as the other drivers would have run it. */
        public fun postponing(db: QueryDatabase): QueryDriver =
            QueryDriver(db, deduplicating = false, maxDepth = Int.MAX_VALUE, postponing = true)
    }

    /** No rule is entered or effect run before the first pull. */
    public fun run(
        query: QQuery,
        variables: List<QVar>,
    ): Sequence<QFrame> =
        sequence {
            val start = QFrame(emptyMap())
            val answers = if (postponing) settledFrames(query, start, 0) else queryFrames(query, start, 0)
            yieldAll(if (deduplicating) answers.distinctBy { renderAnswer(it, variables) } else answers)
        }

    private fun queryFrames(
        query: QQuery,
        frame: QFrame,
        depth: Int,
    ): Sequence<QFrame> =
        when (query) {
            is QPattern -> {
                patternFrames(query.term, frame, depth)
            }

            is QAnd -> {
                query.parts.fold(sequenceOf(frame)) { frames, part ->
                    frames.flatMap { queryFrames(part, it, depth) }
                }
            }

            is QOr -> {
                interleave(query.parts, frame, depth)
            }

            is QNot -> {
                sequence {
                    if (!queryFrames(query.part, frame, depth).iterator().hasNext()) yield(frame)
                }
            }

            is QGuard -> {
                sequence {
                    val bound = query.args.map { reify(it, frame) }
                    if (bound.none { containsVariable(it) } && query.predicate(bound)) yield(frame)
                }
            }

            is QUnique -> {
                sequence {
                    val matches = queryFrames(query.part, frame, depth).iterator()
                    if (matches.hasNext()) {
                        val only = matches.next()
                        if (!matches.hasNext()) yield(only)
                    }
                }
            }
        }

    private fun interleave(
        parts: List<QQuery>,
        frame: QFrame,
        depth: Int,
    ): Sequence<QFrame> = roundRobin { parts.map { queryFrames(it, frame, depth).iterator() } }

    /** One answer from each stream in turn until every stream is exhausted;
     * the streams are opened at the first pull, not before. */
    private fun <T> roundRobin(open: () -> List<Iterator<T>>): Sequence<T> =
        sequence {
            val pending = ArrayDeque(open())
            while (pending.isNotEmpty()) {
                val current = pending.removeFirst()
                if (current.hasNext()) {
                    yield(current.next())
                    pending.addLast(current)
                }
            }
        }

    /** A frame together with the filters that met unbound variables and
     * still wait for them. */
    private class Waiting(
        val frame: QFrame,
        val filters: List<QQuery>,
    )

    /** The frames [query] answers from [frame] with filters postponed; a
     * filter still waiting when the query ends runs then, on the final frame. */
    private fun settledFrames(
        query: QQuery,
        frame: QFrame,
        depth: Int,
    ): Sequence<QFrame> =
        waitingFrames(query, Waiting(frame, emptyList()), depth).mapNotNull { state ->
            state.frame.takeIf { state.filters.all { holds(it, state.frame, depth) } }
        }

    private fun waitingFrames(
        query: QQuery,
        state: Waiting,
        depth: Int,
    ): Sequence<Waiting> =
        when (query) {
            is QPattern -> {
                waitingPattern(query.term, state, depth)
            }

            is QAnd -> {
                query.parts.fold(sequenceOf(state)) { states, part ->
                    states.flatMap { waitingFrames(part, it, depth) }
                }
            }

            is QOr -> {
                roundRobin { query.parts.map { waitingFrames(it, state, depth).iterator() } }
            }

            is QNot, is QGuard -> {
                sequence { release(Waiting(state.frame, state.filters + query), depth)?.let { yield(it) } }
            }

            is QUnique -> {
                sequence {
                    val matches = settledFrames(query.part, state.frame, depth).iterator()
                    if (!matches.hasNext()) return@sequence
                    val only = matches.next()
                    if (!matches.hasNext()) release(Waiting(only, state.filters), depth)?.let { yield(it) }
                }
            }
        }

    private fun waitingPattern(
        pattern: QTerm,
        state: Waiting,
        depth: Int,
    ): Sequence<Waiting> =
        sequence {
            val head = headOf(pattern)
            for (fact in db.factsWithHead(head)) {
                val extended = unify(pattern, fact.term, state.frame) ?: continue
                release(Waiting(extended, state.filters), depth)?.let { yield(it) }
            }
            if (depth >= maxDepth) return@sequence
            for (unrenamed in db.rulesWithHead(head)) {
                val rule = freshRule(unrenamed)
                val matched = unify(pattern, rule.conclusion, state.frame) ?: continue
                val entered = release(Waiting(matched, state.filters), depth) ?: continue
                yieldAll(waitingFrames(rule.body, entered, depth + 1))
            }
        }

    /** Runs every waiting filter whose variables are all bound now; the frame
     * is dropped (null) when one of them fails, and the rest keep waiting. */
    private fun release(
        state: Waiting,
        depth: Int,
    ): Waiting? {
        val (ready, blocked) =
            state.filters.partition { filter ->
                variablesOf(filter).none { containsVariable(reify(it, state.frame)) }
            }
        if (ready.any { !holds(it, state.frame, depth) }) return null
        return if (ready.isEmpty()) state else Waiting(state.frame, blocked)
    }

    /** The filter's own test on [frame]: `QNot` succeeds when its subquery
     * answers nothing, `QGuard` when its arguments are ground and its
     * predicate accepts them. */
    private fun holds(
        filter: QQuery,
        frame: QFrame,
        depth: Int,
    ): Boolean =
        when (filter) {
            is QNot -> {
                !settledFrames(filter.part, frame, depth).iterator().hasNext()
            }

            is QGuard -> {
                val bound = filter.args.map { reify(it, frame) }
                bound.none { containsVariable(it) } && filter.predicate(bound)
            }

            else -> {
                true
            }
        }

    private fun patternFrames(
        pattern: QTerm,
        frame: QFrame,
        depth: Int,
    ): Sequence<QFrame> =
        sequence {
            val head = headOf(pattern)
            for (fact in db.factsWithHead(head)) unify(pattern, fact.term, frame)?.let { yield(it) }
            if (depth >= maxDepth) return@sequence
            for (unrenamed in db.rulesWithHead(head)) {
                val rule = freshRule(unrenamed)
                val matched = unify(pattern, rule.conclusion, frame) ?: continue
                yieldAll(queryFrames(rule.body, matched, depth + 1))
            }
        }

    private fun freshRule(rule: QRule): QRule {
        val suffix = "#${nextRuleInstance++}"

        fun term(value: QTerm): QTerm =
            when (value) {
                is QSym -> value
                is QVar -> QVar(value.name + suffix)
                is QList -> QList(value.items.map(::term), value.tail?.let(::term))
            }

        fun query(value: QQuery): QQuery =
            when (value) {
                is QPattern -> QPattern(term(value.term))
                is QAnd -> QAnd(value.parts.map(::query))
                is QOr -> QOr(value.parts.map(::query))
                is QNot -> QNot(query(value.part))
                is QUnique -> QUnique(query(value.part))
                is QGuard -> QGuard(value.predicate, value.args.map(::term))
            }
        return QRule(term(rule.conclusion), query(rule.body))
    }

    private fun unify(
        pattern: QTerm,
        datum: QTerm,
        frame: QFrame,
    ): QFrame? {
        val left = if (pattern is QVar) reify(pattern, frame) else pattern
        val right = if (datum is QVar) reify(datum, frame) else datum
        return when {
            left is QVar -> frame.extend(left, right)
            right is QVar -> frame.extend(right, left)
            left is QSym && right is QSym -> if (left.name == right.name) frame else null
            left is QList && right is QList -> unifyList(left, right, frame)
            else -> null
        }
    }

    private fun unifyList(
        left: QList,
        right: QList,
        frame: QFrame,
    ): QFrame? {
        val common = minOf(left.items.size, right.items.size)
        var current = frame
        for (index in 0 until common) current = unify(left.items[index], right.items[index], current) ?: return null
        if (left.items.size > common) {
            val tail = right.tail ?: return null
            return unify(tail, QList(left.items.drop(common), left.tail), current)
        }
        if (right.items.size > common) {
            val tail = left.tail ?: return null
            return unify(tail, QList(right.items.drop(common), right.tail), current)
        }
        if (left.tail == null && right.tail == null) return current
        return unify(left.tail ?: QList(emptyList(), null), right.tail ?: QList(emptyList(), null), current)
    }

    private fun QFrame.extend(
        variable: QVar,
        term: QTerm,
    ): QFrame? {
        if (variable == term) return this
        if (occurs(variable, term, this)) return null
        return QFrame(bindings + (variable to term))
    }

    private fun occurs(
        variable: QVar,
        term: QTerm,
        frame: QFrame,
    ): Boolean =
        when (val value = reify(term, frame)) {
            is QSym -> {
                false
            }

            is QVar -> {
                value == variable
            }

            is QList -> {
                value.items.any { occurs(variable, it, frame) } ||
                    (value.tail?.let { occurs(variable, it, frame) } ?: false)
            }
        }
}

private fun variablesOf(query: QQuery): Set<QVar> =
    when (query) {
        is QPattern -> variablesOf(query.term)
        is QAnd -> query.parts.flatMapTo(mutableSetOf(), ::variablesOf)
        is QOr -> query.parts.flatMapTo(mutableSetOf(), ::variablesOf)
        is QNot -> variablesOf(query.part)
        is QUnique -> variablesOf(query.part)
        is QGuard -> query.args.flatMapTo(mutableSetOf(), ::variablesOf)
    }

private fun variablesOf(term: QTerm): Set<QVar> =
    when (term) {
        is QSym -> emptySet()
        is QVar -> setOf(term)
        is QList -> (term.items + listOfNotNull(term.tail)).flatMapTo(mutableSetOf(), ::variablesOf)
    }

private fun containsVariable(term: QTerm): Boolean =
    when (term) {
        is QSym -> false
        is QVar -> true
        is QList -> term.items.any(::containsVariable) || term.tail?.let(::containsVariable) == true
    }

/** Instantiates [term] through [frame]: every bound variable is replaced by
 * its instantiated value, and the result is canonical -- a list whose tail
 * instantiates to a list is spliced into one `QList`, so `[a | [b]]`
 * becomes `[a, b]` and `[ | t]` becomes `t`. Unbound variables stay. */
public fun reify(
    term: QTerm,
    frame: QFrame,
): QTerm =
    when (term) {
        is QSym -> {
            term
        }

        is QVar -> {
            frame.bindings[term]?.let { reify(it, frame) } ?: term
        }

        is QList -> {
            val items = term.items.map { reify(it, frame) }
            when (val tail = term.tail?.let { reify(it, frame) }) {
                is QList -> QList(items + tail.items, tail.tail)
                null -> QList(items, null)
                else -> if (items.isEmpty()) tail else QList(items, tail)
            }
        }
    }

/** The pinned answer rendering of section 4.3: `?name = <rendered term>`. */
public fun renderAnswer(
    frame: QFrame,
    variables: List<QVar>,
): List<String> =
    variables.map { variable ->
        "?${variable.name} = ${renderTerm(reify(variable, frame))}"
    }

/** `QSym` renders its name, `QVar` renders `?name`, a proper `QList`
 * renders `[a, b, c]`, an improper one `[a, b | rest]`. */
public fun renderTerm(term: QTerm): String =
    when (term) {
        is QSym -> {
            term.name
        }

        is QVar -> {
            "?${term.name}"
        }

        is QList -> {
            val items = ArrayList<QTerm>(term.items.size)
            items.addAll(term.items)
            var tail = term.tail
            while (tail is QList) {
                items.addAll(tail.items)
                tail = tail.tail
            }
            if (items.isEmpty() && tail != null) {
                renderTerm(tail)
            } else {
                val body = items.joinToString(", ") { renderTerm(it) }
                if (tail == null) "[$body]" else "[$body | ${renderTerm(tail)}]"
            }
        }
    }
