// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.4

/**
 * The query evaluator (host-subsets grammar section 7): the query language is
 * typed host data — `Query`, `Term`, `Rule`, `Frame` — never extra host
 * syntax and never a claim of built-in language features. Patterns are
 * finite recursive term data; frames are immutable binding chains; streams
 * memoize their tails so recursive rules interleave fairly. Search,
 * unification, and rule application run over that data; recoverable failures
 * are the declared `QueryError` values.
 */
import type { RunResult } from "./01-metacircular.ts";
import { ok } from "./runtime/errors.ts";
import { makePair, makeRecord, type Value } from "./runtime/value.ts";

/** A query term: a variable, a literal, or finite cons/nil list data. */
export type Term =
  | { readonly tag: "var"; readonly name: string }
  | { readonly tag: "text"; readonly value: string | number | boolean }
  | { readonly tag: "cons"; readonly head: Term; readonly tail: Term }
  | { readonly tag: "nil" };

/** A query over the assertion database. */
export type Query =
  | { readonly tag: "atom"; readonly relation: string; readonly fields: ReadonlyArray<Term> }
  | { readonly tag: "and"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "or"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "not"; readonly clause: Query }
  | { readonly tag: "unique"; readonly clause: Query }
  | {
      readonly tag: "lisp-value";
      readonly predicate: (args: ReadonlyArray<Value>) => boolean;
      readonly args: ReadonlyArray<Term>;
    }
  | { readonly tag: "always-true" };

/** A rule: a head pattern plus its body conditions. */
export interface Rule {
  readonly head: Query;
  readonly body: ReadonlyArray<Query>;
}

/** One variable binding. */
export interface Binding {
  readonly name: string;
  readonly value: Term;
}

/** An immutable binding chain; the empty frame is the empty array. */
export type Frame = ReadonlyArray<Binding>;

/** The declared failures of the query system. */
export type QueryError =
  | { readonly tag: "unbound-pattern"; readonly detail: string }
  | { readonly tag: "bad-rule"; readonly detail: string }
  | { readonly tag: "query-failed"; readonly detail: string };

/** A memoized lazy stream over answers. */
export type Stream<A> =
  | { readonly kind: "empty" }
  | { readonly kind: "singleton"; readonly value: A }
  | { readonly kind: "delayed"; readonly force: () => Stream<A> }
  | { readonly kind: "cons"; readonly head: A; readonly rest: () => Stream<A> };

const emptyStream = <A>(): Stream<A> => ({ kind: "empty" });
const singletonStream = <A>(value: A): Stream<A> => ({ kind: "singleton", value });
const delayedStream = <A>(force: () => Stream<A>): Stream<A> => {
  let computed: Stream<A> | undefined;
  return {
    kind: "delayed",
    force: () => {
      if (computed === undefined) {
        computed = force();
      }
      return computed;
    },
  };
};
const consStream = <A>(head: A, rest: () => Stream<A>): Stream<A> => ({ kind: "cons", head, rest });

/** The elements of a stream, in order. */
export const streamToList = <A>(stream: Stream<A>): ReadonlyArray<A> => {
  const out: A[] = [];
  let current = stream;
  for (;;) {
    if (current.kind === "empty") {
      return out;
    }
    if (current.kind === "singleton") {
      out.push(current.value);
      return out;
    }
    if (current.kind === "delayed") {
      current = current.force();
      continue;
    }
    out.push(current.head);
    current = current.rest();
  }
};

/** The stream append that defers its tail (the book's `stream-append-delayed`). */
export const streamAppendDelayed = <A>(left: Stream<A>, right: () => Stream<A>): Stream<A> => {
  if (left.kind === "empty") {
    return delayedStream(right);
  }
  if (left.kind === "singleton") {
    return consStream(left.value, right);
  }
  if (left.kind === "delayed") {
    return delayedStream(() => streamAppendDelayed(left.force(), right));
  }
  return consStream(left.head, () => streamAppendDelayed(left.rest(), right));
};

/** The interleave that defers its tail (the book's `interleave-delayed`). */
export const interleaveDelayed = <A>(left: Stream<A>, right: () => Stream<A>): Stream<A> => {
  if (left.kind === "empty") {
    return delayedStream(right);
  }
  if (left.kind === "delayed") {
    return delayedStream(() => interleaveDelayed(left.force(), right));
  }
  if (left.kind === "singleton") {
    return consStream(left.value, right);
  }
  return consStream(left.head, () => interleaveDelayed(right(), left.rest));
};

