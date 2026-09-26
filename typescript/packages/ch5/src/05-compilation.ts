// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.5

/**
 * The compiler of section 5.5. `compile` translates object-language
 * words into instruction sequences, the book's three-part records of
 * needed registers, modified registers, and controller statements; the
 * combinators (`append`, `preserving`, `tack-on`, `parallel`) combine
 * them. Compiled code runs on the 5.4 evaluator machine whose
 * apply-dispatch has learned the compiled-procedure case (5.5.7), and
 * the exercise switches of `CompilerConfig` turn on lexical addressing
 * (5.40 to 5.42), internal-definition scanning (5.43), open-coded
 * primitives (5.38 and 5.44), operand order (5.36), the preserving
 * mechanism itself (5.37), and compiled calls to interpreted
 * procedures (5.47).
 */

import {
  type Arg,
  assemble,
  assign,
  c as baseConst,
  branch,
  type ControllerLine,
  jump,
  jumpReg,
  lbl,
  type Machine,
  makeNewMachine,
  mark,
  type Operation,
  op,
  perform,
  reg,
  restore,
  type Source,
  save,
  test,
  type Value,
} from "./02-simulator.js";
import {
  baseOperations,
  EvaluatorFault,
  evaluatorController,
  evaluatorRegisters,
  formatWord,
  INPUT_EXHAUSTED,
  isTaggedWord,
  makeEnvironmentWord,
  makeTaggedWord,
  nil,
  type PairWord,
  parse,
  type State,
  type TaggedWord,
  type Word,
} from "./04-eceval.js";

// ---------------------------------------------------------------------------
// Words: the small helpers the code generators need
// ---------------------------------------------------------------------------

const symbol = (name: string): Value => ({ symbol: name });
const pair = (car: Word, cdr: Word): PairWord => ({ symbol: "pair", car, cdr });
const isPair = (v: Word): v is PairWord =>
  typeof v === "object" && v !== null && "car" in v && "cdr" in v;
const isNil = (v: Word): boolean =>
  v === nil || (typeof v === "object" && "symbol" in v && v.symbol === "nil" && !("car" in v));
const list = (xs: readonly Word[]): Word =>
  xs.reduceRight((tail, item) => pair(item, tail), nil as Word);
const items = (v: Word): Word[] => {
  const out: Word[] = [];
  let p = v;
  while (isPair(p)) {
    out.push(p.car);
    p = p.cdr;
  }
  if (!isNil(p)) throw new EvaluatorFault("expected a proper list");
  return out;
};
const isSymbolWord = (v: Word): v is Value & { readonly symbol: string } =>
  typeof v === "object" &&
  v !== null &&
  !("car" in v) &&
  !("wordTag" in v) &&
  "symbol" in v &&
  v.symbol !== "nil";
const nameOf = (v: Word): string => {
  if (isSymbolWord(v)) return v.symbol;
  throw new EvaluatorFault("expected a variable");
};
const envIndex = (w: Word): number => {
  if (!isTaggedWord(w, "environment") || typeof (w as TaggedWord).payload !== "number")
    throw new EvaluatorFault("expected an environment word");
  return (w as TaggedWord).payload as number;
};
const primitiveWord = (name: string): TaggedWord => makeTaggedWord("primitive", { name });

/** A constant operand: a machine word, or a label reference. */
const wconst = (value: Word | { readonly label: string }): Arg =>
  typeof value === "object" && value !== null && "label" in value
    ? lbl(value.label)
    : baseConst(value as Value);

/** The spelling a word takes inside `(const ...)`: Scheme notation,
 * with `quote` abbreviated the way the reader wrote it. */
export const constSpelling = (word: Word): string => {
  if (typeof word === "string") return JSON.stringify(word);
  if (isPair(word)) {
    const xs = items(word);
    const head = xs[0];
    if (xs.length === 2 && head !== undefined && isSymbolWord(head) && head.symbol === "quote") {
      return `'${constSpelling(xs[1] ?? nil)}`;
    }
  }
  return formatWord(word);
};

const renderArgText = (arg: Arg): string =>
  arg.tag === "reg"
    ? `(reg ${arg.name})`
    : arg.tag === "const"
      ? `(const ${constSpelling(arg.value)})`
      : `(label ${arg.name})`;

const renderCallText = (name: string, args: readonly Arg[]): string => {
  const head = `(op ${name})`;
  const operands = args.map(renderArgText).join(" ");
  return operands === "" ? head : `${head} ${operands}`;
};

const renderSourceText = (source: Source): string =>
  source.tag === "op" ? renderCallText(source.op, source.args) : renderArgText(source);

/** One statement the way the book prints it; labels print as their bare
 * names, and `(const ...)` operands spell machine words in Scheme
 * notation, which the simulator's own renderer cannot. */
export const renderStatement = (line: ControllerLine): string => {
  switch (line.tag) {
    case "label":
      return line.name;
    case "assign":
      return `(assign ${line.reg} ${renderSourceText(line.source)})`;
    case "test":
      return `(test ${renderCallText(line.op, line.args)})`;
    case "branch":
      return `(branch (label ${line.label}))`;
    case "goto":
      return line.target.tag === "label"
        ? `(goto (label ${line.target.name}))`
        : `(goto (reg ${line.target.name}))`;
    case "save":
      return `(save ${line.reg})`;
    case "restore":
      return `(restore ${line.reg})`;
    case "perform":
      return `(perform ${renderCallText(line.op, line.args)})`;
  }
};

/** The statements of one sequence, one controller line each. */
export const statementsText = (seq: Seq): string => seq.stmts.map(renderStatement).join("\n");

// ---------------------------------------------------------------------------
// Instruction sequences: the book's three-part records
// ---------------------------------------------------------------------------

export interface Seq {
  /** The registers that must hold values before the code runs. */
  readonly needs: readonly string[];
  /** The registers the code's statements modify. */
  readonly modifies: readonly string[];
  /** The controller statements, in execution order. */
  readonly stmts: readonly ControllerLine[];
}

export const makeInstructionSequence = (
  needs: readonly string[],
  modifies: readonly string[],
  stmts: readonly ControllerLine[],
): Seq => ({ needs, modifies, stmts });

export const emptyInstructionSequence = (): Seq => ({ needs: [], modifies: [], stmts: [] });

export const listUnion = (s1: readonly string[], s2: readonly string[]): string[] => {
  const out = [...s1];
  for (const name of s2) if (!out.includes(name)) out.push(name);
  return out;
};

export const listDifference = (s1: readonly string[], s2: readonly string[]): string[] =>
  s1.filter((name) => !s2.includes(name));

export const append2Sequences = (seq1: Seq, seq2: Seq): Seq => ({
  needs: listUnion(seq1.needs, listDifference(seq2.needs, seq1.modifies)),
  modifies: listUnion(seq1.modifies, seq2.modifies),
  stmts: [...seq1.stmts, ...seq2.stmts],
});

export const appendSequences = (seqs: readonly Seq[]): Seq =>
  seqs.reduce((acc, seq) => append2Sequences(acc, seq), emptyInstructionSequence());

export const tackOnInstructionSequence = (seq: Seq, bodySeq: Seq): Seq => ({
  needs: seq.needs,
  modifies: seq.modifies,
  stmts: [...seq.stmts, ...bodySeq.stmts],
});

export const parallelInstructionSequences = (seq1: Seq, seq2: Seq): Seq => ({
  needs: listUnion(seq1.needs, seq2.needs),
  modifies: listUnion(seq1.modifies, seq2.modifies),
  stmts: [...seq1.stmts, ...seq2.stmts],
});

