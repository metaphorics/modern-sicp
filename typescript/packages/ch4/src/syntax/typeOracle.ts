// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The type oracle: operand types come from the pinned TypeScript 7.0.2
 * checker (host-subsets grammar sections 1 and 2), never from a second,
 * hand-written type system. The edition parses source with its own grammar
 * parser, then asks the pinned checker what each operand expression's type
 * is (span-resolved through the checker's own AST) so the subset's stricter
 * rules — boolean-only conditions, compatible arithmetic operands — are
 * enforced with the type authority's data. Host type failures are preserved
 * as their own diagnostic layer with file/span/code (grammar section 8).
 * Admission is boundary code: it writes the unit into the edition's ignored
 * admission workspace and consults the checker over a cached API session.
 */
import { mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { threadId } from "node:worker_threads";

import type { Node } from "typescript/unstable/ast";
import { API, type Project, type Type } from "typescript/unstable/sync";

import { type Diagnostic, diagnostic, type Span } from "./diagnostics.ts";

/** The checker's AST node type, traversed with `forEachChild`. */
type CheckerNode = Node;

const childNodesOf = (node: CheckerNode): ReadonlyArray<CheckerNode> => {
  const kids: CheckerNode[] = [];
  node.forEachChild((child) => {
    kids.push(child);
    return undefined;
  });
  return kids;
};

const triviaEnd = (source: string, offset: number): number => {
  let i = Math.max(0, Math.min(offset, source.length));
  for (;;) {
    const c = source[i] ?? "";
    if (c === " " || c === "\t" || c === "\n" || c === "\r" || c === "\f" || c === "\v") {
      i += 1;
      continue;
    }
    if (c === "/" && source[i + 1] === "/") {
      while (i < source.length && source[i] !== "\n") {
        i += 1;
      }
      continue;
    }
    if (c === "/" && source[i + 1] === "*") {
      i += 2;
      while (i < source.length && !(source[i] === "*" && source[i + 1] === "/")) {
        i += 1;
      }
      i += 2;
      continue;
    }
    return i;
  }
};

/** What the oracle knows about one operand expression's type. */
export type TypeProbeKind =
  | "boolean"
  | "number"
  | "string"
  | "null"
  | "undefined"
  | "void"
  | "any"
  | "unknown"
  | "error"
  | "other"
  | "unresolved";

/** One probed operand type. */
export interface TypeProbe {
  readonly kind: TypeProbeKind;
  readonly text: string;
}

/** A host TypeScript diagnostic, preserved with its code and span. */
export interface HostDiagnostic {
  readonly code: number;
  readonly span: Span;
  readonly message: string;
}

/** Span-resolved type data plus the host checker's own diagnostics. */
export interface TypeOracle {
  probe(span: Span): TypeProbe;
  hostDiagnostics(): ReadonlyArray<HostDiagnostic>;
}

interface Session {
  readonly api: API;
  readonly configPath: string;
  readonly filePath: string;
}

let session: Session | undefined;

const ownerAlive = (pid: number): boolean => {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error instanceof Error && "code" in error && error.code === "EPERM";
  }
};

/** Each session owns a workspace named by its process and thread: the checker
 * project includes every unit in its directory, so a shared directory lets
 * concurrent test workers overwrite one another's unit between write and
 * check. Test runners end workers without an exit event, so opening a session
 * removes the workspaces whose owning process is gone. */
const openSession = (): Session => {
  const root = fileURLToPath(new URL("../../../../.admission", import.meta.url));
  mkdirSync(root, { recursive: true });
  for (const entry of readdirSync(root)) {
    const owner = /^session-(\d+)-\d+$/.exec(entry)?.[1];
    if (owner !== undefined && !ownerAlive(Number(owner))) {
      rmSync(join(root, entry), { recursive: true, force: true });
    }
  }
  const workspace = join(root, `session-${process.pid}-${threadId}`);
  mkdirSync(workspace, { recursive: true });
  const configPath = join(workspace, "tsconfig.json");
  writeFileSync(
    configPath,
    `${JSON.stringify({ extends: "../../tsconfig.base.json", include: ["*.ts"] }, null, 2)}\n`,
  );
  return { api: new API({ cwd: workspace }), configPath, filePath: join(workspace, "guest.ts") };
};

const sessionOf = (): Session => {
  if (session === undefined) {
    session = openSession();
  }
  return session;
};

const positionOf = (source: string, offset: number): { line: number; column: number } => {
  let line = 1;
  let column = 1;
  const limit = Math.max(0, Math.min(offset, source.length));
  for (let i = 0; i < limit; i += 1) {
    if (source[i] === "\n") {
      line += 1;
      column = 1;
      continue;
    }
    column += 1;
  }
  return { line, column };
};

const spanFromOffsets = (source: string, start: number, end: number): Span => {
  const begin = Math.max(0, start);
  const position = positionOf(source, begin);
  return { start: begin, end: Math.max(begin, end), line: position.line, column: position.column };
};

const classify = (checker: Project["checker"], type: Type | undefined): TypeProbe => {
  if (type === undefined) {
    return { kind: "unresolved", text: "no-type" };
  }
  const text = checker.typeToString(type);
  if (type.isErrorType()) {
    return { kind: "error", text };
  }
  if (text === "any") {
    return { kind: "any", text };
  }
  if (text === "unknown") {
    return { kind: "unknown", text };
  }
  if (checker.isTypeAssignableTo(type, checker.getBooleanType())) {
    return { kind: "boolean", text };
  }
  if (checker.isTypeAssignableTo(type, checker.getNumberType())) {
    return { kind: "number", text };
  }
  if (checker.isTypeAssignableTo(type, checker.getStringType())) {
    return { kind: "string", text };
  }
  if (text === "null") {
    return { kind: "null", text };
  }
  if (text === "undefined") {
    return { kind: "undefined", text };
  }
  if (text === "void") {
    return { kind: "void", text };
  }
  return { kind: "other", text };
};

const nodeCovering = (root: CheckerNode, source: string, span: Span): CheckerNode | undefined => {
  let node = root;
  for (;;) {
    const next = childNodesOf(node).find(
      (child) => child.pos <= span.start && child.end >= span.end,
    );
    if (next === undefined) {
      break;
    }
    node = next;
  }
  if (node === root) {
    return undefined;
  }
  return triviaEnd(source, node.pos) === span.start && node.end === span.end ? node : undefined;
};

/** Opens the oracle over `source`, checked as one unit by the pinned checker.
 * `declarations` appends boundary-only type declarations (named experiment
 * extensions) after the unit, leaving source offsets unchanged. */
export const openTypeOracle = (source: string, declarations = ""): TypeOracle => {
  const active = sessionOf();
  writeFileSync(active.filePath, `${source}\n${declarations}\nexport {};\n`);
  const snapshot = active.api.updateSnapshot({
    openProjects: [active.configPath],
    fileChanges: { changed: [active.filePath] },
  });
  const project = snapshot.getProject(active.configPath) ?? snapshot.getProjects()[0];
  if (project === undefined) {
    throw new Error("the type oracle could not open the admission project");
  }
  const sourceFile = project.program.getSourceFile(active.filePath);
  if (sourceFile === undefined) {
    throw new Error("the type oracle could not load the admission unit");
  }
  const checker = project.checker;
  return {
    probe(span: Span): TypeProbe {
      const node = nodeCovering(sourceFile, source, span);
      return classify(checker, node === undefined ? undefined : checker.getTypeAtLocation(node));
    },
    hostDiagnostics(): ReadonlyArray<HostDiagnostic> {
      const reported = (d: {
        code: number;
        pos: number;
        end: number;
        text: string;
      }): HostDiagnostic => ({
        code: d.code,
        span: spanFromOffsets(source, d.pos, d.end),
        message: d.text,
      });
      const syntactic = project.program.getSyntacticDiagnostics(active.filePath);
      const semantic = project.program.getSemanticDiagnostics(active.filePath);
      return [...syntactic, ...semantic].map(reported);
    },
  };
};

/** A host diagnostic rendered as a subset diagnostic for uniform reporting. */
export const hostDiagnosticText = (d: HostDiagnostic): string => `TS${d.code}: ${d.message}`;

/** Wraps a host diagnostic into the shared diagnostic shape for transcripts. */
export const hostDiagnostic = (d: HostDiagnostic): Diagnostic =>
  diagnostic("UnsupportedType", `TS${d.code}`, d.span, d.message);