/** Maps a stream-producing function over a stream, flattening the results. */
export const streamFlatmap = <A, B>(
  stream: Stream<A>,
  transform: (value: A) => Stream<B>,
): Stream<B> => {
  if (stream.kind === "empty") {
    return emptyStream();
  }
  if (stream.kind === "singleton") {
    return transform(stream.value);
  }
  if (stream.kind === "delayed") {
    return delayedStream(() => streamFlatmap(stream.force(), transform));
  }
  return interleaveDelayed(transform(stream.head), () => streamFlatmap(stream.rest(), transform));
};

/** Flattens a stream of streams by interleaving. */
export const flattenStream = <A>(streams: Stream<Stream<A>>): Stream<A> =>
  streamFlatmap(streams, (inner) => inner);

/** The no-delay flatten variant of the exercises. */
export const simpleFlatten = <A>(streams: Stream<Stream<A>>): Stream<A> =>
  simpleStreamFlatmap(streams, (inner) => inner);

/** The no-delay flatmap variant of the exercises. */
export const simpleStreamFlatmap = <A, B>(
  stream: Stream<A>,
  transform: (value: A) => Stream<B>,
): Stream<B> => {
  if (stream.kind === "empty") {
    return emptyStream();
  }
  if (stream.kind === "singleton") {
    return transform(stream.value);
  }
  if (stream.kind === "delayed") {
    return simpleStreamFlatmap(stream.force(), transform);
  }
  return streamAppendDelayed(transform(stream.head), () =>
    simpleStreamFlatmap(stream.rest(), transform),
  );
};

// ---------------------------------------------------------------------
// Frames and bindings
// ---------------------------------------------------------------------

/** Builds one binding. */
export const makeBinding = (name: string, value: Term): Binding => ({ name, value });

/** The value bound to `name`, or `undefined` when the frame is silent. */
export const bindingInFrame = (name: string, frame: Frame): Term | undefined => {
  for (const binding of frame) {
    if (binding.name === name) {
      return binding.value;
    }
  }
  return undefined;
};

/** Extends a frame with one more binding (the book's `extend`). */
export const extend = (name: string, value: Term, frame: Frame): Frame => [
  makeBinding(name, value),
  ...frame,
];

// ---------------------------------------------------------------------
// Terms and pattern matching
// ---------------------------------------------------------------------

/** A query variable. */
export const qvar = (name: string): Term => ({ tag: "var", name });
/** A literal term. */
export const qtext = (value: string | number | boolean): Term => ({ tag: "text", value });
/** A list term over cons/nil data. */
export const qlist = (...items: ReadonlyArray<Term>): Term =>
  items.reduceRight<Term>((tail, head) => ({ tag: "cons", head, tail }), { tag: "nil" });
/** A dotted pair term. */
export const qpair = (head: Term, tail: Term): Term => ({ tag: "cons", head, tail });

const isVariable = (term: Term): term is Extract<Term, { readonly tag: "var" }> =>
  term.tag === "var";

/** Pattern matching: `pattern` against `data` in `frame`, or `undefined`. */
export const patternMatch = (pattern: Term, data: Term, frame: Frame): Frame | undefined => {
  if (pattern.tag === "var") {
    const bound = bindingInFrame(pattern.name, frame);
    return bound === undefined
      ? extend(pattern.name, data, frame)
      : extendIfConsistent(pattern.name, data, frame);
  }
  if (pattern.tag === "cons" && data.tag === "cons") {
    const afterHead = patternMatch(pattern.head, data.head, frame);
    return afterHead === undefined ? undefined : patternMatch(pattern.tail, data.tail, afterHead);
  }
  if (pattern.tag === "nil" && data.tag === "nil") {
    return frame;
  }
  return pattern.tag === "text" && data.tag === "text" && pattern.value === data.value
    ? frame
    : undefined;
};

/** Adds `name = data` when consistent with the frame. */
export const extendIfConsistent = (name: string, data: Term, frame: Frame): Frame | undefined => {
  const bound = bindingInFrame(name, frame);
  return bound === undefined ? extend(name, data, frame) : unifyMatch(bound, data, frame);
};

