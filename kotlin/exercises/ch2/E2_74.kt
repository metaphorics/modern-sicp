// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.74

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.74: Insatiable Enterprises' divisions each keep personnel
 * files in a genuinely different internal structure. Headquarters needs
 * `get-record` and `get-salary` procedures that work across every
 * division's file without knowing its structure in advance, plus a
 * `find-employee-record` that searches every division.
 *
 * (a) Implement `getRecord`: it must be applicable to any division's
 * file. Each division's file must carry enough type information for
 * `getRecord` to dispatch to that division's own lookup.
 *
 * (b) Implement `getSalary`: the record `getRecord` returns must carry
 * the same kind of type information, since the record's internal shape
 * still varies by division.
 *
 * (c) Implement `findEmployeeRecord`, searching a list of all the
 * divisions' files for one employee's record.
 *
 * (d) When Insatiable takes over a new company, headquarters needs one
 * new adapter package for the new division's format, installed the same
 * way as an existing one; no code that already works needs to change.
 *
 * The scaffold returns `(benSalary, alyssaSalary, missingIsFound)` for
 * two employees in two divisions built with genuinely different internal
 * structures, plus whether a name absent from every division is found.
 */
public fun ex_2_74(): Triple<String?, String?, Boolean> = throw PendingSolution()
