// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.74

package sicp.ch2.exercises

/**
 * One division's personnel file, tagged by the division's own storage
 * shape (part a's "type information"). North keeps a map of employee
 * name to a field map, an alist in spirit; South keeps a flat list per
 * employee alternating field name and value, a property list in spirit —
 * a genuinely different structure, not a relabeling of North's.
 */
public sealed interface DivisionFile {
    public data class North(
        val recordsByName: Map<String, Map<String, String>>,
    ) : DivisionFile

    public data class South(
        val propertyLists: List<List<String>>,
    ) : DivisionFile
}

/** A record `getRecord` returns, tagged with the division it came from
 * (part b's requirement: `getSalary` still needs to know the shape). */
public data class TaggedRecord(
    val division: String,
    val fields: Map<String, String>,
)

private fun propertyListValue(
    plist: List<String>,
    key: String,
): String? {
    var i = 0
    while (i + 1 < plist.size) {
        if (plist[i] == key) return plist[i + 1]
        i += 2
    }
    return null
}

private fun propertyListFields(plist: List<String>): Map<String, String> {
    val fields = LinkedHashMap<String, String>()
    var i = 0
    while (i + 1 < plist.size) {
        fields[plist[i]] = plist[i + 1]
        i += 2
    }
    return fields
}

/** Part (a): dispatches on the division's own storage shape, tagging the result. */
public fun getRecord(
    name: String,
    file: DivisionFile,
): TaggedRecord? =
    when (file) {
        is DivisionFile.North -> {
            file.recordsByName[name]?.let { TaggedRecord("north", it) }
        }

        is DivisionFile.South -> {
            file.propertyLists
                .firstOrNull { propertyListValue(it, "name") == name }
                ?.let { TaggedRecord("south", propertyListFields(it)) }
        }
    }

/** Part (b): every division's record answers `salary` the same way once tagged. */
public fun getSalary(record: TaggedRecord): String? = record.fields["salary"]

/** Part (c): the first hit across every division's file, in order. */
public fun findEmployeeRecord(
    name: String,
    files: List<DivisionFile>,
): TaggedRecord? = files.firstNotNullOfOrNull { getRecord(name, it) }

/**
 * Two divisions with deliberately different internal shapes, searched
 * through the same three generic operations. Part (d): a third division
 * would need only its own `DivisionFile` case and a `when` arm in
 * `getRecord` recognizing it, both additions, not edits to North's or
 * South's own lookup code.
 */
public fun ex_2_74(): Triple<String?, String?, Boolean> {
    val north =
        DivisionFile.North(
            mapOf("Ben Bitdiddle" to mapOf("salary" to "60000", "address" to "Slumerville")),
        )
    val south =
        DivisionFile.South(
            listOf(listOf("name", "Alyssa P. Hacker", "salary", "80000", "address", "Cambridge")),
        )
    val files = listOf(north, south)
    val benSalary = findEmployeeRecord("Ben Bitdiddle", files)?.let(::getSalary)
    val alyssaSalary = findEmployeeRecord("Alyssa P. Hacker", files)?.let(::getSalary)
    val missingIsFound = findEmployeeRecord("Eva Lu Ator", files) != null
    return Triple(benSalary, alyssaSalary, missingIsFound)
}
