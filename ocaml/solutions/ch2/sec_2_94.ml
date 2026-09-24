(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.94 *)

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

let mul_term_by_all_terms t1 l =
  List.map (fun t2 -> make_term (t1.order + t2.order) (t1.coeff *. t2.coeff)) l
;;

(* [div_terms] repeats exercise 2.91's answer: this file stays self-contained
   because [solutions/ch2/dune] gives each exercise module its own library
   with no sibling dependency, matching every other 2.5 solution. *)
let rec div_terms l1 l2 =
  match l1 with
  | [] -> [], []
  | t1 :: _ ->
    let t2 =
      match l2 with
      | [] -> invalid_arg "div_terms: division by the empty term list"
      | t2 :: _ -> t2
    in
    if t2.order > t1.order
    then [], l1
    else (
      let new_term = make_term (t1.order - t2.order) (t1.coeff /. t2.coeff) in
      let quotient_rest, remainder =
        div_terms (sub_terms l1 (mul_term_by_all_terms new_term l2)) l2
      in
      adjoin_term new_term quotient_rest, remainder)
;;

let remainder_terms l1 l2 = snd (div_terms l1 l2)

let rec gcd_terms a b =
  match b with
  | [] -> a
  | _ :: _ -> gcd_terms b (remainder_terms a b)
;;

let gcd_poly p1 p2 =
  if String.equal p1.var p2.var
  then make_poly p1.var (gcd_terms p1.term_list p2.term_list)
  else invalid_arg "gcd_poly: polys not in the same variable"
;;

let rec int_gcd a b = if b = 0 then abs a else int_gcd b (a mod b)

type numeric =
  | Poly_value of poly
  | Int_value of int

let greatest_common_divisor a b =
  match a, b with
  | Poly_value p1, Poly_value p2 -> Poly_value (gcd_poly p1 p2)
  | Int_value n1, Int_value n2 -> Int_value (int_gcd n1 n2)
  | (Poly_value _ | Int_value _), (Poly_value _ | Int_value _) ->
    invalid_arg
      "greatest_common_divisor: arguments must both be polys or both be integers"
;;

(* The book's own test case: P1 = x^4 - x^3 - 2x^2 + 2x, P2 = x^3 - x.
   By hand, P1 = x(x-1)(x^2-2) and P2 = x(x-1)(x+1), so their GCD is
   x^2 - x up to a unit factor; [gcd_terms] returns -(x^2 - x) here
   because Euclid's algorithm only determines a GCD up to sign. Checking
   the result "by hand" becomes checking that it divides both inputs
   with no remainder. *)
let ex_2_94 () =
  let p1 =
    make_poly
      "x"
      [ make_term 4 1.0; make_term 3 (-1.0); make_term 2 (-2.0); make_term 1 2.0 ]
  in
  let p2 = make_poly "x" [ make_term 3 1.0; make_term 1 (-1.0) ] in
  match greatest_common_divisor (Poly_value p1) (Poly_value p2) with
  | Poly_value g ->
    let divides_p1 = List.length (remainder_terms p1.term_list g.term_list) = 0 in
    let divides_p2 = List.length (remainder_terms p2.term_list g.term_list) = 0 in
    List.map (fun t -> t.order, t.coeff) g.term_list, divides_p1, divides_p2
  | Int_value _ -> invalid_arg "ex_2_94: expected a polynomial result"
;;
