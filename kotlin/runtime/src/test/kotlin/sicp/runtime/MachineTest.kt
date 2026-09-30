// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.either
import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.guest.GValue
import sicp.runtime.Source.ConstSrc
import sicp.runtime.Source.LabelSrc
import sicp.runtime.Source.OpSrc
import sicp.runtime.Source.RegSrc

public class MachineTest :
    FunSpec({
        test("assemble rejects duplicate labels") {
            val program = listOf(Label("top"), Assign("a", ConstSrc(GValue.VInt(1))), Label("top"))
            assemble(program).isLeft() shouldBe true
        }

        test("assemble rejects unknown branch targets") {
            val program = listOf(Test(OpCond("truth", emptyList())), Branch("nowhere"))
            assemble(program).isLeft() shouldBe true
        }

        test("assemble rejects label operands inside operation arguments") {
            val program = listOf(Assign("a", OpSrc("combine", listOf(LabelSrc("later")))), Label("later"))
            assemble(program).isLeft() shouldBe true
        }

        test("reading a register before any write raises UnassignedRegister") {
            val machine = Machine(setOf("a"), emptyMap(), listOf(Assign("b", RegSrc("a"))))
            either { machine.run() }.isLeft() shouldBe true
        }

        test("test and branch dispatch on the flag and run halts past the end") {
            val ops = mapOf<String, MachineOp>("even" to { args -> GValue.VBool((args[0] as GValue.VInt).value % 2 == 0) })
            val program =
                listOf(
                    Assign("a", ConstSrc(GValue.VInt(4))),
                    Test(OpCond("even", listOf(RegSrc("a")))),
                    Branch("hit"),
                    Assign("b", ConstSrc(GValue.VInt(0))),
                    Goto(GotoTarget.Lbl("done")),
                    Label("hit"),
                    Assign("b", ConstSrc(GValue.VInt(1))),
                    Label("done"),
                )
            val machine = Machine(setOf("a", "b"), ops, program)
            either { machine.run() }.isRight() shouldBe true
            machine.halted() shouldBe true
            machine.registers["b"]?.content shouldBe GValue.VInt(1)
        }

        test("save and restore move values through the monitored stack") {
            val program =
                listOf(
                    Assign("a", ConstSrc(GValue.VInt(7))),
                    Save("a"),
                    Assign("a", ConstSrc(GValue.VInt(0))),
                    Restore("a"),
                )
            val machine = Machine(setOf("a"), emptyMap(), program)
            either { machine.run() }.isRight() shouldBe true
            assertSoftly {
                machine.registers["a"]?.content shouldBe GValue.VInt(7)
                machine.stack.pushes shouldBe 1L
                machine.stack.maxDepth shouldBe 1L
                machine.stack.depth shouldBe 0
            }
        }

        test("continue holds a label index and goto-by-reg jumps to it") {
            val program =
                listOf(
                    Assign("continue", LabelSrc("done")),
                    Assign("a", ConstSrc(GValue.VInt(1))),
                    Goto(GotoTarget.ByReg("continue")),
                    Assign("a", ConstSrc(GValue.VInt(2))),
                    Label("done"),
                )
            val machine = Machine(setOf("a", "continue"), emptyMap(), program)
            either { machine.run() }.isRight() shouldBe true
            machine.registers["a"]?.content shouldBe GValue.VInt(1)
        }

        test("step executes one instruction at a time") {
            val program = listOf(Assign("a", ConstSrc(GValue.VInt(1))), Assign("a", ConstSrc(GValue.VInt(2))))
            val machine = Machine(setOf("a"), emptyMap(), program)
            either { machine.step() }.isRight() shouldBe true
            assertSoftly {
                machine.instructions shouldBe 1L
                machine.halted() shouldBe false
            }
            either { machine.run() }.isRight() shouldBe true
            machine.instructions shouldBe 2L
        }
    })