/** Unification of two patterns in a frame, or `undefined`. */
export const unifyMatch = (left: Term, right: Term, frame: Frame): Frame | undefined => {
  if (left.tag === "var") {
    return extendIfConsistent(left.name, right, frame);
  }
  if (right.tag === "var") {
    return extendIfConsistent(right.name, left, frame);
  }
  if (left.tag === "cons" && right.tag === "cons") {
    const afterHead = unifyMatch(left.head, right.head, frame);
    return afterHead === undefined ? undefined : unifyMatch(left.tail, right.tail, afterHead);
  }
  if (left.tag === "nil" && right.tag === "nil") {
    return frame;
  }
  return left.tag === "text" && right.tag === "text" && left.value === right.value
    ? frame
    : undefined;
};

/** Unifies two atomic queries in a frame (rule heads against patterns). */
export const unifyQueries = (left: Query, right: Query, frame: Frame): Frame | undefined => {
  if (left.tag !== "atom" || right.tag !== "atom" || left.relation !== right.relation) {
    return undefined;
  }
  if (left.fields.length !== right.fields.length) {
    return undefined;
  }
  let current: Frame | undefined = frame;
  for (let i = 0; i < left.fields.length; i += 1) {
    const leftField = left.fields[i];
    const rightField = right.fields[i];
    if (leftField === undefined || rightField === undefined || current === undefined) {
      return undefined;
    }
    current = unifyMatch(leftField, rightField, current);
  }
  return current;
};

/** Unification that refuses to extend a frame through a dependent value. */
export const extendIfPossible = (name: string, value: Term, frame: Frame): Frame | undefined => {
  const bound = bindingInFrame(name, frame);
  if (bound !== undefined) {
    return unifyMatch(bound, value, frame);
  }
  if (isVariable(value) && bindingInFrame(value.name, frame) !== undefined) {
    const found = bindingInFrame(value.name, frame);
    return found === undefined ? undefined : extendIfPossible(name, found, frame);
  }
  return dependsOn(value, name, frame) ? undefined : extend(name, value, frame);
};

/** Whether `term` mentions the variable `name` through the frame. */
export const dependsOn = (term: Term, name: string, frame: Frame): boolean => {
  if (term.tag === "var") {
    return term.name === name;
  }
  if (term.tag !== "cons") {
    return false;
  }
  return dependsOn(term.head, name, frame) || dependsOn(term.tail, name, frame);
};

/** Substitutes frame bindings into a term. */
export const instantiate = (term: Term, frame: Frame, unbound: (name: string) => Term): Term => {
  if (term.tag === "var") {
    const bound = bindingInFrame(term.name, frame);
    return bound === undefined ? unbound(term.name) : instantiate(bound, frame, unbound);
  }
  if (term.tag !== "cons") {
    return term;
  }
  return {
    tag: "cons",
    head: instantiate(term.head, frame, unbound),
    tail: instantiate(term.tail, frame, unbound),
  };
};

// ---------------------------------------------------------------------
// Query syntax accessors
// ---------------------------------------------------------------------

/** The query's type tag (the book's `type`). */
export const typeOf = (query: Query): Query["tag"] => query.tag;
/** The query's contents payload. */
export const contentsOf = (query: Query): Query => query;
/** The rule's conclusion. */
export const conclusion = (rule: Rule): Query => rule.head;
/** The rule's body conditions. */
export const ruleBody = (rule: Rule): ReadonlyArray<Query> => rule.body;
/** A rule's empty body means the head asserts a fact. */
export const isAssertionToBeAdded = (rule: Rule): boolean => rule.body.length === 0;
/** The assertion a rule adds: its instantiated head. */
export const addAssertionBody = (rule: Rule): Query => rule.head;

let ruleCounterValue = 0;
/** The rule application counter (the book's `rule-counter`). */
export const ruleCounter = (): number => ruleCounterValue;
/** A fresh rule application id (the book's `new-rule-application-id`). */
export const newRuleApplicationId = (): number => {
  ruleCounterValue += 1;
  return ruleCounterValue;
};

/** A fresh variable for one rule application. */
export const makeNewVariable = (name: string, id: number): Term => qvar(`${name}-${id}`);

