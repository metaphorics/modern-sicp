(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.92 *)

type term =
  { order : int
  ; coeff : float
  }

type poly =
  { var : string
  ; term_list : term list
  }

let make_term order coeff = { order; coeff }
let make_poly var term_list = { var; term_list }

let adjoin_term term term_list =
  if Float.equal term.coeff 0.0 then term_list else term :: term_list
;;

let rec add_terms l1 l2 =
  match l1, l2 with
  | [], l | l, [] -> l
  | t1 :: rest1, t2 :: rest2 ->
    if t1.order > t2.order
    then adjoin_term t1 (add_terms rest1 l2)
    else if t1.order < t2.order
    then adjoin_term t2 (add_terms l1 rest2)
    else adjoin_term (make_term t1.order (t1.coeff +. t2.coeff)) (add_terms rest1 rest2)
;;

let var_less a b = String.compare a b < 0

type coeff =
  | Num of float
  | Sub_poly of poly

let coeff_as_float = function
  | Num f -> f
  | Sub_poly _ -> invalid_arg "coeff_as_float: expected a plain coefficient"
;;

let coeff_as_poly = function
  | Sub_poly p -> p
  | Num _ -> invalid_arg "coeff_as_poly: expected a promoted polynomial"
;;

let add_across_variables p1 p2 =
  let dominant_var = if var_less p1.var p2.var then p1.var else p2.var in
  let dominant, other = if String.equal p1.var dominant_var then p1, p2 else p2, p1 in
  if List.exists (fun t -> t.order = 0) dominant.term_list
  then
    invalid_arg
      "add_across_variables: the dominant polynomial already has a constant term; \
       combining it with the promoted coefficient needs full multivariate addition"
  else (
    let dominant_terms = List.map (fun t -> t.order, Num t.coeff) dominant.term_list in
    let combined = (0, Sub_poly other) :: dominant_terms in
    let sorted = List.sort (fun (o1, _) (o2, _) -> compare o2 o1) combined in
    dominant_var, sorted)
;;

let ex_2_92 () =
  let p1 = make_poly "x" [ make_term 1 1.0 ] in
  let p2 = make_poly "y" [ make_term 1 3.0; make_term 0 2.0 ] in
  add_across_variables p1 p2
;;
