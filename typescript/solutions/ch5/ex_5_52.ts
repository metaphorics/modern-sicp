// SPDX-License-Identifier: GPL-3.0-only
import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  EvaluatorFault,
  formatWord,
  type PairWord,
  parse,
  type Word,
} from "../../packages/ch5/src/04-eceval.js";
import {
  compile,
  defaultConfig,
  LinkageNext,
  newState,
  renderStatement,
  topCenv,
} from "../../packages/ch5/src/05-compilation.js";

const RUNTIME_C = (): string =>
  readFileSync(new URL("./metacircular_backend_5_52.c", import.meta.url), "utf8");

const METACIRCULAR = (): string =>
  readFileSync(new URL("../../packages/ch5/src/metacircular.scm", import.meta.url), "utf8");

/** A C identifier for a controller label: dashes become underscores. */
const cLabel = (name: string): string => name.replaceAll("-", "_");

/** A C function name for a machine operation. */
const cOp = (name: string): string => {
  let out = "op_";
  for (const ch of name) {
    if (/[a-zA-Z0-9_]/.test(ch)) out += ch;
    else if (ch === "?") out += "_p";
    else if (ch === "!") out += "_x";
    else if (ch === "*") out += "_star";
    else if (ch === "+") out += "_plus";
    else if (ch === "=") out += "_eq";
    else if (ch === "<") out += "_lt";
    else if (ch === ">") out += "_gt";
    else if (ch === "/") out += "_slash";
    else out += "_";
  }
  return out;
};

/** Splits the inside of one parenthesized statement into its top-level
 * items, keeping nested structure and strings intact. */
const splitTop = (line: string): string[] => {
  const inner = line.slice(1, -1);
  const items: string[] = [];
  let depth = 0;
  let inString = false;
  let escaped = false;
  let start = 0;
  for (let i = 0; i <= inner.length; i += 1) {
    const end = i === inner.length;
    const ch = end ? " " : (inner[i] ?? " ");
    if (inString) {
      if (escaped) escaped = false;
      else if (ch === "\\") escaped = true;
      else if (ch === '"') inString = false;
    } else if (ch === '"') {
      inString = true;
    } else if (ch === "(") {
      depth += 1;
    } else if (ch === ")") {
      depth -= 1;
      if (depth < 0) throw new EvaluatorFault(`the C backend rejects the line: ${line}`);
    }
    const boundary = end || (!inString && depth === 0 && /\s/.test(ch));
    if (boundary) {
      if (start < i) items.push(inner.slice(start, i));
      start = i + 1;
    }
  }
  if (inString || depth !== 0) throw new EvaluatorFault(`the C backend rejects the line: ${line}`);
  return items;
};

/** The operand of an operation or assignment: a register, a constant
 * slot, or a label address. */
const operandC = (item: string, consts: string[], labels: ReadonlySet<string>): string => {
  const fail = (): Error => new EvaluatorFault(`the C backend rejects the operand: ${item}`);
  if (item.startsWith("(reg ") && item.endsWith(")")) return `R_${item.slice(5, -1)}`;
  if (item.startsWith("(label ") && item.endsWith(")")) return `&&${cLabel(item.slice(7, -1))}`;
  if (item.startsWith("(const ") && item.endsWith(")")) {
    const payload = item.slice(7, -1);
    if (labels.has(payload)) return `&&${cLabel(payload)}`;
    const index = consts.indexOf(payload);
    if (index >= 0) return `K${index}`;
    consts.push(payload);
    return `K${consts.length - 1}`;
  }
  throw fail();
};

const cString = (text: string): string =>
  `"${text.replaceAll("\\", "\\\\").replaceAll('"', '\\"').replaceAll("\n", "\\n")}"`;

/** The value a `(const ...)` spelling names: the reader parses the
 * spelling and the one leading quote the writer printed is stripped,
 * exactly one level. An inner quote print inside a datum is itself a
 * datum, a `(quote x)` list, and must survive. */
const constDatum = (spelling: string): Word => {
  const words = parse(spelling);
  const word = words[0];
  if (word === undefined) throw new EvaluatorFault(`the C backend rejects: ${spelling}`);
  if (typeof word === "object" && word !== null && "car" in word) {
    const items = flatten(word as PairWord);
    const head = items[0];
    if (items.length === 2 && head !== undefined && isSymNamed(head, "quote")) {
      const datum = items[1];
      if (datum === undefined) throw new EvaluatorFault("the C backend rejects an empty quote");
      return datum;
    }
  }
  return word;
};

const isSymNamed = (word: Word, name: string): boolean =>
  typeof word === "object" &&
  word !== null &&
  "symbol" in word &&
  !("car" in word) &&
  !("wordTag" in word) &&
  word.symbol === name;

const flatten = (word: PairWord): Word[] => {
  const out: Word[] = [];
  let p: Word = word;
  while (typeof p === "object" && p !== null && "car" in p && "cdr" in p) {
    out.push(p.car);
    p = p.cdr;
  }
  return out;
};

/** A C expression building one compile-time constant value. */
const constC = (word: Word): string => {
  if (typeof word === "number") return `mk_int(${word}LL)`;
  if (word === true) return "V_TRUE";
  if (word === false) return "V_FALSE";
  if (typeof word === "object" && word !== null && "car" in word) {
    return `cons(${constC(word.car)}, ${constC(word.cdr)})`;
  }
  if (typeof word === "object" && word !== null && "symbol" in word && !("wordTag" in word)) {
    if (word.symbol === "nil") return "V_NIL";
    return `mk_sym(${cString(word.symbol)})`;
  }
  throw new EvaluatorFault(`the C backend rejects the constant: ${formatWord(word)}`);
};

