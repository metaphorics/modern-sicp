// scratch smoke: pinned typescript/unstable/sync API mechanics (deleted after run)
import { mkdirSync, writeFileSync } from "node:fs";
import { API } from "typescript/unstable/sync";

mkdirSync("/tmp/tsprobe", { recursive: true });
writeFileSync(
  "/tmp/tsprobe/probe.ts",
  'const x = 1;\nif (x) {}\nconst b = true;\nif (b) {}\nconst s = "a";\nconst sum = x + s;\n',
);
writeFileSync(
  "/tmp/tsprobe/tsconfig.json",
  JSON.stringify({
    compilerOptions: {
      strict: true,
      target: "es2024",
      lib: ["es2024"],
      types: [],
      module: "nodenext",
      moduleResolution: "nodenext",
    },
    include: ["*.ts"],
  }),
);

const api = new API({ cwd: "/tmp/tsprobe" });
const snap = api.updateSnapshot({ openProjects: ["/tmp/tsprobe/tsconfig.json"] });
const project = snap.getProject("/tmp/tsprobe/tsconfig.json");
console.log("project?", Boolean(project));
const checker = project.checker;
const file = "/tmp/tsprobe/probe.ts";
console.log(
  "checker methods:",
  Object.getOwnPropertyNames(Object.getPrototypeOf(checker)).join(","),
);
const t1 = checker.getTypeAtPosition(file, 17);
const t2 = checker.getTypeAtPosition(file, 43);
const t3 = checker.getTypeAtPosition(file, 60);
console.log(
  "t1:",
  t1 && t1.flags,
  t1 && t1.isErrorType(),
  checker.typeToString && checker.typeToString(t1),
);
console.log("t2:", t2 && t2.flags, checker.typeToString && checker.typeToString(t2));
console.log("t3:", t3 && t3.flags, checker.typeToString && checker.typeToString(t3));
console.log(
  "bool type:",
  checker.getBooleanType().flags,
  "number:",
  checker.getNumberType().flags,
  "str:",
  checker.getStringType().flags,
);
console.log("assignable num<-t1:", checker.isTypeAssignableTo(t1, checker.getNumberType()));
console.log("assignable bool<-t1:", checker.isTypeAssignableTo(t1, checker.getBooleanType()));
const diags = project.program.getSemanticDiagnostics(file);
console.log(
  "diags:",
  diags.map((d) => `${d.code}@${d.start}+${d.length}: ${d.message}`).join(" | "),
);
api.close();
console.log("closed ok");
