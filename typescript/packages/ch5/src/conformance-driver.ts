// SPDX-License-Identifier: GPL-3.0-only

/**
 * The TypeScript host-subsets conformance driver: one observation per run,
 * printed as a single JSON object on stdout. Engines named in the manifest
 * dispatch to the published teaching engines; `reference` dispatches to an
 * independent implementation of the case family's declared semantics that
 * never calls a teaching engine:
 *
 * - lazy: the case is desugared along the shared AST (each `delay` becomes a
 *   memoizing thunk object, each `force` a demand on one) and run natively;
 * - amb: the case is desugared the same way (each `choose` becomes a replayed
 *   choice point, each `require` a failure signal) and searched depth-first
 *   by deterministic re-execution along recorded choice paths;
 * - query: the case's typed Query data is answered by a separate SICP 4.4.4
 *   query evaluator over memoized streams;
 * - machine: the case's typed controller data is run by a separate register
 *   machine simulator.
 *
 * Query and machine cases are host data (grammar section 7): their source is
 * a native module exporting typed values, imported here at the boundary.
 * Engines run on a worker thread with an enlarged stack so deep non-tail
 * recursion measures the engine, not the host's default stack.
 *
 * Usage: conformance-driver --case <case-id> --engine <engine> --source <path> --work <dir>
 * Exits zero only when an observation is reported; diagnostics go to stderr.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";
import { runAnalyzedSource, runSource } from "@sicp-ts/ch4/01-metacircular";
import { runLazySource } from "@sicp-ts/ch4/02-lazy";
import { runAmbSource } from "@sicp-ts/ch4/03-nondeterministic";
import {
  makeDatabase,
  type Query,
  queryDriverLoop,
  type Rule,
  type Term,
} from "@sicp-ts/ch4/04-logic";
import type { Expr } from "@sicp-ts/ch4/syntax/ast";
import { type ExperimentMode, parseExperiment } from "@sicp-ts/ch4/syntax/parse";
import type { MachineStatement, MachineValue, Operation, Source } from "./01-register-machines.ts";
import { makeMachine } from "./02-simulator.ts";
import { runEvaluator } from "./04-eceval.ts";
import { compileAndRun } from "./05-compilation.ts";

interface Observation {
  readonly termination: "value" | "error" | "rejected";
  readonly stdout: string;
}

interface Arguments {
  readonly caseId: string;
  readonly engine: string;
  readonly source: string;
  readonly work: string;
}

/** A driver-level failure: the run cannot produce an observation. */
class DriverFailure extends Error {}

const parseArguments = (argv: ReadonlyArray<string>): Arguments => {
  const read = (name: string): string => {
    const at = argv.indexOf(`--${name}`);
    return at >= 0 ? (argv[at + 1] ?? "") : "";
  };
  return {
    caseId: read("case"),
    engine: read("engine"),
    source: read("source"),
    work: read("work"),
  };
};

/** Each transcript line is one `console.log` write, so it ends in a newline exactly as the guest wrote it. */
const transcriptText = (lines: ReadonlyArray<string>): string =>
  lines.map((line) => `${line}\n`).join("");

const observationOf = (
  outcome: { readonly tag: string },
  transcript: ReadonlyArray<string>,
): Observation => ({
  termination:
    outcome.tag === "ok" ? "value" : outcome.tag === "unknown-syntax" ? "rejected" : "error",
  stdout: transcriptText(transcript),
});

// ---------------------------------------------------------------------
// Boundary narrowing of imported host data (grammar sections 2 and 7).
// ---------------------------------------------------------------------

type Fields = Readonly<Record<string, unknown>>;

const isFields = (value: unknown): value is Fields =>
  typeof value === "object" && value !== null && !Array.isArray(value);

const fieldsOf = (value: unknown, what: string): Fields => {
  if (!isFields(value)) {
    throw new DriverFailure(`${what}: expected a record`);
  }
  return value;
};

const listOf = (value: unknown, what: string): ReadonlyArray<unknown> => {
  if (!Array.isArray(value)) {
    throw new DriverFailure(`${what}: expected an array`);
  }
  return value;
};

const textOf = (value: unknown, what: string): string => {
  if (typeof value !== "string") {
    throw new DriverFailure(`${what}: expected a string`);
  }
  return value;
};

