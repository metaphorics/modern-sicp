(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.88 *)

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

let negate_terms l = List.map (fun t -> make_term t.order (-.t.coeff)) l
let sub_terms l1 l2 = add_terms l1 (negate_terms l2)

let sub_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (sub_terms p1.term_list p2.term_list)
  else invalid_arg "sub_poly: polys not in same var"
;;

let ex_2_88 () =
  let p1 = make_poly "x" [ make_term 2 1.0; make_term 1 3.0; make_term 0 1.0 ] in
  let p2 = make_poly "x" [ make_term 2 1.0; make_term 0 1.0 ] in
  List.map (fun t -> t.order, t.coeff) (sub_poly p1 p2).term_list
;;