export const preservingInstructionSequences = (
  cfg: CompilerConfig,
  regs: readonly string[],
  seq1: Seq,
  seq2: Seq,
): Seq => {
  let current = seq1;
  for (const name of regs) {
    const needed = seq2.needs.includes(name) && current.modifies.includes(name);
    if (cfg.preservingOn ? needed : true) {
      current = {
        needs: listUnion([name], current.needs),
        modifies: listDifference(current.modifies, [name]),
        stmts: [save(name), ...current.stmts, restore(name)],
      };
    }
  }
  return append2Sequences(current, seq2);
};

/** Wraps a sequence in a save and restore of one register: the code's
 * tail writes the register as scratch and reads its entry value after,
 * so the value the caller left there survives. The open-coded operand
 * paths use this shield where the book's `preserving` cannot, because
 * the protected value is an output of the surrounding code, not an
 * input to it. */
export const shieldRegister = (name: string, seq: Seq): Seq => ({
  needs: listUnion([name], seq.needs),
  modifies: seq.modifies,
  stmts: [save(name), ...seq.stmts, restore(name)],
});

// ---------------------------------------------------------------------------
// Compiler state: the label counter of 5.35
// ---------------------------------------------------------------------------

export interface CompilerState {
  counter: number;
  entries: number;
}

export const newState = (): CompilerState => ({ counter: 0, entries: 0 });

/** A counter seeded at n: exercise 5.35 seeds 14 to reproduce Figure 5.18. */
export const newStateSeeded = (n: number): CompilerState => ({ counter: n, entries: 0 });

export const makeLabel = (state: CompilerState, name: string): string => {
  state.counter += 1;
  return `${name}${state.counter}`;
};

/** Counts one compiled block, so 5.48's recorded entries stay distinct. */
export const bumpEntry = (state: CompilerState): number => {
  state.entries += 1;
  return state.entries;
};

// ---------------------------------------------------------------------------
// Targets, linkages, compile-time environments, configuration
// ---------------------------------------------------------------------------

export type Linkage =
  | { readonly kind: "next" }
  | { readonly kind: "return" }
  | { readonly kind: "label"; readonly name: string };

export const LinkageNext: Linkage = { kind: "next" };
export const LinkageReturn: Linkage = { kind: "return" };
export const linkageLabel = (name: string): Linkage => ({ kind: "label", name });

/** The compile-time environment: frames of parameter names, newest first. */
export type Cenv = readonly (readonly string[])[];

export type LexicalAddress =
  | { readonly found: true; readonly frame: number; readonly displacement: number }
  | { readonly found: false };

export const topCenv = (): Cenv => [];

export const extendCenv = (params: readonly string[], frames: Cenv): Cenv => [params, ...frames];

export const findVariable = (name: string, frames: Cenv): LexicalAddress => {
  for (let f = 0; f < frames.length; f += 1) {
    const frame = frames[f] ?? [];
    const displacement = frame.indexOf(name);
    if (displacement >= 0) return { found: true, frame: f, displacement };
  }
  return { found: false };
};

/** The exercise switches of the section. */
export interface CompilerConfig {
  /** 5.40 to 5.42: emit lexical-address accesses. */
  readonly lexical: boolean;
  /** 5.43: scan internal definitions out of bodies. */
  readonly scanOut: boolean;
  /** 5.38 and 5.44: open-code the named primitives. */
  readonly openCode: boolean;
  /** 5.36: evaluate operands left to right. */
  readonly leftToRight: boolean;
  /** 5.37: the preserving mechanism itself; off saves blindly. */
  readonly preservingOn: boolean;
  /** 5.47: compiled code may call interpreted procedures. */
  readonly compoundCalls: boolean;
  /** 5.40: every variable reference reports the compile-time
   * environment it was compiled against. */
  readonly trace?: (frames: Cenv, name: string) => void;
}

export const defaultConfig = (): CompilerConfig => ({
  lexical: false,
  scanOut: false,
  openCode: false,
  leftToRight: false,
  preservingOn: true,
  compoundCalls: false,
});

/** The primitives the open-coding dispatch of 5.38 recognizes. */
export const openCodedPrimitives: readonly string[] = ["+", "-", "*", "<", "="];

// ---------------------------------------------------------------------------
// Syntax over the object language's list structure
// ---------------------------------------------------------------------------

const isSelfEvaluating = (word: Word): boolean =>
  typeof word === "number" || typeof word === "string" || typeof word === "boolean";

const taggedItems = (word: Word, tag: string): Word[] | null => {
  if (!isPair(word)) return null;
  const xs = items(word);
  const head = xs[0];
  if (head !== undefined && isSymbolWord(head) && head.symbol === tag) return xs;
  return null;
};

const isTaggedForm = (word: Word, tag: string): boolean => taggedItems(word, tag) !== null;

const operandItems = (word: Word): Word[] => (isPair(word) ? items(word) : []);

const parameterNames = (word: Word): string[] => {
  const names: string[] = [];
  for (const item of operandItems(word)) names.push(nameOf(item));
  return names;
};

const lambdaForm = (parameters: Word, body: readonly Word[]): Word =>
  list([symbol("lambda"), parameters, ...body]);

/** The book's `cond->if` (4.1.2): the clauses become nested `if`s. */
export const condToIf = (exp: Word): Word => {
  const xs = taggedItems(exp, "cond");
  if (xs === null) throw new EvaluatorFault("cond->if needs a cond");
  let result: Word = false;
  for (let i = xs.length - 1; i >= 1; i -= 1) {
    const parts = operandItems(xs[i] ?? nil);
    const testExp = parts[0];
    if (testExp === undefined) continue;
    const actions = parts.slice(1);
    const consequent = actions.length === 1 ? (actions[0] ?? nil) : list(actions);
    if (isSymbolWord(testExp) && testExp.symbol === "else") {
      result = consequent;
    } else {
      result = list([symbol("if"), testExp, consequent, result]);
    }
  }
  return result;
};

/** The 4.1.6 `let`-to-combination transformation. */
export const letToCombination = (exp: Word): Word => {
  const xs = taggedItems(exp, "let");
  if (xs === null) throw new EvaluatorFault("let->combination needs a let");
  const bindings = operandItems(xs[1] ?? nil);
  const body = xs.slice(2);
  const params: Word[] = [];
  const inits: Word[] = [];
  for (const binding of bindings) {
    const both = operandItems(binding);
    if (both.length !== 2) throw new EvaluatorFault("let->combination: bad binding");
    params.push(both[0] ?? nil);
    inits.push(both[1] ?? nil);
  }
  return list([lambdaForm(list(params), body), ...inits]);
};

const definitionParts = (exp: Word): { name: string; value: Word } => {
  const xs = taggedItems(exp, "define");
  if (xs === null) throw new EvaluatorFault("compile-definition needs a define");
  const target = xs[1];
  if (target === undefined) throw new EvaluatorFault("compile-definition needs a target");
  if (isSymbolWord(target)) {
    const value = xs[2];
    if (value === undefined) throw new EvaluatorFault("compile-definition needs a value");
    return { name: target.symbol, value };
  }
  if (isPair(target)) {
    const signature = items(target);
    const head = signature[0];
    if (head === undefined || !isSymbolWord(head))
      throw new EvaluatorFault("compile-definition needs a name");
    return { name: head.symbol, value: lambdaForm(list(signature.slice(1)), xs.slice(2)) };
  }
  throw new EvaluatorFault("compile-definition: bad target");
};

// ---------------------------------------------------------------------------
// The code generators
// ---------------------------------------------------------------------------

/** The registers a compiled procedure call may disturb. */
const ALL_REGS: readonly string[] = ["env", "proc", "val", "argl", "continue"];

export const compileLinkage = (linkage: Linkage): Seq => {
  switch (linkage.kind) {
    case "return":
      return makeInstructionSequence(["continue"], [], [jumpReg("continue")]);
    case "next":
      return emptyInstructionSequence();
    case "label":
      return makeInstructionSequence([], [], [jump(linkage.name)]);
  }
};

