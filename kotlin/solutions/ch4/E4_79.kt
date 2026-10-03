// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.79: rules over parent-linked frames.

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.79: rule evaluation against explicit frames, without
// renaming. Each application runs in a fresh scope parented on the
// caller's frame, so the rule's `boss` resolves through its parent
// chain, while `x` and `title` bind in the application scope. The
// rule body shares that scope with the conclusion: its `x` must agree
// with the conclusion's `x`, not a freshened copy. A fact that
// disagrees on that shared variable fails the application.

/** Pattern matching and rule application over kernel frames. */
internal val MATCH_SOURCE: String =
    """
data class GuestRule(val conclusion: GExpr, val body: GExpr)

fun matchValue(pattern: GExpr, value: GValue, env: GFrame): Boolean {
    if (pattern is GVar) {
        val bound = env.lookup(pattern.name)
        if (bound == null) {
            env.define(pattern.name, value)
            return true
        }
        return valueEqual(bound, value)
    }
    if (pattern is GStr && value is GStrV) {
        return pattern.text == value.text
    }
    return false
}

fun matchList(pattern: GExpr, value: GValue, env: GFrame): Boolean {
    if (pattern is GConstruct && pattern.className == "List" && value is GListV) {
        if (pattern.arguments.size != value.items.size) {
            return false
        }
        var index = 0
        while (index < pattern.arguments.size) {
            if (!matchValue(pattern.arguments.get(index), value.items.get(index), env)) {
                return false
            }
            index = index + 1
        }
        return true
    }
    return false
}

fun applyRule(rule: GuestRule, conclusionFact: GValue, bodyFact: GValue, parent: GFrame): GValue? {
    val scope = GFrame(mutableMapOf<String, GValue>(), parent)
    if (!matchList(rule.conclusion, conclusionFact, scope)) {
        return null
    }
    if (!matchList(rule.body, bodyFact, scope)) {
        return null
    }
    val person = scope.lookup("x") ?: return null
    val title = scope.lookup("title") ?: return null
    return GListV(listOf(person, title))
}

fun fact(head: String, first: String, second: String): GValue =
    GListV(listOf(GStrV(head), GStrV(first), GStrV(second)))

fun supervisorRule(): GuestRule =
    GuestRule(
        GConstruct("List", listOf(GStr("supervisor"), GVar("x"), GVar("boss"))),
        GConstruct("List", listOf(GStr("job"), GVar("x"), GVar("title"))),
    )
    """.trimIndent()

/** Apply one parent-scoped rule to each fact pair: reports of Ben
 * joined with their job titles. => "Hacker\nprogrammer\nFect\nprogrammer\n" */
public fun scopedVersusRenaming(): List<String> {
    val out =
        outcomeText(
            Direct.run(
                KERNEL_SOURCE + "\n" + MATCH_SOURCE + "\n" +
                    """
fun main() {
    val parent = GFrame(mutableMapOf<String, GValue>("boss" to GStrV("Ben")), null)
    val supervisors = listOf(
        fact("supervisor", "Hacker", "Ben"),
        fact("supervisor", "Fect", "Ben"),
        fact("supervisor", "Ben", "Warbucks"),
    )
    val jobs = listOf(
        fact("job", "Hacker", "programmer"),
        fact("job", "Fect", "programmer"),
        fact("job", "Ben", "wizard"),
    )
    val rule = supervisorRule()
    var i = 0
    while (i < supervisors.size) {
        var j = 0
        while (j < jobs.size) {
            val result = applyRule(rule, supervisors.get(i), jobs.get(j), parent)
            if (result is GListV) {
                println(renderValue(result.items.get(0)))
                println(renderValue(result.items.get(1)))
            }
            j = j + 1
        }
        i = i + 1
    }
}
                    """.trimIndent(),
            ),
        )
    return out.lines().filter { line -> line.isNotEmpty() }
}