const toTerm = (value: unknown): Term => {
  const fields = fieldsOf(value, "term");
  switch (fields["tag"]) {
    case "var":
      return { tag: "var", name: textOf(fields["name"], "var name") };
    case "text": {
      const literal = fields["value"];
      if (
        typeof literal !== "string" &&
        typeof literal !== "number" &&
        typeof literal !== "boolean"
      ) {
        throw new DriverFailure("text term: expected a string, number, or boolean");
      }
      return { tag: "text", value: literal };
    }
    case "cons":
      return { tag: "cons", head: toTerm(fields["head"]), tail: toTerm(fields["tail"]) };
    case "nil":
      return { tag: "nil" };
    default:
      throw new DriverFailure(`term: unknown tag ${String(fields["tag"])}`);
  }
};

const toQuery = (value: unknown): Query => {
  const fields = fieldsOf(value, "query");
  switch (fields["tag"]) {
    case "atom":
      return {
        tag: "atom",
        relation: textOf(fields["relation"], "relation"),
        fields: listOf(fields["fields"], "atom fields").map(toTerm),
      };
    case "and":
    case "or":
      return { tag: fields["tag"], clauses: listOf(fields["clauses"], "clauses").map(toQuery) };
    case "not":
      return { tag: "not", clause: toQuery(fields["clause"]) };
    default:
      throw new DriverFailure(`query: unsupported tag ${String(fields["tag"])}`);
  }
};

interface QueryProgram {
  readonly assertions: ReadonlyArray<Query>;
  readonly rules: ReadonlyArray<Rule>;
  readonly queries: ReadonlyArray<Query>;
}

const toQueryProgram = (value: unknown): QueryProgram => {
  const fields = fieldsOf(value, "program");
  return {
    assertions: listOf(fields["assertions"], "assertions").map(toQuery),
    rules: listOf(fields["rules"], "rules").map((entry) => {
      const rule = fieldsOf(entry, "rule");
      return { head: toQuery(rule["head"]), body: listOf(rule["body"], "rule body").map(toQuery) };
    }),
    queries: listOf(fields["queries"], "queries").map(toQuery),
  };
};

/** One machine word crossing the boundary: the case's operations compute over numbers and booleans. */
type Word = number | boolean | undefined;

const toWord = (value: unknown, what: string): Word => {
  if (value !== undefined && typeof value !== "number" && typeof value !== "boolean") {
    throw new DriverFailure(`${what}: expected a number, boolean, or undefined`);
  }
  return value;
};

const toSource = (value: unknown): Source => {
  const fields = fieldsOf(value, "source");
  switch (fields["tag"]) {
    case "reg":
      return { tag: "reg", name: textOf(fields["name"], "register") };
    case "const":
      return { tag: "const", value: toWord(fields["value"], "constant") };
    case "label":
      return { tag: "label", name: textOf(fields["name"], "label") };
    case "op":
      return {
        tag: "op",
        operation: textOf(fields["operation"], "operation"),
        args: listOf(fields["args"], "operation args").map(toSource),
      };
    default:
      throw new DriverFailure(`source: unknown tag ${String(fields["tag"])}`);
  }
};

const toStatement = (value: unknown): MachineStatement => {
  const fields = fieldsOf(value, "statement");
  const tag = fields["tag"];
  switch (tag) {
    case "label":
      return { tag, name: textOf(fields["name"], "label") };
    case "assign":
      return {
        tag,
        register: textOf(fields["register"], "register"),
        source: toSource(fields["source"]),
      };
    case "test":
    case "perform":
      return {
        tag,
        operation: textOf(fields["operation"], "operation"),
        args: listOf(fields["args"], "args").map(toSource),
      };
    case "branch":
    case "goto-label":
      return { tag, label: textOf(fields["label"], "label") };
    case "goto-register":
    case "save":
    case "restore":
      return { tag, register: textOf(fields["register"], "register") };
    default:
      throw new DriverFailure(`statement: unknown tag ${String(tag)}`);
  }
};

type HostOperation = (args: ReadonlyArray<unknown>) => unknown;

interface MachineProgram {
  readonly registers: ReadonlyArray<string>;
  readonly operations: ReadonlyMap<string, HostOperation>;
  readonly controller: ReadonlyArray<MachineStatement>;
  readonly inputs: ReadonlyArray<readonly [string, number]>;
  readonly outputs: ReadonlyArray<string>;
}