export const endWithLinkage = (cfg: CompilerConfig, linkage: Linkage, seq: Seq): Seq =>
  preservingInstructionSequences(cfg, ["continue"], seq, compileLinkage(linkage));

const compileSelfEvaluating = (
  cfg: CompilerConfig,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq =>
  endWithLinkage(
    cfg,
    linkage,
    makeInstructionSequence([], [target], [assign(target, wconst(exp))]),
  );

const compileQuoted = (cfg: CompilerConfig, exp: Word, target: string, linkage: Linkage): Seq => {
  const xs = taggedItems(exp, "quote") ?? [];
  const datum = xs[1];
  if (datum === undefined) throw new EvaluatorFault("compile-quoted needs a datum");
  return endWithLinkage(
    cfg,
    linkage,
    makeInstructionSequence([], [target], [assign(target, wconst(datum))]),
  );
};

const compileVariable = (
  cfg: CompilerConfig,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const name = nameOf(exp);
  if (cfg.trace) cfg.trace(cenv, name);
  const address = cfg.lexical ? findVariable(name, cenv) : ({ found: false } as LexicalAddress);
  const access: ControllerLine[] = address.found
    ? [
        assign(
          target,
          op(
            "lexical-address-lookup",
            baseConst(address.frame),
            baseConst(address.displacement),
            reg("env"),
          ),
        ),
      ]
    : [assign(target, op("lookup-variable-value", baseConst(symbol(name)), reg("env")))];
  return endWithLinkage(cfg, linkage, makeInstructionSequence(["env"], [target], access));
};

const compileAssignment = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const xs = taggedItems(exp, "set!") ?? [];
  const nameWord = xs[1];
  const value = xs[2];
  if (nameWord === undefined || !isSymbolWord(nameWord))
    throw new EvaluatorFault("compile-assignment needs a variable");
  if (value === undefined) throw new EvaluatorFault("compile-assignment needs a value");
  const name = nameWord.symbol;
  const valueCode = compile(cfg, state, cenv, value, "val", LinkageNext);
  const found = cfg.lexical ? findVariable(name, cenv) : ({ found: false } as LexicalAddress);
  const store = found.found
    ? perform(
        "lexical-address-set!",
        baseConst(found.frame),
        baseConst(found.displacement),
        reg("val"),
        reg("env"),
      )
    : perform("set-variable-value!", baseConst(symbol(name)), reg("val"), reg("env"));
  const tail = makeInstructionSequence(
    ["env", "val"],
    [target],
    [store, assign(target, wconst(symbol("ok")))],
  );
  return endWithLinkage(
    cfg,
    linkage,
    preservingInstructionSequences(cfg, ["env"], valueCode, tail),
  );
};

const compileDefinition = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const { name, value } = definitionParts(exp);
  const valueCode = compile(cfg, state, cenv, value, "val", LinkageNext);
  const tail = makeInstructionSequence(
    ["env"],
    [target],
    [
      perform("define-variable!", baseConst(symbol(name)), reg("val"), reg("env")),
      assign(target, wconst(symbol("ok"))),
    ],
  );
  return endWithLinkage(
    cfg,
    linkage,
    preservingInstructionSequences(cfg, ["env"], valueCode, tail),
  );
};

const compileIf = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const xs = taggedItems(exp, "if") ?? [];
  const predicate = xs[1];
  const consequent = xs[2];
  if (predicate === undefined) throw new EvaluatorFault("compile-if needs a predicate");
  if (consequent === undefined) throw new EvaluatorFault("compile-if needs a consequent");
  // The label allocations and the compilation order (alternative,
  // consequent, predicate) are the orders the book's own figures show.
  const afterIf = makeLabel(state, "after-if");
  const fBranch = makeLabel(state, "false-branch");
  const tBranch = makeLabel(state, "true-branch");
  const consequentLinkage = linkage.kind === "next" ? linkageLabel(afterIf) : linkage;
  const alternativeExp = xs[3] ?? false;
  const aCode = compile(cfg, state, cenv, alternativeExp, target, linkage);
  const cCode = compile(cfg, state, cenv, consequent, target, consequentLinkage);
  const pCode = compile(cfg, state, cenv, predicate, "val", LinkageNext);
  const testCode = makeInstructionSequence(
    ["val"],
    [],
    [test("false?", reg("val")), branch(fBranch)],
  );
  const trueSide = makeInstructionSequence([], [], [mark(tBranch)]);
  const falseSide = makeInstructionSequence([], [], [mark(fBranch)]);
  const branches = parallelInstructionSequences(
    append2Sequences(trueSide, cCode),
    append2Sequences(falseSide, aCode),
  );
  const withBranches: Seq = { ...branches, stmts: [...branches.stmts, mark(afterIf)] };
  const withTest = append2Sequences(testCode, withBranches);
  return preservingInstructionSequences(cfg, ["env", "continue"], pCode, withTest);
};

const compileSequence = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  seq: readonly Word[],
  target: string,
  linkage: Linkage,
): Seq => {
  const first = seq[0];
  if (first === undefined) throw new EvaluatorFault("compile-sequence needs a sequence");
  const rest = seq.slice(1);
  if (rest.length === 0) return compile(cfg, state, cenv, first, target, linkage);
  const firstCode = compile(cfg, state, cenv, first, target, LinkageNext);
  const restCode = compileSequence(cfg, state, cenv, rest, target, linkage);
  return preservingInstructionSequences(cfg, ["env", "continue"], firstCode, restCode);
};

/** 5.43: internal definitions become `*unassigned*` bindings followed
 * by assignments, one `let` ahead of the body. */
export const scanOutDefines = (body: readonly Word[]): Word[] => {
  if (!body.some((form) => isTaggedForm(form, "define"))) return [...body];
  const names: string[] = [];
  const sets: Word[] = [];
  for (const form of body) {
    if (!isTaggedForm(form, "define")) continue;
    const { name, value } = definitionParts(form);
    names.push(name);
    sets.push(list([symbol("set!"), symbol(name), value]));
  }
  const bindings = list(
    names.map((name) => list([symbol(name), list([symbol("quote"), symbol("*unassigned*")])])),
  );
  return [
    list([symbol("let"), bindings, ...sets, ...body.filter((f) => !isTaggedForm(f, "define"))]),
  ];
};

const compileLambda = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const xs = taggedItems(exp, "lambda") ?? [];
  if (xs.length < 2) throw new EvaluatorFault("compile-lambda needs parameters");
  // after-lambda is allocated before entry, the order the book's
  // figures show.
  const afterLambda = makeLabel(state, "after-lambda");
  const procEntry = makeLabel(state, "entry");
  const lambdaLinkage = linkage.kind === "next" ? linkageLabel(afterLambda) : linkage;
  const construct = endWithLinkage(
    cfg,
    lambdaLinkage,
    makeInstructionSequence(
      ["env"],
      [target],
      [assign(target, op("make-compiled-procedure", wconst({ label: procEntry }), reg("env")))],
    ),
  );
  const body = compileLambdaBody(cfg, state, cenv, exp, procEntry);
  const combined = tackOnInstructionSequence(construct, body);
  // The body is not executed in line, so its register use stays
  // invisible; the after-lambda label it falls through to closes the
  // construction.
  return { ...combined, stmts: [...combined.stmts, mark(afterLambda)] };
};

const compileLambdaBody = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  procEntry: string,
): Seq => {
  const xs = taggedItems(exp, "lambda") ?? [];
  const parameters = xs[1];
  if (parameters === undefined) throw new EvaluatorFault("compile-lambda-body needs parameters");
  const names = parameterNames(parameters);
  const rawBody = xs.slice(2);
  const body = cfg.scanOut ? scanOutDefines(rawBody) : rawBody;
  const head = makeInstructionSequence(
    ["env", "proc", "argl"],
    ["env"],
    [
      mark(procEntry),
      assign("env", op("compiled-procedure-env", reg("proc"))),
      assign(
        "env",
        op("extend-environment", wconst(list(names.map(symbol))), reg("argl"), reg("env")),
      ),
    ],
  );
  const bodyCode = compileSequence(cfg, state, extendCenv(names, cenv), body, "val", LinkageReturn);
  return append2Sequences(head, bodyCode);
};

