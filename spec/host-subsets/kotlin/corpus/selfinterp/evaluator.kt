// SPDX-License-Identifier: GPL-3.0-only
// The guest self-interpreter of section 5.50: this file is guest SOURCE,
// admitted, checked, and executed as a guest program that interprets the
// witness syntax family of the host-subsets specification. The value and
// frame declarations mirror the section 4.1 kernel listing verbatim;
// gEval/gApply run over translated programs as ordinary data, closures
// capture frames, gApply is the sole unwrapper of GReturnV, and unclaimed
// markers answer null.

/** The witness syntax family of the specification, plus the extensions and
 * explicit-control statements required by section 7. */
sealed interface GExpr

data class GNum(val n: Long) : GExpr

data class GBool(val b: Boolean) : GExpr

data class GStr(val s: String) : GExpr

data class GDouble(val d: Double) : GExpr

data object GNull : GExpr

data class GVar(val name: String) : GExpr

data class GLam(val param: String, val body: GExpr) : GExpr

data class GApp(val fn: GExpr, val arg: GExpr) : GExpr

data class GLet(val name: String, val init: GExpr, val body: GExpr) : GExpr

data class GLetRec(val name: String, val init: GExpr, val body: GExpr) : GExpr

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

data class GMember(val target: GExpr, val name: String) : GExpr

data class GIndex(val target: GExpr, val index: GExpr) : GExpr

data class GIs(val value: GExpr, val typeName: String) : GExpr

data class GConstruct(val className: String, val args: List<GExpr>) : GExpr

sealed interface GStmt

data class GExprStmt(val expression: GExpr) : GStmt

data class GVarStmt(val name: String, val init: GExpr) : GStmt

data class GAssignStmt(val name: String, val value: GExpr) : GStmt

data class GWhileStmt(val cond: GExpr, val body: GBlock) : GStmt

data class GReturnStmt(val value: GExpr) : GStmt

data object GBreakStmt : GStmt

data object GContinueStmt : GStmt

data class GCase(val pattern: GExpr?, val body: GExpr)

/** The interpreter's values: the section 4.1 kernel listing. Control
 * signals travel as values; gApply is the sole unwrapper of GReturnV, and
 * unclaimed markers answer null. */
sealed interface GValue

data class GNumV(val n: Long) : GValue

data class GBoolV(val b: Boolean) : GValue

data class GStrV(val s: String) : GValue

data class GDoubleV(val d: Double) : GValue

data object GNullV : GValue

data object GUnitV : GValue

data object GUnassigned : GValue

data class GClosV(val param: String, val body: GExpr, val env: GFrame) : GValue

data class GListV(val items: List<GValue>) : GValue

data class GObjectV(val className: String, val fields: Map<String, GValue>) : GValue

data class GReturnV(val value: GValue) : GValue

data object GBreakV : GValue

data object GContinueV : GValue