const toMachineProgram = (value: unknown): MachineProgram => {
  const fields = fieldsOf(value, "machine");
  const operations = new Map<string, HostOperation>();
  for (const [name, operation] of Object.entries(fieldsOf(fields["operations"], "operations"))) {
    if (typeof operation !== "function") {
      throw new DriverFailure(`operation ${name}: expected a function`);
    }
    operations.set(name, (args) => Reflect.apply(operation, undefined, [args]));
  }
  return {
    registers: listOf(fields["registers"], "registers").map((name) => textOf(name, "register")),
    operations,
    controller: listOf(fields["controller"], "controller").map(toStatement),
    inputs: listOf(fields["inputs"], "inputs").map((entry) => {
      const pair = listOf(entry, "input");
      const initial = pair[1];
      if (typeof initial !== "number") {
        throw new DriverFailure("input: expected a number");
      }
      return [textOf(pair[0], "input register"), initial] as const;
    }),
    outputs: listOf(fields["outputs"], "outputs").map((name) => textOf(name, "output register")),
  };
};

/** Case modules are named by the checker at run time, so a static import cannot name them. */
const importCase = async (source: string, exportName: string): Promise<unknown> => {
  const module: unknown = await import(pathToFileURL(source).href);
  return fieldsOf(module, "case module")[exportName];
};

const loadMachine = async (
  source: string,
  emit: (line: string) => void,
): Promise<MachineProgram> => {
  const factory = await importCase(source, "machine");
  if (typeof factory !== "function") {
    throw new DriverFailure("machine case: expected an exported `machine` factory");
  }
  const built: unknown = Reflect.apply(factory, undefined, [emit]);
  return toMachineProgram(built);
};

const renderWord = (value: unknown): string =>
  value === undefined || typeof value === "number" || typeof value === "boolean"
    ? String(value)
    : JSON.stringify(value);

// ---------------------------------------------------------------------
// Reference: lazy and amb by AST-directed desugaring and native execution.
// ---------------------------------------------------------------------

const EXTENSION_TAGS = new Set([
  "delay",
  "force",
  "choose",
  "require",
  "ramb",
  "permanent-assign",
  "if-fail",
]);

type Extension = Extract<Expr, { tag: "delay" | "force" | "choose" | "require" }>;

const isExtension = (value: Fields): boolean =>
  typeof value["tag"] === "string" && EXTENSION_TAGS.has(value["tag"]);

/** The outermost extension nodes inside `node`, in source order. */
const outermostExtensions = (node: unknown): Fields[] => {
  const found: Fields[] = [];
  const visit = (value: unknown): void => {
    if (Array.isArray(value)) {
      value.forEach(visit);
      return;
    }
    if (!isFields(value)) {
      return;
    }
    if (isExtension(value)) {
      found.push(value);
      return;
    }
    Object.values(value).forEach(visit);
  };
  visit(node);
  return found.sort((a, b) => spanStart(a) - spanStart(b));
};

const spanOfNode = (node: Fields): { readonly start: number; readonly end: number } => {
  const span = fieldsOf(node["span"], "span");
  const start = span["start"];
  const end = span["end"];
  if (typeof start !== "number" || typeof end !== "number") {
    throw new DriverFailure("span: expected numeric offsets");
  }
  return { start, end };
};

const spanStart = (node: Fields): number => spanOfNode(node).start;

/** Source text of `node` with every extension form inside it rewritten to its runtime call. */
const desugar = (text: string, node: Fields): string => {
  if (isExtension(node)) {
    return rewriteExtension(text, node);
  }
  const { start, end } = spanOfNode(node);
  let out = "";
  let at = start;
  for (const inner of outermostExtensions(Object.values(node))) {
    const span = spanOfNode(inner);
    out += text.slice(at, span.start) + rewriteExtension(text, inner);
    at = span.end;
  }
  return out + text.slice(at, end);
};

const rewriteExtension = (text: string, node: Fields): string => {
  const tag = node["tag"];
  const operand = (key: string): string =>
    desugar(text, fieldsOf(node[key], `${String(tag)} operand`));
  switch (tag) {
    case "delay":
      return `__reference.delay(() => (${operand("expr")}))`;
    case "force":
      return `__reference.force(${operand("expr")})`;
    case "require":
      return `__reference.require(${operand("condition")})`;
    case "choose": {
      const alternatives = listOf(node["alternatives"], "alternatives").map(
        (alternative) => `() => (${desugar(text, fieldsOf(alternative, "alternative"))})`,
      );
      return `__reference.choose([${alternatives.join(", ")}])`;
    }
    default:
      throw new DriverFailure(`reference model does not cover the ${String(tag)} extension`);
  }
};