const compileApplication = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const xs = items(exp);
  const operator = xs[0];
  if (operator === undefined) throw new EvaluatorFault("compile-application needs an operator");
  const operands = xs.slice(1);
  const procCode = compile(cfg, state, cenv, operator, "proc", LinkageNext);
  const operandCodes: Seq[] = [];
  for (const operand of operands)
    operandCodes.push(compile(cfg, state, cenv, operand, "val", LinkageNext));
  const arglistCode = constructArglist(cfg, operandCodes);
  const call = compileProcedureCall(cfg, state, target, linkage);
  const inner = preservingInstructionSequences(cfg, ["proc", "continue"], arglistCode, call);
  return preservingInstructionSequences(cfg, ["env", "continue"], procCode, inner);
};

/** The book's `construct-arglist`: the default evaluates the operands
 * right to left, the last operand initializing `argl` and each earlier
 * operand consing onto it. The 5.36 left-to-right configuration
 * evaluates first to last, adjoining each argument at the end. */
const constructArglist = (cfg: CompilerConfig, operandCodes: readonly Seq[]): Seq => {
  const ordered = cfg.leftToRight ? [...operandCodes] : [...operandCodes].toReversed();
  if (ordered.length === 0)
    return makeInstructionSequence([], ["argl"], [assign("argl", wconst(nil))]);
  const consOp = cfg.leftToRight
    ? assign("argl", op("adjoin-arg", reg("val"), reg("argl")))
    : assign("argl", op("cons", reg("val"), reg("argl")));
  const codeToGetLastArg = append2Sequences(
    ordered[0] ?? emptyInstructionSequence(),
    makeInstructionSequence(["val"], ["argl"], [assign("argl", op("list", reg("val")))]),
  );
  if (ordered.length === 1) return codeToGetLastArg;
  const consStep = makeInstructionSequence(["val", "argl"], ["argl"], [consOp]);
  let restCode = emptyInstructionSequence();
  for (let i = ordered.length - 1; i >= 1; i -= 1) {
    const operandCode = ordered[i] ?? emptyInstructionSequence();
    const codeForNextArg = preservingInstructionSequences(cfg, ["argl"], operandCode, consStep);
    restCode = preservingInstructionSequences(cfg, ["env"], codeForNextArg, restCode);
  }
  return preservingInstructionSequences(cfg, ["env"], codeToGetLastArg, restCode);
};

const compileProcedureCall = (
  cfg: CompilerConfig,
  state: CompilerState,
  target: string,
  linkage: Linkage,
): Seq => {
  // after-call is allocated before compiled-branch before
  // primitive-branch, the order the book's figures show.
  const afterCall = makeLabel(state, "after-call");
  const compiledBranch = makeLabel(state, "compiled-branch");
  const primitiveBranch = makeLabel(state, "primitive-branch");
  const compoundBranch = cfg.compoundCalls ? makeLabel(state, "compound-branch") : null;
  const compiledLinkage = linkage.kind === "next" ? linkageLabel(afterCall) : linkage;
  const applCode = compileProcAppl(state, target, compiledLinkage);
  // compound-apply answers in `val`; a call compiled into another
  // register needs the copy the compiled branch's proc-return
  // performs, so the interpreted branch lands on compound-return
  // first.
  const compoundReturn =
    compoundBranch !== null && target !== "val" ? makeLabel(state, "compound-return") : null;
  const primitiveTail: ControllerLine[] =
    compoundBranch !== null && linkage.kind === "next" ? [jump(afterCall)] : [];
  const primitiveCode = endWithLinkage(
    cfg,
    linkage,
    makeInstructionSequence(
      ["proc", "argl"],
      [target],
      [assign(target, op("apply-primitive-procedure", reg("proc"), reg("argl"))), ...primitiveTail],
    ),
  );
  const testCode = makeInstructionSequence(
    [],
    [],
    [test("primitive-procedure?", reg("proc")), branch(primitiveBranch)],
  );
  const compoundTest =
    compoundBranch === null
      ? emptyInstructionSequence()
      : makeInstructionSequence(
          ["proc"],
          [],
          [test("compound-procedure?", reg("proc")), branch(compoundBranch)],
        );
  const dispatched = parallelInstructionSequences(
    append2Sequences(makeInstructionSequence([], [], [mark(compiledBranch)]), applCode),
    append2Sequences(makeInstructionSequence([], [], [mark(primitiveBranch)]), primitiveCode),
  );
  const compoundLabel =
    compoundBranch === null
      ? emptyInstructionSequence()
      : (() => {
          const returnLabel = compoundReturn ?? null;
          const continueTarget =
            returnLabel ??
            (linkage.kind === "next" ? afterCall : linkage.kind === "label" ? linkage.name : null);
          const stmts: ControllerLine[] = [mark(compoundBranch)];
          if (continueTarget !== null)
            stmts.push(assign("continue", wconst({ label: continueTarget })));
          stmts.push(
            save("continue"),
            assign("unev", wconst({ label: "compound-apply" })),
            jumpReg("unev"),
          );
          return makeInstructionSequence(["proc"], ["unev", "continue"], stmts);
        })();
  const compoundReturnBlock =
    compoundReturn === null
      ? emptyInstructionSequence()
      : (() => {
          const exit = linkage.kind === "label" ? linkage.name : afterCall;
          return makeInstructionSequence(
            ["val"],
            [target],
            [mark(compoundReturn), assign(target, reg("val")), jump(exit)],
          );
        })();
  const after = makeInstructionSequence([], [], [mark(afterCall)]);
  return appendSequences([
    testCode,
    compoundTest,
    dispatched,
    compoundLabel,
    compoundReturnBlock,
    after,
  ]);
};

const compileProcAppl = (state: CompilerState, target: string, linkage: Linkage): Seq => {
  const entry = assign("val", op("compiled-procedure-entry", reg("proc")));
  const goto = jumpReg("val");
  if (linkage.kind === "return") {
    if (target !== "val") throw new EvaluatorFault("return linkage, target not val: COMPILE");
    return makeInstructionSequence(["proc", "continue"], ALL_REGS, [entry, goto]);
  }
  if (linkage.kind === "label") {
    if (target === "val")
      return makeInstructionSequence(["proc"], ALL_REGS, [
        assign("continue", wconst({ label: linkage.name })),
        entry,
        goto,
      ]);
    const procReturn = makeLabel(state, "proc-return");
    return makeInstructionSequence(["proc"], ALL_REGS, [
      assign("continue", wconst({ label: procReturn })),
      entry,
      goto,
      mark(procReturn),
      assign(target, reg("val")),
      jump(linkage.name),
    ]);
  }
  throw new EvaluatorFault("compile-proc-appl: the call carries no next linkage");
};

// ---------------------------------------------------------------------------
// 5.38 and 5.44: open-coded primitives
// ---------------------------------------------------------------------------

const isOpenCoded = (cfg: CompilerConfig, cenv: Cenv, operator: Word): boolean => {
  if (!cfg.openCode || !isSymbolWord(operator)) return false;
  const name = operator.symbol;
  return openCodedPrimitives.includes(name) && !findVariable(name, cenv).found;
};

/** 5.38(a): the operands are evaluated into successive argument
 * registers, with the registers still to come preserved around each
 * evaluation, because an operand may itself be an open-coded call;
 * the environment is preserved with them, because a nested call
 * operand rebinds it and a later operand (a variable reference) reads
 * the caller's frame. */
