// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.4

/** The query evaluator for section 4.4. It reuses the chapter's Value and
 * reader surface; logic variables live in query syntax, not in the evaluator's
 * lexical environments. Frames are immutable binding chains and streams memoize
 * their tails, so recursive rules can be fairly interleaved. */
import { Effect } from "effect";
import { isSymbol, symbol } from "./01-metacircular.js";
import type { Value } from "./core.js";
import { RuntimeError } from "./errors.js";
import { type Cons, cons, type List, nil, toArray } from "./list.js";
import { format, read, readAll } from "./read.js";

const DOTTED_TAIL = "__logic_dotted_tail__";
const readQueryDatum = (text: string): Value => read(text.replace(/\s+\.\s+/g, ` ${DOTTED_TAIL} `));
const readProgram = (text: string): ReadonlyArray<Value> =>
  readAll(text.replace(/\s+\.\s+/g, ` ${DOTTED_TAIL} `));

/** Walks two list spines elementwise with the sub-matcher. A dotted-tail
 * marker at any spine position on either side splices against the whole
 * remainder of the other side, the book's dotted-pair matching; with no
 * marker aboard it degrades to car/cdr recursion. */
const matchSpines = (
  sub: (left: Value, right: Value, frame: Frame) => Frame | undefined,
  left: Value,
  right: Value,
  frame: Frame,
): Frame | undefined => {
  let first: Value = left;
  let second: Value = right;
  let current: Frame | undefined = frame;
  for (;;) {
    if (current === undefined) return undefined;
    if (pair(first) && isSymbol(first.head) && first.head.name === DOTTED_TAIL) {
      const rest = first.tail;
      if (!pair(rest) || rest.tail._tag !== "Nil") return undefined;
      return sub(rest.head, second, current);
    }
    if (pair(second) && isSymbol(second.head) && second.head.name === DOTTED_TAIL) {
      const rest = second.tail;
      if (!pair(rest) || rest.tail._tag !== "Nil") return undefined;
      return sub(first, rest.head, current);
    }
    if (!pair(first) || !pair(second)) return sub(first, second, current);
    current = sub(first.head, second.head, current);
    first = first.tail;
    second = second.tail;
  }
};

const pair = (v: Value): v is Cons<Value> => v._tag === "Cons";
export const listValue = (v: Value): List<Value> =>
  v._tag === "Cons" || v._tag === "Nil" ? v : nil;
const cadr = (v: Value): Value => (pair(v) && pair(v.tail) ? v.tail.head : symbol("<malformed>"));
const caddr = (v: Value): Value =>
  pair(v) && pair(v.tail) && pair(v.tail.tail) ? v.tail.tail.head : symbol("<malformed>");
const tagged = (name: string, v: Value): v is Cons<Value> =>
  pair(v) && isSymbol(v.head) && v.head.name === name;
const equal = (a: Value, b: Value): boolean => {
  if (a._tag === "Cons" && b._tag === "Cons") return equal(a.head, b.head) && equal(a.tail, b.tail);
  if (a._tag === "Symbol" && b._tag === "Symbol") return a.name === b.name;
  if (a._tag === "Number" && b._tag === "Number") return a.n === b.n;
  if (a._tag === "String" && b._tag === "String") return a.s === b.s;
  if (a._tag === "Boolean" && b._tag === "Boolean") return a.b === b.b;
  return a._tag === b._tag;
};

/** Internal query variable, the book's (? name) list. */
export const isVar = (v: Value): boolean => tagged("?", v);
const mapList = (items: List<Value>, f: (x: Value) => Value): List<Value> =>
  items._tag === "Nil" ? nil : cons(f(items.head), mapList(items.tail, f));
export const querySyntaxProcess = (v: Value): Value => {
  if (isSymbol(v) && v.name.startsWith("?") && v.name.length > 1)
    return cons(symbol("?"), cons(symbol(v.name.slice(1)), nil));
  if (pair(v)) return cons(querySyntaxProcess(v.head), mapList(v.tail, querySyntaxProcess));
  return v;
};
export const contractQuestionMark = (v: Value): Value => {
  if (!isVar(v) || !pair(v)) return v;
  const id = cadr(v);
  const name = caddr(v);
  if (id._tag === "Number" && isSymbol(name)) return symbol(`?${name.name}-${id.n}`);
  return isSymbol(id) ? symbol(`?${id.name}`) : v;
};
export const makeNewVariable = (v: Value, id: number): Value =>
  isVar(v) && pair(v) ? cons(symbol("?"), cons({ _tag: "Number", n: id }, cons(cadr(v), nil))) : v;
