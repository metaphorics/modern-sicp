// SPDX-License-Identifier: GPL-3.0-only
// Section 5.5: the compiler. The book's `compile` and its code
// generators, emitting instruction sequences in the register-machine
// language of 5.1.5 as `Stmt` data -- the same instruction lists the 5.2
// simulator assembles and the 5.4 machine runs -- so a compilation
// installs on the evaluator machine unchanged.
//
// An instruction sequence is the book's three parts: the registers the
// code needs, the registers it modifies, and the statements.
// `preserving` reads the recorded register use, never the code.
//
// The spellings are the book's own: the procedure entry rides in a
// `(label entryN)` operation input (the base assembler admits labels
// wherever a primitive expression may stand) and the lambda's formals
// ride in one `(const (p1 p2 ...))` list constant, so the compiled
// statements replay the book's Figure 5.17 line for line. The knobs the
// section's exercises turn live in [CompilerConfig]: lexical addressing
// (5.40 to 5.42), scanning out internal definitions (5.43), open-coded
// primitives (5.38 and 5.44), the operand evaluation order (5.36), the
// preserving mechanism itself (5.37), and 5.47's compiled calls to
// interpreted procedures.

package sicp.ch5

import arrow.core.raise.Raise
import sicp.runtime.Assign
import sicp.runtime.Branch
import sicp.runtime.Goto
import sicp.runtime.GotoTarget
import sicp.runtime.Label
import sicp.runtime.Source
import sicp.runtime.Stmt
import sicp.runtime.Test
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VReal
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

// ---------------------------------------------------------------------------
// Instruction sequences
// ---------------------------------------------------------------------------

/** One instruction sequence: the book's three parts. */
public class InstructionSequence(
    public val needs: List<String>,
    public val modifies: List<String>,
    public val stmts: List<Stmt>,
)

/** The constructor of instruction sequences. */
public fun makeInstructionSequence(
    needs: List<String>,
    modifies: List<String>,
    stmts: List<Stmt>,
): InstructionSequence = InstructionSequence(needs, modifies, stmts)

/** The sequence with no statements. */
public fun emptyInstructionSequence(): InstructionSequence = InstructionSequence(emptyList(), emptyList(), emptyList())

private fun listUnion(
    s1: List<String>,
    s2: List<String>,
): List<String> = s1 + s2.filterNot { it in s1 }

private fun listDifference(
    s1: List<String>,
    s2: List<String>,
): List<String> = s1.filterNot { it in s2 }

/** Sequential append: the second sequence needs only what the first
 *  leaves standing. */
public fun append2Sequences(
    seq1: InstructionSequence,
    seq2: InstructionSequence,
): InstructionSequence =
    InstructionSequence(
        listUnion(seq1.needs, listDifference(seq2.needs, seq1.modifies)),
        listUnion(seq1.modifies, seq2.modifies),
        seq1.stmts + seq2.stmts,
    )

/** The book's `append-instruction-sequences` over any number. */
public fun appendSequences(sequences: List<InstructionSequence>): InstructionSequence =
    sequences.fold(emptyInstructionSequence(), ::append2Sequences)

/** `tack-on-instruction-sequence`: the body rides in the sequence but is
 *  not part of its execution, so the register use is [seq]'s alone. */
public fun tackOnInstructionSequence(
    seq: InstructionSequence,
    bodySeq: InstructionSequence,
): InstructionSequence = InstructionSequence(seq.needs, seq.modifies, seq.stmts + bodySeq.stmts)

/** The two branches after a test are never executed sequentially, so the
 *  combined sequence modifies what either branch modifies. */
public fun parallelInstructionSequences(
    seq1: InstructionSequence,
    seq2: InstructionSequence,
): InstructionSequence =
    InstructionSequence(
        listUnion(seq1.needs, seq2.needs),
        listUnion(seq1.modifies, seq2.modifies),
        seq1.stmts + seq2.stmts,
    )

/** `preserving`: wraps a `save`/`restore` of every register the first
 *  sequence modifies and the second needs around the first; the last
 *  register of the set is saved first, the book's own order. With
 *  [CompilerConfig.preservingOn] off (the 5.37 comparison) every register
 *  in the set is saved unconditionally. */
public fun preserving(
    cfg: CompilerConfig,
    regs: List<String>,
    seq1: InstructionSequence,
    seq2: InstructionSequence,
): InstructionSequence {
    var wrapped = seq1
    for (firstReg in regs) {
        val needed = firstReg in seq2.needs && firstReg in wrapped.modifies
        if (cfg.preservingOn && !needed) continue
        wrapped =
            InstructionSequence(
                listOf(firstReg) + wrapped.needs,
                listDifference(wrapped.modifies, listOf(firstReg)),
                listOf(sicp.runtime.Save(firstReg)) + wrapped.stmts + sicp.runtime.Restore(firstReg),
            )
    }
    return append2Sequences(wrapped, seq2)
}

// ---------------------------------------------------------------------------
// The compiler configuration and state
// ---------------------------------------------------------------------------

/** The knobs the section's exercises turn. Every default is the base
 *  compiler of the section's prose. */
