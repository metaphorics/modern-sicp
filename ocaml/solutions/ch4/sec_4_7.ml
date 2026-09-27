(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.7 *)

(** [let*] as nested lets. The shared grammar has no [let*] form and
    the core never parses text, so the exercise carries its forms as
    [shape] values and [let_star_to_nested_lets] folds the bindings
    into nested [let]s, one binding each, rightmost innermost. The
    answer to the statement's question is yes: once the rewrite
    exists, evaluating the result needs only an [eval] that handles
    [let], which is the one clause this module's dispatch adds. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** A let* shape: the bindings and the body of one let* form. *)
type shape = (string * Ast.expr) list * Ast.expr list

(** [let_star_to_nested_lets shape] lowers one let* shape to nested
    lets of the typed subset. *)
let let_star_to_nested_lets (bindings, body) =
  let rec expand = function
    | [] ->
      (match body with
       | [] -> Error (Eval_error.Invalid_form "let*: the body is empty")
       | [ last ] -> Ok last
       | exps -> Ast.sequence exps)
    | (name, init) :: rest ->
      expand rest >>= fun inner -> Ast.let_ [ name, init ] [ inner ]
  in
  expand bindings
;;

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let eval exp env =
    match Ast.view exp with
    | Ast.Let (bindings, body) ->
      let parameters = List.map fst bindings in
      let inits = List.map snd bindings in
      Ast.lambda parameters body >>= fun proc -> Ev.eval (Ast.application proc inits) env
    | _ -> C.eval exp env
  ;;
end

(** [eval_shape shape env] lowers [shape] and evaluates the result. *)
let eval_shape shape env =
  let_star_to_nested_lets shape >>= fun lowered -> Ev.eval lowered env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [statement_shape] is the statement's program: [x] is 3, [y] sees
    [x], [z] sees both, and the body multiplies [x] by [z]. The
    substrate's arithmetic primitives are strictly binary, so the
    three-operand [(+ x y 5)] is written as its nested equivalent. *)
let statement_shape =
  ( [ "x", Ast.int 3
    ; "y", Ast.application (Ast.variable "+") [ Ast.variable "x"; Ast.int 2 ]
    ; ( "z"
      , Ast.application
          (Ast.variable "+")
          [ Ast.application (Ast.variable "+") [ Ast.variable "x"; Ast.variable "y" ]
          ; Ast.int 5
          ] )
    ]
  , [ Ast.application (Ast.variable "*") [ Ast.variable "x"; Ast.variable "z" ] ] )
;;

(** [ex_4_07 ()] evaluates the statement's program through the shape
      lowering, the same program written as the nested lets the
      lowering produces, and the empty-bindings boundary. *)
let ex_4_07 () =
  let env = SE.the_global_environment () in
  let lowered = eval_shape statement_shape env in
  let equivalent_lets =
    Reader.read "(let ((x 3)) (let ((y (+ x 2))) (let ((z (+ (+ x y) 5))) (* x z))))"
    |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
    >>= fun exp -> Ev.eval exp env
  in
  let empty_bindings = eval_shape ([], [ Ast.int 7 ]) env in
  List.map render [ lowered; equivalent_lets; empty_bindings ]
;;
