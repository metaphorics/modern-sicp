// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_09aTest :
    FunSpec({
        test("Exercise 3.9a: on a 64 KiB stack the recursive run overflows and the tailrec run answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(StackOutcome.Overflow, runRecursive(100_000L, 64L * 1024L))
            org.junit.jupiter.api.Assertions
                .assertEquals(StackOutcome.Completed(120L), runTailrec(5L, 64L * 1024L))
        }
    })
