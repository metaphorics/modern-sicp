// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";
import { wheelRule } from "./ex_4_65.js";

export interface AccumulationCounts {
  readonly proofCount: number;
  readonly distinctAnswerCount: number;
}

/** Counts proof frames and distinct instantiated answers without conflating them. */
export const wheelAccumulationCounts = (): AccumulationCounts => {
  const engine = microshaft();
  engine.load(wheelRule);
  const frames = engine.answers("(wheel ?who)");
  return { proofCount: frames.length, distinctAnswerCount: new Set(frames).size };
};

export function ex_4_66(): string {
  const { proofCount, distinctAnswerCount } = wheelAccumulationCounts();
  return `A stream contains one frame per proof, not one per distinct person: direct accumulation would count ${proofCount} wheel proofs where only ${distinctAnswerCount} distinct answers exist, so deduplicate by the instantiated value of the accumulated variable before applying sum, average, or maximum.`;
}
