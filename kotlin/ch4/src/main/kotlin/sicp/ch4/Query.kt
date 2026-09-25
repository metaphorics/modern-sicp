// SPDX-License-Identifier: GPL-3.0-only
// The query system of section 4.4: the book's four layers over the typed
// value model the chapter's evaluators compute with. Assertions, rules,
// patterns, and frames are ordinary `Value`s; a pattern variable is the
// tagged datum `VTagged("?", VSym(name))` after `querySyntaxProcess` (the
// book's internal `(? x)` form, which `VTagged` renders verbatim), and a
// renamed rule variable carries its application id, `(? 7 x)` -- a value
// distinct from every data symbol, so a renamed variable cannot collide
// with one written explicitly (the frame analog of 4.1's `*unassigned*`
// discipline). The four layers keep the book's shape:
//
// - Driver (`run`, `repl`): reads one input with the chapter's D23 reader,
//   files an `assert!` body, or answers with the lazy stream of
//   instantiated query patterns.
// - Evaluator (`qeval`): data-directed dispatch through the `qeval` table
//   (the book's `put`/`get`, one keyed dimension: every handler installs
//   under `qeval`), plus `simpleQuery` for untagged patterns.
// - Matcher and unifier (4.4.4.3/4.4.4.4) with the book's clause structure.
// - Frame streams: the section's own memoized stream -- the chapter 3
//   `LStream`, whose tail is a memoized thunk -- because the listings
//   demand it where laziness is load-bearing: `streamAppendDelayed` and
//   `interleaveDelayed` take delayed second arguments, `flattenStream`
//   delays its recursive rest, and the self-reference shape of the data
//   base discipline (exercise 4.70) is a stream hazard, not a list one.
//   One choice, made once here: every frame stream inside the engine is a
//   memoized `LStream<Frame>`; `Sequence` appears only where laziness is
//   honest, at the answer boundary (`LStream.asSequence`).
//
// Frames follow the unmarked-inhabitant rule: a frame is an immutable
// persistent map from variable to value, built only through `Frame.Empty`
// and `extended`; the matcher's and unifier's failure is the option type
// (`null`), never a sentinel stored in the data. The data base is
// CHRONOLOGICAL: the book's `add-assertion!` conses the newest entry in
// front, yet every sample interaction the book pins -- the `job` queries,
// `lives-near`, `append-to-form` -- lists answers in insertion order, so
// assertions, rules, and their index buckets are appended lists behind the
// stream interface, indexed by the leading symbol exactly as 4.4.4.5
// describes; `addAssertion` binds the old collection before installing the
// new one, the discipline the book's `let` enforces (exercise 4.70).
//
// The query reader is the chapter's shared D23 reader (`readDatum`), not a
// scanner of its own: the query language's `()`, dotted patterns such as
// `(computer . ?type)`, and tokens such as `9am` (4.59's meeting times)
// all read as ordinary data under the shared grammar, whose symbol class
// is letters, digits, and `-?!*+</>=_.`; the pinned evidence lives in
// `S4_4_4SyntaxTest`. `lisp-value` applies the underlying system's
// primitives (the registry the 4.1 driver installs), the edition's
// `user-initial-environment`. Faults raised while a lazy answer stream is
// being forced -- the entry scope has returned by then -- surface as
// [QueryFault] carrying the typed [SchemeError].

package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.PersistentMap
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.persistentMapOf
import sicp.runtime.LStream
import sicp.runtime.Op
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.VTagged
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.consStream
import sicp.runtime.equalv
import sicp.runtime.isTrue

/** A typed fault raised while a lazy answer stream is being forced. */
public class QueryFault(
    public val error: SchemeError,
) : RuntimeException(formatError(error))

/** Runs one Raise scope to a value; a typed error leaves as a fault the
 * answer-stream consumer sees. */
internal fun <T> queryScoped(block: Raise<SchemeError>.() -> T): T = either { block(this) }.fold({ e -> throw QueryFault(e) }, { it })

/**
 * The book's frames of 4.4.4.8: bindings as a persistent map from variable
 * to value, immutable, with no inhabitant standing for "unbound" -- the
 * matcher's failure is the option type, not a sentinel.
 */
