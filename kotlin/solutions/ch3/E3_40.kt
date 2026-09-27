// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.40

package sicp.ch3.exercises

public class Square40 {
    public var first: Long = 0L

    public var second: Long = 0L
}

public class Cube40 {
    public var first: Long = 0L

    public var second: Long = 0L

    public var third: Long = 0L
}

/** P1, first of its two reads of the shared variable. */
public suspend fun squareReadFirst(
    x: Cell,
    p: Square40,
) {
    p.first = x.value
}

/** P1, second read. */
public suspend fun squareReadSecond(
    x: Cell,
    p: Square40,
) {
    p.second = x.value
}

/** P1, the write: x becomes first times second. */
public suspend fun squareWrite(
    x: Cell,
    p: Square40,
) {
    x.value = p.first * p.second
}

/** P2, first of its three reads. */
public suspend fun cubeReadFirst(
    x: Cell,
    p: Cube40,
) {
    p.first = x.value
}

/** P2, second read. */
public suspend fun cubeReadSecond(
    x: Cell,
    p: Cube40,
) {
    p.second = x.value
}

/** P2, third read. */
public suspend fun cubeReadThird(
    x: Cell,
    p: Cube40,
) {
    p.third = x.value
}

/** P2, the write: x becomes first times second times third. */
public suspend fun cubeWrite(
    x: Cell,
    p: Cube40,
) {
    x.value = p.first * p.second * p.third
}

/** P1 wholly inside one serialized block. */
public suspend fun serializedSquare(
    x: Cell,
    s: Serializer,
) {
    s.serialized { x.value = x.value * x.value }()
}

/** P2 wholly inside one serialized block. */
public suspend fun serializedCube(
    x: Cell,
    s: Serializer,
) {
    s.serialized { x.value = x.value * x.value * x.value }()
}