/** Renames a rule's variables for one application (the book's `rename-variables-in`). */
export const renameVariablesIn = (rule: Rule): Rule => {
  const id = newRuleApplicationId();
  const renameTerm = (term: Term): Term => {
    if (term.tag === "var") {
      return makeNewVariable(term.name, id);
    }
    if (term.tag !== "cons") {
      return term;
    }
    return { tag: "cons", head: renameTerm(term.head), tail: renameTerm(term.tail) };
  };
  const renameQuery = (query: Query): Query => {
    if (query.tag === "atom") {
      return { tag: "atom", relation: query.relation, fields: query.fields.map(renameTerm) };
    }
    if (query.tag === "and" || query.tag === "or") {
      return { tag: query.tag, clauses: query.clauses.map(renameQuery) };
    }
    if (query.tag === "not" || query.tag === "unique") {
      return { tag: query.tag, clause: renameQuery(query.clause) };
    }
    if (query.tag === "lisp-value") {
      return { tag: "lisp-value", predicate: query.predicate, args: query.args.map(renameTerm) };
    }
    return query;
  };
  return { head: renameQuery(rule.head), body: rule.body.map(renameQuery) };
};

/** Replaces `?x` markers in query text with variable terms (the book's
 * `query-syntax-process`); it is the naming layer over typed terms. */
export const querySyntaxProcess = (query: Query): Query => query;
/** Maps a term function over every term of a query. */
export const mapOverSymbols = (query: Query, transform: (term: Term) => Term): Query => {
  if (query.tag === "atom") {
    return { tag: "atom", relation: query.relation, fields: query.fields.map(transform) };
  }
  if (query.tag === "and" || query.tag === "or") {
    return {
      tag: query.tag,
      clauses: query.clauses.map((clause) => mapOverSymbols(clause, transform)),
    };
  }
  if (query.tag === "not" || query.tag === "unique") {
    return { tag: query.tag, clause: mapOverSymbols(query.clause, transform) };
  }
  if (query.tag === "lisp-value") {
    return { tag: "lisp-value", predicate: query.predicate, args: query.args.map(transform) };
  }
  return query;
};
/** Expands a `?x` name into a variable term. */
export const expandQuestionMark = (name: string): Term => qvar(name.replace(/^\?/, ""));
/** Contracts a variable term back into its `?x` name. */
export const contractQuestionMark = (term: Term): string =>
  term.tag === "var" ? `?${term.name}` : formatTerm(term);

// ---------------------------------------------------------------------
// Assertions, rules, and the database
// ---------------------------------------------------------------------

/** The assertion database: facts and rules indexed by relation. */
export class Database {
  readonly assertions = new Map<string, Query[]>();
  readonly rules = new Map<string, Rule[]>();

  /** Adds one fact. */
  addAssertion(assertion: Query): void {
    if (assertion.tag !== "atom") {
      return;
    }
    const bucket = this.assertions.get(assertion.relation) ?? [];
    bucket.push(assertion);
    this.assertions.set(assertion.relation, bucket);
  }

  /** Adds one rule keyed by its conclusion's relation. */
  addRule(rule: Rule): void {
    if (rule.head.tag !== "atom") {
      return;
    }
    const bucket = this.rules.get(rule.head.relation) ?? [];
    bucket.push(rule);
    this.rules.set(rule.head.relation, bucket);
  }
}

/** An empty database. */
export const makeDatabase = (): Database => new Database();

/** Every assertion, or the ones under the pattern's relation. */
export const fetchAssertions = (pattern: Query, db: Database): ReadonlyArray<Query> =>
  pattern.tag === "atom" ? (db.assertions.get(pattern.relation) ?? []) : [];

/** Every rule, or the ones whose conclusion shares the pattern's relation. */
export const fetchRules = (pattern: Query, db: Database): ReadonlyArray<Rule> =>
  pattern.tag === "atom" ? (db.rules.get(pattern.relation) ?? []) : [];

/** Checks one assertion against a pattern (the book's `check-an-assertion`). */
export const checkAnAssertion = (
  assertion: Query,
  pattern: Query,
  frame: Frame,
): Frame | undefined => {
  if (
    assertion.tag !== "atom" ||
    pattern.tag !== "atom" ||
    assertion.relation !== pattern.relation ||
    assertion.fields.length !== pattern.fields.length
  ) {
    return undefined;
  }
  let current: Frame | undefined = frame;
  for (let i = 0; i < pattern.fields.length; i += 1) {
    const pat = pattern.fields[i];
    const dat = assertion.fields[i];
    if (pat === undefined || dat === undefined || current === undefined) {
      return undefined;
    }
    current = patternMatch(pat, dat, current);
  }
  return current;
};

