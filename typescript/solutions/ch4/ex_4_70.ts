// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

export function addAssertionBinding<T>(value: T, extend: (x: T) => T): () => T {
  const added = extend(value);
  return () => added;
}
export function ex_4_70() {
  let calls = 0;
  const tail = addAssertionBinding("fact", (x) => {
    calls++;
    return x;
  });
  tail();
  return (
    "The let binding extends once before constructing the delayed tail; without it, a self-referential stream may repeat insertion forever. Calls: " +
    calls
  );
}
