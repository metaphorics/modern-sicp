// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import sicp.runtime.VInt
import sicp.runtime.VSym
import sicp.runtime.Value

/**
 * One node of the tree table: a (key, value) record with the two
 * subtree slots, mutable in place like the records of the section.
 */
public class TreeNode(
    public val key: Value,
    public var value: Value,
    public var left: TreeNode? = null,
    public var right: TreeNode? = null,
)

/**
 * The key ordering the tree assumes: symbols compare lexicographically
 * by name, integers numerically; the two never match and a comparison
 * between them is a caller error.
 */
public fun compareKeys(
    a: Value,
    b: Value,
): Int =
    when {
        a is VSym && b is VSym -> a.name.compareTo(b.name)
        a is VInt && b is VInt -> a.n.compareTo(b.n)
        else -> throw IllegalArgumentException("incomparable table keys: $a against $b")
    }

/**
 * The tree-organized table of exercise 3.26: the records hang off one
 * root instead of an unordered backbone, so lookup and insert walk one
 * root-to-leaf path, in-place overwriting a found record and hanging a
 * fresh TreeNode on the null slot where the walk falls off.
 */
public class TreeTable {
    private var root: TreeNode? = null

    public fun lookup(key: Value): Value? {
        var node = root
        while (node != null) {
            val cmp = compareKeys(key, node.key)
            if (cmp == 0) {
                return node.value
            }
            node = if (cmp < 0) node.left else node.right
        }
        return null
    }

    public fun insert(
        key: Value,
        value: Value,
    ) {
        val fresh = TreeNode(key, value)
        val head = root
        if (head == null) {
            root = fresh
            return
        }
        var node: TreeNode = head
        while (true) {
            val cmp = compareKeys(key, node.key)
            if (cmp == 0) {
                node.value = value
                return
            }
            if (cmp < 0) {
                val left = node.left
                if (left == null) {
                    node.left = fresh
                    return
                }
                node = left
            } else {
                val right = node.right
                if (right == null) {
                    node.right = fresh
                    return
                }
                node = right
            }
        }
    }
}
