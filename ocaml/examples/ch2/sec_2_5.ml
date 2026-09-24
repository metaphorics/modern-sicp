(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.5 *)

type value =
  | Num of float
  | Ratpair of int * int
  | Cpx of float * float
  | Bool of bool
  | Poly of poly
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

and term =
  { order : int
  ; coeff : value
  }

and poly =
  { var : string
  ; term_list : term list
  }

let attach_tag tag contents = { tag; contents }
let type_tag t = t.tag
let contents_of t = t.contents
let table : (string * string list, value list -> value) Hashtbl.t = Hashtbl.create 32
let put op type_tags proc = Hashtbl.replace table (op, type_tags) proc
let get op type_tags = Hashtbl.find_opt table (op, type_tags)

let untag op = function
  | Tagged t -> t
  | Num _ | Ratpair _ | Cpx _ | Bool _ | Poly _ ->
    invalid_arg (op ^ ": apply_generic expects tagged arguments")
;;

let apply_generic op args =
  let tagged_args = List.map (untag op) args in
  let type_tags = List.map type_tag tagged_args in
  match get op type_tags with
  | Some proc -> proc (List.map contents_of tagged_args)
  | None ->
    invalid_arg
      (Printf.sprintf
         "apply_generic: no method for these types: %s (%s)"
         op
         (String.concat ", " type_tags))
;;

let install_scheme_number_package () =
  put "add" [ "scheme-number"; "scheme-number" ] (function
    | [ Num a; Num b ] -> Tagged (attach_tag "scheme-number" (Num (a +. b)))
    | _ -> invalid_arg "add: scheme-number expects two numbers");
  put "sub" [ "scheme-number"; "scheme-number" ] (function
    | [ Num a; Num b ] -> Tagged (attach_tag "scheme-number" (Num (a -. b)))
    | _ -> invalid_arg "sub: scheme-number expects two numbers");
  put "mul" [ "scheme-number"; "scheme-number" ] (function
    | [ Num a; Num b ] -> Tagged (attach_tag "scheme-number" (Num (a *. b)))
    | _ -> invalid_arg "mul: scheme-number expects two numbers");
  put "div" [ "scheme-number"; "scheme-number" ] (function
    | [ Num a; Num b ] -> Tagged (attach_tag "scheme-number" (Num (a /. b)))
    | _ -> invalid_arg "div: scheme-number expects two numbers");
  put "=zero?" [ "scheme-number" ] (function
    | [ Num a ] -> Bool (Float.equal a 0.0)
    | _ -> invalid_arg "=zero?: scheme-number expects one number");
  put "make" [ "scheme-number" ] (function
    | [ Num n ] -> Tagged (attach_tag "scheme-number" (Num n))
    | _ -> invalid_arg "make: scheme-number expects one number")
;;

let install_rational_package () =
  let numer = function
    | Ratpair (n, _) -> n
    | Num _ | Cpx _ | Bool _ | Poly _ | Tagged _ ->
      invalid_arg "numer: expected a rational pair"
  in
  let denom = function
    | Ratpair (_, d) -> d
    | Num _ | Cpx _ | Bool _ | Poly _ | Tagged _ ->
      invalid_arg "denom: expected a rational pair"
  in
  let make_rat n d =
    let sign = if d < 0 then -1 else 1 in
    let n, d = sign * n, sign * d in
    let g = Sicp_ch1.Sec_1_2.Gcd.gcd (abs n) (abs d) in
    Ratpair (n / g, d / g)
  in
  put "add" [ "rational"; "rational" ] (function
    | [ x; y ] ->
      Tagged
        (attach_tag
           "rational"
           (make_rat ((numer x * denom y) + (numer y * denom x)) (denom x * denom y)))
    | _ -> invalid_arg "add: rational expects two pairs");
  put "sub" [ "rational"; "rational" ] (function
    | [ x; y ] ->
      Tagged
        (attach_tag
           "rational"
           (make_rat ((numer x * denom y) - (numer y * denom x)) (denom x * denom y)))
    | _ -> invalid_arg "sub: rational expects two pairs");
  put "mul" [ "rational"; "rational" ] (function
    | [ x; y ] ->
      Tagged (attach_tag "rational" (make_rat (numer x * numer y) (denom x * denom y)))
    | _ -> invalid_arg "mul: rational expects two pairs");
  put "div" [ "rational"; "rational" ] (function
    | [ x; y ] ->
      Tagged (attach_tag "rational" (make_rat (numer x * denom y) (denom x * numer y)))
    | _ -> invalid_arg "div: rational expects two pairs");
  put "=zero?" [ "rational" ] (function
    | [ x ] -> Bool (numer x = 0)
    | _ -> invalid_arg "=zero?: rational expects one pair");
  put "make" [ "rational" ] (function
    | [ Num n; Num d ] ->
      Tagged (attach_tag "rational" (make_rat (int_of_float n) (int_of_float d)))
    | _ -> invalid_arg "make: rational expects two numbers")