const spreadArguments = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  operands: readonly Word[],
  targets: readonly string[],
): Seq => {
  const target = targets[0];
  if (target === undefined)
    throw new EvaluatorFault("spread-arguments ran out of argument registers");
  const operand = operands[0];
  if (operand === undefined) return emptyInstructionSequence();
  const code = compile(cfg, state, cenv, operand, target, LinkageNext);
  const rest = operands.slice(1);
  if (rest.length === 0) return code;
  const restCode = spreadArguments(cfg, state, cenv, rest, targets.slice(1));
  // A later operand may itself be open-coded and write this operand's
  // register as scratch, so the result just computed is shielded
  // across the remaining operand code.
  const shieldedRest = restCode.modifies.includes(target)
    ? shieldRegister(target, restCode)
    : restCode;
  return preservingInstructionSequences(cfg, [...targets.slice(1), "env"], code, shieldedRest);
};

const compileOpenCode = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  const xs = items(exp);
  const name = nameOf(xs[0] ?? nil);
  const operands = xs.slice(1);
  if (operands.length > 2 && (name === "+" || name === "*"))
    return compileOpenCodeNary(cfg, state, cenv, name, operands, target, linkage);
  if (operands.length !== 2) throw new EvaluatorFault(`open coding needs two operands for ${name}`);
  const spread = spreadArguments(cfg, state, cenv, operands, ["arg1", "arg2"]);
  const applyCode = makeInstructionSequence(
    ["arg1", "arg2"],
    [target],
    [assign(target, op(name, reg("arg1"), reg("arg2")))],
  );
  return endWithLinkage(cfg, linkage, append2Sequences(spread, applyCode));
};

/** 5.38(d): more than two operands fold through one register: each
 * operand is evaluated into `arg1` and folded into `val`, which then
 * moves to the requested target. The accumulated sum in `val` is
 * shielded across every remaining operand evaluation, since an
 * operand that calls a procedure writes `val`; the environment is
 * preserved around an evaluation whose tail reads it; `arg1` itself
 * is never preserved around its own evaluation, it is the
 * evaluation's output. */
const compileOpenCodeNary = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  name: string,
  operands: readonly Word[],
  target: string,
  linkage: Linkage,
): Seq => {
  const first = operands[0];
  const second = operands[1];
  if (first === undefined || second === undefined)
    throw new EvaluatorFault("open coding needs operands");
  const c1 = compile(cfg, state, cenv, first, "arg1", LinkageNext);
  let c2 = compile(cfg, state, cenv, second, "arg2", LinkageNext);
  // The second operand's evaluation may clobber arg1 internally (an
  // open-coded operand), so the first operand's result is shielded
  // across it.
  if (c2.modifies.includes("arg1")) c2 = shieldRegister("arg1", c2);
  const foldFirst = makeInstructionSequence(
    ["arg1", "arg2"],
    ["val"],
    [assign("val", op(name, reg("arg1"), reg("arg2")))],
  );
  const openStep = makeInstructionSequence(
    ["arg1", "val"],
    ["val"],
    [assign("val", op(name, reg("arg1"), reg("val")))],
  );
  let restCode = emptyInstructionSequence();
  for (let i = operands.length - 1; i >= 2; i -= 1) {
    let code = compile(cfg, state, cenv, operands[i] ?? nil, "arg1", LinkageNext);
    // An operand that calls a procedure writes val, the fold's
    // accumulator, so the accumulator is shielded across it.
    if (code.modifies.includes("val")) code = shieldRegister("val", code);
    restCode = preservingInstructionSequences(
      cfg,
      ["env"],
      code,
      append2Sequences(openStep, restCode),
    );
  }
  // The first two operands preserve `env` the way the fold loop does:
  // an earlier operand that is a call rebinds `env`, and any later
  // operand reading a variable must see the caller's frame.
  const afterSecond = preservingInstructionSequences(
    cfg,
    ["env"],
    c2,
    append2Sequences(foldFirst, restCode),
  );
  const firstTwo = preservingInstructionSequences(cfg, ["env"], c1, afterSecond);
  const result =
    target === "val"
      ? firstTwo
      : append2Sequences(
          firstTwo,
          makeInstructionSequence(["val"], [target], [assign(target, reg("val"))]),
        );
  return endWithLinkage(cfg, linkage, result);
};

// ---------------------------------------------------------------------------
// The book's `compile`, the top-level dispatch
// ---------------------------------------------------------------------------

export const compile = (
  cfg: CompilerConfig,
  state: CompilerState,
  cenv: Cenv,
  exp: Word,
  target: string,
  linkage: Linkage,
): Seq => {
  if (isSelfEvaluating(exp)) return compileSelfEvaluating(cfg, exp, target, linkage);
  if (isTaggedForm(exp, "quote")) return compileQuoted(cfg, exp, target, linkage);
  if (isSymbolWord(exp)) return compileVariable(cfg, cenv, exp, target, linkage);
  if (isTaggedForm(exp, "set!")) return compileAssignment(cfg, state, cenv, exp, target, linkage);
  if (isTaggedForm(exp, "define")) return compileDefinition(cfg, state, cenv, exp, target, linkage);
  if (isTaggedForm(exp, "if")) return compileIf(cfg, state, cenv, exp, target, linkage);
  if (isTaggedForm(exp, "lambda")) return compileLambda(cfg, state, cenv, exp, target, linkage);
  if (isTaggedForm(exp, "begin")) {
    const xs = taggedItems(exp, "begin") ?? [];
    return compileSequence(cfg, state, cenv, xs.slice(1), target, linkage);
  }
  if (isTaggedForm(exp, "cond")) return compile(cfg, state, cenv, condToIf(exp), target, linkage);
  if (isTaggedForm(exp, "let"))
    return compile(cfg, state, cenv, letToCombination(exp), target, linkage);
  if (isPair(exp)) {
    const operator = items(exp)[0] ?? nil;
    if (isOpenCoded(cfg, cenv, operator))
      return compileOpenCode(cfg, state, cenv, exp, target, linkage);
    return compileApplication(cfg, state, cenv, exp, target, linkage);
  }
  throw new EvaluatorFault("Unknown expression type: COMPILE");
};

/** Compiles the forms of one program: every form but the last
 * continues to the next, and the last carries the requested linkage;
 * the forms append with `env` and `continue` preserved. */
export const compileForms = (
  cfg: CompilerConfig,
  state: CompilerState,
  forms: readonly Word[],
  linkage: Linkage,
): Seq => {
  const last = forms.length - 1;
  if (last < 0) throw new EvaluatorFault("compile-forms needs a program");
  let acc: Seq | null = null;
  for (let i = 0; i <= last; i += 1) {
    const form = forms[i] ?? nil;
    const formLinkage = i === last ? linkage : LinkageNext;
    const code = compile(cfg, state, topCenv(), form, "val", formLinkage);
    acc = acc === null ? code : preservingInstructionSequences(cfg, ["env", "continue"], acc, code);
  }
  return acc ?? emptyInstructionSequence();
};

/** Reads `source` and compiles it as one program. */
export const compileProgram = (
  cfg: CompilerConfig,
  state: CompilerState,
  source: string,
  linkage: Linkage,
): Seq => compileForms(cfg, state, parse(source), linkage);

/** Compiles the forms of `source` under a fresh entry label: the shape
 * `compileAndGo`, 5.48's recorded blocks, and 5.49's chained forms
 * share. The entry label counts its own sequence, so two
 * `compileBlock` calls on one state never collide. */
export const compileBlock = (
  cfg: CompilerConfig,
  state: CompilerState,
  source: string,
): { entry: string; lines: readonly ControllerLine[] } => {
  const seq = compileProgram(cfg, state, source, LinkageReturn);
  const entry = `compiled-entry-${bumpEntry(state)}`;
  return { entry, lines: [mark(entry), ...seq.stmts] };
};

// ---------------------------------------------------------------------------
// The 5.5.7 machine: compiled code beside the interpreted evaluator
// ---------------------------------------------------------------------------

