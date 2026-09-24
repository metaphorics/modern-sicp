// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.73

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.73: 2.3.2 performed symbolic differentiation by dispatching
 * on the type of the expression, where the algebraic operator is the
 * "type tag" and the operation is `deriv`. This exercise rewrites that
 * dispatch in data-directed style, given as the starting skeleton below:
 * [OperatorExpr] carries its own operator, [printOperatorExpr] prints it,
 * and [derivDataDirected] is the book's rewritten `deriv`, dispatching
 * through a [DerivTable] instead of a `when` over the operator.
 *
 * (a) `Num` and `Var` cannot join the data-directed dispatch: a number or
 * a variable carries no operator to index the table by, so [derivDataDirected]
 * must still recognize them directly before ever consulting the table.
 *
 * (b) Write the sum and product rules and install them in the table.
 *
 * (c) Choose one more differentiation rule and install it too. This
 * solution adds the power rule of exercise 2.56.
 *
 * (d) Flipping the table's key order to `get(operator, "deriv")` only
 * renames the axes the table is keyed by; nothing about [derivDataDirected]
 * or the rules it calls needs to change, only the argument order every
 * `put`/`get` call passes.
 *
 * The scaffold returns the printed derivative of `x + 3`, `x * y`, and
 * `x ** 3`, each with respect to `x`.
 */
public sealed interface OperatorExpr {
    public data class Num(
        val n: Long,
    ) : OperatorExpr

    public data class Var(
        val name: String,
    ) : OperatorExpr

    public data class Sum(
        val a1: OperatorExpr,
        val a2: OperatorExpr,
    ) : OperatorExpr

    public data class Product(
        val a1: OperatorExpr,
        val a2: OperatorExpr,
    ) : OperatorExpr

    public data class Pow(
        val base: OperatorExpr,
        val n: Long,
    ) : OperatorExpr
}

/** Prints an [OperatorExpr] in the book's parenthesized prefix notation. */
public fun printOperatorExpr(e: OperatorExpr): String =
    when (e) {
        is OperatorExpr.Num -> e.n.toString()
        is OperatorExpr.Var -> e.name
        is OperatorExpr.Sum -> "(+ ${printOperatorExpr(e.a1)} ${printOperatorExpr(e.a2)})"
        is OperatorExpr.Product -> "(* ${printOperatorExpr(e.a1)} ${printOperatorExpr(e.a2)})"
        is OperatorExpr.Pow -> "(** ${printOperatorExpr(e.base)} ${e.n})"
    }

/** A single differentiation rule: given the whole expression node it was
 * installed under and the variable, returns the derivative. */
public fun interface DerivRule {
    public fun differentiate(
        exp: OperatorExpr,
        variable: String,
        table: DerivTable,
    ): OperatorExpr
}

/** The book's operation-and-type table, specialized to one operation
 * (`deriv`) and keyed by the operator symbol alone. */
public class DerivTable {
    private val rules: MutableMap<String, DerivRule> = HashMap()

    /** The book's `put`, restricted to `deriv`'s one operation. */
    public fun put(
        operator: String,
        rule: DerivRule,
    ) {
        rules[operator] = rule
    }

    /** The book's `get`, restricted to `deriv`'s one operation. */
    public fun get(operator: String): DerivRule? = rules[operator]
}

private fun dispatchRule(
    operator: String,
    exp: OperatorExpr,
    variable: String,
    table: DerivTable,
): OperatorExpr {
    val rule = table.get(operator) ?: throw IllegalStateException("unknown expression type: DERIV ${printOperatorExpr(exp)}")
    return rule.differentiate(exp, variable, table)
}

/** The book's `deriv`, rewritten data-directed: `Num` and `Var` are
 * recognized directly (part a), every other case dispatches through
 * `table`. */
public fun derivDataDirected(
    exp: OperatorExpr,
    variable: String,
    table: DerivTable,
): OperatorExpr =
    when (exp) {
        is OperatorExpr.Num -> OperatorExpr.Num(0)
        is OperatorExpr.Var -> OperatorExpr.Num(if (exp.name == variable) 1 else 0)
        is OperatorExpr.Sum -> dispatchRule("+", exp, variable, table)
        is OperatorExpr.Product -> dispatchRule("*", exp, variable, table)
        is OperatorExpr.Pow -> dispatchRule("**", exp, variable, table)
    }

public fun ex_2_73(): Triple<String, String, String> = throw PendingSolution()
