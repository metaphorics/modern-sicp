(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib-iter in SICP section 1.2
   exercise 1.19 *)

(** Exercise 1.19: applying [T_pq] twice is [T_p'q'] with
    [p' = p^2 + q^2] and [q' = q^2 + 2pq]; a routine expansion of
    [T_pq(T_pq(a, b))] and matching coefficients against [T_p'q'(a, b)]
    confirms it, and is written out in [ex_1_19.md]. With those in
    hand, [fib_iter_log] computes @math{T^n} by successive squaring
    exactly as [fast_expt] computes @math{b^n}: halve [count] and
    square the transform on an even count, apply one step and decrement
    on an odd one. Every value here is a plain [int]; the answer is
    exact for every [n] up to 90 and silently wrong beyond it, as
    [ex_1_19.md] records. *)

let is_even n = n mod 2 = 0

let rec fib_iter_log a b p q count =
  if count = 0
  then b
  else if is_even count
  then (
    let p' = (p * p) + (q * q) in
    let q' = (q * q) + (2 * p * q) in
    fib_iter_log a b p' q' (count / 2))
  else fib_iter_log ((b * q) + (a * q) + (a * p)) ((b * p) + (a * q)) p q (count - 1)
;;

let ex_1_19 n = fib_iter_log 1 0 0 1 n
