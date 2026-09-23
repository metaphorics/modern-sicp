(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program product in SICP section 1.3
   exercise 1.31 *)

(** Exercise 1.31, part a: [product] mirrors [Sec_1_3.Sum_abstraction.sum]
    exactly, with [*.] and the multiplicative identity [1.] in place of
    [+.] and [0.]. Part b: the accumulation below is already
    tail-recursive, generating an iterative process. *)

let product term a next b =
  let rec iter a result = if a > b then result else iter (next a) (result *. term a) in
  iter a 1.0
;;

let factorial n = product (fun x -> x) 1.0 (fun x -> x +. 1.0) (float_of_int n)

(** The book's Wallis formula, pi/4 = (2*4*4*6*6*8*...)/(3*3*5*5*7*7*...),
    read as the single-index sequence of fractions 2/3, 4/3, 4/5, 6/5,
    6/7, 8/7, ...: even [i] contributes [i / (i + 1)], odd [i]
    contributes [(i + 1) / i], starting from [i = 2]. *)
let pi_approx n =
  let wallis_term i =
    if Float.equal (Float.rem i 2.0) 0.0 then i /. (i +. 1.0) else (i +. 1.0) /. i
  in
  4.0 *. product wallis_term 2.0 (fun i -> i +. 1.0) (float_of_int n +. 1.0)
;;

let ex_1_31 () = factorial 6, pi_approx 1000
