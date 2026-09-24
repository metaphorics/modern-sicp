// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.49

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.string.shouldContain
import kotlinx.coroutines.test.runTest

public class E3_49Test :
    FunSpec({
        test("Exercise 3.49: the directory scenario deadlocks despite any numbering") {
            runTest {
                directoryDeadlockDemo() shouldContain "deadlock"
            }
        }
    })
