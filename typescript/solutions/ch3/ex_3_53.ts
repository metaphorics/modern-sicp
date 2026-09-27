// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  addStreams,
  consStream,
  type StreamCell,
  scaleStream,
} from "../../packages/ch3/src/05-streams.js";

/**
 * Exercise 3.53: the statement defines, without running it,
 *
 *   (define s (cons-stream 1 (add-streams s s)))
 *
 * and asks what the elements of `s` are. The answer is the powers of
 * 2. The head of `s` is 1. Every later element is the sum of the two
 * copies of `s` fed to `add-streams`, which at position n is
 * s[n-1] + s[n-1], twice the element before. So s[n] = 2^n: 1, 2, 4,
 * 8, 16, ... The tests run the definition and pin that prediction.
 */

/** The statement's stream: each element is the stream added to
 * itself, so every element after the head is twice the one before. */
export const s: StreamCell<number> = consStream(1, () => addStreams(s, s));

/** The same stream read as a doubling: 1 followed by `double` scaled
 * by 2. Element-wise it agrees with the statement's `s`, which the
 * tests check; the equivalence is the description the exercise asks
 * for, spelled as a second construction. */
export const double: StreamCell<number> = consStream(1, () => scaleStream(double, 2));
