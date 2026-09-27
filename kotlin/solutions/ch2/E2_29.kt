// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.29

package sicp.ch2.exercises

/** One branch: a rod of [length], carrying a [structure]. */
public data class Branch(
    val length: Long,
    val structure: MobileStructure,
)

/** What hangs from a branch: a simple weight or another mobile. */
public sealed interface MobileStructure

/** A simple weight. */
public data class Weight(
    val value: Long,
) : MobileStructure

/** A mobile: the book's two-branch construction. */
public data class Mobile(
    val left: Branch,
    val right: Branch,
) : MobileStructure

/** The book's `make-mobile`. */
public fun makeMobile(
    left: Branch,
    right: Branch,
): Mobile = Mobile(left, right)

/** The book's `make-branch`. */
public fun makeBranch(
    length: Long,
    structure: MobileStructure,
): Branch = Branch(length, structure)

/** The book's selectors, part (a); the data-class properties carry them. */
public fun leftBranch(mobile: Mobile): Branch = mobile.left

public fun rightBranch(mobile: Mobile): Branch = mobile.right

public fun branchLength(branch: Branch): Long = branch.length

public fun branchStructure(branch: Branch): MobileStructure = branch.structure

/** (b) The book's `total-weight`. */
public fun totalWeight(structure: MobileStructure): Long =
    when (structure) {
        is Weight -> structure.value
        is Mobile -> totalWeight(branchStructure(leftBranch(structure))) + totalWeight(branchStructure(rightBranch(structure)))
    }

/** (c) The torque of one branch: its length times the weight hanging from it. */
private fun branchTorque(branch: Branch): Long = branchLength(branch) * totalWeight(branchStructure(branch))

/** (c) The book's `balanced`: equal torques at the top and balanced submobiles. */
public fun balanced(structure: MobileStructure): Boolean =
    when (structure) {
        is Weight -> {
            true
        }

        is Mobile -> {
            branchTorque(leftBranch(structure)) == branchTorque(rightBranch(structure)) &&
                balanced(branchStructure(leftBranch(structure))) &&
                balanced(branchStructure(rightBranch(structure)))
        }
    }

/**
 * (d) The book's second representation, pairs all the way down: a mobile
 * is a pair of branches and a branch is a pair of a length and a
 * structure. Only the constructors and selectors change.
 */
public sealed interface ConsStructure {
    /** A simple weight under the pair representation. */
    public data class ConsWeight(
        val value: Long,
    ) : ConsStructure

    /** Another mobile hanging from a branch. */
    public data class ConsSubMobile(
        val mobile: ConsMobile,
    ) : ConsStructure
}

public typealias ConsMobile = Pair<ConsBranch, ConsBranch>

public typealias ConsBranch = Pair<Long, ConsStructure>

public fun consMakeMobile(
    left: ConsBranch,
    right: ConsBranch,
): ConsMobile = left to right

public fun consMakeBranch(
    length: Long,
    structure: ConsStructure,
): ConsBranch = length to structure

public fun consLeftBranch(mobile: ConsMobile): ConsBranch = mobile.first

public fun consRightBranch(mobile: ConsMobile): ConsBranch = mobile.second

public fun consBranchLength(branch: ConsBranch): Long = branch.first

public fun consBranchStructure(branch: ConsBranch): ConsStructure = branch.second

/** (d) `totalWeight` re-expressed over the pair representation's selectors. */
public fun consTotalWeight(structure: ConsStructure): Long =
    when (structure) {
        is ConsStructure.ConsWeight -> {
            structure.value
        }

        is ConsStructure.ConsSubMobile -> {
            consTotalWeight(consBranchStructure(consLeftBranch(structure.mobile))) +
                consTotalWeight(consBranchStructure(consRightBranch(structure.mobile)))
        }
    }

/** The book-style sample mobile: (2, weight 3) beside (2, mobile of (1, 4) and (1, 5)). */
public fun sampleMobile(): Mobile =
    makeMobile(
        makeBranch(2L, Weight(3L)),
        makeBranch(2L, makeMobile(makeBranch(1L, Weight(4L)), makeBranch(1L, Weight(5L)))),
    )

/** `totalWeight` of the sample mobile is 12. */
public fun ex_2_29(): Long = totalWeight(sampleMobile())
