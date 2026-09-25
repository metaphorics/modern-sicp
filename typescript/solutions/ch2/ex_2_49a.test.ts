// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";

import {
  rogersIdentitySvg,
  transformedFrame,
  waveIdentitySvg,
  waveTransformedSvg,
} from "./ex_2_49a.js";

describe("exercise 2.49a", () => {
  it("the wave pin under the identity frame is the checked-in figure, byte for byte", () => {
    const figure = readFileSync(
      new URL("../../book/figures/generated/chap2/wave.std.svg", import.meta.url),
      "utf8",
    );
    expect(waveIdentitySvg()).toBe(figure);
  });

  it("the rogers pin under the identity frame is this exact document", () => {
    expect(rogersIdentitySvg()).toBe(
      [
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">',
        '<rect width="200" height="200" fill="white"/>',
        '<g stroke="#1a1a1a" stroke-width="1.2" stroke-linecap="round" fill="none">',
        '<line x1="60.00" y1="10.00" x2="50.00" y2="40.00"/>',
        '<line x1="50.00" y1="40.00" x2="60.00" y2="76.00"/>',
        '<line x1="60.00" y1="76.00" x2="100.00" y2="90.00"/>',
        '<line x1="100.00" y1="90.00" x2="140.00" y2="76.00"/>',
        '<line x1="140.00" y1="76.00" x2="150.00" y2="40.00"/>',
        '<line x1="150.00" y1="40.00" x2="140.00" y2="10.00"/>',
        '<line x1="140.00" y1="10.00" x2="100.00" y2="0.00"/>',
        '<line x1="100.00" y1="0.00" x2="60.00" y2="10.00"/>',
        '<line x1="72.00" y1="30.00" x2="84.00" y2="30.00"/>',
        '<line x1="116.00" y1="30.00" x2="128.00" y2="30.00"/>',
        '<line x1="100.00" y1="40.00" x2="100.00" y2="64.00"/>',
        '<line x1="88.00" y1="68.00" x2="112.00" y2="68.00"/>',
        '<line x1="100.00" y1="90.00" x2="100.00" y2="110.00"/>',
        '<line x1="40.00" y1="140.00" x2="100.00" y2="110.00"/>',
        '<line x1="160.00" y1="140.00" x2="100.00" y2="110.00"/>',
        '<line x1="40.00" y1="140.00" x2="30.00" y2="200.00"/>',
        '<line x1="160.00" y1="140.00" x2="170.00" y2="200.00"/>',
        "</g>",
        "</svg>",
        "",
      ].join("\n"),
    );
  });

  it("the wave pin under the transformed frame is this exact document", () => {
    expect(waveTransformedSvg()).toBe(
      [
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200" width="200" height="200">',
        '<rect width="200" height="200" fill="white"/>',
        '<g stroke="#1a1a1a" stroke-width="1.2" stroke-linecap="round" fill="none">',
        '<line x1="37.00" y1="98.00" x2="51.60" y2="125.60"/>',
        '<line x1="51.60" y1="125.60" x2="81.60" y2="118.40"/>',
        '<line x1="81.60" y1="118.40" x2="101.60" y2="113.60"/>',
        '<line x1="101.60" y1="113.60" x2="117.20" y2="116.00"/>',
        '<line x1="117.20" y1="116.00" x2="128.00" y2="128.00"/>',
        '<line x1="128.00" y1="128.00" x2="133.00" y2="146.00"/>',
        '<line x1="133.00" y1="146.00" x2="188.00" y2="152.00"/>',
        '<line x1="81.60" y1="118.40" x2="71.00" y2="134.00"/>',
        '<line x1="71.00" y1="134.00" x2="33.00" y2="122.00"/>',
        '<line x1="128.00" y1="128.00" x2="147.60" y2="125.60"/>',
        '<line x1="147.60" y1="125.60" x2="167.00" y2="134.00"/>',
        '<line x1="93.00" y1="146.00" x2="91.20" y2="176.00"/>',
        '<line x1="91.20" y1="176.00" x2="76.00" y2="200.00"/>',
        '<line x1="93.00" y1="146.00" x2="117.00" y2="146.00"/>',
        '<line x1="117.00" y1="146.00" x2="120.00" y2="176.00"/>',
        '<line x1="120.00" y1="176.00" x2="132.00" y2="200.00"/>',
        '<line x1="83.60" y1="125.60" x2="75.80" y2="134.00"/>',
        '<line x1="88.00" y1="137.60" x2="104.00" y2="137.60"/>',
        "</g>",
        "</svg>",
        "",
      ].join("\n"),
    );
  });

  it("the transformed frame's map is the pin's premise", () => {
    expect(transformedFrame.origin).toStrictEqual({ x: 0.1, y: 0 });
    expect(transformedFrame.edge1).toStrictEqual({ x: 0.8, y: 0 });
    expect(transformedFrame.edge2).toStrictEqual({ x: 0.1, y: 0.6 });
  });

  it("the pins are stable across repeated renders", () => {
    expect(waveIdentitySvg()).toBe(waveIdentitySvg());
    expect(rogersIdentitySvg()).toBe(rogersIdentitySvg());
    expect(waveTransformedSvg()).toBe(waveTransformedSvg());
  });
});