/** One controller statement as C lines, with entry_open remembering
 * that a compiled-procedure entry was just read into entry_addr. */
const statementC = (line: string, consts: string[], labels: ReadonlySet<string>): string[] => {
  const fail = (): Error => new EvaluatorFault(`the C backend rejects the line: ${line}`);
  if (!line.startsWith("(")) return [`${cLabel(line)}:;`];
  const items = splitTop(line);
  const head = items[0] ?? "";
  if (head === "save") {
    const regName = items[1];
    if (regName === undefined) throw fail();
    return [regName === "continue" ? "push_addr(R_continue);" : `push_val(R_${regName});`];
  }
  if (head === "restore") {
    const regName = items[1];
    if (regName === undefined) throw fail();
    return [regName === "continue" ? "R_continue = pop_addr();" : `R_${regName} = pop_val();`];
  }
  if (head === "goto") {
    const target = items[1];
    if (target === undefined) throw fail();
    if (target.startsWith("(label ") && target.endsWith(")"))
      return [`goto ${cLabel(target.slice(7, -1))};`];
    if (target === "(reg continue)") return ["goto *R_continue;"];
    if (target === "(reg val)") return ["goto *entry_addr;"];
    throw fail();
  }
  if (head === "branch") {
    const target = items[1];
    if (target === undefined || !target.startsWith("(label ") || !target.endsWith(")"))
      throw fail();
    return [`if (flag) goto ${cLabel(target.slice(7, -1))};`];
  }
  if (head === "test" || head === "perform") {
    const opItem = items[1];
    if (opItem === undefined || !opItem.startsWith("(op ") || !opItem.endsWith(")")) throw fail();
    const name = cOp(opItem.slice(4, -1));
    const args = items.slice(2).map((item) => operandC(item, consts, labels));
    const call = `${name}(${args.join(", ")})`;
    return [head === "test" ? `flag = is_true(${call});` : `(void)${call};`];
  }
  if (head === "assign") {
    const target = items[1];
    const source = items[2];
    if (target === undefined || source === undefined) throw fail();
    if (source.startsWith("(reg ") && source.endsWith(")"))
      return [`R_${target} = R_${source.slice(5, -1)};`];
    if (source.startsWith("(label ") && source.endsWith(")")) {
      if (target !== "continue") throw fail();
      return [`R_continue = &&${cLabel(source.slice(7, -1))};`];
    }
    if (source.startsWith("(const ")) return [`R_${target} = ${operandC(source, consts, labels)};`];
    if (source.startsWith("(op ")) {
      const name = source.slice(4, -1);
      if (name === "make-compiled-procedure") {
        const entryItem = items[3];
        const envItem = items[4];
        if (entryItem === undefined || envItem === undefined) throw fail();
        const entry = operandC(entryItem, consts, labels);
        const env = operandC(envItem, consts, labels);
        return [`R_${target} = mk_compiled(${entry}, ${env});`];
      }
      if (name === "compiled-procedure-entry") {
        const procItem = items[3];
        if (procItem === undefined) throw fail();
        const proc = operandC(procItem, consts, labels);
        return [`entry_addr = compiled_entry(${proc});`];
      }
      const args = items.slice(3).map((item) => operandC(item, consts, labels));
      return [`R_${target} = ${cOp(name)}(${args.join(", ")});`];
    }
    throw fail();
  }
  throw fail();
};

/** The whole C file: the runtime, the constant initializer, and the
 * compiled forms with a print between neighbors. */
export const compileToC = (source: string): string => {
  const cfg = defaultConfig();
  const state = newState();
  const consts: string[] = [];
  const bodies: string[][] = [];
  for (const form of parse(source)) {
    const seq = compile(cfg, state, topCenv(), form, "val", LinkageNext);
    bodies.push(seq.stmts.map(renderStatement));
  }
  const labels = new Set<string>(bodies.flat().filter((line) => !line.startsWith("(")));
  const program: string[] = [];
  bodies.forEach((stmts, index) => {
    program.push(`prog_entry_${index}:;`);
    for (const line of stmts) program.push(...statementC(line, consts, labels));
    program.push("user_print(R_val); emit_newline();");
  });
  const initializers = consts.map((spelling, index) => {
    return `K${index} = ${constC(constDatum(spelling))};`;
  });
  const declarations = consts.map((_, index) => `static Val *K${index};`);
  return `${RUNTIME_C()}
${declarations.join("\n")}
static void init_constants(void) {
${initializers.join("\n")}
}
int main(void) {
init_symbols();
init_global();
init_constants();
${program.join("\n")}
return 0;
}
`;
};

/** Exercise 5.52: compile the adapted metacircular source to C, build
 * it with the system compiler, and run the object session through the
 * C-compiled interpreter: factorial answers 120 and the tick
 * combination answers its triple. */
export const ex_5_52 = (): readonly string[] => {
  const cSource = compileToC(METACIRCULAR());
  const dir = mkdtempSync(join(tmpdir(), "sicp_ts_5_52_"));
  try {
    const source = join(dir, "compiled.c");
    const binary = join(dir, "compiled");
    writeFileSync(source, cSource);
    const build = spawnSync("cc", ["-O1", "-o", binary, source], { encoding: "utf8" });
    if (build.status !== 0) throw new Error(`the C backend failed to build: ${build.stderr}`);
    const run = spawnSync(binary, [], { encoding: "utf8" });
    const output = `${run.stdout}${run.stderr}`;
    if (run.status !== 0) throw new Error(`the C program failed: ${output}`);
    if (!output.includes("120")) throw new Error(`the C session lost 120: ${output}`);
    if (!output.includes("(tick tick tick)"))
      throw new Error(`the C session lost the tick: ${output}`);
    return [output];
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
};
