// SPDX-License-Identifier: GPL-3.0-only
// Corpus case amb/03-multiple-dwelling: the multiple-dwelling puzzle
fun distinct(a: Long, b: Long, c: Long, d: Long, e: Long): Boolean =
    a != b && a != c && a != d && a != e && b != c && b != d && b != e && c != d && c != e && d != e

fun main() {
    val baker = choose(1L, 2L, 3L, 4L, 5L)
    val cooper = choose(1L, 2L, 3L, 4L, 5L)
    val fletcher = choose(1L, 2L, 3L, 4L, 5L)
    val miller = choose(1L, 2L, 3L, 4L, 5L)
    val smith = choose(1L, 2L, 3L, 4L, 5L)
    demand(distinct(baker, cooper, fletcher, miller, smith))
    demand(baker != 5L)
    demand(cooper != 1L)
    demand(fletcher != 5L)
    demand(fletcher != 1L)
    demand(miller > cooper)
    demand(smith - fletcher != 1L)
    demand(fletcher - smith != 1L)
    println(baker * 10000L + cooper * 1000L + fletcher * 100L + miller * 10L + smith)
}
