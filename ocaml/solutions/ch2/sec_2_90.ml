(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.90 *)

module type Term_list = sig
  type t

  val the_empty_termlist : t
  val is_empty : t -> bool
  val first_term : t -> int * int
  val rest_terms : t -> t
  val adjoin_term : int -> int -> t -> t
  val of_pairs : (int * int) list -> t
  val to_pairs : t -> (int * int) list
end

module Sparse = struct
  type t = (int * int) list

  let the_empty_termlist = []

  let is_empty = function
    | [] -> true
    | _ :: _ -> false
  ;;

  let first_term = function
    | [] -> invalid_arg "Sparse.first_term: empty term list"
    | t :: _ -> t
  ;;

  let rest_terms = function
    | [] -> invalid_arg "Sparse.rest_terms: empty term list"
    | _ :: rest -> rest
  ;;

  let adjoin_term order coeff tl = if coeff = 0 then tl else (order, coeff) :: tl
  let of_pairs pairs = List.filter (fun (_, coeff) -> coeff <> 0) pairs
  let to_pairs l = l
end

module Dense = struct
  type t = int list

  let the_empty_termlist = []

  let is_empty = function
    | [] -> true
    | _ :: _ -> false
  ;;

  let first_term = function
    | [] -> invalid_arg "Dense.first_term: empty term list"
    | c :: rest -> List.length rest, c
  ;;

  let rest_terms = function
    | [] -> invalid_arg "Dense.rest_terms: empty term list"
    | _ :: rest -> rest
  ;;

  let adjoin_term order coeff tl =
    let expected = List.length tl in
    if order <> expected
    then invalid_arg "Dense.adjoin_term: order must be the next slot down"
    else coeff :: tl
  ;;

  let of_pairs pairs =
    let max_order = List.fold_left (fun acc (order, _) -> max acc order) 0 pairs in
    List.init (max_order + 1) (fun i ->
      let order = max_order - i in
      match List.assoc_opt order pairs with
      | Some coeff -> coeff
      | None -> 0)
  ;;

  let to_pairs l =
    let n = List.length l in
    List.filter_map
      (fun (i, coeff) -> if coeff = 0 then None else Some (n - 1 - i, coeff))
      (List.mapi (fun i coeff -> i, coeff) l)
  ;;
end

module Make_ops (T : Term_list) = struct
  let rec add_terms l1 l2 =
    if T.is_empty l1
    then l2
    else if T.is_empty l2
    then l1
    else (
      let o1, c1 = T.first_term l1
      and o2, c2 = T.first_term l2 in
      if o1 > o2
      then T.adjoin_term o1 c1 (add_terms (T.rest_terms l1) l2)
      else if o2 > o1
      then T.adjoin_term o2 c2 (add_terms l1 (T.rest_terms l2))
      else T.adjoin_term o1 (c1 + c2) (add_terms (T.rest_terms l1) (T.rest_terms l2)))
  ;;
end

module Sparse_ops = Make_ops (Sparse)
module Dense_ops = Make_ops (Dense)

let ex_2_90 () =
  let p1 = [ 2, 1; 0, 1 ] in
  let p2 = [ 1, 1; 0, 1 ] in
  let sparse_result = Sparse_ops.add_terms (Sparse.of_pairs p1) (Sparse.of_pairs p2) in
  let dense_result = Dense_ops.add_terms (Dense.of_pairs p1) (Dense.of_pairs p2) in
  Sparse.to_pairs sparse_result, Dense.to_pairs dense_result
;;