;;

let install_rectangular_package () =
  let open Sec_2_4.Untagged in
  put "real_part" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (Rectangular.real_part (x, y))
    | _ -> invalid_arg "real_part: rectangular expects one pair");
  put "imag_part" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (Rectangular.imag_part (x, y))
    | _ -> invalid_arg "imag_part: rectangular expects one pair");
  put "magnitude" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (Rectangular.magnitude (x, y))
    | _ -> invalid_arg "magnitude: rectangular expects one pair");
  put "angle" [ "rectangular" ] (function
    | [ Cpx (x, y) ] -> Num (Rectangular.angle (x, y))
    | _ -> invalid_arg "angle: rectangular expects one pair");
  put "make_from_real_imag" [ "rectangular" ] (function
    | [ Num x; Num y ] -> Tagged (attach_tag "rectangular" (Cpx (x, y)))
    | _ -> invalid_arg "make_from_real_imag: rectangular expects two numbers");
  put "make_from_mag_ang" [ "rectangular" ] (function
    | [ Num r; Num a ] ->
      let x, y = Rectangular.make_from_mag_ang r a in
      Tagged (attach_tag "rectangular" (Cpx (x, y)))
    | _ -> invalid_arg "make_from_mag_ang: rectangular expects two numbers")
;;

let install_polar_package () =
  let open Sec_2_4.Untagged in
  put "real_part" [ "polar" ] (function
    | [ Cpx (r, a) ] -> Num (Polar.real_part (r, a))
    | _ -> invalid_arg "real_part: polar expects one pair");
  put "imag_part" [ "polar" ] (function
    | [ Cpx (r, a) ] -> Num (Polar.imag_part (r, a))
    | _ -> invalid_arg "imag_part: polar expects one pair");
  put "magnitude" [ "polar" ] (function
    | [ Cpx (r, _) ] -> Num r
    | _ -> invalid_arg "magnitude: polar expects one pair");
  put "angle" [ "polar" ] (function
    | [ Cpx (_, a) ] -> Num a
    | _ -> invalid_arg "angle: polar expects one pair");
  put "make_from_mag_ang" [ "polar" ] (function
    | [ Num r; Num a ] -> Tagged (attach_tag "polar" (Cpx (r, a)))
    | _ -> invalid_arg "make_from_mag_ang: polar expects two numbers");
  put "make_from_real_imag" [ "polar" ] (function
    | [ Num x; Num y ] ->
      let r, a = Polar.make_from_real_imag x y in
      Tagged (attach_tag "polar" (Cpx (r, a)))
    | _ -> invalid_arg "make_from_real_imag: polar expects two numbers")
;;

let as_number = function
  | Num n -> n
  | Ratpair _ | Cpx _ | Bool _ | Poly _ | Tagged _ ->
    invalid_arg "expected a number result"
;;

let real_part z = as_number (apply_generic "real_part" [ z ])
let imag_part z = as_number (apply_generic "imag_part" [ z ])
let magnitude z = as_number (apply_generic "magnitude" [ z ])
let angle z = as_number (apply_generic "angle" [ z ])

let make_from_real_imag_rect x y =
  match get "make_from_real_imag" [ "rectangular" ] with
  | Some proc -> proc [ Num x; Num y ]
  | None -> invalid_arg "make_from_real_imag: no rectangular constructor installed"
;;

let make_from_mag_ang_polar r a =
  match get "make_from_mag_ang" [ "polar" ] with
  | Some proc -> proc [ Num r; Num a ]
  | None -> invalid_arg "make_from_mag_ang: no polar constructor installed"
;;