const mapTree = (v: Value, f: (x: Value) => Value): Value =>
  isVar(v)
    ? f(v)
    : pair(v)
      ? cons(
          mapTree(v.head, f),
          mapList(v.tail, (x) => mapTree(x, f)),
        )
      : v;
export const renameVariablesIn = (v: Value, id: number): Value =>
  mapTree(v, (x) => makeNewVariable(x, id));

/** An immutable frame: newest binding first. */
export class Frame {
  readonly bindings: ReadonlyArray<readonly [Value, Value]>;
  constructor(bindings: ReadonlyArray<readonly [Value, Value]> = []) {
    this.bindings = bindings;
  }
  extend(variable: Value, value: Value): Frame {
    return new Frame([[variable, value], ...this.bindings]);
  }
  bindingInFrame(variable: Value): Value | undefined {
    return this.bindings.find(([v]) => equal(v, variable))?.[1];
  }
  get isEmpty(): boolean {
    return this.bindings.length === 0;
  }
}
export const frameOf = (bindings: ReadonlyArray<readonly [Value, Value]> = []): Frame =>
  new Frame(bindings);

/** Memoized lazy stream. */
export class Stream<A> {
  private tailCache: Stream<A> | undefined;
  readonly empty: boolean;
  private readonly value: A | undefined;
  private readonly tailThunk: (() => Stream<A>) | undefined;
  private constructor(empty: boolean, value?: A, tailThunk?: () => Stream<A>) {
    this.empty = empty;
    this.value = value;
    this.tailThunk = tailThunk;
  }
  static empty<A>(): Stream<A> {
    return new Stream<A>(true);
  }
  static cons<A>(head: A, tail: () => Stream<A>): Stream<A> {
    return new Stream(false, head, tail);
  }
  tail(): Stream<A> {
    if (this.empty) return Stream.empty();
    this.tailCache ??= this.tailThunk?.() ?? Stream.empty();
    return this.tailCache;
  }
  get head(): A | undefined {
    return this.value;
  }
  take(n: number): ReadonlyArray<A> {
    const out: A[] = [];
    let s: Stream<A> = this;
    while (n > 0 && !s.empty) {
      if (s.value !== undefined) {
        out.push(s.value);
        n -= 1;
        if (n === 0) break;
      }
      s = s.tail();
    }
    return out;
  }
}
export const singletonStream = <A>(x: A): Stream<A> => Stream.cons(x, () => Stream.empty());
export const streamMap = <A, B>(f: (x: A) => B, s: Stream<A>): Stream<B> => {
  if (s.empty || s.head === undefined) return Stream.empty();
  return Stream.cons(f(s.head), () => streamMap(f, s.tail()));
};
export const streamAppend = <A>(a: Stream<A>, b: Stream<A>): Stream<A> => {
  if (a.empty || a.head === undefined) return b;
  return Stream.cons(a.head, () => streamAppend(a.tail(), b));
};
export const interleave = <A>(a: Stream<A>, b: Stream<A>): Stream<A> => {
  if (a.empty || a.head === undefined) return b;
  return Stream.cons(a.head, () => interleave(b, a.tail()));
};
export const streamAppendDelayed = <A>(a: Stream<A>, b: () => Stream<A>): Stream<A> => {
  if (a.empty || a.head === undefined) return b();
  return Stream.cons(a.head, () => streamAppendDelayed(a.tail(), b));
};
export const interleaveDelayed = <A>(a: Stream<A>, b: () => Stream<A>): Stream<A> => {
  if (a.empty || a.head === undefined) return b();
  return Stream.cons(a.head, () => interleaveDelayed(b(), () => a.tail()));
};
export const flattenStream = <A>(s: Stream<Stream<A>>): Stream<A> => {
  if (s.empty || s.head === undefined) return Stream.empty();
  return interleaveDelayed(s.head, () => flattenStream(s.tail()));
};
export const flattenStreamUndelayed = <A>(s: Stream<Stream<A>>): Stream<A> => {
  if (s.empty || s.head === undefined) return Stream.empty();
  return interleave(s.head, flattenStreamUndelayed(s.tail()));
};
export const streamFlatmap = <A, B>(f: (x: A) => Stream<B>, s: Stream<A>): Stream<B> =>
  flattenStream(streamMap(f, s));
const streamFrom = <A>(items: ReadonlyArray<A>, i = 0): Stream<A> => {
  const value = items[i];
  return value === undefined ? Stream.empty() : Stream.cons(value, () => streamFrom(items, i + 1));
};