public data class Frame(
    public val bindings: PersistentMap<Value, Value>,
) {
    public companion object {
        /** The empty frame the driver starts every query from. */
        public val Empty: Frame = Frame(persistentMapOf())
    }

    /** The value bound to `variable`, or null when unbound. */
    public operator fun get(variable: Value): Value? = bindings[variable]

    /** The book's `extend`: this frame with one more binding. */
    public fun extended(
        variable: Value,
        value: Value,
    ): Frame = Frame(bindings.putting(variable, value))
}

/** The book's `binding-in-frame`: the value bound to `variable`, or null. */
public fun bindingInFrame(
    variable: Value,
    frame: Frame,
): Value? = frame[variable]

/** The book's `extend`. */
public fun extend(
    variable: Value,
    value: Value,
    frame: Frame,
): Frame = frame.extended(variable, value)

/** The book's `var?`: the internal `(? name)` form. */
public fun isVar(v: Value): Boolean = v is VTagged && v.tag == "?"

/** The book's `constant-symbol?`: after `querySyntaxProcess` the symbols
 * are exactly the constants. */
public fun isConstantSymbol(v: Value): Boolean = v is VSym

/** The written name of a pattern variable, written `(? x)` or renamed
 * `(? 7 x)`. */
public fun patternVarName(v: Value): String {
    val name = ((v as VTagged).data as VSym).name
    val space = name.indexOf(' ')
    return if (space < 0) name else name.substring(space + 1)
}

/** The book's `make-new-variable`: `(? name)` becomes `(? id name)`. The
 * renamed form keeps the identifier in the symbol so two occurrences of
 * one renamed variable stay `equal?` -- the book's list form does too,
 * and the printed form is the book's `(? 7 x)`. */
public fun makeNewVariable(
    variable: Value,
    ruleApplicationId: Int,
): Value = VTagged("?", VSym("$ruleApplicationId ${patternVarName(variable)}"))

/** The book's `contract-question-mark`: `(? x)` prints `?x`, `(? 7 x)`
 * prints `?x-7`. */
public fun contractQuestionMark(variable: Value): String {
    val name = ((variable as VTagged).data as VSym).name
    val space = name.indexOf(' ')
    return if (space < 0) "?$name" else "?${name.substring(space + 1)}-${name.substring(0, space)}"
}

/** A pattern variable in internal form, as `querySyntaxProcess` writes it. */
public fun patternVar(name: String): Value = VTagged("?", VSym(name))

/** Builds a proper list of values. */
public fun valueListOf(items: List<Value>): Value = items.foldRight(VNil as Value) { v, acc -> cons(v, acc) }

/** Flattens a proper list of values. */
public fun listValueOf(v: Value): List<Value> {
    val out = mutableListOf<Value>()
    var cursor = v
    while (cursor is VPair) {
        out.add(cursor.car)
        cursor = cursor.cdr
    }
    return out
}

/** The book's `stream-map` over the memoized stream. */
public fun <T, R> streamMap(
    proc: (T) -> R,
    s: LStream<T>,
): LStream<R> =
    when (s) {
        is LStream.Empty -> LStream.Empty
        is LStream.Cons -> consStream(proc(s.head)) { streamMap(proc, s.tail) }
    }

/** A data-base bucket as a stream, chronological order. */
public fun <T> listStream(items: List<T>): LStream<T> = items.foldRight(LStream.Empty as LStream<T>) { v, acc -> consStream(v) { acc } }

/** The book's `singleton-stream`. */
public fun <T> singletonStream(x: T): LStream<T> = consStream(x) { LStream.Empty }

/** The book's plain `stream-append` of 3.5.3, for the 4.71/4.72 probes. */
public fun <T> streamAppend(
    s1: LStream<T>,
    s2: () -> LStream<T>,
): LStream<T> =
    when (s1) {
        is LStream.Empty -> s2()
        is LStream.Cons -> consStream(s1.head) { streamAppend(s1.tail, s2) }
    }