let install_complex_package () =
  let tag z = attach_tag "complex" z in
  let add_complex z1 z2 =
    make_from_real_imag_rect (real_part z1 +. real_part z2) (imag_part z1 +. imag_part z2)
  in
  let sub_complex z1 z2 =
    make_from_real_imag_rect (real_part z1 -. real_part z2) (imag_part z1 -. imag_part z2)
  in
  let mul_complex z1 z2 =
    make_from_mag_ang_polar (magnitude z1 *. magnitude z2) (angle z1 +. angle z2)
  in
  let div_complex z1 z2 =
    make_from_mag_ang_polar (magnitude z1 /. magnitude z2) (angle z1 -. angle z2)
  in
  put "add" [ "complex"; "complex" ] (function
    | [ z1; z2 ] -> Tagged (tag (add_complex z1 z2))
    | _ -> invalid_arg "add: complex expects two complex numbers");
  put "sub" [ "complex"; "complex" ] (function
    | [ z1; z2 ] -> Tagged (tag (sub_complex z1 z2))
    | _ -> invalid_arg "sub: complex expects two complex numbers");
  put "mul" [ "complex"; "complex" ] (function
    | [ z1; z2 ] -> Tagged (tag (mul_complex z1 z2))
    | _ -> invalid_arg "mul: complex expects two complex numbers");
  put "div" [ "complex"; "complex" ] (function
    | [ z1; z2 ] -> Tagged (tag (div_complex z1 z2))
    | _ -> invalid_arg "div: complex expects two complex numbers");
  put "make_from_real_imag" [ "complex" ] (function
    | [ Num x; Num y ] -> Tagged (tag (make_from_real_imag_rect x y))
    | _ -> invalid_arg "make_from_real_imag: complex expects two numbers");
  put "make_from_mag_ang" [ "complex" ] (function
    | [ Num r; Num a ] -> Tagged (tag (make_from_mag_ang_polar r a))
    | _ -> invalid_arg "make_from_mag_ang: complex expects two numbers")
;;

let add_complex_to_schemenum z x =
  make_from_real_imag_rect (real_part z +. x) (imag_part z)
;;

let install_cross_type_example () =
  put "add" [ "complex"; "scheme-number" ] (function
    | [ z; Num x ] -> Tagged (attach_tag "complex" (add_complex_to_schemenum z x))
    | _ -> invalid_arg "add: complex+scheme-number expects a complex number and a number")
;;

let make_scheme_number n =
  match get "make" [ "scheme-number" ] with
  | Some proc -> proc [ Num n ]
  | None -> invalid_arg "make_scheme_number: no scheme-number constructor installed"
;;

let make_rational n d =
  match get "make" [ "rational" ] with
  | Some proc -> proc [ Num (float_of_int n); Num (float_of_int d) ]
  | None -> invalid_arg "make_rational: no rational constructor installed"
;;

let make_complex_from_real_imag x y =
  match get "make_from_real_imag" [ "complex" ] with
  | Some proc -> proc [ Num x; Num y ]
  | None -> invalid_arg "make_complex_from_real_imag: no complex constructor installed"
;;

let make_complex_from_mag_ang r a =
  match get "make_from_mag_ang" [ "complex" ] with
  | Some proc -> proc [ Num r; Num a ]
  | None -> invalid_arg "make_complex_from_mag_ang: no complex constructor installed"
;;

let scheme_number_to_complex n =
  match n with
  | { tag = "scheme-number"; contents = Num x } -> make_complex_from_real_imag x 0.0
  | _ -> invalid_arg "scheme_number_to_complex: expected a tagged scheme-number"
;;

let coercion_table : (string * string, value -> value) Hashtbl.t = Hashtbl.create 8
let put_coercion t1 t2 proc = Hashtbl.replace coercion_table (t1, t2) proc
let get_coercion t1 t2 = Hashtbl.find_opt coercion_table (t1, t2)

let install_coercions () =
  put_coercion "scheme-number" "complex" (function
    | Tagged t -> scheme_number_to_complex t
    | Num _ | Ratpair _ | Cpx _ | Bool _ | Poly _ ->
      invalid_arg "scheme-number->complex: expected a tagged value")
;;