/** Desugars a whole experiment unit into a native module exporting `__program(__reference)`. */
const loadDesugared = async (
  text: string,
  mode: ExperimentMode,
  work: string,
): Promise<(runtime: unknown) => void> => {
  const program = parseExperiment(text, mode);
  let body = "";
  let at = 0;
  for (const node of outermostExtensions(program)) {
    const span = spanOfNode(node);
    body += text.slice(at, span.start) + rewriteExtension(text, node);
    at = span.end;
  }
  body += text.slice(at);
  const path = join(work, `reference-${mode}.mts`);
  writeFileSync(path, `export const __program = (__reference) => {\n${body}\n};\n`);
  const entry = await importCase(path, "__program");
  if (typeof entry !== "function") {
    throw new DriverFailure("desugared module did not export its program");
  }
  return (runtime) => {
    Reflect.apply(entry, undefined, [runtime]);
  };
};

/** Runs `body` with `console.log` writes captured through `sink`. */
const capturingConsole = (sink: (line: string) => void, body: () => void): void => {
  const original = console.log;
  console.log = (...values: unknown[]): void => {
    sink(values.map((value) => (typeof value === "string" ? value : String(value))).join(" "));
  };
  try {
    body();
  } finally {
    console.log = original;
  }
};

class Thunk {
  readonly compute: () => unknown;
  forced = false;
  value: unknown = undefined;

  constructor(compute: () => unknown) {
    this.compute = compute;
  }
}

const lazyReference = async (text: string, work: string): Promise<Observation> => {
  const run = await loadDesugared(text, "lazy-memoized-experiment", work);
  const runtime = {
    delay: (compute: () => unknown): Thunk => new Thunk(compute),
    force: (value: unknown): unknown => {
      if (!(value instanceof Thunk)) {
        throw new DriverFailure("force of a value that is not a delayed thunk");
      }
      if (!value.forced) {
        value.value = value.compute();
        value.forced = true;
      }
      return value.value;
    },
  };
  const lines: string[] = [];
  try {
    capturingConsole(
      (line) => lines.push(line),
      () => run(runtime),
    );
  } catch {
    return { termination: "error", stdout: transcriptText(lines) };
  }
  return { termination: "value", stdout: transcriptText(lines) };
};

/** Signals a failed `require` or an exhausted `choose` on the current path. */
class SearchFailure {}

interface ChoicePoint {
  index: number;
  readonly arity: number;
}

const searchReference = async (text: string, work: string): Promise<Observation> => {
  const run = await loadDesugared(text, "amb-depth-first-experiment", work);
  const lines: string[] = [];
  const path: ChoicePoint[] = [];
  // The choice point this run resumes: effects before it already happened on
  // the path that reached it, so they are replayed silently.
  let resumeAt = -1;
  for (;;) {
    let position = 0;
    let emitting = resumeAt < 0;
    const runtime = {
      choose: (alternatives: ReadonlyArray<() => unknown>): unknown => {
        const at = position;
        position += 1;
        if (at === resumeAt) {
          emitting = true;
        }
        if (alternatives.length === 0) {
          throw new SearchFailure();
        }
        const recorded = path[at];
        if (recorded === undefined) {
          path.push({ index: 0, arity: alternatives.length });
        }
        const chosen = alternatives[recorded?.index ?? 0];
        if (chosen === undefined) {
          throw new DriverFailure("replayed choice point changed arity");
        }
        return chosen();
      },
      require: (condition: unknown): void => {
        if (condition !== true) {
          throw new SearchFailure();
        }
      },
    };
    try {
      capturingConsole(
        (line) => {
          if (emitting) {
            lines.push(line);
          }
        },
        () => run(runtime),
      );
    } catch (error) {
      if (!(error instanceof SearchFailure)) {
        return { termination: "error", stdout: transcriptText(lines) };
      }
    }
    path.length = position;
    for (;;) {
      const last = path[path.length - 1];
      if (last === undefined || last.index + 1 < last.arity) {
        break;
      }
      path.pop();
    }
    const last = path[path.length - 1];
    if (last === undefined) {
      return { termination: "value", stdout: transcriptText(lines) };
    }
    last.index += 1;
    resumeAt = path.length - 1;
  }
};

// ---------------------------------------------------------------------
// Reference: the SICP 4.4.4 query evaluator over memoized streams.
// ---------------------------------------------------------------------

type Stream<A> = { readonly head: A; readonly tail: () => Stream<A> } | null;
type Frame = ReadonlyMap<string, Term>;

const streamCons = <A>(head: A, tail: () => Stream<A>): Stream<A> => {
  let forced = false;
  let rest: Stream<A> = null;
  return {
    head,
    tail: () => {
      if (!forced) {
        rest = tail();
        forced = true;
      }
      return rest;
    },
  };
};

