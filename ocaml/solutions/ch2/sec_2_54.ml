(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.54 *)

(** Exercise 2.54: recursive equality over a symbol tree. A [datum] is
    a symbol or a sequence of data, so the equality that the book
    states over symbols and lists becomes one recursive match over the
    variant: two symbols are equal when their names are, and two
    sequences are equal when they hold the same length and their
    elements are pairwise equal. *)

type datum =
  | Sym of string
  | Seq of datum list

let rec equal_datum a b =
  match a, b with
  | Sym x, Sym y -> x = y
  | Seq xs, Seq ys -> equal_datum_list xs ys
  | (Sym _ | Seq _), _ -> false

and equal_datum_list xs ys =
  match xs, ys with
  | [], [] -> true
  | x :: xs', y :: ys' -> equal_datum x y && equal_datum_list xs' ys'
  | ([] | _ :: _), _ -> false
;;

let words l = Seq (List.map (fun s -> Sym s) l)

(** [ex_2_54 ()] is the book's example pair: [(this is a list)]
    compared with itself. *)
let ex_2_54 () =
  equal_datum (words [ "this"; "is"; "a"; "list" ]) (words [ "this"; "is"; "a"; "list" ])
;;
