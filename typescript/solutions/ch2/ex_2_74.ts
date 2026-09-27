// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { none, type Option, some } from "../../packages/ch2/src/02-picture-language.js";
import { get, makeOpTable, type OpTable, put } from "../../packages/ch2/src/04-data-directed.js";

/**
 * Exercise 2.74: Insatiable Enterprises' division files, integrated by
 * data-directed programming. Each division's file and each employee
 * record is a tagged datum --- a discriminated union member --- and each
 * division installs `get-record` and `get-salary` under its own tag in
 * headquarters' table. The layouts genuinely differ: the north division
 * keys its file by name and holds each record as a field map; the south
 * division keeps a flat list of records, each a property list keyed by
 * field name. Headquarters never sees a layout, only tags. The
 * TypeScript override for this row rules out `Schema`: the tags are
 * plain union discriminants (D33).
 */

/** A field value in any division's records. */
export type FieldValue = string | number;

/** The north division: a file is a name-indexed list of pairs, and a
 * record is a plain object keyed by field name. */
export type NorthRecord = Readonly<Record<string, FieldValue>>;
export type NorthFile = ReadonlyArray<readonly [name: string, record: NorthRecord]>;

/** The south division: a flat list of records, each a property list of
 * field name and value pairs. */
export type SouthField = readonly [field: string, value: FieldValue];
export type SouthRecord = ReadonlyArray<SouthField>;
export type SouthFile = ReadonlyArray<SouthRecord>;

/** The east division, added in part (d): a name-indexed map of records,
 * each record an object with named properties. */
export type EastRecord = { readonly address: string; readonly salary: number };
export type EastFile = Readonly<Record<string, EastRecord>>;

/** An employee record, tagged by division: headquarters dispatches on
 * the tag and each division reads its own layout. */
export type EmployeeRecord =
  | { readonly _tag: "north"; readonly contents: NorthRecord }
  | { readonly _tag: "south"; readonly contents: SouthRecord }
  | { readonly _tag: "east"; readonly contents: EastRecord };

/** A division's personnel file, tagged by division. */
export type DivisionFile =
  | { readonly _tag: "north"; readonly contents: NorthFile }
  | { readonly _tag: "south"; readonly contents: SouthFile }
  | { readonly _tag: "east"; readonly contents: EastFile };

/** A division's installed procedures. Each division answers for its own
 * tag only; the off-tag arm is unreachable when dispatch went through
 * the table, and answers no record if it ever ran. */
export type DivisionHandler =
  | {
      readonly _tag: "GetRecord";
      readonly fn: (name: string, file: DivisionFile) => Option<EmployeeRecord>;
    }
  | {
      readonly _tag: "GetSalary";
      readonly fn: (record: EmployeeRecord) => Option<number>;
    };

/** Headquarters' operation-and-type table: `get-record` and `get-salary`
 * keyed by division tag. */
export type DivisionTable = OpTable<DivisionHandler>;

/** The north layout's named record: the pair whose key is `name`. */
const recordOfNorth = (name: string, file: NorthFile): Option<NorthRecord> => {
  const pair = file.find(([key]) => key === name);
  return pair === undefined ? none : some(pair[1]);
};

/** The north layout's salary field, read from the field map. */
const salaryOfNorth = (record: NorthRecord): Option<number> => {
  const { salary: value } = record;
  return typeof value === "number" ? some(value) : none;
};

/** The south layout's named record: the first property list whose
 * `name` property matches. */
const recordOfSouth = (name: string, file: SouthFile): Option<SouthRecord> => {
  const found = file.find((record) =>
    record.some(([field, value]) => field === "name" && value === name),
  );
  return found === undefined ? none : some(found);
};

/** The south layout's salary field: the property list is scanned for
 * the `salary` key. */
const salaryOfSouth = (record: SouthRecord): Option<number> => {
  const field = record.find(([name]) => name === "salary");
  return field === undefined ? none : typeof field[1] === "number" ? some(field[1]) : none;
};

/** Installs the north division's two procedures under its tag. */
export const installNorthDivision = (table: DivisionTable): void => {
  put(table, "get-record", ["north"], {
    _tag: "GetRecord",
    fn: (name, file) => {
      if (file._tag !== "north") {
        return none;
      }
      const record = recordOfNorth(name, file.contents);
      return record._tag === "Some" ? some({ _tag: "north", contents: record.value }) : none;
    },
  });
  put(table, "get-salary", ["north"], {
    _tag: "GetSalary",
    fn: (record) => (record._tag === "north" ? salaryOfNorth(record.contents) : none),
  });
};

/** Installs the south division's two procedures under its tag. */
export const installSouthDivision = (table: DivisionTable): void => {
  put(table, "get-record", ["south"], {
    _tag: "GetRecord",
    fn: (name, file) => {
      if (file._tag !== "south") {
        return none;
      }
      const record = recordOfSouth(name, file.contents);
      return record._tag === "Some" ? some({ _tag: "south", contents: record.value }) : none;
    },
  });
  put(table, "get-salary", ["south"], {
    _tag: "GetSalary",
    fn: (record) => (record._tag === "south" ? salaryOfSouth(record.contents) : none),
  });
};

/** Part (d): a newly acquired company installs its own two procedures
 * under a third tag; no headquarters procedure changes. */
export const installEastDivision = (table: DivisionTable): void => {
  put(table, "get-record", ["east"], {
    _tag: "GetRecord",
    fn: (name, file) => {
      if (file._tag !== "east") {
        return none;
      }
      const record = file.contents[name];
      return record === undefined ? none : some({ _tag: "east", contents: record });
    },
  });
  put(table, "get-salary", ["east"], {
    _tag: "GetSalary",
    fn: (record) => (record._tag === "east" ? some(record.contents.salary) : none),
  });
};

/** Headquarters' table with the two founding divisions installed. */
export const headquarters: DivisionTable = makeOpTable();
installNorthDivision(headquarters);
installSouthDivision(headquarters);

/** The book's get-record: the named employee's record from any
 * division's file. */
export const getRecord = (name: string, file: DivisionFile): Option<EmployeeRecord> => {
  const handler = get(headquarters, "get-record", [file._tag]);
  if (handler._tag === "None" || handler.value._tag !== "GetRecord") {
    return none;
  }
  return handler.value.fn(name, file);
};

/** The book's get-salary: the salary in a record from any division. */
export const getSalary = (record: EmployeeRecord): Option<number> => {
  const handler = get(headquarters, "get-salary", [record._tag]);
  if (handler._tag === "None" || handler.value._tag !== "GetSalary") {
    return none;
  }
  return handler.value.fn(record);
};

/** The book's find-employee-record: searches every division's file and
 * answers the first record found, or nothing. */
export const findEmployeeRecord = (
  name: string,
  files: ReadonlyArray<DivisionFile>,
): Option<EmployeeRecord> => {
  for (const file of files) {
    const record = getRecord(name, file);
    if (record._tag === "Some") {
      return record;
    }
  }
  return none;
};
