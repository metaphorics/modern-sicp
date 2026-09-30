// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/** The compile-time environment of exercise 5.40 and 5.41: frames of
 * parameter names, newest first, threaded through the code generators
 * so every variable reference can be answered at compile time. */
export type CompileTimeEnv = ReadonlyArray<ReadonlyArray<string>>;

/** The find-variable answer: a lexical address, or the fact that the
 * name is free. */
export type VariableAddress =
  | { readonly found: true; readonly frame: number; readonly position: number }
  | { readonly found: false };

/** The exercise's traversal: walk the frames outward and the names
 * inward, and answer the first hit as a lexical address. */
export const findVariable = (name: string, env: CompileTimeEnv): VariableAddress => {
  for (let frame = 0; frame < env.length; frame += 1) {
    const names = env[frame] ?? [];
    const position = names.indexOf(name);
    if (position >= 0) {
      return { found: true, frame, position };
    }
  }
  return { found: false };
};

/** Exercise 5.41's probes over the book's example environment. */
export const ex_5_41 = (): readonly string[] => {
  const env: CompileTimeEnv = [
    ["a", "b", "c", "d", "e"],
    ["y", "z"],
    ["x", "y"],
  ];
  const probes = ["a", "y", "z", "x", "free"];
  return probes.map((name) => {
    const address = findVariable(name, env);
    return address.found
      ? `${name} -> frame ${address.frame}, position ${address.position}`
      : `${name} -> free`;
  });
};
