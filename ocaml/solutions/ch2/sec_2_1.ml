(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-rat in SICP section 2.1
   exercise 2.1 *)

(** [ex_2_01]'s only new work beyond @ref{2.1.2}'s [Rational.make] is
    the sign: the book's own gcd-reducing [make_rat] already leaves a
    negative denominator's sign untouched (dividing by a positive
    [gcd] preserves every input sign), so this solution takes the
    [gcd] of absolute values, exactly as @ref{2.1.2}'s [Rational.make]
    does, and then moves any negative sign from the denominator onto
    the numerator. *)

type rational_error = Zero_denominator of int

let rec gcd a b = if b = 0 then a else gcd b (a mod b)

let ex_2_01 n d =
  if d = 0
  then Error (Zero_denominator n)
  else (
    let g = gcd (abs n) (abs d) in
    let sign = if d < 0 then -1 else 1 in
    Ok (sign * n / g, sign * d / g))
;;