public data class CompilerConfig(
    /** 5.40 to 5.42: compile variable references to lexical addresses. */
    val lexical: Boolean = false,
    /** 5.43: scan internal definitions out of every procedure body. */
    val scanOut: Boolean = false,
    /** 5.38 and 5.44: open-code the primitive arithmetic. */
    val openCode: Boolean = false,
    /** 5.36: evaluate operands left to right; the default is the book's
     *  right-to-left `reverse`. */
    val leftToRight: Boolean = false,
    /** 5.37: the preserving mechanism itself. */
    val preservingOn: Boolean = true,
    /** 5.47: compiled code may call interpreted procedures. */
    val compoundCalls: Boolean = false,
    /** 5.40: every variable reference reports the compile-time
     *  environment it was compiled against. */
    val trace: ((cenv: List<List<String>>, name: String) -> Unit)? = null,
)

/** The compiler's state: the book's label counter, seeded at [seed] so
 *  5.35 can reproduce the book's session (Figure 5.18 seeds 14, the
 *  labels the book's session had already generated), and the entry
 *  counter of [compileBlock]. */
public class CompilerState(
    seed: Int = 0,
) {
    private var counter: Int = seed
    private var entries: Int = 0

    /** Rebinding warnings emitted for open-coded names. */
    public val warnings: MutableList<String> = mutableListOf()

    /** The book's `make-label`: the name with the next counter value. */
    public fun makeLabel(name: String): String {
        counter += 1
        return "$name$counter"
    }

    /** The next compiled-block entry number. */
    public fun bumpEntry(): Int {
        entries += 1
        return entries
    }
}

// ---------------------------------------------------------------------------
// Linkage
// ---------------------------------------------------------------------------

/** The linkage descriptor: where the code goes when it is done. */
public sealed interface Linkage {
    /** Continue at the next instruction in sequence. */
    public data object Next : Linkage

    /** Return from the procedure being compiled. */
    public data object Return : Linkage

    /** Jump to a named entry point. */
    public data class Lab(
        val name: String,
    ) : Linkage
}

private fun compileLinkage(linkage: Linkage): InstructionSequence =
    when (linkage) {
        is Linkage.Return -> {
            makeInstructionSequence(listOf("continue"), emptyList(), listOf(Goto(GotoTarget.ByReg("continue"))))
        }

        is Linkage.Next -> {
            emptyInstructionSequence()
        }

        is Linkage.Lab -> {
            makeInstructionSequence(emptyList(), emptyList(), listOf(Goto(GotoTarget.Lbl(linkage.name))))
        }
    }

private fun endWithLinkage(
    cfg: CompilerConfig,
    linkage: Linkage,
    seq: InstructionSequence,
): InstructionSequence = preserving(cfg, listOf("continue"), seq, compileLinkage(linkage))

// ---------------------------------------------------------------------------
// The compile-time environment
// ---------------------------------------------------------------------------

/** The compile-time environment: the frames of parameter names, newest
 *  first. */
public typealias CompileTimeEnv = List<List<String>>

/** Extends the compile-time environment with a parameter frame. */
public fun extendCompileTimeEnv(
    params: List<String>,
    frames: CompileTimeEnv,
): CompileTimeEnv = listOf(params) + frames

/** Exercise 5.41's `find-variable`: the lexical address of [name] as the
 *  frame number and displacement, or null when the compile-time
 *  environment cannot address it (a global). */
public fun findVariable(
    name: String,
    frames: CompileTimeEnv,
): Pair<Int, Int>? {
    frames.forEachIndexed { frameNumber, names ->
        val displacement = names.indexOf(name)
        if (displacement >= 0) return frameNumber to displacement
    }
    return null
}

// ---------------------------------------------------------------------------
// Syntax over the reader's list structure
// ---------------------------------------------------------------------------

private fun isHead(
    w: Value,
    name: String,
): Boolean = w is VPair && w.car is VSym && (w.car as VSym).name == name

private fun VPair.toItems(): List<Value> {
    val items = ArrayList<Value>()
    var cursor: Value = this
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    return items
}

context(r: Raise<MachineError>)
private fun itemsOf(
    op: String,
    w: Value,
): List<Value> {
    val items = if (w is VPair) w.toItems() else emptyList()
    if (items.isEmpty()) r.raise(EvaluatorFault("$op needs a well-formed form"))
    return items
}

context(r: Raise<MachineError>)
private fun secondOf(
    op: String,
    w: Value,
): Value = itemsOf(op, w).getOrNull(1) ?: r.raise(EvaluatorFault("$op needs a well-formed form"))

context(r: Raise<MachineError>)
private fun thirdOf(
    op: String,
    w: Value,
): Value = itemsOf(op, w).getOrNull(2) ?: r.raise(EvaluatorFault("$op needs a well-formed form"))

/** The variable name an assignment, definition, or lookup names, as the
 *  operations' `(const name)` inputs spell it. */
context(r: Raise<MachineError>)
private fun nameOf(
    op: String,
    w: Value,
): String =
    when (val target = secondOf(op, w)) {
        is VSym -> target.name
        is VPair -> (target.car as? VSym)?.name ?: r.raise(EvaluatorFault("$op needs a variable"))
        else -> r.raise(EvaluatorFault("$op needs a variable"))
    }