/** The book's plain `interleave` of 3.5.3, for the 4.72 probes. */
public fun <T> interleave(
    s1: LStream<T>,
    s2: () -> LStream<T>,
): LStream<T> {
    val first = s1 as? LStream.Cons ?: return s2()
    return consStream(first.head) {
        val second = s2()
        interleave(second) { first.tail }
    }
}

/** The book's `stream-append-delayed`: the second stream is a thunk, so
 * demanding the head of the result never forces it (4.71). */
public fun <T> streamAppendDelayed(
    s1: LStream<T>,
    delayedS2: () -> LStream<T>,
): LStream<T> {
    val first = s1 as? LStream.Cons ?: return delayedS2()
    return consStream(first.head) { streamAppendDelayed(first.tail, delayedS2) }
}

/** The book's `interleave-delayed`: alternates the heads of the two
 * streams, keeping the second delayed between demands (4.72). */
public fun <T> interleaveDelayed(
    s1: LStream<T>,
    delayedS2: () -> LStream<T>,
): LStream<T> {
    val first = s1 as? LStream.Cons ?: return delayedS2()
    return consStream(first.head) { interleaveDelayed(delayedS2()) { first.tail } }
}

/** The book's `stream-filter` of 3.5.3 over the memoized stream. */
public fun <T> streamFilter(
    pred: (T) -> Boolean,
    s: LStream<T>,
): LStream<T> =
    when (s) {
        is LStream.Empty -> {
            LStream.Empty
        }

        is LStream.Cons -> {
            if (pred(s.head)) {
                consStream(s.head) { streamFilter(pred, s.tail) }
            } else {
                streamFilter(pred, s.tail)
            }
        }
    }

/** The book's `stream-flatmap` over frames, accumulated by interleaving. */
public fun <T> streamFlatmap(
    proc: (T) -> LStream<Frame>,
    s: LStream<T>,
): LStream<Frame> = flattenStream(streamMap(proc, s))

/** The book's `flatten-stream`: the delayed recursion keeps an infinite
 * first substream from starving the rest (4.73). */
public fun flattenStream(stream: LStream<LStream<Frame>>): LStream<Frame> {
    val first = stream as? LStream.Cons ?: return LStream.Empty
    return interleaveDelayed(first.head) { flattenStream(first.tail) }
}

/** The book's `pattern-match` (4.4.4.3): failure is null, success the
 * extended frame. */
public fun patternMatch(
    pat: Value,
    dat: Value,
    frame: Frame?,
): Frame? {
    if (frame == null) return null
    if (equalv(pat, dat)) return frame
    if (isVar(pat)) return extendIfConsistent(pat, dat, frame)
    if (pat is VPair && dat is VPair) {
        return patternMatch(pat.cdr, dat.cdr, patternMatch(pat.car, dat.car, frame))
    }
    return null
}

/** The book's `extend-if-consistent`. */
public fun extendIfConsistent(
    v: Value,
    dat: Value,
    frame: Frame,
): Frame? {
    val bound = bindingInFrame(v, frame) ?: return frame.extended(v, dat)
    return patternMatch(bound, dat, frame)
}

/** The book's `unify-match` (4.4.4.4): symmetrical, variables on both
 * sides. */
public fun unifyMatch(
    p1: Value,
    p2: Value,
    frame: Frame?,
): Frame? {
    if (frame == null) return null
    if (equalv(p1, p2)) return frame
    if (isVar(p1)) return extendIfPossible(p1, p2, frame)
    if (isVar(p2)) return extendIfPossible(p2, p1, frame)
    if (p1 is VPair && p2 is VPair) {
        return unifyMatch(p1.cdr, p2.cdr, unifyMatch(p1.car, p2.car, frame))
    }
    return null
}

/** The book's `extend-if-possible` with its two consistency checks. */
public fun extendIfPossible(
    v: Value,
    value: Value,
    frame: Frame,
): Frame? {
    val bound = bindingInFrame(v, frame)
    if (bound != null) return unifyMatch(bound, value, frame)
    if (isVar(value)) {
        val boundValue = bindingInFrame(value, frame)
        return if (boundValue != null) unifyMatch(v, boundValue, frame) else frame.extended(v, value)
    }
    if (dependsOn(value, v, frame)) return null
    return frame.extended(v, value)
}

