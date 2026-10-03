// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/16-symbols-quotation: explicit syntax data replaces quotation
sealed interface Term

data class Sym(val name: String) : Term

data class Cons(val head: Term, val tail: Term) : Term

data object Empty : Term

fun render(t: Term): String =
    when {
        t is Sym -> "Sym(" + t.name + ")"
        t is Cons -> "Cons(" + render(t.head) + ", " + render(t.tail) + ")"
        else -> "Empty"
    }

fun main() {
    val quoted: Term = Cons(Sym("a"), Cons(Sym("b"), Empty))
    println(render(quoted))
}
