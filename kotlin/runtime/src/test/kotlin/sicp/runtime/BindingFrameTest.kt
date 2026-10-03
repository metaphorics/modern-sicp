// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class BindingFrameTest :
    FunSpec({
        test("bind and read operate on the current frame") {
            val frame = BindingFrame.root()
            frame.bind("x", Whole(10))

            either { frame.read("x") } shouldBe Either.Right(Whole(10))
        }

        test("read walks parent frames and inner bindings shadow") {
            val root = BindingFrame.root()
            root.bind("outer", Whole(1))
            root.bind("shadowed", Whole(2))
            val child = BindingFrame.child(root)
            child.bind("inner", Whole(3))
            child.bind("shadowed", Whole(4))

            assertSoftly {
                either { child.read("outer") } shouldBe Either.Right(Whole(1))
                either { child.read("inner") } shouldBe Either.Right(Whole(3))
                either { child.read("shadowed") } shouldBe Either.Right(Whole(4))
                either { root.read("shadowed") } shouldBe Either.Right(Whole(2))
            }
        }

        test("assign updates the nearest existing binding through shared frames") {
            val root = BindingFrame.root()
            root.bind("balance", Whole(100))
            val child = BindingFrame.child(root)
            val capture = child

            either { capture.assign("balance", Whole(60)) } shouldBe Either.Right(Unit)
            either { root.read("balance") } shouldBe Either.Right(Whole(60))
        }

        test("assign rebinds the nearest shadow and never creates a binding") {
            val root = BindingFrame.root()
            root.bind("x", Whole(1))
            val child = BindingFrame.child(root)
            child.bind("x", Whole(2))

            either { BindingFrame.child(child).assign("x", Whole(3)) } shouldBe Either.Right(Unit)
            either { child.read("x") } shouldBe Either.Right(Whole(3))
            either { root.read("x") } shouldBe Either.Right(Whole(1))
            either { root.assign("missing", Whole(9)) } shouldBe Either.Left(DatumError.BadDatum("missing"))
            either { root.read("missing") } shouldBe Either.Left(DatumError.BadDatum("missing"))
        }

        test("snapshot isolates frame updates and retains datum object identities") {
            val root = BindingFrame.root()
            val cell = pair(Whole(1), Empty)
            root.bind("cell", cell)
            val child = BindingFrame.child(root)
            child.bind("local", Truth(true))
            val snapshot = child.snapshot()

            snapshot.bind("snapshot-only", Text("copy"))
            either { snapshot.assign("cell", Whole(9)) } shouldBe Either.Right(Unit)

            either { child.read("cell") } shouldBe Either.Right(cell)
            either { snapshot.read("cell") } shouldBe Either.Right(Whole(9))
            either { child.read("snapshot-only") } shouldBe Either.Left(DatumError.BadDatum("snapshot-only"))
            either { snapshot.read("local") } shouldBe Either.Right(Truth(true))
        }
    })