const streamOf = <A>(items: ReadonlyArray<A>, from = 0): Stream<A> => {
  const item = items[from];
  return from >= items.length || item === undefined
    ? null
    : streamCons(item, () => streamOf(items, from + 1));
};

const streamAppendDelayed = <A>(first: Stream<A>, second: () => Stream<A>): Stream<A> =>
  first === null
    ? second()
    : streamCons(first.head, () => streamAppendDelayed(first.tail(), second));

const interleaveDelayed = <A>(first: Stream<A>, second: () => Stream<A>): Stream<A> =>
  first === null ? second() : streamCons(first.head, () => interleaveDelayed(second(), first.tail));

const streamMap = <A, B>(f: (item: A) => B, stream: Stream<A>): Stream<B> =>
  stream === null ? null : streamCons(f(stream.head), () => streamMap(f, stream.tail()));

const flattenStream = <A>(streams: Stream<Stream<A>>): Stream<A> =>
  streams === null ? null : interleaveDelayed(streams.head, () => flattenStream(streams.tail()));

const streamFlatmap = <A, B>(f: (item: A) => Stream<B>, stream: Stream<A>): Stream<B> =>
  flattenStream(streamMap(f, stream));

const streamItems = <A>(stream: Stream<A>): A[] => {
  const items: A[] = [];
  for (let at = stream; at !== null; at = at.tail()) {
    items.push(at.head);
  }
  return items;
};

const termsEqual = (a: Term, b: Term): boolean => {
  switch (a.tag) {
    case "var":
      return b.tag === "var" && a.name === b.name;
    case "text":
      return b.tag === "text" && a.value === b.value;
    case "nil":
      return b.tag === "nil";
    case "cons":
      return b.tag === "cons" && termsEqual(a.head, b.head) && termsEqual(a.tail, b.tail);
  }
};

const extend = (name: string, value: Term, frame: Frame): Frame => new Map(frame).set(name, value);

const patternMatch = (pattern: Term, datum: Term, frame: Frame | undefined): Frame | undefined => {
  if (frame === undefined) {
    return undefined;
  }
  if (termsEqual(pattern, datum)) {
    return frame;
  }
  if (pattern.tag === "var") {
    const bound = frame.get(pattern.name);
    return bound === undefined
      ? extend(pattern.name, datum, frame)
      : patternMatch(bound, datum, frame);
  }
  if (pattern.tag === "cons" && datum.tag === "cons") {
    return patternMatch(pattern.tail, datum.tail, patternMatch(pattern.head, datum.head, frame));
  }
  return undefined;
};

const dependsOn = (term: Term, name: string, frame: Frame): boolean => {
  if (term.tag === "var") {
    if (term.name === name) {
      return true;
    }
    const bound = frame.get(term.name);
    return bound !== undefined && dependsOn(bound, name, frame);
  }
  return (
    term.tag === "cons" && (dependsOn(term.head, name, frame) || dependsOn(term.tail, name, frame))
  );
};

const extendIfPossible = (name: string, value: Term, frame: Frame): Frame | undefined => {
  const bound = frame.get(name);
  if (bound !== undefined) {
    return unifyMatch(bound, value, frame);
  }
  if (value.tag === "var") {
    const valueBinding = frame.get(value.name);
    if (valueBinding !== undefined) {
      return unifyMatch({ tag: "var", name }, valueBinding, frame);
    }
  }
  return dependsOn(value, name, frame) ? undefined : extend(name, value, frame);
};

const unifyMatch = (left: Term, right: Term, frame: Frame | undefined): Frame | undefined => {
  if (frame === undefined) {
    return undefined;
  }
  if (termsEqual(left, right)) {
    return frame;
  }
  if (left.tag === "var") {
    return extendIfPossible(left.name, right, frame);
  }
  if (right.tag === "var") {
    return extendIfPossible(right.name, left, frame);
  }
  if (left.tag === "cons" && right.tag === "cons") {
    return unifyMatch(left.tail, right.tail, unifyMatch(left.head, right.head, frame));
  }
  return undefined;
};

/** Atomic queries as the book's list data: relation symbol followed by the fields. */
const atomTerm = (query: Extract<Query, { tag: "atom" }>): Term =>
  query.fields.reduceRight<Term>((tail, head) => ({ tag: "cons", head, tail }), { tag: "nil" });

class QueryReference {
  readonly #assertions: ReadonlyArray<Query>;
  readonly #rules: ReadonlyArray<Rule>;
  #applications = 0;

