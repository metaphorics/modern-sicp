// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.43

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_43Test :
    FunSpec({
        test("scan-out replaces internal definitions with unassigned bindings") {
            val result = scanOutShapes()
            result[0] shouldBe "plain: marker=false, define=true"
            result[1] shouldBe "scanned: marker=true, define=false"
            result[2] shouldBe "scanned run: ok"
        }
    })
