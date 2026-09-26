// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { microshaft } from "../../packages/ch4/src/04-logic.js";

export const genealogy = `
(son Adam Cain) (son Cain Enoch) (son Enoch Irad)
(son Irad Mehujael) (son Mehujael Methushael) (son Methushael Lamech)
(wife Lamech Ada) (son Ada Jabal) (son Ada Jubal)
(rule (grandson ?grandparent ?grandchild)
  (and (son ?grandparent ?parent) (son ?parent ?grandchild)))
(rule (son ?father ?child)
  (and (wife ?father ?mother) (son ?mother ?child)))
`;

export const genealogyAnswers = (): readonly [
  ReadonlyArray<string>,
  ReadonlyArray<string>,
  ReadonlyArray<string>,
] => {
  const engine = microshaft();
  engine.load(genealogy);
  return [
    engine.answers("(grandson Cain ?who)"),
    engine.answers("(son Lamech ?who)"),
    engine.answers("(grandson Methushael ?who)"),
  ];
};

export function ex_4_63(): string {
  const [cainsGrandsons, lamechsSons, methushaelsGrandsons] = genealogyAnswers();
  return `Cain's grandson: ${cainsGrandsons.join(", ")}; Lamech's sons: ${lamechsSons.join(", ")}; Methushael's grandsons: ${methushaelsGrandsons.join(", ")}.`;
}