/** The book's `depends-on?`: whether a proposed binding value depends on
 * the variable, through the frame's stored bindings. */
public fun dependsOn(
    exp: Value,
    v: Value,
    frame: Frame,
): Boolean =
    when {
        isVar(exp) -> {
            if (equalv(exp, v)) true else bindingInFrame(exp, frame)?.let { dependsOn(it, v, frame) } ?: false
        }

        exp is VPair -> {
            dependsOn(exp.car, v, frame) || dependsOn(exp.cdr, v, frame)
        }

        else -> {
            false
        }
    }

/** The book's `assertion-to-be-added?`. */
public fun isAssertionToBeAdded(exp: Value): Boolean = exp is VPair && exp.car is VSym && (exp.car as VSym).name == "assert!"

/** The book's `add-assertion-body`: the rule or assertion under `assert!`. */
public fun addAssertionBody(exp: Value): Value = ((exp as VPair).cdr as VPair).car

/** The book's conjunction and disjunction selectors of 4.4.4.7. */
public fun isEmptyConjunction(exps: Value): Boolean = exps is VNil

public fun firstConjunct(exps: Value): Value = (exps as VPair).car

public fun restConjuncts(exps: Value): Value = (exps as VPair).cdr

public fun isEmptyDisjunction(exps: Value): Boolean = exps is VNil

public fun firstDisjunct(exps: Value): Value = (exps as VPair).car

public fun restDisjuncts(exps: Value): Value = (exps as VPair).cdr

public fun negatedQuery(exps: Value): Value = (exps as VPair).car

public fun predicateOf(exps: Value): Value = (exps as VPair).car

public fun argsOf(exps: Value): Value = (exps as VPair).cdr

/** The book's `rule?`. */
public fun isRule(statement: Value): Boolean = statement is VPair && statement.car is VSym && (statement.car as VSym).name == "rule"

/** The book's `conclusion`. */
public fun conclusionOf(rule: Value): Value = ((rule as VPair).cdr as VPair).car

/** The book's `rule-body`: a bodyless rule's conclusion is always true. */
public fun ruleBodyOf(rule: Value): Value {
    val rest = ((rule as VPair).cdr as VPair).cdr
    return if (rest is VNil) valueListOf(listOf(VSym("always-true"))) else (rest as VPair).car
}

/** The book's `query-syntax-process` and its helpers. */
public fun querySyntaxProcess(exp: Value): Value =
    mapOverSymbols({ symbol -> if (symbol.name.startsWith("?")) patternVar(symbol.name.substring(1)) else symbol }, exp)

public fun mapOverSymbols(
    proc: (VSym) -> Value,
    exp: Value,
): Value =
    when (exp) {
        is VPair -> cons(mapOverSymbols(proc, exp.car), mapOverSymbols(proc, exp.cdr))
        is VSym -> proc(exp)
        else -> exp
    }

/** The book's `indexable?`. */
public fun isIndexable(pat: Value): Boolean = pat is VPair && (isConstantSymbol(pat.car) || isVar(pat.car))

/** The book's `index-key-of`: the leading symbol, or `?` for a variable. */
public fun indexKeyOf(pat: Value): Value {
    val key = (pat as VPair).car
    return if (isVar(key)) VSym("?") else key
}

/** The book's `use-index?`. */
public fun useIndex(pat: Value): Boolean = pat is VPair && isConstantSymbol(pat.car)

/** One query outcome of the driver: an assertion was filed, or the lazy
 * stream of instantiated answers. */
public sealed interface QueryOutcome {
    /** The input was an `assert!`; the data base grew. */
    public data object Asserted : QueryOutcome

    /** The input was a query; the answers force lazily. */
    public data class Answered(
        public val answers: LStream<Value>,
    ) : QueryOutcome
}

/** The handler installed under a query type in the `qeval` table. */
public typealias QProc = (Value, LStream<Frame>) -> LStream<Frame>

/**
 * One query system: the driver's data base, the dispatch table, and the
 * rule-application counter. Exercises extend it by overriding the open
 * seams ([applyARule], [negate], [lispValue], [conjoin]) or by installing
 * handlers through [putQuery], the book's `(put 'unique 'qeval ...)`.
 */