/** Every assertion that matches the pattern (the book's `find-assertions`). */
export const findAssertions = (pattern: Query, frame: Frame, db: Database): Stream<Frame> => {
  const matches: Frame[] = [];
  for (const assertion of fetchAssertions(pattern, db)) {
    const result = checkAnAssertion(assertion, pattern, frame);
    if (result !== undefined) {
      matches.push(result);
    }
  }
  return matches.reduceRight<Stream<Frame>>(
    (rest, head) => consStream(head, () => rest),
    emptyStream(),
  );
};

/** Applies one renamed rule to a pattern (the book's `apply-a-rule`). */
export const applyARule = (
  rule: Rule,
  pattern: Query,
  frame: Frame,
  db: Database,
): Stream<Frame> => {
  const renamed = renameVariablesIn(rule);
  const unified = unifyQueries(renamed.head, pattern, frame);
  if (unified === undefined) {
    return emptyStream();
  }
  return qevalConjunction(renamed.body, unified, db);
};

/** Every rule application that answers the pattern (the book's `apply-rules`). */
export const applyRules = (pattern: Query, frame: Frame, db: Database): Stream<Frame> =>
  fetchRules(pattern, db).reduceRight<Stream<Frame>>(
    (rest, rule) => interleaveDelayed(applyARule(rule, pattern, frame, db), () => rest),
    emptyStream(),
  );

// ---------------------------------------------------------------------
// qeval
// ---------------------------------------------------------------------

const qevalConjunction = (
  clauses: ReadonlyArray<Query>,
  frame: Frame,
  db: Database,
): Stream<Frame> => {
  if (clauses.length === 0) {
    return singletonStream(frame);
  }
  const [first, ...rest] = clauses;
  if (first === undefined) {
    return singletonStream(frame);
  }
  return streamFlatmap(qeval(first, singletonStream(frame), db), (next) =>
    qevalConjunction(rest, next, db),
  );
};

