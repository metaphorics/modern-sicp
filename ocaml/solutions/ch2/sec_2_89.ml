(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.89 *)

type dense = int list

let is_empty_termlist = function
  | [] -> true
  | _ :: _ -> false
;;

let first_term = function
  | [] -> invalid_arg "first_term: empty dense term list"
  | c :: rest -> List.length rest, c
;;

let rest_terms = function
  | [] -> invalid_arg "rest_terms: empty dense term list"
  | _ :: rest -> rest
;;

let of_terms sparse =
  let max_order = List.fold_left (fun acc (order, _) -> max acc order) 0 sparse in
  List.init (max_order + 1) (fun i ->
    let order = max_order - i in
    match List.assoc_opt order sparse with
    | Some coeff -> coeff
    | None -> 0)
;;

let to_terms l =
  let n = List.length l in
  List.filter_map
    (fun (i, coeff) -> if coeff = 0 then None else Some (n - 1 - i, coeff))
    (List.mapi (fun i coeff -> i, coeff) l)
;;

let ex_2_89 () =
  let sparse = [ 5, 1; 4, 2; 2, 3; 1, -2; 0, -5 ] in
  let dense = of_terms sparse in
  dense, first_term dense, to_terms dense
;;
