(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.9 *)

(** Exercise 3.9: stack growth of the two factorials of @ref{1.2.1}
    under OCaml's guaranteed tail calls. The traced variants thread the
    current depth through every call and report the deepest frame live
    at any point, so the drawing in the statement can be checked
    against measurement instead of trust. *)

let rec factorial n = if n = 1 then 1 else n * factorial (n - 1)

let factorial_iter n =
  let rec fact_iter product counter =
    if counter > n then product else fact_iter (counter * product) (counter + 1)
  in
  fact_iter 1 1
;;

let factorial_traced n =
  let deepest = ref 1 in
  let rec go depth n =
    if depth > !deepest then deepest := depth;
    if n = 1 then 1 else n * go (depth + 1) (n - 1)
  in
  let result = go 1 n in
  result, !deepest
;;

let factorial_iter_traced n =
  let deepest = ref 1 in
  let rec fact_iter product counter depth =
    if depth > !deepest then deepest := depth;
    if counter > n
    then product
    else
      (* A call in tail position takes the caller's frame's place, so
         the depth it reports is the depth it was called at; OCaml's
         compiler guarantees the machine follows the same rule. *)
      fact_iter (counter * product) (counter + 1) depth
  in
  let result = fact_iter 1 1 1 in
  result, !deepest
;;

let ex_3_09 () = factorial_traced 6, factorial_iter_traced 6
