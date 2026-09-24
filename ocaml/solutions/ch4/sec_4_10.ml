(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.10 *)

(** Exercise 4.10: a new surface syntax for the same language. [eval]
    and [apply] never mention the surface syntax: the terms of the
    alternative syntax translate into the typed [Ast], and the
    unmodified base evaluator runs the result. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** One term of the alternative surface syntax: a self-evaluating
      literal, a name, a function written [fn], a [do] sequence, or a
      call written [term ! args]. *)
type term =
  | Lit of Value.t
  | Name of string
  | Fn of string list * term
  | Do of term list
  | Call of term * term list

(** [value_to_datum v] is the datum whose evaluation is [v], the inverse
    of the substrate's [datum_to_value] over the data values. A
    procedure is not a literal and reports a type error. *)
let rec value_to_datum (v : Value.t) : (Ast.datum, Eval_error.t) result =
  match Value.view v with
  | Value.Int n -> Ok (Ast.DInt n)
  | Value.Float f -> Ok (Ast.DFloat f)
  | Value.Bool b -> Ok (Ast.DBool b)
  | Value.String s -> Ok (Ast.DString s)
  | Value.Symbol s -> Ok (Ast.DSymbol s)
  | Value.Nil -> Ok Ast.DNil
  | Value.Pair (car, cdr) ->
    value_to_datum car
    >>= fun car -> value_to_datum cdr >>= fun cdr -> Ok (Ast.DPair (car, cdr))
  | Value.Primitive_procedure _ | Value.Compound_procedure _ ->
    Error (Eval_error.Invalid_form "from_new_syntax: a procedure is not a literal")
;;

(** [from_new_syntax term] translates one term of the alternative
    syntax into the standard AST: [Lit] to a self-evaluating expression
    (quoted when the literal is data), [Name] to a variable reference,
    [Fn] to a one-expression [lambda], [Do] to a [begin], and [Call] to
    an application. *)
let rec from_new_syntax (term : term) : (Ast.expr, Eval_error.t) result =
  match term with
  | Lit v ->
    (match Value.view v with
     | Value.Int n -> Ok (Ast.int n)
     | Value.Float f -> Ok (Ast.float f)
     | Value.Bool b -> Ok (Ast.bool b)
     | Value.String s -> Ok (Ast.string s)
     | _ -> value_to_datum v >>= fun datum -> Ok (Ast.quote datum))
  | Name name -> Ok (Ast.variable name)
  | Fn (parameters, body) ->
    from_new_syntax body >>= fun body -> Ast.lambda parameters [ body ]
  | Do terms ->
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | term :: rest -> from_new_syntax term >>= fun e -> go (e :: acc) rest
    in
    go [] terms >>= Ast.sequence
  | Call (operator, operands) ->
    from_new_syntax operator
    >>= fun operator ->
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | term :: rest -> from_new_syntax term >>= fun e -> go (e :: acc) rest
    in
    go [] operands >>= fun operands -> Ok (Ast.application operator operands)
;;

(** [eval] is the untouched base evaluator: the exercise's point is that
    a syntax change stops at the translation. *)
let eval : Sicp_ch4.Sec_4_1.eval_t = Sicp_ch4.Sec_4_1.eval

(** [render r] is the printed outcome of one demonstration step. *)
let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_10 ()] writes [square] in the new syntax, stores the
    translated [Fn] under [square] with a define built from the
    translation, and calls it on [4] through a [Call] inside a [Do]. The
    trace is the call's value, then the printed value of the
    intermediate translated [Fn] evaluated on its own. *)
let ex_4_10 () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let square = Fn ([ "x" ], Call (Name "*", [ Name "x"; Name "x" ])) in
  let call = Call (Name "square", [ Lit (Value.int 4) ]) in
  let intermediate = from_new_syntax square >>= fun lam -> eval lam env in
  let program =
    from_new_syntax square
    >>= fun lam ->
    from_new_syntax call
    >>= fun call ->
    let define = Ast.definition (Ast.define_variable "square" lam) in
    Ast.sequence [ define; call ] >>= fun program -> eval program env
  in
  [ render program; render intermediate ]
;;
