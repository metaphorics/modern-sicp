// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4
// Chapter 4, section 4.4.4.1, the driver and instantiation: an answer is
// the instantiated query in the pinned line form -- `?name = <rendered
// term>` -- a variable the match never bound contracts to its own name,
// and an assertion is filed into the data base instead of answered. The
// old prompt and echo lines are gone: the driver emits no prompts.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

public class S4_4_4DriverTest :
    FunSpec({
        val x = variable("x")

        test("the driver answers with instantiated variables") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), sym("technician")))), listOf(x)) shouldBe
                listOf("?x = [Tweakit, Lem, E]")
        }

        test("an assertion is filed into the data base instead of answered") {
            val database = QueryDatabase()
            database.assertFact(fact(terms(sym("meeting"), sym("whole-company"), terms(sym("Wednesday"), sym("4pm")))))
            val driver = QueryDriver.streaming(database)
            val who = variable("who")
            answerLines(driver, pattern(terms(sym("meeting"), who, terms(sym("Wednesday"), sym("4pm")))), listOf(who)) shouldBe
                listOf("?who = whole-company")
        }

        test("a variable the match never bound contracts to its own name") {
            val driver = QueryDriver.streaming(microshaftDatabase())
            val extra = variable("extra")
            val lines = answerLines(driver, pattern(terms(sym("job"), x, terms(sym("computer"), sym("wizard")))), listOf(x, extra))
            lines shouldBe
                listOf(
                    "?x = [Bitdiddle, Ben]",
                    "?extra = ?extra",
                )
        }
    })