export const patternMatch = (pat: Value, dat: Value, frame: Frame): Frame | undefined => {
  if (equal(pat, dat)) return frame;
  if (isVar(pat)) return extendIfConsistent(pat, dat, frame);
  if (pair(pat) && pair(dat)) return matchSpines(patternMatch, pat, dat, frame);
  return undefined;
};
export const extendIfConsistent = (v: Value, dat: Value, frame: Frame): Frame | undefined => {
  const old = frame.bindingInFrame(v);
  return old === undefined ? frame.extend(v, dat) : patternMatch(old, dat, frame);
};
export const dependsOn = (exp: Value, variable: Value, frame: Frame): boolean => {
  if (isVar(exp)) {
    if (equal(exp, variable)) return true;
    const val = frame.bindingInFrame(exp);
    return val !== undefined && dependsOn(val, variable, frame);
  }
  return (
    pair(exp) && (dependsOn(exp.head, variable, frame) || dependsOn(exp.tail, variable, frame))
  );
};
export const unifyMatch = (a: Value, b: Value, frame: Frame): Frame | undefined => {
  if (equal(a, b)) return frame;
  if (isVar(a)) return extendIfPossible(a, b, frame);
  if (isVar(b)) return extendIfPossible(b, a, frame);
  if (pair(a) && pair(b)) return matchSpines(unifyMatch, a, b, frame);
  return undefined;
};
export const extendIfPossible = (v: Value, val: Value, frame: Frame): Frame | undefined => {
  const old = frame.bindingInFrame(v);
  if (old !== undefined) return unifyMatch(old, val, frame);
  if (isVar(val)) {
    const bound = frame.bindingInFrame(val);
    return bound === undefined ? frame.extend(v, val) : unifyMatch(v, bound, frame);
  }
  return dependsOn(val, v, frame) ? undefined : frame.extend(v, val);
};
export const instantiate = (
  exp: Value,
  frame: Frame,
  unbound: (v: Value) => Value = contractQuestionMark,
): Value => {
  if (isVar(exp)) {
    const value = frame.bindingInFrame(exp);
    return value === undefined ? unbound(exp) : instantiate(value, frame, unbound);
  }
  return pair(exp)
    ? cons(
        instantiate(exp.head, frame, unbound),
        mapList(exp.tail, (x) => instantiate(x, frame, unbound)),
      )
    : exp;
};
/** Prints an instantiated query, splicing dotted-tail markers wherever they
 * sit: rule conclusions can nest marker lists inside bound values, so the
 * walk splices each marker's instantiated tail in place and recurses. */
const renderQueryAnswer = (exp: Value, frame: Frame): string => {
  if (isVar(exp)) return renderQueryAnswer(instantiate(exp, frame), frame);
  if (!pair(exp)) return format(exp);
  const items: string[] = [];
  let rest: Value = exp;
  for (;;) {
    if (!pair(rest)) {
      return rest._tag === "Nil"
        ? `(${items.join(" ")})`
        : `(${items.join(" ")} . ${renderQueryAnswer(instantiate(rest, frame), frame)})`;
    }
    const head: Value = rest.head;
    const tail: Value = rest.tail;
    if (isSymbol(head) && head.name === DOTTED_TAIL && pair(tail) && tail.tail._tag === "Nil") {
      rest = instantiate(tail.head, frame);
      continue;
    }
    items.push(renderQueryAnswer(head, frame));
    rest = tail;
  }
};

export const isRule = (v: Value): boolean => tagged("rule", v);
export const conclusion = (rule: Value): Value => cadr(rule);
export const ruleBody = (rule: Value): Value =>
  pair(rule) && pair(rule.tail) && pair(rule.tail.tail)
    ? rule.tail.tail.head
    : read("(always-true)");
export const assertionToBeAdded = (v: Value): boolean => tagged("assert!", v);
export const addAssertionBody = (v: Value): Value => cadr(v);

export type Predicate = (args: ReadonlyArray<Value>) => boolean;
type Processor = (query: Value, frames: Stream<Frame>) => Stream<Frame>;

