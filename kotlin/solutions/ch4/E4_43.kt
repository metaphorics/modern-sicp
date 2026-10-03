// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.43

package sicp.ch4.solutions

// Exercise 4.43: the yacht puzzle. The daughters' names are the search's
// data, the puzzle's facts are requirements, and the question -- whose
// daughter is Lorna -- is read off the surviving assignment. Told that
// Mary Ann is Moore's daughter, Lorna's father is Downing. Untold, Parker
// and Downing both fit.

/** The yacht puzzle, told and untold, as guest source. */
internal val YACHT_SOURCE: String =
    AMB_BASE_PRELUDE + "\n" +
        """
fun lornasFather(moores: String, downings: String, halls: String, parkers: String): String =
    if (downings == "lorna") {
        "downing"
    } else if (halls == "lorna") {
        "hall"
    } else if (parkers == "lorna") {
        "parker"
    } else {
        "moore"
    }

fun gabriellesFatherRule(moores: String, downings: String, halls: String, parkers: String): Boolean =
    if (moores == "gabrielle") {
        parkers == "lorna"
    } else if (halls == "gabrielle") {
        parkers == "rosalind"
    } else if (downings == "gabrielle") {
        parkers == "melissa"
    } else {
        false
    }

fun yachtBody(moores: String): Unit {
    val downings = anElementOfString(listOf("lorna", "gabrielle", "rosalind", "melissa", "mary-ann"))
    val halls = anElementOfString(listOf("lorna", "gabrielle", "rosalind", "melissa", "mary-ann"))
    val barnacles = "melissa"
    val parkers = anElementOfString(listOf("lorna", "gabrielle", "rosalind", "melissa", "mary-ann"))
    requireThat(isDistinctText(listOf(moores, downings, halls, barnacles, parkers)))
    requireThat(halls != "rosalind")
    requireThat(downings != "melissa")
    requireThat(gabriellesFatherRule(moores, downings, halls, parkers))
    println("((lornas-father " + lornasFather(moores, downings, halls, parkers) + "))")
}

fun yachtTold(): Unit {
    yachtBody("mary-ann")
}

fun yachtUntold(): Unit {
    yachtBody(anElementOfString(listOf("gabrielle", "rosalind", "melissa", "mary-ann")))
}
        """.trimIndent()

/** Told Mary Ann is Moore's daughter. => [((lornas-father downing))] */
public fun yachtToldAnswers(): List<String> = searchLines(YACHT_SOURCE + "\nfun main() {\n    budgetCap = 1000000L\n    yachtTold()\n}\n")

/** Untold, Parker and Downing both fit.
 * => [((lornas-father parker)), ((lornas-father downing))] */
public fun yachtUntoldAnswers(): List<String> =
    searchLines(YACHT_SOURCE + "\nfun main() {\n    budgetCap = 1000000L\n    yachtUntold()\n}\n")
