// SPDX-License-Identifier: GPL-3.0-only
// Section 4.1 shared fixtures: the chapter's guest-evaluator kernel, written
// in admitted ED39 guest source and mirroring the settled kernel family
// (grammar section 5 witness plus the section-7 extensions). Every 4.1
// solution appends its variant and probe to [KERNEL_SOURCE] and runs the
// composed unit through the checked engines; nothing here reaches a host
// evaluator or re-parses old source.

package sicp.ch4.solutions

/**
 * The guest-evaluator kernel of section 4.1: explicit syntax-tree values
 * (the `GExpr` family), closures and frames, `gEval`/`gApply` with
 * null-means-error, and the control markers `GReturnV`/`GBreakV`/`GContinueV`
 * that `gApply` alone unwraps. The expression family and markers are frozen
 * with the book; the value family follows the settled 4.1 spelling
 * (`GClosV`/`GUnassigned`/`GListV`/`GObjectV`/`GUnitV`/`GFrame`).
 *
 * The unit declares no `main`: each exercise appends its own variant,
 * probe, and `fun main()` block. Only checked source ever executes. Guest
 * string building uses same-type `String + String` and one primitive
 * template ([showLong]), exactly as the grammar's surface admits; the host
 * raw string escapes every dollar so nothing interpolates host-side.
 */
