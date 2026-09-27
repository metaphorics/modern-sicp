# The Scheme subset printer

;; SPDX-License-Identifier: GPL-3.0-only
;; Adapted from SICP sections 4.1 to 5.2

This document fixes the printed form of every value of the Scheme subset and
the shape of a program's output. Every evaluator that runs a capability in
`manifest.txt` must produce exactly these bytes, so the expected files under
`expected/` are byte-comparable across the four editions. All output uses LF
line endings, and the last output line ends with a final LF.

## The output of a program

A program is a sequence of top-level forms. The output is:

- A top-level `define` prints nothing.
- Every other top-level expression prints the printed form of its value as
  one line.
- Anything `display` or `newline` emits appears inline, in order, interleaved
  with the value lines.
- On an `error` call, the `Error:` line is printed as fixed below, execution
  stops, and no later form produces output.
- Driver chatter from the book's transcripts, such as `;;; Amb-Eval input:`,
  `;;; Query input:`, `;;; Eval value:`, or `;;; Starting a new problem`, is
  not printed.

In capability `amb`, a top-level form that is the bare symbol `try-again`
requests the next value of the current problem and prints that value as its
line. When the current problem has no more values, the driver prints the two
lines

```
;;; There are no more values of
⟨the input form⟩
```

where the second line is the printed form of the driver's input expression,
exactly as the book prints it (4.3.1).

## Values

### Booleans

`#t` and `#f`.

### Integers

Decimal digits with a minus sign for negatives. No plus sign, no separators.

### Floats

A float prints as the shortest decimal string that reads back as the same
IEEE 754 double, and the string always contains a decimal point. A float
with an integral value prints its digits followed by `.0` (`3.0`, `441.0`).
When the absolute value is at least `1e-6` and below `1e21`, the string is
fixed notation, as in `2.5`, `0.001`, and `100000000000000000000.0`. Outside
that range the value prints as a mantissa, an `e`, and an exponent: the
mantissa is the shortest round-trip string in `[1, 10)` and always carries a
decimal point, the exponent is a decimal integer with a minus sign when
negative, as in `1.0e22` and `2.5e-7`. Negative floats carry a leading minus
sign in both forms.

### Symbols

The symbol's characters, bare, no quoting.

### Strings

The string's characters between double quotes, where a double quote in the
string prints as `\"` and a backslash prints as `\\`, and no other character
is escaped. This is the printed form used for values. `display` prints the
same string without the quotes and without the escapes.

### Pairs and lists

A chain of pairs whose last cdr is the empty list prints as the elements
separated by single spaces inside parentheses: `(1 2 3)`. A pair whose cdr is
neither a pair nor the empty list prints with the dot: `(1 . 2)`, and a chain
ending in such a pair keeps the dot at the tail: `(a b . c)`. Elements print
recursively by these rules: `(a (b . c) d)` for a pair inside a list.

### The empty list

`()`, two characters.

### Compound procedures

A named compound procedure prints `#[compound-procedure name]` with its
definition name. An anonymous one, a `lambda` value that was never named,
prints `#[compound-procedure]`.

### Primitive procedures

`#[primitive-procedure name]`.

## Errors

An `(error ⟨message⟩ ⟨irritant⟩...)` call prints one line:

```
Error: ⟨message⟩ ⟨irritant⟩...
```

The message string prints without quotes; each irritant is printed with the
value rules, and single spaces separate the message from the irritants and
the irritants from each other. Execution stops after the line.

## Query results

A query form prints each result assertion on its own line, in the
instantiated form of the assertion, in the order the book's transcripts show
for that query (4.4.1). `assert!` forms print nothing. The result order of
each corpus query is fixed by its expected file: simple queries yield
assertions in data-base order, conjunctions process conjuncts left to right,
and rule queries follow the data-base order the standard query evaluator of
4.4 produces. A disagreement between an edition and an expected file is a
defect in one of the two, never a skip.

## Machine traces

Machine programs follow the value rules for every top-level call:
`set-register-contents!` and `start` print `done`, and each
`get-register-contents` call prints

```
⟨register⟩ = ⟨value⟩
```

with the register's name, a space, an equals sign, a space, and the value in
the value form, one line per call in call order (5.2.2).

## Expected files

Every file under `expected/` begins with the two license header lines

```
;; SPDX-License-Identifier: GPL-3.0-only
;; Adapted from SICP section ⟨n.m⟩
```

and the program's output starts at line 3. A checker compares the program's
output bytes against the expected file from line 3 to the final LF.
