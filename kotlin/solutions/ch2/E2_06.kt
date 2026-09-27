// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.6

package sicp.ch2.exercises

public typealias Church<T> = ((T) -> T) -> (T) -> T

public fun <T> churchZero(): Church<T> = { _ -> { x -> x } }

public fun <T> churchAddOne(n: Church<T>): Church<T> = { f -> { x -> f(n(f)(x)) } }

public fun churchToLong(n: Church<Long>): Long = n({ x -> x + 1L })(0L)

/** `churchAddOne(churchZero())` substitutes to `{ f -> { x -> f(x) } }`: apply `f` once. */
public fun <T> churchOne(): Church<T> = { f -> { x -> f(x) } }

/** Apply `f` twice. */
public fun <T> churchTwo(): Church<T> = { f -> { x -> f(f(x)) } }

/** Applying `f` [a] times, then [b] more times, applies it `a + b` times in total. */
public fun <T> plusChurch(
    a: Church<T>,
    b: Church<T>,
): Church<T> = { f -> { x -> a(f)(b(f)(x)) } }

public fun ex_2_06(): Long = churchToLong(plusChurch(churchOne(), churchTwo()))
