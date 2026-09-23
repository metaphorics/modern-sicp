// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import kotlinx.collections.immutable.PersistentList

/**
 * The parsed expression of the interpreted language: the chapter 4
 * evaluators analyze these, and the chapter 5 explicit-control evaluator and
 * compiler consume the same tree, so the type lives in the runtime rather
 * than in any chapter. Programs arrive as [Value] data (D23: no reader in
 * the text) and a chapter's parser maps them onto these nodes.
 */
public sealed interface Expr

/** A self-evaluating literal: number, boolean, or string. */
public data class LitE(
    val v: Value,
) : Expr

/** A variable reference. */
public data class VarE(
    val name: String,
) : Expr

/** `(quote datum)`: the datum passes through unevaluated. */
public data class QuoteE(
    val datum: Value,
) : Expr

/** `(if predicate consequent alternative)`; a missing alternative is
 * [VNil], the book's unspecified value. */
public data class IfE(
    val predicate: Expr,
    val consequent: Expr,
    val alternative: Expr,
) : Expr

/** `(lambda (params . rest) body)`; `rest` is null when the parameter list
 * is proper. */
public data class LambdaE(
    val params: PersistentList<String>,
    val rest: String?,
    val body: PersistentList<Expr>,
) : Expr

/** `(begin action ...)`; the last action's value is the answer. */
public data class BeginE(
    val actions: PersistentList<Expr>,
) : Expr

/** One `cond` clause; the book's `else` is its own variant. */
public sealed interface CondClause {
    /** `(test body ...)`. */
    public data class Clause(
        val test: Expr,
        val body: PersistentList<Expr>,
    ) : CondClause

    /** `(else body ...)`. */
    public data class Else(
        val body: PersistentList<Expr>,
    ) : CondClause
}

/** `(cond clause ...)`. */
public data class CondE(
    val clauses: PersistentList<CondClause>,
) : Expr

/** One `let` binding. */
public data class LetBinding(
    val name: String,
    val value: Expr,
)

/** `(let ((name value) ...) body)`; derived into a lambda application by
 * the evaluator, kept as a node so exercises can choose the order. */
public data class LetE(
    val bindings: PersistentList<LetBinding>,
    val body: PersistentList<Expr>,
) : Expr

/** `(define name value)`; the procedure-defining sugar parses to a
 * [LambdaE] value before reaching this node. */
public data class DefineE(
    val name: String,
    val value: Expr,
) : Expr

/** `(set! name value)`. */
public data class SetE(
    val name: String,
    val value: Expr,
) : Expr

/** `(operator operand ...)`; every application, including the sugar forms
 * the parser does not recognize. */
public data class AppE(
    val operator: Expr,
    val operands: PersistentList<Expr>,
) : Expr
