// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.21: register machines for both count-leaves
// variants of the statement, running on the 5.2 simulator with the
// list-structure memory operations of 5.3.1 as primitives. The tree
// lives in the memory as the book's pointers: a leaf is the number word
// itself, a node is the list of its planted subtrees, e0-terminated.
// The recursive machine (a) combines the two answers with +, parking
// the car answer in temp while the cdr is counted; the explicit-counter
// machine (b) accumulates n through both subcalls so only continue and
// tree ever take stack room. Each run is read beside the host oracle
// over the same tree, and the monitored stack meters of 5.2.4 price
// the recursion the statement asks to compare.

package sicp.ch5.solutions

import arrow.core.raise.Raise
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Reg
import sicp.runtime.Restore
import sicp.runtime.Save
import sicp.runtime.Stmt
import sicp.runtime.Test

/** The object-language tree the machines count: a leaf is a number, a
 *  node a list of subtrees. */
public sealed class Tree {
    /** A leaf: the book's `(not (pair? tree))` case, worth one. */
    public data class Leaf(
        val n: Long,
    ) : Tree()

    /** A node: the book's pair case, the planted list of subtrees. */
    public data class Node(
        val subtrees: List<Tree>,
    ) : Tree()
}

/** Plants the tree's cells through the same allocation path the
 *  machine's `cons` uses, and answers the pointer to the root. */
context(r: Raise<GuestError>)
public fun plantTree(
    memory: Memory,
    tree: Tree,
): GValue =
    when (tree) {
        is Tree.Leaf -> {
            numberWord(tree.n)
        }

        is Tree.Node -> {
            var acc: GValue = GValue.VNull
            for (sub in tree.subtrees.asReversed()) {
                acc = memory.cons(plantTree(memory, sub), acc)
            }
            acc
        }
    }

/** The machine of exercise 5.21a: pure recursion. */
public val countLeavesRecursive: List<Stmt> =
    listOf(
        Assign("continue", labelSrc("count-done")),
        Label("count-loop"),
        Test(opCond("null?", reg("tree"))),
        Branch("null-answer"),
        Test(opCond("pair?", reg("tree"))),
        Branch("tree-case"),
        Assign("val", constV(1)),
        Goto(GotoTarget.ByReg("continue")),
        Label("null-answer"),
        Assign("val", constV(0)),
        Goto(GotoTarget.ByReg("continue")),
        Label("tree-case"),
        Save("continue"),
        Save("tree"),
        Assign("continue", labelSrc("after-car")),
        Assign("tree", opSrc("car", reg("tree"))),
        Goto(GotoTarget.Lbl("count-loop")),
        Label("after-car"),
        Restore("tree"),
        Assign("tree", opSrc("cdr", reg("tree"))),
        Save("val"),
        Assign("continue", labelSrc("after-cdr")),
        Goto(GotoTarget.Lbl("count-loop")),
        Label("after-cdr"),
        Restore("temp"),
        Assign("val", opSrc("+", reg("val"), reg("temp"))),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
        Label("count-done"),
    )

/** The machine of exercise 5.21b: the explicit counter. */
public val countLeavesIterative: List<Stmt> =
    listOf(
        Assign("n", constV(0)),
        Assign("continue", labelSrc("count-done")),
        Label("count-loop"),
        Test(opCond("null?", reg("tree"))),
        Branch("null-case"),
        Test(opCond("pair?", reg("tree"))),
        Branch("tree-case"),
        Assign("n", opSrc("+", reg("n"), constV(1))),
        Goto(GotoTarget.ByReg("continue")),
        Label("null-case"),
        Goto(GotoTarget.ByReg("continue")),
        Label("tree-case"),
        Save("continue"),
        Save("tree"),
        Assign("continue", labelSrc("after-car")),
        Assign("tree", opSrc("car", reg("tree"))),
        Goto(GotoTarget.Lbl("count-loop")),
        Label("after-car"),
        Restore("tree"),
        Assign("tree", opSrc("cdr", reg("tree"))),
        Assign("continue", labelSrc("after-cdr")),
        Goto(GotoTarget.Lbl("count-loop")),
        Label("after-cdr"),
        Restore("continue"),
        Goto(GotoTarget.ByReg("continue")),
        Label("count-done"),
    )

/** The host oracle: the statement's definition, read directly. */
public fun countLeavesHost(tree: Tree): Int =
    when (tree) {
        is Tree.Leaf -> 1
        is Tree.Node -> tree.subtrees.sumOf { countLeavesHost(it) }
    }

/** One planted run: the answer word rendered in the book's pointer
 *  notation, and the monitored stack statistics of the run. */
private class CountRun(
    val answer: String,
    val statistics: String,
)

private fun runCountLeaves(
    controller: List<Stmt>,
    answerReg: Reg,
    tree: Tree,
): CountRun =
    machineScope {
        val memory = Memory(size = 64)
        val root = plantTree(memory, tree)
        val machine =
            freshMachine(
                setOf("tree", "val", "n", "continue", "temp"),
                listOperations(memory) + machineArithmetic,
                controller,
                mapOf("tree" to root),
            )
        runToHalt(machine)
        CountRun(wordToString(machine.registers.getValue(answerReg).content), statisticsLine(machine))
    }

/** Both machines over three planted trees, each line reading the two
 *  answers beside the oracle's, with each machine's stack use. */
public fun countLeavesRuns(): List<String> {
    val trees =
        listOf(
            "(1 2 (3 (4 5)))" to
                Tree.Node(
                    listOf(
                        Tree.Leaf(1),
                        Tree.Leaf(2),
                        Tree.Node(listOf(Tree.Leaf(3), Tree.Node(listOf(Tree.Leaf(4), Tree.Leaf(5))))),
                    ),
                ),
            "((7))" to Tree.Node(listOf(Tree.Node(listOf(Tree.Leaf(7))))),
            "()" to Tree.Node(emptyList()),
        )
    return trees.map { (name, tree) ->
        val recursive = runCountLeaves(countLeavesRecursive, "val", tree)
        val iterative = runCountLeaves(countLeavesIterative, "n", tree)
        val oracle = countLeavesHost(tree)
        "$name: recursive ${recursive.answer}, iterative ${iterative.answer}, oracle $oracle; " +
            "recursive stack ${recursive.statistics}, iterative stack ${iterative.statistics}"
    }
}