context(r: Raise<MachineError>)
private fun lambdaExpr(
    params: List<Value>,
    body: List<Value>,
): Value = cons(VSym("lambda"), cons(vlist(params), vlist(body)))

/** The definition's variable: the name itself, or the head of the
 *  procedure sugar's signature. */
context(r: Raise<MachineError>)
private fun definitionVariable(w: Value): Value {
    val target = secondOf("definition-variable", w)
    return when (target) {
        is VPair -> target.car
        else -> target
    }
}

/** The lambda expression a `begin` of [body] spells. */
private fun beginExpr(body: List<Value>): Value = cons(VSym("begin"), vlist(body))

/** The definition's value: the plain expression, or the lambda the
 *  procedure sugar's signature and body spell. */
context(r: Raise<MachineError>)
private fun definitionValue(w: Value): Value {
    val items = itemsOf("definition-value", w)
    val target = items.getOrNull(1) ?: r.raise(EvaluatorFault("definition-value needs a well-formed define"))
    if (target is VPair) return lambdaExpr(target.toItems().drop(1), items.drop(2))
    return items.getOrNull(2) ?: r.raise(EvaluatorFault("definition-value needs a well-formed define"))
}

// ---------------------------------------------------------------------------
// Derived expressions
// ---------------------------------------------------------------------------

/** The 4.1.2 `cond->if` transformation over the list structure; a clause
 *  with no actions answers its test, and the missing else answers the
 *  global `false`, the evaluator's own convention. */
context(r: Raise<MachineError>)
public fun condToIf(exp: Value): Value {
    val items = itemsOf("cond->if", exp)
    val elseBody = items.filter { isHead(it, "else") }.firstOrNull()?.let { ((it as VPair).cdr as VPair).toItems() }
    val base: Value =
        when (elseBody) {
            null -> VSym("false")
            else -> beginExpr(elseBody)
        }
    return buildCondIf(items.drop(1).filterNot { isHead(it, "else") }, base)
}

context(r: Raise<MachineError>)
private fun buildCondIf(
    clauses: List<Value>,
    base: Value,
): Value {
    if (clauses.isEmpty()) return base
    val clause = itemsOf("cond->if", clauses.first())
    val test = clause.getOrNull(0) ?: r.raise(EvaluatorFault("cond->if needs a well-formed clause"))
    val consequent = if (clause.size > 1) beginExpr(clause.drop(1)) else test
    val alternative = buildCondIf(clauses.drop(1), base)
    return vlist(listOf(VSym("if"), test, consequent, alternative))
}

/** `(let ((n e) ...) body...)` as the call of a lambda on the inits. */
context(r: Raise<MachineError>)
public fun letToCombination(exp: Value): Value {
    val items = itemsOf("let->combination", exp)
    val bindingForms = items.getOrNull(1) ?: r.raise(EvaluatorFault("let->combination needs a well-formed let"))
    val bindings =
        itemsOf("let->combination", bindingForms).map { binding ->
            val parts = itemsOf("let->combination", binding)
            val name = parts.getOrNull(0) ?: r.raise(EvaluatorFault("let->combination needs a name binding"))
            val init = parts.getOrNull(1) ?: r.raise(EvaluatorFault("let->combination needs an init"))
            name to init
        }
    val lambda = lambdaExpr(bindings.map { it.first }, items.drop(2))
    return cons(lambda, vlist(bindings.map { it.second }))
}

// ---------------------------------------------------------------------------
// The code generators
// ---------------------------------------------------------------------------

/** The open-coded primitives of 5.38 and 5.44. */
public val openCodedPrimitives: List<String> = listOf("+", "-", "*", "<", "=")

private fun selfEvaluating(w: Value): Boolean = w is VInt || w is VReal || w is VBool || w is VStr

/** Whether the operator names an open-coded primitive the compile-time
 *  environment does not shadow (5.38 dispatch, 5.44's caveat). */
private fun isOpenCoded(
    cfg: CompilerConfig,
    cenv: CompileTimeEnv,
    operator: Value,
): Boolean =
    operator is VSym &&
        cfg.openCode &&
        operator.name in openCodedPrimitives &&
        findVariable(operator.name, cenv) == null

/** The book's `compile`: the top-level dispatch on the syntactic type of
 *  [exp], to the specialized code generator. [target] names the register
 *  the code answers in; [linkage] is where it goes next; [cenv] is the
 *  compile-time environment (empty at top level). */