const compiledProcedureWord = (entry: Value, environment: Word): TaggedWord =>
  makeTaggedWord("compiled-procedure", { entry, env: envIndex(environment) });

const compiledProcedureParts = (word: Word): { entry: Value; env: number } | null => {
  if (!isTaggedWord(word, "compiled-procedure")) return null;
  const payload = word.payload as { entry: Value; env: number };
  return payload;
};

/** Renders a word on the compiled machine's transcript: compiled
 * procedures print the way the book's sessions show them, everything
 * else as 5.4 renders. */
export const renderCompiledWord = (word: Word): string =>
  compiledProcedureParts(word) !== null ? "#[compiled-procedure]" : formatWord(word);

/** The apply-dispatch of 5.5.7: the compiled-procedure test joins the
 * book's dispatch, before the unknown-type stop, and `compiled-apply`
 * restores `continue` and jumps to the compiled code's entry. */
export const compiledApplyDispatch: readonly ControllerLine[] = [
  mark("apply-dispatch"),
  test("primitive-procedure?", reg("proc")),
  branch("primitive-apply"),
  test("compound-procedure?", reg("proc")),
  branch("compound-apply"),
  test("compiled-procedure?", reg("proc")),
  branch("compiled-apply"),
  jump("unknown-procedure-type"),
  mark("compiled-apply"),
  restore("continue"),
  assign("val", op("compiled-procedure-entry", reg("proc"))),
  jumpReg("val"),
];

/** The external entry: reached when the machine starts armed, it
 * points `continue` at `print-result` and jumps to the compiled code
 * in `val`. */
export const externalEntry: readonly ControllerLine[] = [
  mark("external-entry"),
  perform("initialize-stack"),
  assign("env", op("get-global-environment")),
  assign("continue", wconst({ label: "print-result" })),
  jumpReg("val"),
];

const fragmentBounds = (
  controller: readonly ControllerLine[],
): { driver: number; evalDispatch: number; applyDispatch: number; primitiveApply: number } => {
  const at = (name: string): number => {
    const index = controller.findIndex((line) => line.tag === "label" && line.name === name);
    if (index < 0) throw new EvaluatorFault(`the evaluator controller lacks ${name}`);
    return index;
  };
  return {
    driver: at("read-eval-print-loop"),
    evalDispatch: at("eval-dispatch"),
    applyDispatch: at("apply-dispatch"),
    primitiveApply: at("primitive-apply"),
  };
};

/** The driver with the armed-entry guard in front: the test of the
 * armed latch stands in for the book's flag branch, because the 5.2
 * machine's flag is written only by tests. */
export const guardedDriver: readonly ControllerLine[] = [
  test("compiled-entry-armed?"),
  branch("external-entry"),
  ...evaluatorController.slice(
    fragmentBounds(evaluatorController).driver,
    fragmentBounds(evaluatorController).evalDispatch,
  ),
];

/** The plain 5.4 driver fragment. */
export const plainDriver: readonly ControllerLine[] = evaluatorController.slice(
  fragmentBounds(evaluatorController).driver,
  fragmentBounds(evaluatorController).evalDispatch,
);

/** The 5.4.4 monitored driver: it prints the stack statistics between
 * each interaction. */
export const monitoredDriver: readonly ControllerLine[] = (() => {
  const bounds = fragmentBounds(evaluatorController);
  const block = evaluatorController.slice(bounds.driver, bounds.evalDispatch);
  const printResult = block.findIndex(
    (line) => line.tag === "label" && line.name === "print-result",
  );
  if (printResult < 0) throw new EvaluatorFault("the driver lacks print-result");
  return [
    ...block.slice(0, printResult + 1),
    perform("print-stack-statistics"),
    ...block.slice(printResult + 1),
  ];
})();

const controllerWithDriver = (driver: readonly ControllerLine[]): readonly ControllerLine[] => {
  const bounds = fragmentBounds(evaluatorController);
  return [
    ...driver,
    ...evaluatorController.slice(bounds.evalDispatch, bounds.applyDispatch),
    ...compiledApplyDispatch,
    ...evaluatorController.slice(bounds.primitiveApply),
    ...externalEntry,
  ];
};

/** The 5.5.7 controller: the guarded driver, the evaluator's core with
 * the compiled apply-dispatch in place of its 5.4 counterpart, and the
 * external entry. */
export const ecevalController: readonly ControllerLine[] = controllerWithDriver(guardedDriver);

/** The monitored 5.5.7 controller: the stack-measuring exercises
 * (5.45, 5.46) and the book's monitored 5.5.7 session run on it. */
export const monitoredEcevalController: readonly ControllerLine[] = controllerWithDriver([
  test("compiled-entry-armed?"),
  branch("external-entry"),
  ...monitoredDriver,
]);

/** A 5.5.7 controller with another driver fragment: the chained driver
 * of 5.49 composes this way. */
export const controllerReplacingDriver = (
  driver: readonly ControllerLine[],
): readonly ControllerLine[] => controllerWithDriver(driver);

/** A runtime primitive the machine's `apply-primitive-procedure`
 * reaches: the shape 5.36's recording primitive and 5.48's
 * `compile-and-run` take. */
export type RuntimeFn = (args: readonly Word[]) => Word;

const numberArg = (name: string, word: Word | undefined): number => {
  if (typeof word !== "number") throw new EvaluatorFault(`${name}: not a number`);
  return word;
};

/** True when two object-language words are `equal?`: numbers,
 * symbols, and pairs compared structurally. */
const wordsEqual = (a: Word, b: Word): boolean => {
  if (a === b) return true;
  if (isPair(a) && isPair(b)) return wordsEqual(a.car, b.car) && wordsEqual(a.cdr, b.cdr);
  if (isSymbolWord(a) && isSymbolWord(b)) return a.symbol === b.symbol;
  if (isNil(a) && isNil(b)) return true;
  return false;
};

/** The variable name a runtime environment primitive was handed: a
 * symbol, or the `(quote x)` form a quoted name compiles to. */
const variableName = (primitive: string, word: Word): string => {
  if (isSymbolWord(word)) return word.symbol;
  if (isPair(word)) {
    const xs = items(word);
    const head = xs[0];
    if (xs.length === 2 && head !== undefined && isSymbolWord(head) && head.symbol === "quote") {
      const inner = xs[1];
      if (inner !== undefined && isSymbolWord(inner)) return inner.symbol;
    }
  }
  throw new EvaluatorFault(`${primitive}: needs a variable, got ${formatWord(word)}`);
};

/** The runtime primitive names the machine binds in its global
 * environment: the 5.4 object set plus the names the compiled
 * metacircular calls. */
export const runtimeNames: readonly string[] = [
  "cons",
  "car",
  "cdr",
  "cadr",
  "caddr",
  "cadddr",
  "cddr",
  "cdddr",
  "caadr",
  "cdadr",
  "null?",
  "pair?",
  "symbol?",
  "number?",
  "string?",
  "not",
  "eq?",
  "equal?",
  "list",
  "+",
  "-",
  "*",
  "/",
  "=",
  "<",
  ">",
  "<=",
  ">=",
  "remainder",
  "quotient",
  "abs",
  "display",
  "newline",
  "error",
  "extend-environment",
  "lookup-variable-value",
  "set-variable-value!",
  "define-variable!",
  "apply-in-underlying-scheme",
];

/** Applies the runtime primitive `name` to words: the compiled
 * machine's own primitive table, the 5.4 object set plus the names
 * the compiled metacircular calls. The environment primitives walk
 * the machine's own frame chain, and the symbol `the-empty` names the
 * empty environment. */
