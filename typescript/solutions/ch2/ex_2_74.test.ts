// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  type DivisionFile,
  findEmployeeRecord,
  getRecord,
  getSalary,
  headquarters,
  installEastDivision,
} from "./ex_2_74.js";

const northFile: DivisionFile = {
  _tag: "north",
  contents: [
    ["Bitdiddle Ben", { address: "Sunset Road", salary: 60000 }],
    ["Cratchet Robert", { address: "Alder Lane", salary: 30000 }],
  ],
};

const southFile: DivisionFile = {
  _tag: "south",
  contents: [
    [
      ["name", "Hacker Alyssa P"],
      ["address", "Cambridge St"],
      ["salary", 50000],
    ],
    [
      ["name", "Fecter Cy D"],
      ["address", "Beacon St"],
      ["salary", 45000],
    ],
  ],
};

const allFiles = [northFile, southFile];

describe("exercise 2.74: Insatiable Enterprises' division files", () => {
  it("parts a and b: getRecord and getSalary work across layouts", () => {
    const bensRecord = getRecord("Bitdiddle Ben", northFile);
    expect(bensRecord._tag).toBe("Some");
    if (bensRecord._tag === "Some") {
      expect(bensRecord.value._tag).toBe("north");
    }
    // Same extraction, different layout: the north map and the south
    // property list both answer the salary through the table.
    const bens = getRecord("Bitdiddle Ben", northFile);
    expect(bens._tag === "Some" ? getSalary(bens.value) : undefined).toEqual({
      _tag: "Some",
      value: 60000,
    });
    const alyssa = getRecord("Hacker Alyssa P", southFile);
    expect(alyssa._tag === "Some" ? getSalary(alyssa.value) : undefined).toEqual({
      _tag: "Some",
      value: 50000,
    });
  });

  it("a miss in one division's layout answers nothing", () => {
    expect(getRecord("Hacker Alyssa P", northFile)._tag).toBe("None");
    expect(getRecord("Bitdiddle Ben", southFile)._tag).toBe("None");
    expect(getRecord("Nobody", northFile)._tag).toBe("None");
  });

  it("part c: findEmployeeRecord searches every division's file", () => {
    const found = findEmployeeRecord("Fecter Cy D", allFiles);
    expect(found._tag).toBe("Some");
    if (found._tag === "Some") {
      expect(found.value._tag).toBe("south");
      expect(getSalary(found.value)).toEqual({ _tag: "Some", value: 45000 });
    }
    expect(findEmployeeRecord("Nobody", allFiles)._tag).toBe("None");
  });

  it("part d: a third division needs only an install", () => {
    const before = headquarters.get("get-record");
    installEastDivision(headquarters);
    const after = headquarters.get("get-record");
    // The founding divisions' installed handlers are untouched by the
    // acquisition, by reference.
    expect(before).toBe(after);
    expect(after?.get("north")).toBe(before?.get("north"));

    const eastFile: DivisionFile = {
      _tag: "east",
      contents: { "Wheeler Julie": { address: "Elm St", salary: 72000 } },
    };
    const found = findEmployeeRecord("Wheeler Julie", [...allFiles, eastFile]);
    expect(found._tag).toBe("Some");
    if (found._tag === "Some") {
      expect(found.value._tag).toBe("east");
      expect(getSalary(found.value)).toEqual({ _tag: "Some", value: 72000 });
    }
    // headquarters' own procedures never mentioned the east tag: they
    // dispatch through the table, which learned it at install time.
    expect(getRecord("Bitdiddle Ben", northFile)._tag).toBe("Some");
    expect(getSalary({ _tag: "north", contents: { salary: 60000 } })).toEqual({
      _tag: "Some",
      value: 60000,
    });
  });
});
