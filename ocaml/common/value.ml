(* SPDX-License-Identifier: GPL-3.0-only *)

(* The mutually recursive knot of the runtime: environments hold values,
   compound procedures hold environments. [view] is declared separately
   below, so its constructors do not clash with [t]'s in one block; same
   named constructors across the two blocks are disambiguated by the type
   annotations on every function that builds one of them. *)

type compound_view =
  { name : string option
  ; parameters : string list
  ; body : Ast.expr list
  ; env : env
  }

and env = frame list
and frame = (string, t) Hashtbl.t

and t =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Symbol of string
  | Nil
  | Pair of t * t
  | Primitive_procedure of string * primitive
  | Compound_procedure of compound_view

and primitive = t list -> (t, Eval_error.t) result

(* The public view of a value; same constructor names as [t], separate
   declaration block. *)
type view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Symbol of string
  | Nil
  | Pair of t * t
  | Primitive_procedure of string
  | Compound_procedure of compound_view

let view (v : t) : view =
  match v with
  | Int n -> Int n
  | Float f -> Float f
  | Bool b -> Bool b
  | String s -> String s
  | Symbol s -> Symbol s
  | Nil -> Nil
  | Pair (car, cdr) -> Pair (car, cdr)
  | Primitive_procedure (name, _) -> Primitive_procedure name
  | Compound_procedure c -> Compound_procedure c
;;

let int (n : int) : t = Int n
let float (f : float) : t = Float f
let bool (b : bool) : t = Bool b
let string (s : string) : t = String s
let symbol (s : string) : t = Symbol s
let nil : t = Nil
let pair (car : t) (cdr : t) : t = Pair (car, cdr)
let primitive ~(name : string) (f : primitive) : t = Primitive_procedure (name, f)

let compound
      ~(name : string option)
      ~(parameters : string list)
      ~(body : Ast.expr list)
      ~(env : env)
  : t
  =
  Compound_procedure { name; parameters; body; env }
;;

let physical_equal (a : t) (b : t) : bool =
  match a, b with
  | Int _, Int _ | Float _, Float _ | Bool _, Bool _ | Symbol _, Symbol _ -> a = b
  | Nil, Nil -> true
  | _ -> a == b
;;

let rec structural_equal (a : t) (b : t) : bool =
  match a, b with
  | Pair (a1, d1), Pair (a2, d2) -> structural_equal a1 a2 && structural_equal d1 d2
  | Primitive_procedure (n1, _), Primitive_procedure (n2, _) -> String.equal n1 n2
  | Compound_procedure _, Compound_procedure _ -> a == b
  | _ -> a = b
;;

let escape_string s =
  let buf = Buffer.create (String.length s) in
  String.iter
    (fun c ->
       match c with
       | '"' -> Buffer.add_string buf "\\\""
       | '\\' -> Buffer.add_string buf "\\\\"
       | c -> Buffer.add_char buf c)
    s;
  Buffer.contents buf
;;

let float_string v =
  if Float.is_nan v
  then "nan"
  else if Float.is_infinite v
  then if Float.sign_bit v then "-inf" else "inf"
  else (
    let rec shortest precision =
      let s = Printf.sprintf "%.*g" precision v in
      if float_of_string s = v || precision >= 17 then s else shortest (precision + 1)
    in
    let shortest = shortest 1 in
    let magnitude = Float.abs v in
    let in_fixed_range = magnitude >= 1e-6 && magnitude < 1e21 in
    let sign, body =
      if shortest.[0] = '-'
      then "-", String.sub shortest 1 (String.length shortest - 1)
      else "", shortest
    in
    match String.index_opt body 'e' with
    | None ->
      let body = if String.contains body '.' then body else body ^ ".0" in
      sign ^ body
    | Some k ->
      let mantissa = String.sub body 0 k in
      let exponent =
        int_of_string (String.sub body (k + 1) (String.length body - k - 1))
      in
      if in_fixed_range
      then (
        let digits =
          let buf = Buffer.create (String.length mantissa) in
          String.iter (fun c -> if c <> '.' then Buffer.add_char buf c) mantissa;
          Buffer.contents buf
        in
        let point =
          (match String.index_opt mantissa '.' with
           | Some k -> k
           | None -> String.length mantissa)
          + exponent
        in
        let len = String.length digits in
        if point <= 0
        then sign ^ "0." ^ String.make (-point) '0' ^ digits
        else if point >= len
        then sign ^ digits ^ String.make (point - len) '0' ^ ".0"
        else sign ^ String.sub digits 0 point ^ "." ^ String.sub digits point (len - point))
      else (
        let mantissa =
          if String.contains mantissa '.' then mantissa else mantissa ^ ".0"
        in
        sign ^ mantissa ^ "e" ^ string_of_int exponent))
;;

let render (quoted : string -> string) (v : t) : string =
  let rec go (v : t) : string =
    match v with
    | Int n -> string_of_int n
    | Float f -> float_string f
    | Bool b -> if b then "#t" else "#f"
    | String s -> quoted s
    | Symbol s -> s
    | Nil -> "()"
    | Pair _ ->
      let rec parts (acc : t list) (v : t) : t list * t =
        match v with
        | Pair (car, cdr) -> parts (car :: acc) cdr
        | v -> List.rev acc, v
      in
      let elements, tail = parts [] v in
      let inner = String.concat " " (List.map go elements) in
      (match tail with
       | Nil -> "(" ^ inner ^ ")"
       | tail -> "(" ^ inner ^ " . " ^ go tail ^ ")")
    | Primitive_procedure (name, _) -> "#[primitive-procedure " ^ name ^ "]"
    | Compound_procedure { name; _ } ->
      (match name with
       | Some name -> "#[compound-procedure " ^ name ^ "]"
       | None -> "#[compound-procedure]")
  in
  go v
;;

let to_string v = render (fun s -> "\"" ^ escape_string s ^ "\"") v
let display v = render (fun s -> s) v
let env_empty () : env = [ Hashtbl.create 8 ]

let env_extend names values (outer : env) : (env, Eval_error.t) result =
  let expected = List.length names in
  let given = List.length values in
  if expected <> given
  then Error (Eval_error.Arity_mismatch { expected; given })
  else (
    let frame = Hashtbl.create (max 8 expected) in
    List.iter2 (fun name value -> Hashtbl.replace frame name value) names values;
    Ok (frame :: outer))
;;

let rec env_find_binding (env : env) (name : string) : t option =
  match env with
  | [] -> None
  | frame :: outer ->
    (match Hashtbl.find_opt frame name with
     | Some value -> Some value
     | None -> env_find_binding outer name)
;;

let env_define (env : env) (name : string) (value : t) : unit =
  match env with
  | frame :: _ -> Hashtbl.replace frame name value
  | [] -> invalid_arg "Value.env_define: empty environment"
;;

let rec env_set (env : env) (name : string) (value : t) : (unit, Eval_error.t) result =
  match env with
  | [] -> Error (Eval_error.Unbound_variable name)
  | frame :: outer ->
    if Hashtbl.mem frame name
    then (
      Hashtbl.replace frame name value;
      Ok ())
    else env_set outer name value
;;