export const applyRuntimePrimitive = (state: State, name: string, args: readonly Word[]): Word => {
  const first = () => args[0] ?? nil;
  const second = () => args[1] ?? nil;
  const n = (w: Word | undefined): number => numberArg(name, w);
  const findFrame = (env: Word, variable: string): State["frames"][number] => {
    let index = envIndex(env);
    for (;;) {
      const frame = state.frames[index];
      if (!frame) throw new EvaluatorFault("bad environment index");
      if (frame.bindings.has(variable)) return frame;
      if (frame.parent === null) throw new EvaluatorFault(`unbound variable: ${variable}`);
      index = frame.parent;
    }
  };
  const baseEnvIndex = (env: Word): number => {
    if (isSymbolWord(env) && env.symbol === "the-empty") return -1;
    return envIndex(env);
  };
  switch (name) {
    case "cons":
      return pair(first(), second());
    case "car": {
      const p = first();
      if (!isPair(p)) throw new EvaluatorFault("type error: car");
      return p.car;
    }
    case "cdr": {
      const p = first();
      if (!isPair(p)) throw new EvaluatorFault("type error: cdr");
      return p.cdr;
    }
    case "cadr":
      return applyRuntimePrimitive(state, "car", [applyRuntimePrimitive(state, "cdr", args)]);
    case "caddr":
      return applyRuntimePrimitive(state, "car", [applyRuntimePrimitive(state, "cddr", args)]);
    case "cadddr":
      return applyRuntimePrimitive(state, "car", [applyRuntimePrimitive(state, "cdddr", args)]);
    case "cddr":
      return applyRuntimePrimitive(state, "cdr", [applyRuntimePrimitive(state, "cdr", args)]);
    case "cdddr":
      return applyRuntimePrimitive(state, "cdr", [applyRuntimePrimitive(state, "cddr", args)]);
    case "caadr":
      return applyRuntimePrimitive(state, "car", [applyRuntimePrimitive(state, "cadr", args)]);
    case "cdadr":
      return applyRuntimePrimitive(state, "cdr", [applyRuntimePrimitive(state, "cadr", args)]);
    case "null?":
      return isNil(first());
    case "pair?":
      return isPair(first());
    case "symbol?":
      return isSymbolWord(first());
    case "number?":
      return typeof first() === "number";
    case "string?":
      return typeof first() === "string";
    case "not":
      return first() === false;
    case "eq?": {
      const a = first();
      const b = second();
      if (a === b) return true;
      // Two symbol words spell the same symbol whatever object the
      // reader built for each occurrence.
      if (isSymbolWord(a) && isSymbolWord(b)) return a.symbol === b.symbol;
      return typeof a === "number" && a === b;
    }
    case "equal?":
      return wordsEqual(first(), second());
    case "list":
      return list(args);
    case "+":
      return args.reduce<number>((a, b) => a + n(b), 0);
    case "-":
      return args.length === 1
        ? -n(args[0])
        : args.slice(1).reduce<number>((a, b) => a - n(b), n(args[0]));
    case "*":
      return args.reduce<number>((a, b) => a * n(b), 1);
    case "/": {
      let x = n(args[0]);
      for (const a of args.slice(1)) {
        const d = n(a);
        if (d === 0) throw new EvaluatorFault("division by zero");
        x /= d;
      }
      return x;
    }
    case "=":
      return n(args[0]) === n(args[1]);
    case "<":
      return n(args[0]) < n(args[1]);
    case ">":
      return n(args[0]) > n(args[1]);
    case "<=":
      return n(args[0]) <= n(args[1]);
    case ">=":
      return n(args[0]) >= n(args[1]);
    case "remainder":
      return n(args[0]) % n(args[1]);
    case "quotient": {
      const d = n(args[1]);
      if (d === 0) throw new EvaluatorFault("division by zero");
      return Math.trunc(n(args[0]) / d);
    }
    case "abs":
      return Math.abs(n(args[0]));
    case "display": {
      state.output.push(formatWord(first()));
      return first();
    }
    case "newline": {
      state.output.push("");
      return symbol("newline");
    }
    case "error":
      throw new EvaluatorFault(`error: ${args.map((a) => formatWord(a)).join(" ")}`);
    case "extend-environment": {
      const vars = operandItems(first()).map(nameOf);
      const vals = operandItems(second());
      if (vars.length !== vals.length)
        throw new EvaluatorFault(`arity mismatch: expected ${vars.length}, given ${vals.length}`);
      const base = baseEnvIndex(args[2] ?? nil);
      const index = state.frames.length;
      state.frames.push({
        bindings: new Map(vars.map((v, j) => [v, vals[j] ?? nil])),
        parent: base,
      });
      return makeEnvironmentWord(index);
    }
    case "lookup-variable-value": {
      const variable = variableName(name, first());
      const frame = findFrame(second(), variable);
      const found = frame.bindings.get(variable);
      if (found === undefined) throw new EvaluatorFault(`unbound variable: ${variable}`);
      return found;
    }
    case "set-variable-value!": {
      const variable = variableName(name, first());
      findFrame(args[2] ?? nil, variable).bindings.set(variable, second());
      return symbol("ok");
    }
    case "define-variable!": {
      const variable = variableName(name, first());
      const frame = state.frames[baseEnvIndex(args[2] ?? nil)];
      if (!frame) throw new EvaluatorFault("bad environment index");
      frame.bindings.set(variable, second());
      return symbol("ok");
    }
    case "apply-in-underlying-scheme": {
      const proc = first();
      if (!isTaggedWord(proc, "primitive"))
        throw new EvaluatorFault("apply needs a primitive procedure");
      const impl = (proc.payload as { name: string }).name;
      return applyRuntimePrimitive(state, impl, operandItems(second()));
    }
    default:
      throw new EvaluatorFault(`unknown primitive procedure: ${name}`);
  }
};

/** The mutable handle the compiled operations share with the
 * evaluator wrapper: the machine once it exists, and the armed latch
 * the external-entry guard tests. */
export interface MachineHandle {
  machine?: Machine;
  armed: boolean;
}

/** The operations the compiled code names and the interpreted path
 * never does: the footnote-323 compiled-procedure family, the book's
 * `false?`, the argument-list builders, the armed-entry latch the
 * guard tests, the stack printer, the open-coded arithmetic, the
 * lexical accesses of 5.39 to 5.42, and the primitive dispatcher the
 * caller's runtime table extends. Install these over the 5.4 base
 * table, whose `apply-primitive-procedure` they override. */
