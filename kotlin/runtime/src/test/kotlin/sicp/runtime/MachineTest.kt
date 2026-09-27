// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.collections.immutable.persistentMapOf

public class MachineTest :
    FunSpec({
        val ops: kotlinx.collections.immutable.PersistentMap<String, Op> =
            persistentMapOf(
                "=" to
                    { args ->
                        val (a, b) = args
                        VBool((a as VInt).n == (b as VInt).n)
                    },
                "rem" to
                    { args ->
                        val (a, b) = args
                        VInt((a as VInt).n % (b as VInt).n)
                    },
                "print" to { args -> args.firstOrNull() ?: VNil },
            )

        fun gcdMachine(): Machine =
            Machine(
                regs = setOf("a", "b", "t"),
                ops = ops,
                controller =
                    listOf(
                        Label("test-b"),
                        Test(OpCond("=", listOf(Source.RegSrc("b"), Source.ConstSrc(VInt(0))))),
                        Branch("gcd-done"),
                        Assign("t", Source.OpSrc("rem", listOf(Source.RegSrc("a"), Source.RegSrc("b")))),
                        Assign("a", Source.RegSrc("b")),
                        Assign("b", Source.RegSrc("t")),
                        Goto(GotoTarget.Lbl("test-b")),
                        Label("gcd-done"),
                    ),
            )

        test("the gcd machine computes gcd(40, 6) = 2") {
            val m = gcdMachine()
            either {
                m.reg("a").content = VInt(40)
                m.reg("b").content = VInt(6)
                m.run()
            }.isRight() shouldBe true
            m.registers.getValue("a").content shouldBe VInt(2)
        }

        test("save and restore move values through the monitored stack") {
            val m =
                Machine(
                    regs = setOf("x", "y"),
                    ops = Machine.noOps,
                    controller =
                        listOf(
                            Assign("x", Source.ConstSrc(VInt(1))),
                            Save("x"),
                            Assign("x", Source.ConstSrc(VInt(2))),
                            Save("x"),
                            Restore("y"),
                            Restore("x"),
                        ),
                )
            either { m.run() }.isRight() shouldBe true
            m.registers.getValue("x").content shouldBe VInt(1)
            m.registers.getValue("y").content shouldBe VInt(2)
            m.stack.pushes shouldBe 2
            m.stack.maxDepth shouldBe 2
            m.stack.depth shouldBe 0
        }

        test("restore on an empty stack raises StackUnderflow") {
            val m =
                Machine(
                    regs = setOf("x"),
                    ops = Machine.noOps,
                    controller = listOf(Restore("x")),
                )
            either { m.run() } shouldBe Either.Left(SchemeError.StackUnderflow)
        }

        test("a goto through a register jumps to the stored label") {
            val m =
                Machine(
                    regs = setOf("continue", "val"),
                    ops = Machine.noOps,
                    controller =
                        listOf(
                            Assign("continue", Source.LabelSrc("done")),
                            Assign("val", Source.ConstSrc(VInt(1))),
                            Goto(GotoTarget.ByReg("continue")),
                            Assign("val", Source.ConstSrc(VInt(99))),
                            Label("done"),
                        ),
                )
            either { m.run() }.isRight() shouldBe true
            m.registers.getValue("val").content shouldBe VInt(1)
        }

        test("an unknown label raises UnknownLabel") {
            val m =
                Machine(
                    regs = setOf("x"),
                    ops = Machine.noOps,
                    controller = listOf(Goto(GotoTarget.Lbl("nowhere"))),
                )
            either { m.run() } shouldBe Either.Left(SchemeError.UnknownLabel("nowhere"))
        }

        test("an unknown operation raises MachineFault") {
            val m =
                Machine(
                    regs = setOf("x"),
                    ops = Machine.noOps,
                    controller = listOf(Assign("x", Source.OpSrc("nope", emptyList()))),
                )
            either { m.run() } shouldBe Either.Left(SchemeError.MachineFault("unknown operation: nope"))
        }

        test("step executes one instruction at a time") {
            val m = gcdMachine()
            either {
                m.reg("a").content = VInt(40)
                m.reg("b").content = VInt(6)
                m.step()
                m.pc shouldBe 1
                m.step()
                m.testFlag shouldBe false
                while (!m.halted()) m.step()
            }.isRight() shouldBe true
            m.registers.getValue("a").content shouldBe VInt(2)
        }
    })