/** The witness environment frame: mutable cells over a lexical parent. */
class GFrame(val cells: MutableMap<String, GValue>, val parent: GFrame?) {
    fun lookup(name: String): GValue? {
        var here: GFrame? = this
        while (here != null) {
            val current: GFrame = here ?: return null
            val hit = current.cells[name]
            if (hit != null) return hit
            here = current.parent
        }
        return null
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

fun rootFrame(): GFrame = GFrame(mutableMapOf(), null)

/** The direct guest evaluator: gEval computes the value of an expression in
 * a frame, or null when the computation has no value. */
fun gEval(g: GExpr, env: GFrame): GValue? =
    when (g) {
        is GNum -> GNumV(g.n)
        is GBool -> GBoolV(g.b)
        is GStr -> GStrV(g.s)
        is GDouble -> GDoubleV(g.d)
        is GNull -> GNullV
        is GVar -> readVar(g.name, env)
        is GLam -> GClosV(g.param, g.body, env)
        is GApp -> applySafely(gEval(g.fn, env), gEval(g.arg, env))
        is GLet -> evalLet(g, env)
        is GLetRec -> evalLetRec(g, env)
        is GIf -> evalIf(g, env)
        is GAdd -> numOp(gEval(g.left, env), gEval(g.right, env), "+")
        is GSub -> numOp(gEval(g.left, env), gEval(g.right, env), "-")
        is GMul -> numOp(gEval(g.left, env), gEval(g.right, env), "*")
        is GDiv -> numOp(gEval(g.left, env), gEval(g.right, env), "/")
        is GMod -> numOp(gEval(g.left, env), gEval(g.right, env), "%")
        is GLt -> cmpOp(gEval(g.left, env), gEval(g.right, env))
        is GEq -> eqOp(gEval(g.left, env), gEval(g.right, env))
        is GSet -> evalSet(g, env)
        is GBlock -> runStatements(g.statements, env)
        is GWhen -> runWhen(g, env)
        is GMember -> runMember(gEval(g.target, env), g.name)
        is GIndex -> runIndex(gEval(g.target, env), gEval(g.index, env))
        is GIs -> evalIs(g, env)
        is GConstruct -> construct(g.className, g.args, env)
    }

fun one(value: GValue?): GValue = value ?: GUnassigned

fun applySafely(fn: GValue?, arg: GValue?): GValue? {
    val callable: GValue = fn ?: return null
    val input: GValue = arg ?: return null
    return gApply(callable, listOf(input))
}

fun evalLet(g: GLet, env: GFrame): GValue? {
    val value = gEval(g.init, env) ?: return null
    return gEval(g.body, GFrame(mutableMapOf(g.name to value), env))
}

fun evalIs(g: GIs, env: GFrame): GValue? {
    val value = gEval(g.value, env) ?: return null
    return GBoolV(typeName(value) == g.typeName)
}

fun readVar(name: String, env: GFrame): GValue? {
    val hit = env.lookup(name) ?: return null
    if (hit is GUnassigned) return null
    return hit
}

fun evalLetRec(g: GLetRec, env: GFrame): GValue? {
    val frame = GFrame(mutableMapOf<String, GValue>(g.name to GUnassigned), env)
    val value = gEval(g.init, frame) ?: return null
    frame.cells[g.name] = value
    return gEval(g.body, frame)
}

fun evalIf(g: GIf, env: GFrame): GValue? {
    val test = gEval(g.test, env) ?: return null
    if (truthy(test)) return gEval(g.onTrue, env)
    return gEval(g.onFalse, env)
}

fun evalSet(g: GSet, env: GFrame): GValue? {
    val value = gEval(g.value, env) ?: return null
    if (!env.assign(g.name, value)) return null
    return value
}

/** Application, the sole unwrapper of GReturnV: a claimed return unwraps to
 * its value, and unclaimed markers or non-functions answer null. */
fun gApply(fn: GValue, args: List<GValue>): GValue? {
    if (fn !is GClosV) return null
    if (args.size != 1) return null
    val frame = GFrame(mutableMapOf(fn.param to args[0]), fn.env)
    return unwrapReturn(gEval(fn.body, frame))
}

fun unwrapReturn(value: GValue?): GValue? {
    if (value is GReturnV) return value.value
    if (value is GBreakV) return null
    if (value is GContinueV) return null
    return value
}

fun runStatements(statements: List<GStmt>, env: GFrame): GValue? {
    if (statements.isEmpty()) return GUnitV
    val first = runStatement(statements[0], env) ?: return null
    if (first is GReturnV || first is GBreakV || first is GContinueV) return first
    return runStatements(statements.drop(1), env)
}

fun runStatement(statement: GStmt, env: GFrame): GValue? =
    when (statement) {
        is GExprStmt -> gEval(statement.expression, env)
        is GVarStmt -> runVar(statement, env)
        is GAssignStmt -> evalSet(GSet(statement.name, statement.value), env)
        is GWhileStmt -> runWhile(statement, env)
        is GReturnStmt -> wrapReturn(gEval(statement.value, env))
        is GBreakStmt -> GBreakV
        is GContinueStmt -> GContinueV
    }

fun wrapReturn(value: GValue?): GValue? {
    val unwrapped: GValue = value ?: return null
    return GReturnV(unwrapped)
}

fun runVar(statement: GVarStmt, env: GFrame): GValue {
    env.cells[statement.name] = one(gEval(statement.init, env))
    return GUnitV
}

fun runWhile(statement: GWhileStmt, env: GFrame): GValue? {
    val test = gEval(statement.cond, env) ?: return null
    if (!truthy(test)) return GUnitV
    val body = runStatements(statement.body.statements, env) ?: return null
    if (body is GReturnV) return body
    if (body is GBreakV) return GUnitV
    return runWhile(statement, env)
}

fun runWhen(g: GWhen, env: GFrame): GValue? {
    val source: GExpr = g.subject ?: return runCases(g.cases, GNullV, g.otherwise, env)
    val subject: GValue = gEval(source, env) ?: return null
    return runCases(g.cases, subject, g.otherwise, env)
}

fun runCases(cases: List<GCase>, subject: GValue, otherwise: GExpr?, env: GFrame): GValue? {
    if (cases.isEmpty()) {
        val alternative: GExpr = otherwise ?: return GUnitV
        return gEval(alternative, env)
    }
    val current = cases[0]
    if (caseMatches(current, subject, env)) return gEval(current.body, env)
    return runCases(cases.drop(1), subject, otherwise, env)
}

fun caseMatches(current: GCase, subject: GValue, env: GFrame): Boolean {
    val tested: GExpr = current.pattern ?: return true
    val pattern = gEval(tested, env) ?: return false
    val equal = eqOp(pattern, subject) ?: return false
    return truthy(equal)
}

fun truthy(value: GValue): Boolean = value is GBoolV && value.b

fun numOp(left: GValue?, right: GValue?, operator: String): GValue? {
    if (left == null || right == null) return null
    if (left is GNumV && right is GNumV) return GNumV(applyLong(left.n, right.n, operator))
    if (left is GDoubleV && right is GDoubleV) {
        val answer =
            when (operator) {
                "+" -> left.d + right.d
                "-" -> left.d - right.d
                "*" -> left.d * right.d
                "/" -> left.d / right.d
                else -> return null
            }
        return GDoubleV(answer)
    }
    return null
}

fun applyLong(left: Long, right: Long, operator: String): Long =
    when (operator) {
        "+" -> left + right
        "-" -> left - right
        "*" -> left * right
        "/" -> if (right == 0L) 0L else left / right
        else -> if (right == 0L) 0L else left % right
    }


fun cmpOp(left: GValue?, right: GValue?): GValue? {
    if (left == null || right == null) return null
    if (left is GNumV && right is GNumV) return GBoolV(left.n < right.n)
    if (left is GDoubleV && right is GDoubleV) return GBoolV(left.d < right.d)
    return null
}

fun eqOp(left: GValue?, right: GValue?): GValue? {
    if (left == null || right == null) return null
    if (left is GNumV && right is GNumV) return GBoolV(left.n == right.n)
    if (left is GDoubleV && right is GDoubleV) return GBoolV(left.d == right.d)
    if (left is GBoolV && right is GBoolV) return GBoolV(left.b == right.b)
    if (left is GStrV && right is GStrV) return GBoolV(left.s == right.s)
    if (left is GNullV && right is GNullV) return GBoolV(true)
    return GBoolV(false)
}

fun typeName(value: GValue?): String =
    when (value) {
        is GNumV -> "GNumV"
        is GBoolV -> "GBoolV"
        is GStrV -> "GStrV"
        is GDoubleV -> "GDoubleV"
        is GClosV -> "GClosV"
        is GListV -> "GListV"
        is GObjectV -> "GObjectV"
        is GNullV -> "GNullV"
        else -> "marker"
    }

fun runMember(target: GValue?, name: String): GValue? {
    if (target == null) return null
    if (target !is GObjectV) return GNullV
    return target.fields[name] ?: GNullV
}

fun runIndex(target: GValue?, index: GValue?): GValue? {
    if (target == null || index == null) return null
    if (target !is GListV) return GNullV
    if (index !is GNumV) return GNullV
    if (index.n < 0L) return GNullV
    val items = target.items
    if (index.n >= items.size.toLong()) return GNullV
    return items[index.n.toInt()]
}

/** GConstruct is the desugaring target of `to` and object construction:
 * `Pair` builds first/second fields and other classes build empty objects. */
fun construct(className: String, args: List<GExpr>, env: GFrame): GValue? {
    if (className == "Pair") {
        if (args.size != 2) return null
        val first = gEval(args[0], env) ?: return null
        val second = gEval(args[1], env) ?: return null
        return GObjectV("Pair", mapOf("first" to first, "second" to second))
    }
    return GObjectV(className, mapOf())
}

/** The self-interpretation program: letrec factorial over the witness
 * family, interpreted by the evaluator itself. */
fun factorialProgram(): GExpr =
    GLetRec(
        "fact",
        GLam(
            "n",
            GIf(
                GLt(GVar("n"), GNum(1L)),
                GNum(1L),
                GMul(GVar("n"), GApp(GVar("fact"), GSub(GVar("n"), GNum(1L)))),
            ),
        ),
        GApp(GVar("fact"), GNum(5L)),
    )

/** Entry point: the evaluator interprets its own witness program and prints
 * the observation through the native protocol. */
fun main() {
    val result = gEval(factorialProgram(), rootFrame())
    if (result is GNumV) {
        println(result.n)
    } else {
        println(0L)
    }
}