context(r: Raise<MachineError>)
public fun compile(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence =
    when {
        selfEvaluating(exp) -> compileSelfEvaluating(cfg, exp, target, linkage)
        isHead(exp, "quote") -> compileQuoted(cfg, exp, target, linkage)
        exp is VSym -> compileVariable(cfg, cenv, exp.name, target, linkage)
        isHead(exp, "set!") -> compileAssignment(cfg, state, cenv, exp, target, linkage)
        isHead(exp, "define") -> compileDefinition(cfg, state, cenv, exp, target, linkage)
        isHead(exp, "if") -> compileIf(cfg, state, cenv, exp, target, linkage)
        isHead(exp, "lambda") -> compileLambda(cfg, state, cenv, exp, target, linkage)
        isHead(exp, "begin") -> compileSequence(cfg, state, cenv, ((exp as VPair).cdr as VPair).toItems(), target, linkage)
        isHead(exp, "cond") -> compile(cfg, state, cenv, condToIf(exp), target, linkage)
        isHead(exp, "let") -> compile(cfg, state, cenv, letToCombination(exp), target, linkage)
        isHead(exp, "and") || isHead(exp, "or") -> r.raise(EvaluatorFault("Unknown expression type: COMPILE"))
        exp is VPair && isOpenCoded(cfg, cenv, exp.car) -> compileOpenCode(cfg, state, cenv, exp, target, linkage)
        exp is VPair -> compileApplication(cfg, state, cenv, exp.car, (exp.cdr as? VPair)?.toItems().orEmpty(), target, linkage)
        else -> r.raise(EvaluatorFault("Unknown expression type: COMPILE"))
    }

context(r: Raise<MachineError>)
private fun compileSelfEvaluating(
    cfg: CompilerConfig,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence =
    endWithLinkage(
        cfg,
        linkage,
        makeInstructionSequence(
            emptyList(),
            listOf(target),
            listOf(Assign(target, Source.ConstSrc(exp))),
        ),
    )

context(r: Raise<MachineError>)
private fun compileQuoted(
    cfg: CompilerConfig,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence =
    endWithLinkage(
        cfg,
        linkage,
        makeInstructionSequence(
            emptyList(),
            listOf(target),
            listOf(Assign(target, Source.ConstSrc(secondOf("text-of-quotation", exp)))),
        ),
    )

context(r: Raise<MachineError>)
private fun compileVariable(
    cfg: CompilerConfig,
    cenv: CompileTimeEnv,
    name: String,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    cfg.trace?.invoke(cenv, name)
    val lookup =
        when {
            !cfg.lexical -> {
                listOf(Assign(target, opLookup(name)))
            }

            else -> {
                when (val address = findVariable(name, cenv)) {
                    null -> {
                        listOf(Assign(target, opLookup(name)))
                    }

                    else -> {
                        listOf(
                            Assign(
                                target,
                                Source.OpSrc(
                                    "lexical-address-lookup",
                                    listOf(
                                        Source.ConstSrc(vlist(listOf(VInt(address.first.toLong()), VInt(address.second.toLong())))),
                                        Source.RegSrc("env"),
                                    ),
                                ),
                            ),
                        )
                    }
                }
            }
        }
    return endWithLinkage(cfg, linkage, makeInstructionSequence(listOf("env"), listOf(target), lookup))
}

private fun opLookup(name: String): Source.OpSrc =
    Source.OpSrc("lookup-variable-value", listOf(Source.ConstSrc(VSym(name)), Source.RegSrc("env")))

context(r: Raise<MachineError>)
private fun compileAssignment(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val name = nameOf("assignment-variable", exp)
    warnOpenCodeRebinding(cfg, state, name)
    val valueCode = compile(cfg, state, cenv, thirdOf("assignment-value", exp), "val", Linkage.Next)
    val tail =
        makeInstructionSequence(
            listOf("env", "val"),
            listOf(target),
            listOf(performSet(cfg, name, cenv), Assign(target, Source.ConstSrc(VSym("ok")))),
        )
    return endWithLinkage(cfg, linkage, preserving(cfg, listOf("env"), valueCode, tail))
}

private fun warnOpenCodeRebinding(
    cfg: CompilerConfig,
    state: CompilerState,
    name: String,
) {
    if (cfg.openCode && name in openCodedPrimitives) {
        state.warnings.add("open-coded primitive $name is rebound")
    }
}

/** The set!'s perform, direct or lexical (5.42). */
context(r: Raise<MachineError>)
private fun performSet(
    cfg: CompilerConfig,
    name: String,
    cenv: CompileTimeEnv,
): sicp.runtime.Perform {
    val direct =
        sicp.runtime.OpAct(
            "set-variable-value!",
            listOf(Source.ConstSrc(VSym(name)), Source.RegSrc("val"), Source.RegSrc("env")),
        )
    if (!cfg.lexical) return sicp.runtime.Perform(direct)
    val address = findVariable(name, cenv) ?: return sicp.runtime.Perform(direct)
    return sicp.runtime.Perform(
        sicp.runtime.OpAct(
            "lexical-address-set!",
            listOf(lexicalAddressConst(address), Source.RegSrc("val"), Source.RegSrc("env")),
        ),
    )
}

/** The lexical address as the one list constant the book spells. */
private fun lexicalAddressConst(address: Pair<Int, Int>): Source.ConstSrc =
    Source.ConstSrc(vlist(listOf(VInt(address.first.toLong()), VInt(address.second.toLong()))))

context(r: Raise<MachineError>)
private fun compileDefinition(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val name = nameOf("definition-variable", exp)
    warnOpenCodeRebinding(cfg, state, name)
    val valueCode = compile(cfg, state, cenv, definitionValue(exp), "val", Linkage.Next)
    val tail =
        makeInstructionSequence(
            listOf("env"),
            listOf(target),
            listOf(
                sicp.runtime.Perform(
                    sicp.runtime.OpAct(
                        "define-variable!",
                        listOf(Source.ConstSrc(VSym(name)), Source.RegSrc("val"), Source.RegSrc("env")),
                    ),
                ),
                Assign(target, Source.ConstSrc(VSym("ok"))),
            ),
        )
    return endWithLinkage(cfg, linkage, preserving(cfg, listOf("env"), valueCode, tail))
}

context(r: Raise<MachineError>)
private fun compileIf(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    // The label allocations and the compilation order (alternative,
    // consequent, predicate) are the orders the book's own figures show.
    val afterIf = state.makeLabel("after-if")
    val fBranch = state.makeLabel("false-branch")
    val tBranch = state.makeLabel("true-branch")
    val consequentLinkage = if (linkage == Linkage.Next) Linkage.Lab(afterIf) else linkage
    val alternativeExp = ifAlternative(exp)
    val aCode = compile(cfg, state, cenv, alternativeExp, target, linkage)
    val cCode = compile(cfg, state, cenv, ifConsequent(exp), target, consequentLinkage)
    val pCode = compile(cfg, state, cenv, ifPredicate(exp), "val", Linkage.Next)
    val testCode =
        makeInstructionSequence(
            listOf("val"),
            emptyList(),
            listOf(Test(sicp.runtime.OpCond("false?", listOf(Source.RegSrc("val")))), Branch(fBranch)),
        )
    val branches =
        parallelInstructionSequences(
            append2Sequences(makeInstructionSequence(emptyList(), emptyList(), listOf(Label(tBranch))), cCode),
            append2Sequences(makeInstructionSequence(emptyList(), emptyList(), listOf(Label(fBranch))), aCode),
        )
    val branchTail = InstructionSequence(branches.needs, branches.modifies, branches.stmts + Label(afterIf))
    return preserving(cfg, listOf("env", "continue"), pCode, append2Sequences(testCode, branchTail))
}

context(r: Raise<MachineError>)
private fun ifPredicate(exp: Value): Value = secondOf("if-predicate", exp)

context(r: Raise<MachineError>)
private fun ifConsequent(exp: Value): Value = thirdOf("if-consequent", exp)

context(r: Raise<MachineError>)
private fun ifAlternative(exp: Value): Value {
    val items = itemsOf("if-alternative", exp)
    return if (items.size >= 4) items[3] else VSym("false")
}

/** 5.43's scan out: the internal defines of a procedure body become a
 *  `let` of `*unassigned*` bindings whose values are `set!` after it,
 *  the book's transformation, so the body executes no `define`. The
 *  binding value is the quoted `*unassigned*` symbol itself, never a
 *  lookup of it. */
context(r: Raise<MachineError>)
private fun scanOutDefines(body: List<Value>): List<Value> {
    val defines = body.filter { isHead(it, "define") }
    if (defines.isEmpty()) return body
    val rest = body.filterNot { isHead(it, "define") }
    val unassigned = vlist(listOf(VSym("quote"), VSym("*unassigned*")))
    val bindings =
        defines.map { define ->
            vlist(listOf(definitionVariable(define), unassigned))
        }
    val sets =
        defines.map { define ->
            vlist(listOf(VSym("set!"), definitionVariable(define), definitionValue(define)))
        }
    return listOf(cons(vlist(listOf(VSym("let"), vlist(bindings)) + sets + rest), VNil))
}

context(r: Raise<MachineError>)
private fun compileLambdaBody(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    procEntry: String,
): InstructionSequence {
    val paramsForm = secondOf("lambda-parameters", exp)
    val body = (((exp as VPair).cdr as VPair).cdr as VPair).toItems()
    val params =
        when (paramsForm) {
            is VNil -> emptyList()
            is VPair -> paramsForm.toItems().map { it.toString() }
            else -> r.raise(EvaluatorFault("lambda-parameters needs a parameter list"))
        }
    val scannedBody = if (cfg.scanOut) scanOutDefines(body) else body
    val formals =
        when (paramsForm) {
            is VNil -> VNil
            else -> paramsForm
        }
    val entryCode =
        makeInstructionSequence(
            listOf("env", "proc", "argl"),
            listOf("env"),
            listOf(
                Label(procEntry),
                Assign("env", Source.OpSrc("compiled-procedure-env", listOf(Source.RegSrc("proc")))),
                Assign(
                    "env",
                    Source.OpSrc(
                        "extend-environment",
                        listOf(Source.ConstSrc(formals), Source.RegSrc("argl"), Source.RegSrc("env")),
                    ),
                ),
            ),
        )
    val bodyCode = compileSequence(cfg, state, extendCompileTimeEnv(params, cenv), scannedBody, "val", Linkage.Return)
    return append2Sequences(entryCode, bodyCode)
}

context(r: Raise<MachineError>)
private fun compileLambda(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val afterLambda = state.makeLabel("after-lambda")
    val procEntry = state.makeLabel("entry")
    val lambdaLinkage = if (linkage == Linkage.Next) Linkage.Lab(afterLambda) else linkage
    val construct =
        makeInstructionSequence(
            listOf("env"),
            listOf(target),
            listOf(
                Assign(
                    target,
                    Source.OpSrc(
                        "make-compiled-procedure",
                        listOf(Source.LabelSrc(procEntry), Source.RegSrc("env")),
                    ),
                ),
            ),
        )
    val body = compileLambdaBody(cfg, state, cenv, exp, procEntry)
    val withLinkage = endWithLinkage(cfg, lambdaLinkage, construct)
    val tacked = tackOnInstructionSequence(withLinkage, body)
    return append2Sequences(tacked, makeInstructionSequence(emptyList(), emptyList(), listOf(Label(afterLambda))))
}

context(r: Raise<MachineError>)
private fun compileSequence(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    body: List<Value>,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    if (body.isEmpty()) r.raise(EvaluatorFault("compile-sequence of an empty body"))
    if (body.size == 1) return compile(cfg, state, cenv, body[0], target, linkage)
    val firstCode = compile(cfg, state, cenv, body[0], target, Linkage.Next)
    val restCode = compileSequence(cfg, state, cenv, body.drop(1), target, linkage)
    return preserving(cfg, listOf("env", "continue"), firstCode, restCode)
}

private fun constructArglist(
    cfg: CompilerConfig,
    operandCodes: List<InstructionSequence>,
): InstructionSequence {
    val ordered = if (cfg.leftToRight) operandCodes else operandCodes.reversed()
    if (ordered.isEmpty()) {
        return makeInstructionSequence(emptyList(), listOf("argl"), listOf(Assign("argl", Source.ConstSrc(VNil))))
    }
    val codeToGetLastArg =
        append2Sequences(
            ordered[0],
            makeInstructionSequence(listOf("val"), listOf("argl"), listOf(Assign("argl", opListOfVal()))),
        )
    val restCodes = ordered.drop(1)
    if (restCodes.isEmpty()) return codeToGetLastArg
    return preserving(cfg, listOf("env"), codeToGetLastArg, codeToGetRestArgs(cfg, restCodes))
}

private fun opListOfVal(): Source.OpSrc = Source.OpSrc("list", listOf(Source.RegSrc("val")))

private fun codeToGetRestArgs(
    cfg: CompilerConfig,
    operandCodes: List<InstructionSequence>,
): InstructionSequence {
    val consStep =
        makeInstructionSequence(
            listOf("val", "argl"),
            listOf("argl"),
            listOf(
                Assign("argl", Source.OpSrc("cons", listOf(Source.RegSrc("val"), Source.RegSrc("argl")))),
            ),
        )
    val codeForNextArg = preserving(cfg, listOf("argl"), operandCodes[0], consStep)
    val rest = operandCodes.drop(1)
    if (rest.isEmpty()) return codeForNextArg
    return preserving(cfg, listOf("env"), codeForNextArg, codeToGetRestArgs(cfg, rest))
}

context(r: Raise<MachineError>)
private fun compileApplication(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    operator: Value,
    operands: List<Value>,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val procCode = compile(cfg, state, cenv, operator, "proc", Linkage.Next)
    val operandCodes = operands.map { compile(cfg, state, cenv, it, "val", Linkage.Next) }
    val arglistCode = constructArglist(cfg, operandCodes)
    val callCode = compileProcedureCall(cfg, state, target, linkage)
    val callAndArgs = preserving(cfg, listOf("proc", "continue"), arglistCode, callCode)
    return preserving(cfg, listOf("env", "continue"), procCode, callAndArgs)
}

context(r: Raise<MachineError>)
private fun compileProcedureCall(
    cfg: CompilerConfig,
    state: CompilerState,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val afterCall = state.makeLabel("after-call")
    val compiledBranch = state.makeLabel("compiled-branch")
    val primitiveBranch = state.makeLabel("primitive-branch")
    val compoundBranch = if (cfg.compoundCalls) state.makeLabel("compound-branch") else null
    val compiledLinkage = if (linkage == Linkage.Next) Linkage.Lab(afterCall) else linkage
    val applCode = compileProcAppl(state, target, compiledLinkage)
    val primitiveTail = if (compoundBranch != null && linkage == Linkage.Next) listOf(Goto(GotoTarget.Lbl(afterCall))) else emptyList()
    val primitiveCode =
        endWithLinkage(
            cfg,
            linkage,
            makeInstructionSequence(
                listOf("proc", "argl"),
                listOf(target),
                listOf(
                    Assign(
                        target,
                        Source.OpSrc(
                            "apply-primitive-procedure",
                            listOf(Source.RegSrc("proc"), Source.RegSrc("argl")),
                        ),
                    ),
                ) + primitiveTail,
            ),
        )
    val blocks = ArrayList<InstructionSequence>()
    blocks.add(
        makeInstructionSequence(
            listOf("proc"),
            emptyList(),
            listOf(
                Test(sicp.runtime.OpCond("primitive-procedure?", listOf(Source.RegSrc("proc")))),
                Branch(primitiveBranch),
            ),
        ),
    )
    if (compoundBranch != null) {
        blocks.add(
            makeInstructionSequence(
                listOf("proc"),
                emptyList(),
                listOf(
                    Test(sicp.runtime.OpCond("compound-procedure?", listOf(Source.RegSrc("proc")))),
                    Branch(compoundBranch),
                ),
            ),
        )
    }
    blocks.add(
        parallelInstructionSequences(
            append2Sequences(makeInstructionSequence(emptyList(), emptyList(), listOf(Label(compiledBranch))), applCode),
            append2Sequences(makeInstructionSequence(emptyList(), emptyList(), listOf(Label(primitiveBranch))), primitiveCode),
        ),
    )
    if (compoundBranch != null) {
        val contSetup =
            when (linkage) {
                is Linkage.Return -> emptyList()
                is Linkage.Next -> listOf<sicp.runtime.Stmt>(Assign("continue", Source.LabelSrc(afterCall)))
                is Linkage.Lab -> listOf<sicp.runtime.Stmt>(Assign("continue", Source.LabelSrc(linkage.name)))
            }
        // The interpreted compound-apply reaches its body through
        // ev-sequence, whose last-expression path restores continue from
        // the stack: the return address rides the stack, the interpreted
        // calling convention, not the compiled register. The book's
        // compapp register has no name in this machine's fixed register
        // set, so the entry point rides in unev, dead at a call site.
        blocks.add(
            makeInstructionSequence(
                listOf(),
                emptyList(),
                listOf(Label(compoundBranch)),
            ),
        )
        blocks.add(
            makeInstructionSequence(
                listOf("proc"),
                listOf("unev", "continue"),
                contSetup +
                    listOf(
                        sicp.runtime.Save("continue"),
                        Assign("unev", Source.LabelSrc("compound-apply")),
                        Goto(GotoTarget.ByReg("unev")),
                    ),
            ),
        )
    }
    blocks.add(makeInstructionSequence(emptyList(), emptyList(), listOf(Label(afterCall))))
    return appendSequences(blocks)
}

context(r: Raise<MachineError>)
private fun compileProcAppl(
    state: CompilerState,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val allRegs = listOf("env", "proc", "val", "argl", "continue")
    if (target == "val") {
        val statements =
            when (linkage) {
                is Linkage.Return -> {
                    listOf(
                        Assign("val", Source.OpSrc("compiled-procedure-entry", listOf(Source.RegSrc("proc")))),
                        Goto(GotoTarget.ByReg("val")),
                    )
                }

                is Linkage.Lab -> {
                    listOf(
                        Assign("continue", Source.LabelSrc(linkage.name)),
                        Assign("val", Source.OpSrc("compiled-procedure-entry", listOf(Source.RegSrc("proc")))),
                        Goto(GotoTarget.ByReg("val")),
                    )
                }

                is Linkage.Next -> {
                    emptyList()
                }
            }
        if (statements.isNotEmpty()) {
            val needs = if (linkage == Linkage.Return) listOf("proc", "continue") else listOf("proc")
            return makeInstructionSequence(needs, allRegs, statements)
        }
    } else if (linkage == Linkage.Return) {
        r.raise(EvaluatorFault("return linkage, target not val: COMPILE"))
    }
    val procReturn = state.makeLabel("proc-return")
    val gotoLabel = (linkage as? Linkage.Lab)?.name ?: procReturn
    return makeInstructionSequence(
        listOf("proc"),
        allRegs,
        listOf(
            Assign("continue", Source.LabelSrc(procReturn)),
            Assign("val", Source.OpSrc("compiled-procedure-entry", listOf(Source.RegSrc("proc")))),
            Goto(GotoTarget.ByReg("val")),
            Label(procReturn),
            Assign(target, Source.RegSrc("val")),
            Goto(GotoTarget.Lbl(gotoLabel)),
        ),
    )
}

// ---------------------------------------------------------------------------
// Open-coded primitives (5.38, 5.44)
// ---------------------------------------------------------------------------

/** Wraps [seq] in a `save`/`restore` of [reg]: the sequence's tail
 *  reads the register's entry value after writing the register itself
 *  as scratch, so the value the caller left there survives. */
private fun shieldRegister(
    reg: String,
    seq: InstructionSequence,
): InstructionSequence =
    InstructionSequence(
        listUnion(listOf(reg), seq.needs),
        seq.modifies,
        listOf(sicp.runtime.Save(reg)) + seq.stmts + sicp.runtime.Restore(reg),
    )

context(r: Raise<MachineError>)
private fun spreadArguments(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    operands: List<Value>,
    targets: List<String>,
): InstructionSequence {
    if (operands.isEmpty()) return emptyInstructionSequence()
    val target = targets.firstOrNull() ?: r.raise(EvaluatorFault("spread-arguments ran out of argument registers"))
    val code = compile(cfg, state, cenv, operands[0], target, Linkage.Next)
    val rest = operands.drop(1)
    if (rest.isEmpty()) return code
    val restCode = spreadArguments(cfg, state, cenv, rest, targets.drop(1))
    // A later operand may itself be open-coded and write this operand's
    // register as scratch, so the result just computed is shielded
    // across the remaining operand code.
    val shieldedRest =
        if (target in restCode.modifies) {
            shieldRegister(target, restCode)
        } else {
            restCode
        }
    return preserving(cfg, targets.drop(1) + listOf("env"), code, shieldedRest)
}

context(r: Raise<MachineError>)
private fun compileOpenCode(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    exp: Value,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val items = (exp as VPair).toItems()
    val name = (items[0] as VSym).name
    val operands = items.drop(1)
    if (operands.size > 2 && (name == "+" || name == "*")) {
        return compileOpenCodeNary(cfg, state, cenv, name, operands, target, linkage)
    }
    if (operands.size != 2) r.raise(EvaluatorFault("open coding needs two operands for $name"))
    val spread = spreadArguments(cfg, state, cenv, operands, listOf("arg1", "arg2"))
    val applyCode =
        makeInstructionSequence(
            listOf("arg1", "arg2"),
            listOf(target),
            listOf(
                Assign(
                    target,
                    Source.OpSrc(name, listOf(Source.RegSrc("arg1"), Source.RegSrc("arg2"))),
                ),
            ),
        )
    return endWithLinkage(cfg, linkage, append2Sequences(spread, applyCode))
}

/** 5.38(d): more than two operands fold through one register; each
 *  operand is evaluated into `arg1` and folded into `val`, which then
 *  moves to the requested [target]. The accumulated sum in `val` is
 *  shielded across every remaining operand evaluation, since an
 *  operand that calls a procedure writes `val`; the environment is
 *  preserved around an evaluation whose tail reads it; `arg1` and
 *  `arg2` are never preserved around their own evaluations, they are
 *  the evaluations' outputs. */
context(r: Raise<MachineError>)
private fun compileOpenCodeNary(
    cfg: CompilerConfig,
    state: CompilerState,
    cenv: CompileTimeEnv,
    name: String,
    operands: List<Value>,
    target: String,
    linkage: Linkage,
): InstructionSequence {
    val first = operands.getOrNull(0) ?: r.raise(EvaluatorFault("open coding needs operands"))
    val second = operands.getOrNull(1) ?: r.raise(EvaluatorFault("open coding needs operands"))
    val rest = operands.drop(2)
    val c1 = compile(cfg, state, cenv, first, "arg1", Linkage.Next)
    var c2 = compile(cfg, state, cenv, second, "arg2", Linkage.Next)
    // The second operand's evaluation may clobber arg1 internally (an
    // open-coded operand), so the first operand's result is shielded
    // across it.
    if ("arg1" in c2.modifies) {
        c2 = shieldRegister("arg1", c2)
    }
    val openStep =
        makeInstructionSequence(
            listOf("arg1", "val"),
            listOf("val"),
            listOf(Assign("val", Source.OpSrc(name, listOf(Source.RegSrc("arg1"), Source.RegSrc("val"))))),
        )
    var tail = emptyInstructionSequence()
    for (operand in rest.reversed()) {
        var code = compile(cfg, state, cenv, operand, "arg1", Linkage.Next)
        // An operand that calls a procedure writes val, the fold's
        // accumulator, so the accumulator is shielded across it.
        if ("val" in code.modifies) {
            code = shieldRegister("val", code)
        }
        tail = preserving(cfg, listOf("env"), code, append2Sequences(openStep, tail))
    }
    val firstTwo =
        append2Sequences(
            c1,
            append2Sequences(
                c2,
                makeInstructionSequence(
                    listOf("arg1", "arg2"),
                    listOf("val"),
                    listOf(Assign("val", Source.OpSrc(name, listOf(Source.RegSrc("arg1"), Source.RegSrc("arg2"))))),
                ),
            ),
        )
    val fold = append2Sequences(firstTwo, tail)
    val result =
        if (target == "val") {
            fold
        } else {
            append2Sequences(
                fold,
                makeInstructionSequence(
                    listOf("val"),
                    listOf(target),
                    listOf(Assign(target, Source.RegSrc("val"))),
                ),
            )
        }
    return endWithLinkage(cfg, linkage, result)
}

// ---------------------------------------------------------------------------
// Program-level entry points
// ---------------------------------------------------------------------------

/** Compiles the forms of a program in order, the whole body one
 *  sequence with the given linkage. */
context(r: Raise<MachineError>)
public fun compileProgram(
    cfg: CompilerConfig,
    state: CompilerState,
    forms: List<Value>,
    linkage: Linkage = Linkage.Next,
): InstructionSequence {
    if (forms.isEmpty()) return emptyInstructionSequence()
    if (forms.size == 1) return compile(cfg, state, emptyList(), forms[0], "val", linkage)
    val firstCode = compile(cfg, state, emptyList(), forms[0], "val", Linkage.Next)
    val restCode = compileProgram(cfg, state, forms.drop(1), linkage)
    return preserving(cfg, listOf("env", "continue"), firstCode, restCode)
}

/** The statements of [seq], one controller line each, the way the book's
 *  listings read. */
public fun statementsText(seq: InstructionSequence): List<String> = seq.stmts.map { renderStmt(it) }

/** `compile-block`: the controller block of the compiled [forms] under a
 *  fresh entry label, the entry name and the block; the entry label
 *  counts its own sequence, so two blocks on one state cannot collide.
 *  The book compiles the whole expression with target `val` and linkage
 *  `return`: the last form's linkage preserves the caller's `continue`
 *  and returns to it. */
context(r: Raise<MachineError>)
public fun compileBlock(
    cfg: CompilerConfig,
    state: CompilerState,
    forms: List<Value>,
): Pair<String, List<Stmt>> {
    val seq = compileProgram(cfg, state, forms, Linkage.Return)
    val entry = "compiled-entry-${state.bumpEntry()}"
    return entry to (listOf(Label(entry)) + seq.stmts)
}
