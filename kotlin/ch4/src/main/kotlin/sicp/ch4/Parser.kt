// SPDX-License-Identifier: GPL-3.0-only
// The chapter's parser: it maps the `Value` data the reader produced onto
// the runtime's typed `Expr` nodes, the mapping the runtime's `Expr`
// documentation fixes. Forms the base language recognizes become their
// nodes; every other compound form stays an [AppE] whose operator is the
// head symbol, which is exactly the seam the evaluator-editing exercises
// (4.4, 4.5, 4.7 to 4.10, 4.13, 4.20) hook into.

package sicp.ch4

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondClause
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE

/** Every top-level form as an [Expr]. */
context(r: Raise<SchemeError>)
public fun parseProgram(forms: List<Value>): List<Expr> = forms.map { parseExpr(it) }

/** One datum as an [Expr]. */
context(r: Raise<SchemeError>)
public fun parseExpr(v: Value): Expr =
    when (v) {
        is VSym -> VarE(v.name)
        is VStr -> LitE(v)
        is VPair -> parseCombination(v)
        else -> LitE(v) // numbers, booleans, and, for safety, the empty list
    }

/** A pair in expression position: a recognized special form or an application. */
context(r: Raise<SchemeError>)
private fun parseCombination(v: VPair): Expr {
    val head = v.car
    if (head !is VSym) return parseApplication(v)
    return when (head.name) {
        "quote" -> QuoteE(operandOf(v, "quote"))
        "if" -> parseIf(v)
        "set!" -> parseSet(v)
        "define" -> parseDefine(v)
        "lambda" -> parseLambda(v)
        "begin" -> BeginE(parseBody(cdrList(v, "begin"), "begin"))
        "let" -> parseLet(v)
        "cond" -> parseCond(v)
        else -> parseApplication(v)
    }
}

context(r: Raise<SchemeError>)
private fun parseApplication(v: VPair): Expr = AppE(parseExpr(v.car), parseOperandList(v))

/** `(if p c)` keeps the book's missing-alternative choice: the `false` variable. */
context(r: Raise<SchemeError>)
private fun parseIf(v: VPair): Expr {
    val items = cdrList(v, "if")
    if (items.size < 2 || items.size > 3) r.raise(SchemeError.Parse("bad if form: $v"))
    val alternative = if (items.size == 3) parseExpr(items[2]) else VarE("false")
    return IfE(parseExpr(items[0]), parseExpr(items[1]), alternative)
}

context(r: Raise<SchemeError>)
private fun parseSet(v: VPair): Expr {
    val items = cdrList(v, "set!")
    if (items.size != 2 || items[0] !is VSym) r.raise(SchemeError.Parse("bad set! form: $v"))
    return SetE((items[0] as VSym).name, parseExpr(items[1]))
}

/** Both definition shapes: `(define name value)` and the procedure sugar. */
context(r: Raise<SchemeError>)
private fun parseDefine(v: VPair): Expr {
    val items = cdrList(v, "define")
    if (items.isEmpty()) r.raise(SchemeError.Parse("bad define form: $v"))
    val target = items[0]
    return if (target is VSym) {
        if (items.size != 2) r.raise(SchemeError.Parse("bad define form: $v"))
        DefineE(target.name, parseExpr(items[1]))
    } else {
        val signature = target as? VPair ?: r.raise(SchemeError.Parse("bad define form: $v"))
        val name = signature.car as? VSym ?: r.raise(SchemeError.Parse("bad define name: $v"))
        val (params, rest) = parseParamList(signature.cdr)
        val lambda = LambdaE(params, rest, parseBody(items.drop(1), "define"))
        DefineE(name.name, lambda)
    }
}

context(r: Raise<SchemeError>)
private fun parseLambda(v: VPair): Expr {
    val items = cdrList(v, "lambda")
    if (items.size < 2) r.raise(SchemeError.Parse("bad lambda form: $v"))
    val (params, rest) = parseParamList(items[0])
    return LambdaE(params, rest, parseBody(items.drop(1), "lambda"))
}