export const compiledOperations = (
  state: State,
  handle: MachineHandle,
  runtime: Readonly<Record<string, RuntimeFn>> = {},
): Record<string, Operation> => {
  const asWords = (values: ReadonlyArray<Value>): readonly Word[] => values;
  const one = (args: ReadonlyArray<Value>): Word => asWords(args)[0] ?? nil;
  const two = (args: ReadonlyArray<Value>): [Word, Word] => [
    asWords(args)[0] ?? nil,
    asWords(args)[1] ?? nil,
  ];
  const applyPrimitive = (args: ReadonlyArray<Value>): Value => {
    const [proc, argl] = two(args);
    if (!isTaggedWord(proc, "primitive"))
      throw new EvaluatorFault("apply-primitive-procedure needs a primitive procedure");
    const name = (proc.payload as { name: string }).name;
    const extra = runtime[name];
    if (extra) return extra(operandItems(argl)) as Value;
    return applyRuntimePrimitive(state, name, operandItems(argl)) as Value;
  };
  const ops: Record<string, Operation> = {
    "false?": (args) => one(args) === false,
    cons: (args) => {
      const [a, b] = two(args);
      return pair(a, b);
    },
    list: (args) => list(asWords(args)),
    "make-compiled-procedure": (args) => {
      const [entry, env] = two(args);
      if (typeof entry !== "number")
        throw new EvaluatorFault("make-compiled-procedure needs an entry label");
      return compiledProcedureWord(entry, env);
    },
    "compiled-procedure?": (args) => compiledProcedureParts(one(args)) !== null,
    "compiled-procedure-entry": (args) => {
      const parts = compiledProcedureParts(one(args));
      if (parts === null)
        throw new EvaluatorFault("compiled-procedure-entry needs a compiled procedure");
      return parts.entry;
    },
    "compiled-procedure-env": (args) => {
      const parts = compiledProcedureParts(one(args));
      if (parts === null)
        throw new EvaluatorFault("compiled-procedure-env needs a compiled procedure");
      return makeEnvironmentWord(parts.env);
    },
    "compiled-entry-armed?": () => {
      if (handle.armed) {
        handle.armed = false;
        return true;
      }
      return false;
    },
    "print-stack-statistics": () => {
      const machine = handle.machine;
      state.output.push(
        machine ? machine.stack.statisticsLine() : "(total-pushes = 0 maximum-depth = 0)",
      );
      return 0;
    },
    "apply-primitive-procedure": applyPrimitive,
    "+": (args) => applyPrimitive([primitiveWord("+"), list(asWords(args))]),
    "-": (args) => applyPrimitive([primitiveWord("-"), list(asWords(args))]),
    "*": (args) => applyPrimitive([primitiveWord("*"), list(asWords(args))]),
    "<": (args) => applyPrimitive([primitiveWord("<"), list(asWords(args))]),
    "=": (args) => applyPrimitive([primitiveWord("="), list(asWords(args))]),
    "lexical-address-lookup": (args) => {
      const [frameNumber, displacement, env] = asWords(args);
      let index = envIndex(env ?? nil);
      let f = typeof frameNumber === "number" ? frameNumber : -1;
      if (f < 0) throw new EvaluatorFault("lexical address out of range");
      let frame = state.frames[index];
      while (f > 0) {
        const parent = frame?.parent;
        if (parent === null || parent === undefined)
          throw new EvaluatorFault("lexical address out of range");
        frame = state.frames[parent];
        index = parent;
        f -= 1;
      }
      const binding = [...(frame?.bindings.values() ?? [])][
        typeof displacement === "number" ? displacement : -1
      ];
      if (binding === undefined) throw new EvaluatorFault("lexical address out of range");
      if (isSymbolWord(binding) && binding.symbol === "*unassigned*")
        throw new EvaluatorFault("lexical-address-lookup: the variable is *unassigned*");
      return binding as Value;
    },
    "lexical-address-set!": (args) => {
      const [frameNumber, displacement, value, env] = asWords(args);
      let index = envIndex(env ?? nil);
      let f = typeof frameNumber === "number" ? frameNumber : -1;
      if (f < 0) throw new EvaluatorFault("lexical address out of range");
      let frame = state.frames[index];
      while (f > 0) {
        const parent = frame?.parent;
        if (parent === null || parent === undefined)
          throw new EvaluatorFault("lexical address out of range");
        frame = state.frames[parent];
        index = parent;
        f -= 1;
      }
      const d = typeof displacement === "number" ? displacement : -1;
      const keys = [...(frame?.bindings.keys() ?? [])];
      const key = keys[d];
      if (key === undefined || !frame) throw new EvaluatorFault("lexical address out of range");
      frame.bindings.set(key, value ?? nil);
      return symbol("ok");
    },
  };
  return ops;
};

/** The registers of the 5.5.7 machine: the evaluator's seven plus the
 * `arg1` and `arg2` of exercise 5.38. */
export const machineRegisters: readonly string[] = [...evaluatorRegisters, "arg1", "arg2"];

export interface CompiledEvaluator {
  readonly machine: Machine;
  readonly state: State;
  readonly transcript: readonly string[];
  /** Points `val` at the compiled entry and arms the external entry. */
  readonly armEntry: (entry: string) => void;
  readonly run: () => void;
  readonly stackStatistics: () => { readonly pushes: number; readonly maxDepth: number };
  readonly instructionCount: () => number;
}

export interface CompiledEvaluatorOptions {
  /** Extra machine operations, overriding the compiled table on a
   * name collision. */
  readonly operations?: Readonly<Record<string, Operation>>;
  /** Extra runtime primitives, reached through
   * `apply-primitive-procedure` before the built-in table. */
  readonly runtime?: Readonly<Record<string, RuntimeFn>>;
  /** The step ceiling of the machine's own loop. */
  readonly stepLimit?: number;
}

/** The section's compiled evaluator: the controller (the 5.5.7 text,
 * or a composed variant) is assembled by the 5.2 simulator over the
 * 5.4 base operations and the compiled operations; the global
 * environment binds `true`, `false`, and the runtime primitives; and
 * the object program `source` is read into the driver's input queue.
 * The armed latch starts false, so the plain driver path runs. */
export const makeCompiledEvaluator = (
  controller: readonly ControllerLine[] = ecevalController,
  source = "",
  options: CompiledEvaluatorOptions = {},
): CompiledEvaluator => {
  const state: State = {
    frames: [{ bindings: new Map(), parent: null }],
    input: source === "" ? [] : parse(source),
    output: [],
  };
  const global = state.frames[0];
  if (!global) throw new EvaluatorFault("missing global frame");
  global.bindings.set("true", true);
  global.bindings.set("false", false);
  for (const name of runtimeNames) global.bindings.set(name, primitiveWord(name));
  // The caller's runtime primitives are object procedures: they bind
  // into the global frame like any other primitive name.
  for (const name of Object.keys(options.runtime ?? {}))
    global.bindings.set(name, primitiveWord(name));
  const handle: MachineHandle = { armed: false };
  let steps = 0;
  const machine = makeNewMachine(
    machineRegisters,
    {
      ...baseOperations(state),
      ...compiledOperations(state, handle, options.runtime ?? {}),
      ...(options.operations ?? {}),
    },
    {
      ...(options.stepLimit === undefined ? {} : { stepLimit: options.stepLimit }),
      onInstruction: () => {
        steps += 1;
      },
    },
  );
  handle.machine = machine;
  const assembled = assemble([...controller], machine);
  if (!assembled.ok) throw new EvaluatorFault(JSON.stringify(assembled.error));
  machine.install(assembled.value);
  let ran = false;
  return {
    machine,
    state,
    get transcript() {
      return state.output;
    },
    armEntry: (entry) => {
      const address = machine.labels.get(entry);
      if (address === undefined)
        throw new EvaluatorFault(`no label ${entry} in the assembled machine`);
      const val = machine.registers.get("val");
      if (!val) throw new EvaluatorFault("the machine lacks val");
      val.store(address);
      handle.armed = true;
    },
    run: () => {
      if (ran) return;
      ran = true;
      try {
        const result = machine.start();
        if (!result.ok) throw new EvaluatorFault(JSON.stringify(result.error));
      } catch (e) {
        if (!(e instanceof EvaluatorFault) || e.message !== INPUT_EXHAUSTED) throw e;
      }
    },
    stackStatistics: () => machine.stack.statistics(),
    instructionCount: () => steps,
  };
};

/** The book's `compile-and-go`: compiles `compiledSource`, appends its
 * block to the 5.5.7 controller, points `val` at the entry, arms the
 * external entry, and answers the machine whose driver input is
 * `driverSource`. */
export const compileAndGo = (
  cfg: CompilerConfig,
  state: CompilerState,
  compiledSource: string,
  driverSource = "",
  options: CompiledEvaluatorOptions = {},
): CompiledEvaluator => {
  const { entry, lines } = compileBlock(cfg, state, compiledSource);
  const controller = controllerReplacingDriver(guardedDriver);
  const evaluator = makeCompiledEvaluator([...controller, ...lines], driverSource, options);
  evaluator.armEntry(entry);
  return evaluator;
};

/** The interpreted evaluator on the 5.5.7 machine with the monitored
 * driver: the measurement exercises (5.45, 5.46) read the stack
 * counters from its transcript. */
export const monitoredInterpretedSession = (source: string): readonly string[] => {
  const evaluator = makeCompiledEvaluator(monitoredEcevalController, source);
  evaluator.run();
  return evaluator.transcript;
};