  constructor(program: QueryProgram) {
    this.#assertions = program.assertions;
    this.#rules = program.rules;
  }

  qeval(query: Query, frames: Stream<Frame>): Stream<Frame> {
    switch (query.tag) {
      case "atom":
        return streamFlatmap(
          (frame) =>
            streamAppendDelayed(this.#findAssertions(query, frame), () =>
              this.#applyRules(query, frame),
            ),
          frames,
        );
      case "and":
        return query.clauses.reduce((stream, clause) => this.qeval(clause, stream), frames);
      case "or":
        return this.#disjoin(query.clauses, 0, frames);
      case "not":
        return streamFlatmap(
          (frame) =>
            this.qeval(query.clause, streamOf([frame])) === null ? streamOf([frame]) : null,
          frames,
        );
      case "unique":
      case "lisp-value":
      case "always-true":
        throw new DriverFailure(`reference model does not cover ${query.tag}`);
    }
  }

  #disjoin(clauses: ReadonlyArray<Query>, from: number, frames: Stream<Frame>): Stream<Frame> {
    const clause = clauses[from];
    return clause === undefined
      ? null
      : interleaveDelayed(this.qeval(clause, frames), () =>
          this.#disjoin(clauses, from + 1, frames),
        );
  }

  #findAssertions(pattern: Extract<Query, { tag: "atom" }>, frame: Frame): Stream<Frame> {
    return streamFlatmap((assertion) => {
      if (
        assertion.tag !== "atom" ||
        assertion.relation !== pattern.relation ||
        assertion.fields.length !== pattern.fields.length
      ) {
        return null;
      }
      const matched = patternMatch(atomTerm(pattern), atomTerm(assertion), frame);
      return matched === undefined ? null : streamOf([matched]);
    }, streamOf(this.#assertions));
  }

  #applyRules(pattern: Extract<Query, { tag: "atom" }>, frame: Frame): Stream<Frame> {
    return streamFlatmap((rule) => {
      const head = rule.head;
      if (
        head.tag !== "atom" ||
        head.relation !== pattern.relation ||
        head.fields.length !== pattern.fields.length
      ) {
        return null;
      }
      this.#applications += 1;
      const suffix = `#${this.#applications}`;
      const rename = (term: Term): Term => {
        switch (term.tag) {
          case "var":
            return { tag: "var", name: term.name + suffix };
          case "cons":
            return { tag: "cons", head: rename(term.head), tail: rename(term.tail) };
          default:
            return term;
        }
      };
      const renameQuery = (query: Query): Query => {
        switch (query.tag) {
          case "atom":
            return { tag: "atom", relation: query.relation, fields: query.fields.map(rename) };
          case "and":
          case "or":
            return { tag: query.tag, clauses: query.clauses.map(renameQuery) };
          case "not":
            return { tag: "not", clause: renameQuery(query.clause) };
          default:
            return query;
        }
      };
      const unified = unifyMatch(
        atomTerm(pattern),
        atomTerm({ tag: "atom", relation: head.relation, fields: head.fields.map(rename) }),
        frame,
      );
      if (unified === undefined) {
        return null;
      }
      return this.qeval({ tag: "and", clauses: rule.body.map(renameQuery) }, streamOf([unified]));
    }, streamOf(this.#rules));
  }
}

const instantiate = (term: Term, frame: Frame): Term => {
  if (term.tag === "var") {
    const bound = frame.get(term.name);
    return bound === undefined ? term : instantiate(bound, frame);
  }
  return term.tag === "cons"
    ? { tag: "cons", head: instantiate(term.head, frame), tail: instantiate(term.tail, frame) }
    : term;
};

const renderTerm = (term: Term): string => {
  switch (term.tag) {
    case "var":
      return `?${term.name}`;
    case "text":
      return typeof term.value === "string" ? JSON.stringify(term.value) : String(term.value);
    case "nil":
      return "[]";
    case "cons": {
      const items: string[] = [];
      let at: Term = term;
      while (at.tag === "cons") {
        items.push(renderTerm(at.head));
        at = at.tail;
      }
      return at.tag === "nil"
        ? `[${items.join(", ")}]`
        : `[${items.join(", ")} | ${renderTerm(at)}]`;
    }
  }
};

const renderQuery = (query: Query, frame: Frame): string => {
  switch (query.tag) {
    case "atom":
      return `${query.relation}(${query.fields.map((field) => renderTerm(instantiate(field, frame))).join(", ")})`;
    case "and":
    case "or":
      return `${query.tag}(${query.clauses.map((clause) => renderQuery(clause, frame)).join(", ")})`;
    case "not":
    case "unique":
      return `${query.tag}(${renderQuery(query.clause, frame)})`;
    case "lisp-value":
    case "always-true":
      throw new DriverFailure(`reference model does not cover ${query.tag}`);
  }
};

const queryReference = (program: QueryProgram): Observation => {
  const evaluator = new QueryReference(program);
  const lines: string[] = [];
  for (const query of program.queries) {
    lines.push(renderQuery(query, new Map()));
    const answers = streamItems(evaluator.qeval(query, streamOf<Frame>([new Map()])));
    if (answers.length === 0) {
      lines.push("No.");
    }
    for (const frame of answers) {
      lines.push(renderQuery(query, frame));
    }
  }
  return { termination: "value", stdout: transcriptText(lines) };
};

// ---------------------------------------------------------------------
// Reference: a register-machine simulator (SICP 5.2).
// ---------------------------------------------------------------------

/** A label address held in a register: an index into the label-free instruction list. */
class Address {
  readonly index: number;

  constructor(index: number) {
    this.index = index;
  }
}

const MACHINE_STEP_LIMIT = 10_000_000;

const machineReference = (program: MachineProgram, lines: string[]): Observation => {
  const instructions: MachineStatement[] = [];
  const labels = new Map<string, number>();
  for (const statement of program.controller) {
    if (statement.tag === "label") {
      if (labels.has(statement.name)) {
        return { termination: "error", stdout: transcriptText(lines) };
      }
      labels.set(statement.name, instructions.length);
    } else {
      instructions.push(statement);
    }
  }
  const registers = new Map<string, unknown>(program.registers.map((name) => [name, undefined]));
  for (const [name, value] of program.inputs) {
    if (!registers.has(name)) {
      return { termination: "error", stdout: transcriptText(lines) };
    }
    registers.set(name, value);
  }
  const stack: unknown[] = [];
  let pushes = 0;
  let maxDepth = 0;
  let flag = false;
  let pc = 0;
  const fault = (): never => {
    throw new DriverFailure("machine fault");
  };
  const address = (name: string): number => labels.get(name) ?? fault();
  const readRegister = (name: string): unknown =>
    registers.has(name) ? registers.get(name) : fault();
  const writeRegister = (name: string, value: unknown): void => {
    if (!registers.has(name)) {
      fault();
    }
    registers.set(name, value);
  };
  const value = (source: Source): unknown => {
    switch (source.tag) {
      case "reg":
        return readRegister(source.name);
      case "const":
        return source.value;
      case "label":
        return new Address(address(source.name));
      case "op":
        return apply(source.operation, source.args);
    }
  };
  const apply = (name: string, args: ReadonlyArray<Source>): unknown => {
    const operation = program.operations.get(name) ?? fault();
    return operation(args.map(value));
  };
  try {
    for (let steps = 0; pc < instructions.length; steps += 1) {
      if (steps >= MACHINE_STEP_LIMIT) {
        fault();
      }
      const instruction = instructions[pc] ?? fault();
      pc += 1;
      switch (instruction.tag) {
        case "assign":
          writeRegister(instruction.register, value(instruction.source));
          break;
        case "test":
          flag = apply(instruction.operation, instruction.args) === true;
          break;
        case "perform":
          apply(instruction.operation, instruction.args);
          break;
        case "branch":
          if (flag) {
            pc = address(instruction.label);
          }
          break;
        case "goto-label":
          pc = address(instruction.label);
          break;
        case "goto-register": {
          const target = readRegister(instruction.register);
          pc = target instanceof Address ? target.index : fault();
          break;
        }
        case "save":
          stack.push(readRegister(instruction.register));
          pushes += 1;
          maxDepth = Math.max(maxDepth, stack.length);
          break;
        case "restore":
          if (stack.length === 0) {
            fault();
          }
          writeRegister(instruction.register, stack.pop());
          break;
        case "label":
          break;
      }
    }
  } catch (error) {
    if (error instanceof DriverFailure) {
      return { termination: "error", stdout: transcriptText(lines) };
    }
    throw error;
  }
  for (const name of program.outputs) {
    lines.push(`${name} = ${renderWord(registers.get(name))}`);
  }
  lines.push(`pushes ${pushes}`, `max-depth ${maxDepth}`);
  return { termination: "value", stdout: transcriptText(lines) };
};

const machineTeaching = (program: MachineProgram, lines: string[]): Observation => {
  const operations: Record<string, Operation> = {};
  for (const [name, operation] of program.operations) {
    operations[name] = (args): MachineValue => toWord(operation(args), `operation ${name} result`);
  }
  const machine = makeMachine({
    registers: program.registers,
    operations,
    controller: program.controller,
  });
  for (const [name, initial] of program.inputs) {
    if (machine.writeRegister(name, initial) !== null) {
      return { termination: "error", stdout: transcriptText(lines) };
    }
  }
  const run = machine.run(MACHINE_STEP_LIMIT);
  if (run.error !== null) {
    return { termination: "error", stdout: transcriptText(lines) };
  }
  for (const name of program.outputs) {
    lines.push(`${name} = ${renderWord(run.registers[name])}`);
  }
  lines.push(`pushes ${run.stackStats.pushes}`, `max-depth ${run.stackStats.maxDepth}`);
  return { termination: "value", stdout: transcriptText(lines) };
};

// ---------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------

const queryTeaching = (program: QueryProgram): Observation => {
  const database = makeDatabase();
  for (const assertion of program.assertions) {
    database.addAssertion(assertion);
  }
  for (const rule of program.rules) {
    database.addRule(rule);
  }
  const run = queryDriverLoop(database, program.queries);
  return observationOf(run.outcome, run.transcript);
};

const observe = async (args: Arguments): Promise<Observation> => {
  const family = args.caseId.split("/")[0];
  if (family === "query" && (args.engine === "reference" || args.engine === "query")) {
    const program = toQueryProgram(await importCase(args.source, "program"));
    return args.engine === "reference" ? queryReference(program) : queryTeaching(program);
  }
  if (family === "machine" && (args.engine === "reference" || args.engine === "machine")) {
    const lines: string[] = [];
    const program = await loadMachine(args.source, (line) => lines.push(line));
    return args.engine === "reference"
      ? machineReference(program, lines)
      : machineTeaching(program, lines);
  }
  const source = readFileSync(args.source, "utf8");
  switch (args.engine) {
    case "reference":
      if (family === "lazy") {
        return lazyReference(source, args.work);
      }
      if (family === "amb") {
        return searchReference(source, args.work);
      }
      throw new DriverFailure(`no reference model for case family ${String(family)}`);
    case "direct": {
      const run = runSource(source);
      return observationOf(run.outcome, run.transcript);
    }
    case "analyzed": {
      const run = runAnalyzedSource(source);
      return observationOf(run.outcome, run.transcript);
    }
    case "eceval": {
      const run = runEvaluator(source);
      return observationOf(run.outcome, run.transcript);
    }
    case "compiled": {
      const run = compileAndRun(source);
      return observationOf(run.outcome, run.transcript);
    }
    case "lazy": {
      const run = runLazySource(source, "lazy-memoized-experiment");
      return observationOf(run.outcome, run.transcript);
    }
    case "search": {
      const run = runAmbSource(source, "amb-depth-first-experiment");
      return observationOf(run.outcome, run.transcript);
    }
    default:
      throw new DriverFailure(`unknown engine ${args.engine} for case ${args.caseId}`);
  }
};

/** Stack for the engine worker: deep enough for 5000-deep guest recursion through the recursive evaluators. */
const ENGINE_STACK_MB = 512;

if (isMainThread) {
  const args = parseArguments(process.argv.slice(2));
  if (args.engine === "" || args.source === "" || args.work === "") {
    process.stderr.write(
      "usage: conformance-driver --case <id> --engine <engine> --source <path> --work <dir>\n",
    );
    process.exit(1);
  }
  const worker = new Worker(new URL(import.meta.url), {
    workerData: args,
    resourceLimits: { stackSizeMb: ENGINE_STACK_MB },
  });
  worker.on("message", (observation: unknown) => {
    process.stdout.write(`${JSON.stringify(observation)}\n`);
  });
  worker.on("error", (error: unknown) => {
    process.stderr.write(
      `${error instanceof Error ? (error.stack ?? error.message) : String(error)}\n`,
    );
    process.exitCode = 1;
  });
} else {
  const port = parentPort;
  if (port === null) {
    throw new DriverFailure("engine worker has no parent port");
  }
  const data = fieldsOf(workerData, "worker data");
  const text = (key: string): string => textOf(data[key], `worker ${key}`);
  port.postMessage(
    await observe({
      caseId: text("caseId"),
      engine: text("engine"),
      source: text("source"),
      work: text("work"),
    }),
  );
}
