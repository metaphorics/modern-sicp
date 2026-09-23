(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program filtered-accumulate in SICP section
   1.3 exercise 1.33 *)

(** Exercise 1.33: [filtered_accumulate] is [accumulate] with one more
    guard: an index [a] that fails [pred] contributes nothing and is
    skipped, not folded in as [term a]. Kept over [int] rather than
    [float], since both this exercise's uses ([is_prime], [gcd]) are
    integer-domain tests that have no meaningful float form. *)

let filtered_accumulate combiner null_value term a next b pred =
  let rec iter a result =
    if a > b
    then result
    else (
      let result = if pred a then combiner result (term a) else result in
      iter (next a) result)
  in
  iter a null_value
;;

let is_prime n =
  if n < 2
  then false
  else (
    let rec no_divisor_up_to d =
      d * d > n || (n mod d <> 0 && no_divisor_up_to (d + 1))
    in
    no_divisor_up_to 2)
;;

let rec gcd a b = if b = 0 then a else gcd b (a mod b)

let sum_squares_of_primes a b =
  filtered_accumulate ( + ) 0 (fun x -> x * x) a (fun x -> x + 1) b is_prime
;;

let product_relatively_prime n =
  filtered_accumulate
    ( * )
    1
    (fun x -> x)
    1
    (fun x -> x + 1)
    (n - 1)
    (fun i -> gcd i n = 1)
;;

let ex_1_33 () = sum_squares_of_primes 2 20, product_relatively_prime 10