public open class QuerySystem(
    /** The underlying system's procedures `lisp-value` may call. */
    public val primitives: Map<String, Op>,
) {
    /** The book's dispatch table, one keyed dimension: every special form
     * installs under `qeval`. */
    private val qevalTable = HashMap<String, QProc>()

    /** The data base of 4.4.4.5: chronological collections behind the
     * stream interface, with index buckets keyed by leading symbol. */
    private var assertions: PersistentList<Value> = persistentListOf()

    private var rules: PersistentList<Value> = persistentListOf()
    private val assertionIndex = HashMap<Value, MutableList<Value>>()
    private val ruleIndex = HashMap<Value, MutableList<Value>>()

    /** The book's `rule-counter`. */
    private var ruleCounter: Int = 0

    public constructor() : this(defaultQueryPrimitives())

    init {
        installDispatch()
    }

    /** The book's clause chain as data: each special form installs its
     * handler; subclasses may reinstall their own. */
    protected open fun installDispatch() {
        putQuery("and", ::conjoin)
        putQuery("or", ::disjoin)
        putQuery("not", ::negate)
        putQuery("lisp-value", ::lispValue)
        putQuery("always-true", ::alwaysTrue)
    }

    /** The book's `(put ⟨type⟩ 'qeval ⟨proc⟩)`. */
    public fun putQuery(
        type: String,
        proc: QProc,
    ) {
        qevalTable[type] = proc
    }

    /** The book's `qeval` dispatch lookup. */
    public fun getQuery(type: String): QProc? = qevalTable[type]

    /** The book's `new-rule-application-id`. */
    public fun newRuleApplicationId(): Int {
        ruleCounter += 1
        return ruleCounter
    }

    /** The book's `rename-variables-in`. */
    public fun renameVariablesIn(rule: Value): Value {
        val id = newRuleApplicationId()
        return renameTreeWalk(rule, id)
    }

    private fun renameTreeWalk(
        exp: Value,
        id: Int,
    ): Value =
        when {
            isVar(exp) -> makeNewVariable(exp, id)
            exp is VPair -> cons(renameTreeWalk(exp.car, id), renameTreeWalk(exp.cdr, id))
            else -> exp
        }

    /** The driver entry: one input line, an `assert!` filed or the lazy
     * answer stream. The reader is the chapter's D23 `readDatum`. */
    public fun run(text: String): Either<SchemeError, QueryOutcome> =
        either {
            val query = querySyntaxProcess(readDatum(text))
            runParsed(query)
        }

    private fun runParsed(query: Value): QueryOutcome {
        if (isAssertionToBeAdded(query)) {
            addRuleOrAssertion(addAssertionBody(query))
            return QueryOutcome.Asserted
        }
        val stream = qeval(query, singletonStream(Frame.Empty))
        return QueryOutcome.Answered(streamMap({ instantiate(query, it) { vars, _ -> VSym(contractQuestionMark(vars)) } }, stream))
    }

    /** Loads a program text of `assert!` forms: the demo data base. */
    public fun load(text: String) {
        val forms = either { readProgram(text) }.fold({ e -> throw QueryFault(e) }, { it })
        for (datum in forms) {
            runParsed(querySyntaxProcess(datum))
        }
    }

    /** The driver loop's transcript for one input: the book's prompts, the
     * echoed query, and every instantiated answer. */
    public fun repl(text: String): String {
        val lines = mutableListOf(";;; Query input:")
        val parsed = either { readDatum(text) }
        when (parsed) {
            is Either.Left -> {
                lines.add("Error: ${formatError(parsed.value)}")
            }

            is Either.Right -> {
                lines.add(printValue(parsed.value))
                recordOutcome(lines, run(text))
            }
        }
        return lines.joinToString(separator = "\n", postfix = "\n")
    }

    private fun recordOutcome(
        lines: MutableList<String>,
        outcome: Either<SchemeError, QueryOutcome>,
    ) {
        when (outcome) {
            is Either.Left -> {
                lines.add("Error: ${formatError(outcome.value)}")
            }

            is Either.Right -> {
                when (val o = outcome.value) {
                    is QueryOutcome.Asserted -> {
                        lines.add("Assertion added to data base.")
                    }

                    is QueryOutcome.Answered -> {
                        lines.add(";;; Query results:")
                        answerLoop(o, lines)
                    }
                }
            }
        }
    }

    private fun answerLoop(
        answered: QueryOutcome.Answered,
        lines: MutableList<String>,
    ) {
        var cursor = answered.answers
        while (cursor is LStream.Cons) {
            try {
                lines.add(printValue(cursor.head))
            } catch (fault: QueryFault) {
                lines.add("Error: ${formatError(fault.error)}")
                return
            }
            cursor = cursor.tail
        }
    }

    /** The book's `instantiate`: the query pattern with the frame's values
     * substituted; [unboundVarHandler] names what stayed unbound. */
    public fun instantiate(
        exp: Value,
        frame: Frame,
        unboundVarHandler: (Value, Frame) -> Value,
    ): Value =
        when {
            isVar(exp) -> {
                val binding = bindingInFrame(exp, frame)
                if (binding != null) instantiate(binding, frame, unboundVarHandler) else unboundVarHandler(exp, frame)
            }

            exp is VPair -> {
                cons(instantiate(exp.car, frame, unboundVarHandler), instantiate(exp.cdr, frame, unboundVarHandler))
            }

            else -> {
                exp
            }
        }

    /** The book's `qeval`: dispatch on the query's type through the
     * table; an untagged query is simple. */
    public fun qeval(
        query: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (query !is VPair) return simpleQuery(query, frameStream)
        val head = query.car as? VSym ?: return simpleQuery(query, frameStream)
        val proc = getQuery(head.name) ?: return simpleQuery(query, frameStream)
        return proc(query.cdr, frameStream)
    }

    /** The book's `stream-flatmap` as the handlers use it: the seam the
     * exercise variants of 4.73 and 4.74 replace. */
    protected open fun <T> flatmapFrames(
        proc: (T) -> LStream<Frame>,
        s: LStream<T>,
    ): LStream<Frame> = streamFlatmap(proc, s)

    /** The book's `simple-query`: assertion matches first, then rule
     * applications, the rule side delayed (4.71). */
    public open fun simpleQuery(
        queryPattern: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        flatmapFrames(
            { frame ->
                streamAppendDelayed(
                    findAssertions(queryPattern, frame),
                ) { applyRules(queryPattern, frame) }
            },
            frameStream,
        )

    /** The book's `conjoin`, installed under `and`. */
    public open fun conjoin(
        conjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyConjunction(conjuncts)) return frameStream
        return conjoin(restConjuncts(conjuncts), qeval(firstConjunct(conjuncts), frameStream))
    }

    /** The book's `disjoin`, installed under `or`: the disjunct streams
     * interleave, the second delayed (4.71, 4.72). */
    public open fun disjoin(
        disjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyDisjunction(disjuncts)) return LStream.Empty
        return interleaveDelayed(qeval(firstDisjunct(disjuncts), frameStream)) {
            disjoin(restDisjuncts(disjuncts), frameStream)
        }
    }

    /** The book's `negate`, installed under `not`: a filter that keeps the
     * frames the negated query cannot extend. */
    public open fun negate(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        flatmapFrames(
            { frame ->
                if (qeval(negatedQuery(operands), singletonStream(frame)) is LStream.Empty) {
                    singletonStream(frame)
                } else {
                    LStream.Empty
                }
            },
            frameStream,
        )

    /** The book's `lisp-value`, installed under `lisp-value`: a filter
     * over the underlying system's predicates. */
    public open fun lispValue(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        flatmapFrames(
            { frame ->
                if (executeCall(instantiateCall(operands, frame))) singletonStream(frame) else LStream.Empty
            },
            frameStream,
        )

    /** The book's `instantiate` of the `lisp-value` call form; an unbound
     * pattern variable is the book's `error`. */
    protected fun instantiateCall(
        operands: Value,
        frame: Frame,
    ): List<Value> =
        queryScoped {
            listValueOf(
                instantiate(operands, frame) { vars, _ ->
                    raise(SchemeError.UserRaised("Unknown pat var -- LISP-VALUE", listOf(vars)))
                },
            )
        }

    /** The book's `execute`: apply the named underlying procedure. */
    protected fun executeCall(call: List<Value>): Boolean =
        queryScoped {
            val name = (call[0] as VSym).name
            val op =
                primitives[name]
                    ?: raise(SchemeError.NotApplicable(VSym(name)))
            isTrue(op(call.drop(1)))
        }

    /** The book's `always-true`, installed for bodyless rules. */
    public open fun alwaysTrue(
        ignored: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> = frameStream

    /** The book's `find-assertions`. */
    public open fun findAssertions(
        pattern: Value,
        frame: Frame,
    ): LStream<Frame> = flatmapFrames({ checkAnAssertion(it, pattern, frame) }, fetchAssertions(pattern))

    /** The book's `check-an-assertion`. */
    public fun checkAnAssertion(
        assertion: Value,
        queryPat: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        val matchResult = patternMatch(queryPat, assertion, queryFrame) ?: return LStream.Empty
        return singletonStream(matchResult)
    }

    /** The book's `apply-rules`. */
    public fun applyRules(
        pattern: Value,
        frame: Frame,
    ): LStream<Frame> = flatmapFrames({ applyARule(it, pattern, frame) }, fetchRules(pattern))

    /** The book's `apply-a-rule`: rename, unify the conclusion, evaluate
     * the body on the unified frame. */
    public open fun applyARule(
        rule: Value,
        queryPattern: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        val cleanRule = renameVariablesIn(rule)
        val unifyResult = unifyMatch(queryPattern, conclusionOf(cleanRule), queryFrame) ?: return LStream.Empty
        return qeval(ruleBodyOf(cleanRule), singletonStream(unifyResult))
    }

    /** The book's `fetch-assertions`: the index bucket, or everything. */
    public fun fetchAssertions(pattern: Value): LStream<Value> =
        if (useIndex(pattern)) listStream(assertionIndex[indexKeyOf(pattern)].orEmpty()) else listStream(assertions)

    /** The book's `fetch-rules`: the indexed bucket plus every rule whose
     * conclusion starts with a variable. */
    public fun fetchRules(pattern: Value): LStream<Value> {
        if (!useIndex(pattern)) return listStream(rules)
        val indexed = ruleIndex[indexKeyOf(pattern)].orEmpty()
        val variableHeaded = ruleIndex[VSym("?")].orEmpty()
        return listStream(indexed + variableHeaded)
    }

    /** The book's `add-rule-or-assertion!`. */
    public fun addRuleOrAssertion(assertion: Value) {
        if (isRule(assertion)) addRule(assertion) else addAssertion(assertion)
    }

    /** The book's `add-assertion!`: indexed, then appended; the old
     * collection is bound before the new one is installed (4.70). */
    public fun addAssertion(assertion: Value) {
        storeAssertionInIndex(assertion)
        val old = assertions
        assertions = old.adding(assertion)
    }

    /** The book's `add-rule!`. */
    public fun addRule(rule: Value) {
        storeRuleInIndex(rule)
        val old = rules
        rules = old.adding(rule)
    }

    /** The book's `store-assertion-in-index`. */
    private fun storeAssertionInIndex(assertion: Value) {
        if (isIndexable(assertion)) storeInIndex(assertionIndex, indexKeyOf(assertion), assertion)
    }

    /** The book's `store-rule-in-index`: the conclusion is indexed, under
     * `?` when it starts with a variable. */
    private fun storeRuleInIndex(rule: Value) {
        val pattern = conclusionOf(rule)
        if (isIndexable(pattern)) storeInIndex(ruleIndex, indexKeyOf(pattern), rule)
    }

    private fun storeInIndex(
        index: HashMap<Value, MutableList<Value>>,
        key: Value,
        item: Value,
    ) {
        index.getOrPut(key) { mutableListOf() }.add(item)
    }
}

/** The underlying system's procedure registry, the edition's
 * `user-initial-environment` for `lisp-value`. */
public fun defaultQueryPrimitives(): Map<String, Op> = defaultPrimitives(OutputSink()).associate { it.name to it.op }
