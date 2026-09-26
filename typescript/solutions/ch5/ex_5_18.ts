// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  getRegisterContents,
  type Machine,
  makeMachine,
  makeRegister,
  type Register,
  renderValue,
  setRegisterContents,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";

/** A register that reports every store: name, old content, new content.
 * The flag stays an ordinary register, so only the controller's
 * registers report. */
export const makeTracedRegister = (name: string, sink: string[]): Register => {
  const base = makeRegister(name);
  return {
    name,
    content: base.content,
    store: (value) => {
      sink.push(`${name}: ${renderValue(base.content())} -> ${renderValue(value)}`);
      base.store(value);
    },
  };
};

const runGcd = (a: number, b: number, makeRegisterFactory: (name: string) => Register): Machine => {
  const machine = expectOk(
    makeMachine(["a", "b", "t"], arithmeticOperations, gcdController, {
      makeRegister: makeRegisterFactory,
    }),
  );
  expectOk(setRegisterContents(machine, "a", a));
  expectOk(setRegisterContents(machine, "b", b));
  expectOk(machine.start());
  return machine;
};

/** The machine whose named registers are traced registers; the log line
 * "gcd(12, 8) = 4" closes the report. */
export const tracedRegisterGcdLog = (): string[] => {
  const log: string[] = [];
  const machine = runGcd(12, 8, (name) => makeTracedRegister(name, log));
  log.push(`gcd(12, 8) = ${renderValue(expectOk(getRegisterContents(machine, "a")))}`);
  return log;
};
