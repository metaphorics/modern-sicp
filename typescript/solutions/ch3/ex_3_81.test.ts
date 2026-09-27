// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  consStream,
  randomNumbers,
  type Stream,
  streamRef,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { type RandRequest, randStream } from "./ex_3_81.js";

const ofRequests = (rs: RandRequest[]): Stream<RandRequest> =>
  rs.reduceRight<Stream<RandRequest>>((tail, head) => consStream(head, () => tail), null);

const generate: RandRequest = "generate";
const reset = (seed: number): RandRequest => ({ kind: "reset", seed });

describe("exercise 3.81: the request-driven rand stream", () => {
  it("answers generate by emitting rand-update of the carried seed", () => {
    expect(streamTake(randStream(ofRequests([generate, generate, generate])), 3)).toEqual([
      11355432, 2836018348, 476557059,
    ]);
  });

  it("answers reset by emitting the new seed and replaying from it", () => {
    const mixed = streamTake(randStream(ofRequests([generate, generate, reset(42), generate])), 4);
    const fresh = streamTake(randStream(ofRequests([generate, generate])), 2);
    expect(mixed).toEqual([11355432, 2836018348, 42, 11355432]);
    expect(fresh).toEqual([11355432, 2836018348]);
    expect(mixed[3]).toBe(fresh[0]);
  });

  it("matches the module's randomNumbers on pure generates", () => {
    const numbers = streamTake(randStream(ofRequests([generate, generate, generate])), 3);
    expect(numbers).toEqual([11355432, 2836018348, 476557059]);
    expect(numbers).toEqual([
      streamRef(randomNumbers, 1),
      streamRef(randomNumbers, 2),
      streamRef(randomNumbers, 3),
    ]);
  });

  it("replays the same numbers for the same requests, with no assignment", () => {
    const first = streamTake(randStream(ofRequests([generate, reset(7), generate, generate])), 4);
    const second = streamTake(randStream(ofRequests([generate, reset(7), generate, generate])), 4);
    expect(first).toEqual(second);
    expect(first).toEqual([11355432, 7, 1892583, 470389255]);
  });
});
