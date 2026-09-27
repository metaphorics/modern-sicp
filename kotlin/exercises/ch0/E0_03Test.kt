// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E0_03Test :
    FunSpec({
        test("Exercise 0.3: depth of a sealed tree").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(1, depth(Leaf(1L)))
            org.junit.jupiter.api.Assertions.assertEquals(
                3,
                depth(Node(Node(Leaf(1L), Leaf(2L)), Leaf(3L))),
            )
        }
    })
