// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.27

package sicp.ch2.exercises

/**
 * Renders a tree in the book's nested-list surface syntax, so
 * interactions can show tree results the way the book prints them.
 */
public fun toBookString(t: Tree): String =
    when (t) {
        is Tree.Leaf -> t.value.toString()
        is Tree.Node -> t.subtrees.joinToString(prefix = "(", postfix = ")", separator = " ") { toBookString(it) }
    }

/** The book's `reverse` over a tree: reverses the top-level subtrees only. */
public fun reverseTree(t: Tree): Tree =
    when (t) {
        is Tree.Leaf -> t
        is Tree.Node -> Tree.Node(t.subtrees.asReversed())
    }

/** The book's `deep-reverse`: reverses the subtrees at every level. */
public fun deepReverse(t: Tree): Tree =
    when (t) {
        is Tree.Leaf -> t
        is Tree.Node -> Tree.Node(t.subtrees.asReversed().map(::deepReverse))
    }

/** `deepReverse` of the book's `((1 2) (3 4))`, printed as `((4 3) (2 1))`. */
public fun ex_2_27(): String = toBookString(deepReverse(bookPairTree()))

/** The book's x: `((1 2) (3 4))` as a tree. */
public fun bookPairTree(): Tree = tree(tree(leaf(1L), leaf(2L)), tree(leaf(3L), leaf(4L)))