/** The simple query: assertions and rules (the book's `simple-query`). */
export const simpleQuery = (
  query: Query,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> =>
  streamFlatmap(frameStream, (frame) =>
    interleaveDelayed(findAssertions(query, frame, db), () => applyRules(query, frame, db)),
  );

/** The no-delay simple query variant of the exercises. */
export const simpleQueryNoDelay = (
  query: Query,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> =>
  streamFlatmap(frameStream, (frame) =>
    streamAppendDelayed(findAssertions(query, frame, db), () => applyRules(query, frame, db)),
  );

/** Conjunction of clauses (the book's `conjoin`). */
export const conjoin = (
  clauses: ReadonlyArray<Query>,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> => streamFlatmap(frameStream, (frame) => qevalConjunction(clauses, frame, db));

/** Disjunction of clauses (the book's `disjoin`). */
export const disjoin = (
  clauses: ReadonlyArray<Query>,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> => {
  if (clauses.length === 0) {
    return emptyStream();
  }
  return streamFlatmap(frameStream, (frame) =>
    clauses.reduceRight<Stream<Frame>>(
      (rest, clause) => interleaveDelayed(qeval(clause, singletonStream(frame), db), () => rest),
      emptyStream(),
    ),
  );
};

/** The no-delay disjoin variant of the exercises. */
export const disjoinNoDelay = (
  clauses: ReadonlyArray<Query>,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> => {
  if (clauses.length === 0) {
    return emptyStream();
  }
  return streamFlatmap(frameStream, (frame) =>
    clauses.reduceRight<Stream<Frame>>(
      (rest, clause) => streamAppendDelayed(qeval(clause, singletonStream(frame), db), () => rest),
      emptyStream(),
    ),
  );
};

/** Negation as failure (the book's `negate`). */
export const negate = (clause: Query, frameStream: Stream<Frame>, db: Database): Stream<Frame> =>
  streamFlatmap(frameStream, (frame) => {
    const found = streamToList(qeval(clause, singletonStream(frame), db));
    return found.length === 0 ? singletonStream(frame) : emptyStream();
  });

/** The `unique` filter: frames whose clause has exactly one match. */
export const unique = (clause: Query, frameStream: Stream<Frame>, db: Database): Stream<Frame> =>
  streamFlatmap(frameStream, (frame) => {
    const found = streamToList(qeval(clause, singletonStream(frame), db));
    return found.length === 1 ? singletonStream(found[0] ?? frame) : emptyStream();
  });

/** The always-true special form (the book's `always-true`). */
export const alwaysTrue = (frameStream: Stream<Frame>): Stream<Frame> => frameStream;

/** The lisp-value filter over instantiated terms. */
export const lispValue = (
  call: Extract<Query, { tag: "lisp-value" }>,
  frameStream: Stream<Frame>,
  db: Database,
): Stream<Frame> => {
  void db;
  return streamFlatmap(frameStream, (frame) => {
    let unbound = false;
    const args = call.args.map((term) =>
      termToValue(
        instantiate(term, frame, (name) => {
          unbound = true;
          return qvar(name);
        }),
      ),
    );
    return !unbound && call.predicate(args) ? singletonStream(frame) : emptyStream();
  });
};

/** Evaluates one query over a stream of frames (the book's `qeval`). */
export const qeval = (query: Query, frameStream: Stream<Frame>, db: Database): Stream<Frame> => {
  switch (query.tag) {
    case "atom":
      return simpleQuery(query, frameStream, db);
    case "and":
      return conjoin(query.clauses, frameStream, db);
    case "or":
      return disjoin(query.clauses, frameStream, db);
    case "not":
      return negate(query.clause, frameStream, db);
    case "unique":
      return unique(query.clause, frameStream, db);
    case "lisp-value":
      return lispValue(query, frameStream, db);
    case "always-true":
      return alwaysTrue(frameStream);
  }
};

// ---------------------------------------------------------------------
// Construction and rendering
// ---------------------------------------------------------------------

/** A fact: an atomic query with no body. */
export const queryAtom = (relation: string, ...fields: ReadonlyArray<Term>): Query => ({
  tag: "atom",
  relation,
  fields,
});

/** A rule from its head and body conditions. */
export const rule = (head: Query, ...body: ReadonlyArray<Query>): Rule => ({ head, body });

/** Renders one term in the edition's neutral notation. */
export const formatTerm = (term: Term): string => {
  if (term.tag === "var") {
    return `?${term.name}`;
  }
  if (term.tag === "text") {
    return typeof term.value === "string" ? JSON.stringify(term.value) : String(term.value);
  }
  if (term.tag === "nil") {
    return "[]";
  }
  const parts: string[] = [];
  let current: Term = term;
  while (current.tag === "cons") {
    parts.push(formatTerm(current.head));
    current = current.tail;
  }
  return current.tag === "nil"
    ? `[${parts.join(", ")}]`
    : `[${parts.join(", ")} | ${formatTerm(current)}]`;
};

/** Renders one query as a call-shaped line. */
export const formatQuery = (query: Query): string => {
  if (query.tag === "atom") {
    return `${query.relation}(${query.fields.map(formatTerm).join(", ")})`;
  }
  if (query.tag === "and" || query.tag === "or") {
    return `${query.tag}(${query.clauses.map(formatQuery).join(", ")})`;
  }
  if (query.tag === "not" || query.tag === "unique") {
    return `${query.tag}(${formatQuery(query.clause)})`;
  }
  if (query.tag === "lisp-value") {
    return `lisp-value(${query.args.map(formatTerm).join(", ")})`;
  }
  return "always-true";
};

/** Converts an instantiated term to a runtime value for `lisp-value`. */
export const termToValue = (term: Term): Value => {
  if (term.tag === "text") {
    return term.value;
  }
  if (term.tag === "nil") {
    return makeRecord([["tag", "nil"]]);
  }
  if (term.tag === "cons") {
    return makePair(termToValue(term.head), termToValue(term.tail));
  }
  return makeRecord([
    ["tag", "var"],
    ["name", term.name],
  ]);
};

/** The query driver loop: each query's answers in order, as transcript data. */
export const queryDriverLoop = (db: Database, queries: ReadonlyArray<Query>): RunResult => {
  const transcript: string[] = [];
  for (const query of queries) {
    const frames = streamToList(qeval(query, singletonStream([]), db));
    if (frames.length === 0) {
      transcript.push(formatQuery(query), "No.");
      continue;
    }
    transcript.push(formatQuery(query));
    for (const frame of frames) {
      const answer = mapOverSymbols(query, (term) =>
        instantiate(term, frame, (name) => qvar(name)),
      );
      transcript.push(formatQuery(answer));
    }
  }
  return { outcome: ok(undefined), transcript };
};
