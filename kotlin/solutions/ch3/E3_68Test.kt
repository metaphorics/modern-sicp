// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.68

package sicp.ch3.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import sicp.runtime.take

public class E3_68Test :
    FunSpec({
        test("Exercise 3.68: Louis's pairs overflows the stack before the first element") {
            shouldThrow<StackOverflowError> {
                louisPairs(integers, integers).take(1)
            }
        }
    })
