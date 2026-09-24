(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.87 *)

type coeff =
  | Flt of float
  | Poly of poly

and term =
  { order : int
  ; coeff : coeff
  }

and poly =
  { var : string
  ; term_list : term list
  }

let make_term order coeff = { order; coeff }
let make_poly var term_list = { var; term_list }

let is_zero_coeff = function
  | Flt f -> Float.equal f 0.0
  | Poly p ->
    (match p.term_list with
     | [] -> true
     | _ :: _ -> false)
;;

let adjoin_term term term_list =
  if is_zero_coeff term.coeff then term_list else term :: term_list
;;

let ex_2_87 () =
  let empty_y_poly = make_poly "y" [] in
  let nonzero_y_poly = make_poly "y" [ make_term 0 (Flt 5.0) ] in
  let with_zero_coeff = adjoin_term (make_term 3 (Poly empty_y_poly)) [] in
  let with_nonzero_coeff =
    adjoin_term (make_term 2 (Poly nonzero_y_poly)) with_zero_coeff
  in
  List.length with_nonzero_coeff, is_zero_coeff (Poly empty_y_poly)
;;