let rec apply_generic_coerce op args =
  let type_tags = List.map (fun a -> type_tag (untag op a)) args in
  match get op type_tags with
  | Some proc -> proc (List.map (fun a -> contents_of (untag op a)) args)
  | None ->
    (match args with
     | [ a1; a2 ] ->
       let t1, t2 =
         match type_tags with
         | [ t1; t2 ] -> t1, t2
         | _ -> assert false
       in
       (match get_coercion t1 t2, get_coercion t2 t1 with
        | Some t1_to_t2, _ -> apply_generic_coerce op [ t1_to_t2 a1; a2 ]
        | None, Some t2_to_t1 -> apply_generic_coerce op [ a1; t2_to_t1 a2 ]
        | None, None ->
          invalid_arg
            (Printf.sprintf
               "apply_generic_coerce: no method for these types: %s (%s)"
               op
               (String.concat ", " type_tags)))
     | _ ->
       invalid_arg
         (Printf.sprintf
            "apply_generic_coerce: no method for these types: %s (%s)"
            op
            (String.concat ", " type_tags)))
;;

let as_bool = function
  | Bool b -> b
  | Num _ | Ratpair _ | Cpx _ | Poly _ | Tagged _ ->
    invalid_arg "expected a boolean result"
;;

let add x y = apply_generic_coerce "add" [ x; y ]
let sub x y = apply_generic_coerce "sub" [ x; y ]
let mul x y = apply_generic_coerce "mul" [ x; y ]
let div x y = apply_generic_coerce "div" [ x; y ]
let is_zero v = as_bool (apply_generic_coerce "=zero?" [ v ])
let the_empty_termlist : term list = []

let is_empty_termlist = function
  | [] -> true
  | _ -> false
;;

let first_term = function
  | t :: _ -> t
  | [] -> invalid_arg "first_term: empty term list"
;;

let rest_terms = function
  | _ :: ts -> ts
  | [] -> invalid_arg "rest_terms: empty term list"
;;

let make_term order coeff = { order; coeff }
let order t = t.order
let coeff t = t.coeff

let adjoin_term term term_list =
  if is_zero term.coeff then term_list else term :: term_list
;;

let rec add_terms l1 l2 =
  if is_empty_termlist l1
  then l2
  else if is_empty_termlist l2
  then l1
  else (
    let t1 = first_term l1
    and t2 = first_term l2 in
    if t1.order > t2.order
    then adjoin_term t1 (add_terms (rest_terms l1) l2)
    else if t1.order < t2.order
    then adjoin_term t2 (add_terms l1 (rest_terms l2))
    else
      adjoin_term
        (make_term t1.order (add t1.coeff t2.coeff))
        (add_terms (rest_terms l1) (rest_terms l2)))
;;

let mul_term_by_all_terms t1 l =
  let rec loop l =
    if is_empty_termlist l
    then the_empty_termlist
    else (
      let t2 = first_term l in
      adjoin_term
        (make_term (t1.order + t2.order) (mul t1.coeff t2.coeff))
        (loop (rest_terms l)))
  in
  loop l
;;

let rec mul_terms l1 l2 =
  if is_empty_termlist l1
  then the_empty_termlist
  else add_terms (mul_term_by_all_terms (first_term l1) l2) (mul_terms (rest_terms l1) l2)
;;

let make_poly var term_list = { var; term_list }
let variable p = p.var
let term_list p = p.term_list
let same_variable = String.equal

let add_poly p1 p2 =
  if same_variable (variable p1) (variable p2)
  then make_poly (variable p1) (add_terms (term_list p1) (term_list p2))
  else invalid_arg "add_poly: polys not in same var"
;;

let mul_poly p1 p2 =
  if same_variable (variable p1) (variable p2)
  then make_poly (variable p1) (mul_terms (term_list p1) (term_list p2))
  else invalid_arg "mul_poly: polys not in same var"
;;

let install_polynomial_package () =
  put "add" [ "polynomial"; "polynomial" ] (function
    | [ Poly p1; Poly p2 ] -> Tagged (attach_tag "polynomial" (Poly (add_poly p1 p2)))
    | _ -> invalid_arg "add: polynomial expects two polys");
  put "mul" [ "polynomial"; "polynomial" ] (function
    | [ Poly p1; Poly p2 ] -> Tagged (attach_tag "polynomial" (Poly (mul_poly p1 p2)))
    | _ -> invalid_arg "mul: polynomial expects two polys");
  put "make" [ "polynomial" ] (function
    | [ Poly p ] -> Tagged (attach_tag "polynomial" (Poly p))
    | _ -> invalid_arg "make: polynomial expects a poly")
;;

let make_polynomial var term_list =
  match get "make" [ "polynomial" ] with
  | Some proc -> proc [ Poly (make_poly var term_list) ]
  | None -> invalid_arg "make_polynomial: no polynomial constructor installed"
;;
