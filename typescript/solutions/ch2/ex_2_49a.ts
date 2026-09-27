// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Frame,
  makeFrame,
  makeVect,
  renderSvgFrame,
  rogers,
  unitSquare,
  wave,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.49a: the primitive painters render to SVG strings. The
 * renderer fixes the attribute order and the two-decimal precision, so
 * the documents below are exact values a test can pin byte for byte.
 */

/** The one transformed frame the pins use: a parallelogram squashed
 * into the left-center of the unit square. */
export const transformedFrame: Frame = makeFrame(
  makeVect(0.1, 0),
  makeVect(0.8, 0),
  makeVect(0.1, 0.6),
);

/** The wave painter's SVG document under the identity frame. */
export const waveIdentitySvg = (): string => renderSvgFrame(wave(), unitSquare, 200);

/** The rogers painter's SVG document under the identity frame. */
export const rogersIdentitySvg = (): string => renderSvgFrame(rogers(), unitSquare, 200);

/** The wave painter's SVG document under the transformed frame. */
export const waveTransformedSvg = (): string => renderSvgFrame(wave(), transformedFrame, 200);
