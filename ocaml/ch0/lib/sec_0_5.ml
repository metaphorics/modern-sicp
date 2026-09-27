(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Lists and higher-order functions: the named definitions behind the
    listings of section 0.5. *)

let squares = List.map (fun x -> x * x) [ 1; 2; 3; 4 ]
let evens = List.filter (fun x -> x mod 2 = 0) (List.init 10 (fun i -> i + 1))

(* The hand-derived versions the section walks through before the library
   ones. They are teaching forms: [my_map] and [my_filter] are not tail
   recursive, and the book says so; [my_fold_left] is, which is why the
   library's fold is the everyday choice. *)

let rec my_map f = function
  | [] -> []
  | head :: tail -> f head :: my_map f tail
;;

let rec my_filter predicate = function
  | [] -> []
  | head :: tail ->
    if predicate head then head :: my_filter predicate tail else my_filter predicate tail
;;

let my_fold_left f acc xs =
  let rec go acc = function
    | [] -> acc
    | head :: tail -> go (f acc head) tail
  in
  go acc xs
;;