/** One query engine with an append-ordered, symbol-indexed data base. */
export class QueryEngine {
  private readonly assertions: Value[] = [];
  private readonly rules: Value[] = [];
  private readonly processors = new Map<string, Processor>();
  private predicates = new Map<string, Predicate>();
  private nextRuleId = 0;
  private fallback: Processor | undefined;
  constructor() {
    this.put("and", (q, fs) => this.conjoin(listValue(q), fs));
    this.put("or", (q, fs) => this.disjoin(listValue(q), fs));
    this.put("not", (q, fs) => this.negate(pair(q) ? q.head : symbol("<malformed>"), fs));
    this.put("lisp-value", (q, fs) => this.lispValue(q, fs));
    this.put("always-true", (_q, fs) => fs);
  }
  put(name: string, proc: Processor): void {
    this.processors.set(name, proc);
  }
  /** Replaces the processor untagged patterns take (the book's
   * simple-query): the extension seam for evaluator variants such as
   * 4.71's undelayed query or 4.79's scoped rule application. Named
   * special forms install through put instead (4.75's unique,
   * 4.78's appending disjoin). */
  setFallback(proc: Processor): void {
    this.fallback = proc;
  }
  setPredicates(predicates: Readonly<Record<string, Predicate>>): void {
    this.predicates = new Map(Object.entries(predicates));
  }
  addAssertion(assertion: Value): void {
    if (isRule(assertion)) this.rules.push(querySyntaxProcess(assertion));
    else this.assertions.push(assertion);
  }
  load(source: string | ReadonlyArray<string>): void {
    const forms = typeof source === "string" ? readProgram(source) : source.flatMap(readProgram);
    for (const form of forms) {
      if (assertionToBeAdded(form)) this.addAssertion(addAssertionBody(form));
      else this.addAssertion(form);
    }
  }
  query(q: Value, initial: Frame = new Frame()): Stream<Frame> {
    return this.qeval(querySyntaxProcess(q), singletonStream(initial));
  }
  answers(source: string, limit = 1000): ReadonlyArray<string> {
    const q = readQuery(source);
    return this.query(q)
      .take(limit)
      .map((f) => renderQueryAnswer(q, f));
  }
  private qeval(q: Value, frames: Stream<Frame>): Stream<Frame> {
    if (pair(q) && isSymbol(q.head)) {
      const proc = this.processors.get(q.head.name);
      if (proc) return proc(q.tail, frames);
    }
    return this.fallback ? this.fallback(q, frames) : this.simpleQuery(q, frames);
  }
  private simpleQuery(q: Value, frames: Stream<Frame>): Stream<Frame> {
    return streamFlatmap<Frame, Frame>((frame) => {
      const facts = streamFlatmap<Value, Frame>(
        (a) => {
          const f = patternMatch(q, a, frame);
          return f ? singletonStream(f) : Stream.empty();
        },
        streamFrom(this.findAssertions(q)),
      );
      const rules = streamFlatmap<Value, Frame>(
        (r) => this.applyRule(r, q, frame),
        streamFrom(this.findRules(q)),
      );
      return streamAppendDelayed(facts, () => rules);
    }, frames);
  }
  private findAssertions(q: Value): ReadonlyArray<Value> {
    const key = this.indexKey(q);
    return this.assertions.filter((a) => key === undefined || this.indexKey(a) === key);
  }
  private findRules(q: Value): ReadonlyArray<Value> {
    const key = this.indexKey(q);
    return this.rules.filter((r) => {
      const k = this.indexKey(conclusion(r));
      return k === undefined || key === undefined || k === key;
    });
  }
  private indexKey(v: Value): string | undefined {
    if (isVar(v)) return undefined;
    const h = pair(v) ? v.head : v;
    return isSymbol(h) ? h.name : undefined;
  }
  private applyRule(rule: Value, query: Value, frame: Frame): Stream<Frame> {
    const renamed = renameVariablesIn(rule, ++this.nextRuleId);
    const unified = unifyMatch(query, conclusion(renamed), frame);
    return unified ? this.qeval(ruleBody(renamed), singletonStream(unified)) : Stream.empty();
  }
  private conjoin(clauses: List<Value>, frames: Stream<Frame>): Stream<Frame> {
    let out = frames;
    for (const clause of toArray(clauses)) out = this.qeval(clause, out);
    return out;
  }
  private disjoin(clauses: List<Value>, frames: Stream<Frame>): Stream<Frame> {
    const items = toArray(clauses);
    const walk = (i: number): Stream<Frame> => {
      const query = items[i];
      if (query === undefined) return Stream.empty();
      return interleaveDelayed(this.qeval(query, frames), () => walk(i + 1));
    };
    return walk(0);
  }
  private negate(q: Value, frames: Stream<Frame>): Stream<Frame> {
    return streamFlatmap<Frame, Frame>(
      (f) => (this.qeval(q, singletonStream(f)).empty ? singletonStream(f) : Stream.empty()),
      frames,
    );
  }
  private lispValue(call: Value, frames: Stream<Frame>): Stream<Frame> {
    return streamFlatmap<Frame, Frame>((f) => {
      const exp = instantiate(call, f, (v) => {
        throw new Error(`Unknown pat var -- LISP-VALUE: ${format(contractQuestionMark(v))}`);
      });
      const items = toArray(listValue(exp));
      const name = items[0];
      if (name === undefined || !isSymbol(name)) return Stream.empty();
      const pred = this.predicates.get(name.name);
      if (!pred) return Stream.empty();
      return pred(items.slice(1)) ? singletonStream(f) : Stream.empty();
    }, frames);
  }
}
export const makeQueryEngine = (): QueryEngine => new QueryEngine();
/** Runs the book's line-oriented query driver over a finite input session. */
export const queryDriverLoop = (
  engine: QueryEngine,
  inputs: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, RuntimeError> =>
  Effect.try({
    try: () => {
      const output: string[] = [];
      for (const input of inputs) {
        const form = readProgram(input)[0];
        if (form === undefined) continue;
        if (assertionToBeAdded(form)) {
          engine.addAssertion(addAssertionBody(form));
          output.push(";;; Query input:", input, "Assertion added to data base.");
        } else {
          output.push(";;; Query input:", input, ";;; Query results:");
          output.push(...engine.answers(input).map((answer) => `  ${answer}`));
        }
      }
      return output;
    },
    catch: (error) =>
      new RuntimeError({
        message: "query driver failed",
        detail: error instanceof Error ? error.message : String(error),
      }),
  });

/** Query syntax helper: reads and expands variables. */
export const readQuery = (text: string): Value => querySyntaxProcess(readQueryDatum(text));
export const queryFrames = (engine: QueryEngine, query: string): Stream<Frame> =>
  engine.query(readQuery(query));
export const answers = (engine: QueryEngine, query: string, limit = 1000): ReadonlyArray<string> =>
  engine.answers(query, limit);
export const microshaftDatabase = `
(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)) (job (Bitdiddle Ben) (computer wizard)) (salary (Bitdiddle Ben) 60000)
(address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)) (job (Hacker Alyssa P) (computer programmer)) (salary (Hacker Alyssa P) 40000) (supervisor (Hacker Alyssa P) (Bitdiddle Ben))
(address (Fect Cy D) (Cambridge (Ames Street) 3)) (job (Fect Cy D) (computer programmer)) (salary (Fect Cy D) 35000) (supervisor (Fect Cy D) (Bitdiddle Ben))
(address (Tweakit Lem E) (Boston (Bay State Road) 22)) (job (Tweakit Lem E) (computer technician)) (salary (Tweakit Lem E) 25000) (supervisor (Tweakit Lem E) (Bitdiddle Ben))
(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)) (job (Reasoner Louis) (computer programmer trainee)) (salary (Reasoner Louis) 30000) (supervisor (Reasoner Louis) (Hacker Alyssa P))
(supervisor (Bitdiddle Ben) (Warbucks Oliver)) (address (Warbucks Oliver) (Swellesley (Top Heap Road))) (job (Warbucks Oliver) (administration big wheel)) (salary (Warbucks Oliver) 150000)
(address (Scrooge Eben) (Weston (Shady Lane) 10)) (job (Scrooge Eben) (accounting chief accountant)) (salary (Scrooge Eben) 75000) (supervisor (Scrooge Eben) (Warbucks Oliver))
(address (Cratchet Robert) (Allston (N Harvard Street) 16)) (job (Cratchet Robert) (accounting scrivener)) (salary (Cratchet Robert) 18000) (supervisor (Cratchet Robert) (Scrooge Eben))
(address (Aull DeWitt) (Slumerville (Onion Square) 5)) (job (Aull DeWitt) (administration secretary)) (salary (Aull DeWitt) 25000) (supervisor (Aull DeWitt) (Warbucks Oliver))
(can-do-job (computer wizard) (computer programmer)) (can-do-job (computer wizard) (computer technician)) (can-do-job (computer programmer) (computer programmer trainee)) (can-do-job (administration secretary) (administration big wheel))
`;
export const microshaft = (): QueryEngine => {
  const engine = makeQueryEngine();
  engine.load(microshaftDatabase);
  return engine;
};
export const runQueryText = (source: string, limit = 1000): ReadonlyArray<string> => {
  const engine = microshaft();
  engine.load(source);
  return engine.answers(source, limit);
};
export const queryError = (message: string): Effect.Effect<never, RuntimeError> =>
  Effect.fail(new RuntimeError({ message, detail: "" }));
