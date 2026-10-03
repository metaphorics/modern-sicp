// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.26: Louis Reasoner replaced `square(expmod(base, exp / 2, m))`
 * with two separate self-calls multiplied together. In an eager host each
 * argument is evaluated before the multiplication, so every even step
 * spawns two subproblems of half the exponent instead of one: the
 * recursion tree has a node per 1 in the binary expansion times two per
 * level - Theta(n) calls against Theta(log n). The counters make the
 * blow-up a number.
 */
export const expmodExplicit = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (exp % 2 === 0) {
    return (expmodExplicit(base, exp / 2, m) * expmodExplicit(base, exp / 2, m)) % m;
  }
  return (base * expmodExplicit(base, exp - 1, m)) % m;
};

export function expmodSquareCalls(exp: number): number {
  let calls = 0;
  const rec = (e: number): number => {
    calls += 1;
    if (e === 0) {
      return 1;
    }
    if (e % 2 === 0) {
      const squared = (x: number): number => x * x;
      return squared(rec(e / 2)) % 561;
    }
    return (2 * rec(e - 1)) % 561;
  };
  rec(exp);
  return calls;
}

export function expmodExplicitCalls(exp: number): number {
  let calls = 0;
  const rec = (e: number): number => {
    calls += 1;
    if (e === 0) {
      return 1;
    }
    if (e % 2 === 0) {
      return (rec(e / 2) * rec(e / 2)) % 561;
    }
    return (2 * rec(e - 1)) % 561;
  };
  rec(exp);
  return calls;
}
