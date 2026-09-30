// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.73

package sicp.ch2.exercises

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

public fun interface DerivRule {
    public fun differentiate(
        exp: OperatorExpr,
        variable: String,
        table: DerivTable,
    ): OperatorExpr
}

public class DerivTable {
    private val rules: MutableMap<String, DerivRule> = HashMap()

    public fun put(
        operator: String,
        rule: DerivRule,
    ) {
        rules[operator] = rule
    }

    public fun get(operator: String): DerivRule? = rules[operator]
}

private fun dispatchRule(
    operator: String,
    exp: OperatorExpr,
    variable: String,
    table: DerivTable,
): OperatorExpr {
    val rule = table.get(operator) ?: throw IllegalStateException("No differentiation rule installed for operator $operator")
    return rule.differentiate(exp, variable, table)
}

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

private fun isNum(
    e: OperatorExpr,
    n: Long,
): Boolean = e is OperatorExpr.Num && e.n == n

/** The book's `make-sum`, simplifying the identity and constant-folding cases. */
public fun makeOpSum(
    a1: OperatorExpr,
    a2: OperatorExpr,
): OperatorExpr =
    when {
        isNum(a1, 0) -> a2
        isNum(a2, 0) -> a1
        a1 is OperatorExpr.Num && a2 is OperatorExpr.Num -> OperatorExpr.Num(a1.n + a2.n)
        else -> OperatorExpr.Sum(a1, a2)
    }

/** The book's `make-product`, simplifying the zero, identity, and constant-folding cases. */
public fun makeOpProduct(
    m1: OperatorExpr,
    m2: OperatorExpr,
): OperatorExpr =
    when {
        isNum(m1, 0) || isNum(m2, 0) -> OperatorExpr.Num(0)
        isNum(m1, 1) -> m2
        isNum(m2, 1) -> m1
        m1 is OperatorExpr.Num && m2 is OperatorExpr.Num -> OperatorExpr.Num(m1.n * m2.n)
        else -> OperatorExpr.Product(m1, m2)
    }

/** The book's rule that anything to the 0th power is 1 and to the 1st power is itself. */
public fun makeOpPow(
    base: OperatorExpr,
    n: Long,
): OperatorExpr =
    when (n) {
        0L -> OperatorExpr.Num(1)
        1L -> base
        else -> OperatorExpr.Pow(base, n)
    }

/** Part (b): `d(a1 + a2)/dx = da1/dx + da2/dx`. */
public fun installSumRule(table: DerivTable) {
    table.put("+") { exp, variable, t ->
        if (exp !is OperatorExpr.Sum) throw IllegalStateException("sum rule dispatched on $exp")
        makeOpSum(derivDataDirected(exp.a1, variable, t), derivDataDirected(exp.a2, variable, t))
    }
}

/** Part (b): `d(m1 * m2)/dx = m1 * dm2/dx + dm1/dx * m2`. */
public fun installProductRule(table: DerivTable) {
    table.put("*") { exp, variable, t ->
        if (exp !is OperatorExpr.Product) throw IllegalStateException("product rule dispatched on $exp")
        makeOpSum(
            makeOpProduct(exp.a1, derivDataDirected(exp.a2, variable, t)),
            makeOpProduct(derivDataDirected(exp.a1, variable, t), exp.a2),
        )
    }
}

/** Part (c): the power rule of exercise 2.56, `d(u^n)/dx = n * u^(n-1) * du/dx`. */
public fun installPowRule(table: DerivTable) {
    table.put("**") { exp, variable, t ->
        if (exp !is OperatorExpr.Pow) throw IllegalStateException("power rule dispatched on $exp")
        makeOpProduct(
            makeOpProduct(OperatorExpr.Num(exp.n), makeOpPow(exp.base, exp.n - 1)),
            derivDataDirected(exp.base, variable, t),
        )
    }
}

private fun installedTable(): DerivTable {
    val table = DerivTable()
    installSumRule(table)
    installProductRule(table)
    installPowRule(table)
    return table
}

/**
 * Differentiate one sum, one product, and one power expression through the
 * installed rule table, returning their typed result trees.
 */
public fun ex_2_73(): Triple<OperatorExpr, OperatorExpr, OperatorExpr> {
    val table = installedTable()
    val variable = "x"
    val sum = derivDataDirected(OperatorExpr.Sum(OperatorExpr.Var("x"), OperatorExpr.Num(3)), variable, table)
    val product = derivDataDirected(OperatorExpr.Product(OperatorExpr.Var("x"), OperatorExpr.Var("y")), variable, table)
    val power = derivDataDirected(OperatorExpr.Pow(OperatorExpr.Var("x"), 3L), variable, table)
    return Triple(sum, product, power)
}