/** `(let ((name value) ...) body ...)`. */
context(r: Raise<SchemeError>)
private fun parseLet(v: VPair): Expr {
    val items = cdrList(v, "let")
    if (items.size < 2) r.raise(SchemeError.Parse("bad let form: $v"))
    val bindings = items[0] as? VPair ?: r.raise(SchemeError.Parse("bad let bindings: $v"))
    val parsed = mutableListOf<LetBinding>()
    var cursor: Value = bindings
    while (cursor is VPair) {
        val binding = cursor.car as? VPair ?: r.raise(SchemeError.Parse("bad let binding: ${cursor.car}"))
        val name = binding.car as? VSym ?: r.raise(SchemeError.Parse("bad let name: ${binding.car}"))
        val init = cdrList(binding, "let")
        if (init.size != 1) r.raise(SchemeError.Parse("bad let binding: $binding"))
        parsed.add(LetBinding(name.name, parseExpr(init[0])))
        cursor = cursor.cdr
    }
    if (cursor !is VNil) r.raise(SchemeError.Parse("bad let bindings: $v"))
    return LetE(parsed.toPersistentList(), parseBody(items.drop(1), "let"))
}

/** `(cond (test action ...) ... (else action ...))`. */
context(r: Raise<SchemeError>)
private fun parseCond(v: VPair): Expr {
    val clauseData = cdrList(v, "cond")
    val clauses = mutableListOf<CondClause>()
    for ((index, clause) in clauseData.withIndex()) {
        val pair = clause as? VPair ?: r.raise(SchemeError.Parse("bad cond clause: $clause"))
        val actions = parseBody(cdrList(pair, "cond"), "cond")
        val test = pair.car
        if (test is VSym && test.name == "else") {
            if (index != clauseData.size - 1) r.raise(SchemeError.Parse("else clause not last: $v"))
            clauses.add(CondClause.Else(actions))
        } else {
            clauses.add(CondClause.Clause(parseExpr(test), actions))
        }
    }
    return CondE(clauses.toPersistentList())
}

/** The operand list of a combination, as expressions. */
context(r: Raise<SchemeError>)
private fun parseOperandList(v: VPair): PersistentList<Expr> {
    val items = mutableListOf<Expr>()
    var cursor = v.cdr
    while (cursor is VPair) {
        items.add(parseExpr(cursor.car))
        cursor = cursor.cdr
    }
    if (cursor !is VNil) r.raise(SchemeError.Parse("dotted operand list: $v"))
    return items.toPersistentList()
}

/** A body: at least one expression. */
context(r: Raise<SchemeError>)
private fun parseBody(
    items: List<Value>,
    form: String,
): PersistentList<Expr> {
    if (items.isEmpty()) r.raise(SchemeError.Parse("empty $form body"))
    return items.map { parseExpr(it) }.toPersistentList()
}

/** The parameter names of a proper or dotted parameter list. */
context(r: Raise<SchemeError>)
private fun parseParamList(params: Value): Pair<PersistentList<String>, String?> {
    val names = mutableListOf<String>()
    var cursor: Value = params
    while (cursor is VPair) {
        val name = cursor.car as? VSym ?: r.raise(SchemeError.Parse("bad parameter: ${cursor.car}"))
        names.add(name.name)
        cursor = cursor.cdr
    }
    return when (cursor) {
        is VNil -> Pair(names.toPersistentList(), null)
        is VSym -> Pair(names.toPersistentList(), cursor.name)
        else -> r.raise(SchemeError.Parse("bad parameter list: $params"))
    }
}

/** The cdr of `v` as a proper list of data. */
context(r: Raise<SchemeError>)
internal fun cdrList(
    v: VPair,
    form: String,
): List<Value> {
    val items = mutableListOf<Value>()
    var cursor = v.cdr
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    if (cursor !is VNil) r.raise(SchemeError.Parse("dotted $form form: $v"))
    return items
}

/** The single operand of a one-operand special form. */
context(r: Raise<SchemeError>)
internal fun operandOf(
    v: VPair,
    form: String,
): Value {
    val items = cdrList(v, form)
    if (items.size != 1) r.raise(SchemeError.Parse("bad $form form: $v"))
    return items[0]
}

/** The symbols of a proper list, for parameter lists read back as data. */
context(r: Raise<SchemeError>)
internal fun symbolNames(v: Value): List<String> {
    val names = mutableListOf<String>()
    var cursor = v
    while (cursor is VPair) {
        val name = cursor.car as? VSym ?: r.raise(SchemeError.Parse("bad name: ${cursor.car}"))
        names.add(name.name)
        cursor = cursor.cdr
    }
    if (cursor !is VNil) r.raise(SchemeError.Parse("bad name list: $v"))
    return names
}