internal val KERNEL_SOURCE: String =
    """
sealed interface GExpr

data class GNum(val n: Long) : GExpr

data class GBool(val b: Boolean) : GExpr

data class GStr(val text: String) : GExpr

data object GNull : GExpr

data class GVar(val name: String) : GExpr

data class GLam(val param: String, val body: GExpr) : GExpr

data class GApp(val fn: GExpr, val arg: GExpr) : GExpr

data class GLet(val name: String, val value: GExpr, val body: GExpr) : GExpr

data class GLetRec(val name: String, val value: GExpr, val body: GExpr) : GExpr

data class GIf(val test: GExpr, val onTrue: GExpr, val onFalse: GExpr) : GExpr

data class GAdd(val left: GExpr, val right: GExpr) : GExpr

data class GMul(val left: GExpr, val right: GExpr) : GExpr

data class GLt(val left: GExpr, val right: GExpr) : GExpr

data class GSet(val name: String, val value: GExpr) : GExpr

data class GSub(val left: GExpr, val right: GExpr) : GExpr

data class GDiv(val left: GExpr, val right: GExpr) : GExpr

data class GMod(val left: GExpr, val right: GExpr) : GExpr

data class GEq(val left: GExpr, val right: GExpr) : GExpr

data class GBlock(val statements: List<GStmt>) : GExpr

data class GWhen(val subject: GExpr?, val cases: List<GCase>, val otherwise: GExpr?) : GExpr

data class GIs(val value: GExpr, val typeName: String) : GExpr

data class GMember(val target: GExpr, val name: String) : GExpr

data class GIndex(val target: GExpr, val index: GExpr) : GExpr

data class GConstruct(val className: String, val arguments: List<GExpr>) : GExpr

data class GCase(val pattern: GExpr?, val body: GExpr)

sealed interface GStmt

data class GExprStmt(val expression: GExpr) : GStmt

data class GVarStmt(val name: String, val init: GExpr) : GStmt

data class GAssignStmt(val name: String, val value: GExpr) : GStmt

data class GWhileStmt(val condition: GExpr, val body: GBlock) : GStmt

data class GReturnStmt(val value: GExpr) : GStmt

data object GBreakStmt : GStmt

data object GContinueStmt : GStmt

sealed interface GValue

data class GNumV(val n: Long) : GValue

data class GBoolV(val b: Boolean) : GValue

data class GStrV(val text: String) : GValue

data object GNullV : GValue

data object GUnitV : GValue

data object GUnassigned : GValue

data class GListV(val items: List<GValue>) : GValue

data class GObjectV(val className: String, val fields: MutableMap<String, GValue>) : GValue

data class GClosV(val param: String, val body: GExpr, val env: GFrame) : GValue

data class GReturnV(val value: GValue?) : GValue

data object GBreakV : GValue

data object GContinueV : GValue

class GFrame(val cells: MutableMap<String, GValue>, val parent: GFrame?) {
    fun lookup(name: String): GValue? {
        var here: GFrame? = this
        while (here != null) {
            val current: GFrame = here ?: return null
            val hit = current.cells[name]
            if (hit != null) {
                return hit
            }
            here = current.parent
        }
        return null
    }

    fun define(name: String, value: GValue): GValue {
        this.cells[name] = value
        return value
    }

    fun assign(name: String, value: GValue): Boolean {
        var here: GFrame? = this
        while (here != null) {
            val current: GFrame = here ?: return false
            if (current.cells.containsKey(name)) {
                current.cells[name] = value
                return true
            }
            here = current.parent
        }
        return false
    }
}

fun gEval(expr: GExpr, env: GFrame): GValue? =
    when (expr) {
        is GNum -> GNumV(expr.n)
        is GBool -> GBoolV(expr.b)
        is GStr -> GStrV(expr.text)
        is GNull -> GNullV
        is GVar -> {
            val found = env.lookup(expr.name)
            if (found == null || found is GUnassigned) {
                null
            } else {
                found
            }
        }
        is GLam -> GClosV(expr.param, expr.body, env)
        is GApp -> {
            val fn = gEval(expr.fn, env) ?: return null
            if (fn is GClosV) {
                val arg = gEval(expr.arg, env) ?: return null
                gApply(fn, arg)
            } else {
                null
            }
        }
        is GLet -> {
            val value = gEval(expr.value, env) ?: return null
            gEval(expr.body, GFrame(mutableMapOf(expr.name to value), env))
        }
        is GLetRec -> {
            val frame = GFrame(mutableMapOf(expr.name to GUnassigned), env)
            val value = gEval(expr.value, frame) ?: return null
            frame.cells[expr.name] = value
            gEval(expr.body, frame)
        }
        is GIf -> {
            val test = gEval(expr.test, env) ?: return null
            if (test !is GBoolV) {
                null
            } else if (test.b) {
                gEval(expr.onTrue, env)
            } else {
                gEval(expr.onFalse, env)
            }
        }
        is GAdd -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            numOp(left, right, "+")
        }
        is GSub -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            numOp(left, right, "-")
        }
        is GMul -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            numOp(left, right, "*")
        }
        is GDiv -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            numOp(left, right, "/")
        }
        is GMod -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            numOp(left, right, "%")
        }
        is GLt -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            cmpOp(left, right)
        }
        is GEq -> {
            val left = gEval(expr.left, env) ?: return null
            val right = gEval(expr.right, env) ?: return null
            GBoolV(valueEqual(left, right))
        }
        is GSet -> {
            val value = gEval(expr.value, env) ?: return null
            if (env.assign(expr.name, value)) {
                GBoolV(true)
            } else {
                null
            }
        }
        is GBlock -> runStatements(expr.statements, env)
        is GWhen -> runWhen(expr, env)
        is GIs -> {
            val value = gEval(expr.value, env) ?: return null
            GBoolV(typeNameOf(value) == expr.typeName)
        }
        is GMember -> memberOf(gEval(expr.target, env), expr.name)
        is GIndex -> indexOf(gEval(expr.target, env), gEval(expr.index, env))
        is GConstruct -> constructValue(expr, env)
    }

/** Application, the sole unwrapper of the control markers: a claimed return
 * unwraps to its value, an unclaimed marker is an error. */
fun gApply(fn: GValue?, arg: GValue?): GValue? {
    if (fn is GClosV) {
        val frame = GFrame(mutableMapOf(fn.param to (arg ?: GNullV)), fn.env)
        return unwrapApply(gEval(fn.body, frame))
    }
    return null
}

fun unwrapApply(outcome: GValue?): GValue? =
    when (outcome) {
        is GReturnV -> outcome.value
        is GBreakV -> null
        is GContinueV -> null
        else -> outcome
    }

fun numOp(left: GValue?, right: GValue?, operator: String): GValue? {
    if (left is GNumV && right is GNumV) {
        return when (operator) {
            "+" -> GNumV(left.n + right.n)
            "-" -> GNumV(left.n - right.n)
            "*" -> GNumV(left.n * right.n)
            "/" -> if (right.n == 0L) null else GNumV(left.n / right.n)
            else -> if (right.n == 0L) null else GNumV(left.n % right.n)
        }
    }
    return null
}

fun cmpOp(left: GValue?, right: GValue?): GValue? {
    if (left is GNumV && right is GNumV) {
        return GBoolV(left.n < right.n)
    }
    return null
}

fun valueEqual(left: GValue, right: GValue): Boolean =
    when {
        left is GNumV && right is GNumV -> left.n == right.n
        left is GBoolV && right is GBoolV -> left.b == right.b
        left is GStrV && right is GStrV -> left.text == right.text
        left is GNullV && right is GNullV -> true
        left is GListV && right is GListV -> listEqual(left, right)
        else -> false
    }

fun listEqual(left: GListV, right: GListV): Boolean {
    if (left.items.size != right.items.size) {
        return false
    }
    var index = 0
    while (index < left.items.size) {
        if (!valueEqual(left.items.get(index), right.items.get(index))) {
            return false
        }
        index = index + 1
    }
    return true
}

fun typeNameOf(value: GValue): String =
    when (value) {
        is GNumV -> "Long"
        is GBoolV -> "Boolean"
        is GStrV -> "String"
        is GNullV -> "Null"
        is GUnitV -> "Unit"
        is GUnassigned -> "Unassigned"
        is GListV -> "List"
        is GObjectV -> value.className
        is GClosV -> "Function"
        is GReturnV -> "Return"
        is GBreakV -> "Break"
        is GContinueV -> "Continue"
    }

fun memberOf(target: GValue?, name: String): GValue? {
    if (target is GObjectV) {
        return target.fields[name] ?: GNullV
    }
    return null
}

fun indexOf(target: GValue?, index: GValue?): GValue? {
    if (target is GListV && index is GNumV) {
        if (index.n >= 0L && index.n < target.items.size.toLong()) {
            return target.items.get(index.n.toInt())
        }
    }
    return null
}

/** The operand walk of section 4.1.1: one value per operand expression,
 * stopping at the first failure. */
fun listOfValues(operands: List<GExpr>, env: GFrame): List<GValue>? {
    val out = mutableListOf<GValue>()
    var index = 0
    while (index < operands.size) {
        val value = gEval(operands.get(index), env) ?: return null
        out.add(value)
        index = index + 1
    }
    return out
}

fun constructValue(expr: GConstruct, env: GFrame): GValue? {
    val args = listOfValues(expr.arguments, env) ?: return null
    return if (expr.className == "List") {
        GListV(args)
    } else if (expr.className == "Pair" && args.size == 2) {
        val fields = mutableMapOf<String, GValue>()
        fields["first"] = args.get(0)
        fields["second"] = args.get(1)
        GObjectV("Pair", fields)
    } else {
        val fields = mutableMapOf<String, GValue>()
        var at = 0L
        while (at < args.size.toLong()) {
            fields["arg" + showLong(at)] = args.get(at.toInt())
            at = at + 1L
        }
        GObjectV(expr.className, fields)
    }
}

fun runStatements(statements: List<GStmt>, env: GFrame): GValue? {
    var last: GValue = GUnitV
    var index = 0
    while (index < statements.size) {
        val current = statements.get(index)
        val value = runStatement(current, env) ?: return null
        if (value is GReturnV || value is GBreakV || value is GContinueV) {
            return value
        }
        last = value
        index = index + 1
    }
    if (statements.size == 0 || statements.get(statements.size - 1) !is GExprStmt) {
        return GUnitV
    }
    return last
}

fun runStatement(statement: GStmt, env: GFrame): GValue? =
    when (statement) {
        is GExprStmt -> gEval(statement.expression, env)
        is GVarStmt -> {
            val value = gEval(statement.init, env) ?: return null
            env.define(statement.name, value)
            GUnitV
        }
        is GAssignStmt -> {
            val value = gEval(statement.value, env) ?: return null
            if (env.assign(statement.name, value)) {
                GUnitV
            } else {
                null
            }
        }
        is GWhileStmt -> runWhile(statement, env)
        is GReturnStmt -> {
            val value = gEval(statement.value, env) ?: return null
            GReturnV(value)
        }
        is GBreakStmt -> GBreakV
        is GContinueStmt -> GContinueV
    }

fun runWhile(statement: GWhileStmt, env: GFrame): GValue? {
    while (true) {
        val test = gEval(statement.condition, env) ?: return null
        if (test !is GBoolV) {
            return null
        }
        if (!test.b) {
            return GUnitV
        }
        val body = runStatements(statement.body.statements, env) ?: return null
        if (body is GReturnV) {
            return body
        }
        if (body is GBreakV) {
            return GUnitV
        }
    }
    return null
}

fun runWhen(expr: GWhen, env: GFrame): GValue? {
    val subjectExpr = expr.subject
    val subject = if (subjectExpr == null) GNullV else gEval(subjectExpr, env) ?: return null
    var index = 0
    while (index < expr.cases.size) {
        val current = expr.cases.get(index)
        val pattern = current.pattern
        val matched = if (pattern == null) true else valueEqual(gEval(pattern, env) ?: return null, subject)
        if (matched) {
            return gEval(current.body, env)
        }
        index = index + 1
    }
    val fallback = expr.otherwise
    return if (fallback == null) GUnitV else gEval(fallback, env)
}

fun showLong(n: Long): String = "${'$'}{n}"

fun render(g: GExpr): String =
    when (g) {
        is GNum -> showLong(g.n)
        is GBool -> if (g.b) "true" else "false"
        is GStr -> g.text
        is GNull -> "null"
        is GVar -> g.name
        is GLam -> "(lam " + g.param + " " + render(g.body) + ")"
        is GApp -> "(app " + render(g.fn) + " " + render(g.arg) + ")"
        is GLet -> "(let " + g.name + " " + render(g.value) + " " + render(g.body) + ")"
        is GLetRec -> "(letrec " + g.name + " " + render(g.value) + " " + render(g.body) + ")"
        is GIf -> "(if " + render(g.test) + " " + render(g.onTrue) + " " + render(g.onFalse) + ")"
        is GAdd -> "(+ " + render(g.left) + " " + render(g.right) + ")"
        is GSub -> "(- " + render(g.left) + " " + render(g.right) + ")"
        is GMul -> "(* " + render(g.left) + " " + render(g.right) + ")"
        is GDiv -> "(/ " + render(g.left) + " " + render(g.right) + ")"
        is GMod -> "(% " + render(g.left) + " " + render(g.right) + ")"
        is GLt -> "(< " + render(g.left) + " " + render(g.right) + ")"
        is GEq -> "(== " + render(g.left) + " " + render(g.right) + ")"
        is GSet -> "(set! " + g.name + " " + render(g.value) + ")"
        is GBlock -> "(block " + showLong(g.statements.size.toLong()) + ")"
        is GWhen -> "(when " + showLong(g.cases.size.toLong()) + ")"
        is GIs -> "(is " + render(g.value) + " " + g.typeName + ")"
        is GMember -> "(." + g.name + " " + render(g.target) + ")"
        is GIndex -> "(index " + render(g.target) + " " + render(g.index) + ")"
        is GConstruct -> "(construct " + g.className + " " + showLong(g.arguments.size.toLong()) + ")"
    }

fun renderValue(v: GValue?): String =
    when (v) {
        null -> "error"
        is GNumV -> showLong(v.n)
        is GBoolV -> if (v.b) "true" else "false"
        is GStrV -> v.text
        is GNullV -> "null"
        is GUnitV -> "unit"
        is GUnassigned -> "unassigned"
        is GListV -> renderList(v)
        is GObjectV -> renderObject(v)
        is GClosV -> "closure"
        else -> "marker"
    }

fun renderObject(v: GObjectV): String {
    if (v.className != "Pair") {
        return v.className
    }
    val first = v.fields["first"]
    val second = v.fields["second"]
    if (first == null || second == null) {
        return v.className
    }
    return "(" + renderValue(first) + " . " + renderValue(second) + ")"
}
fun renderList(v: GListV): String {
    var out = "["
    var index = 0
    while (index < v.items.size) {
        if (index > 0) {
            out = out + ", "
        }
        out = out + renderValue(v.items.get(index))
        index = index + 1
    }
    return out + "]"
}
    """.trimIndent()
