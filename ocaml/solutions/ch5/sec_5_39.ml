(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let position frames (frame, displacement) =
  List.fold_left
    ( + )
    displacement
    (List.map List.length (List.filteri (fun i _ -> i < frame) frames))
;;

let bad detail = Error (Eval_error.Bad_instruction detail)

let lexical_address_lookup ~frame ~displacement ~position ~name env =
  match Env.find_at env position with
  | Some (bound, v) when bound = name -> Ok v
  | Some (bound, _) ->
    bad
      (Printf.sprintf
         "lexical address (%d, %d) of %s holds %s"
         frame
         displacement
         name
         bound)
  | None ->
    bad
      (Printf.sprintf
         "lexical address (%d, %d) of %s is unassigned"
         frame
         displacement
         name)
;;

let int_of what w =
  match w with
  | W.V v ->
    (match Value.view v with
     | Value.Int n -> Ok n
     | _ -> bad (what ^ " expects an integer"))
  | w -> bad (what ^ " expects an integer, not " ^ W.word_to_string w)
;;

let operation =
  ( "lexical-address-lookup"
  , M.Value_op
      (function
        | [ address; offset; W.Exp e; W.Env env ] ->
          let* position = int_of "lexical-address-lookup" offset in
          let* frame, displacement =
            match address with
            | W.V v ->
              (match Value.view v with
               | Value.Tuple [ f; d ] ->
                 let* f = int_of "lexical-address-lookup" (W.V f) in
                 let* d = int_of "lexical-address-lookup" (W.V d) in
                 Ok (f, d)
               | _ -> bad "lexical-address-lookup expects a (frame, displacement) address")
            | w -> bad ("lexical-address-lookup address " ^ W.word_to_string w)
          in
          let* name =
            match Ast.view e with
            | Ast.Var x -> Ok x
            | _ -> bad "lexical-address-lookup expects a variable"
          in
          Result.map
            (fun v -> W.V v)
            (lexical_address_lookup ~frame ~displacement ~position ~name env)
        | ws when List.length ws = 4 -> bad "lexical-address-lookup operands"
        | ws -> Error (Eval_error.Arity_mismatch { expected = 4; given = List.length ws }))
  )
;;

let lookup_instruction frames (frame, displacement) name target =
  M.Assign_op
    ( target
    , "lexical-address-lookup"
    , [ M.Const (W.V (Value.tuple [ Value.int frame; Value.int displacement ]))
      ; M.Const (W.V (Value.int (position frames (frame, displacement))))
      ; M.Const (W.Exp (Ast.var name))
      ; M.Reg "env"
      ] )
;;

let ex_5_39 () =
  let frames = Sec_5_41.book_environment in
  let values = [ [ 21; 22 ]; [ 11; 12; 13; 14; 15 ]; [ 1; 2 ] ] in
  let env =
    List.fold_right
      (fun (names, vs) env -> Env.extend (List.combine names (List.map Value.int vs)) env)
      (List.combine frames values)
      Env.empty
  in
  let lookup name address =
    let frame, displacement = address in
    let outcome =
      lexical_address_lookup
        ~frame
        ~displacement
        ~position:(position frames address)
        ~name
        env
    in
    Printf.sprintf
      "%s at (%d, %d): %s"
      name
      frame
      displacement
      (match outcome with
       | Ok v -> Value.to_string v
       | Error e -> "error: " ^ Eval_error.to_string e)
  in
  let pending, _cells = Env.extend_recursive [ "loop" ] env in
  let unassigned =
    match
      lexical_address_lookup ~frame:0 ~displacement:0 ~position:0 ~name:"loop" pending
    with
    | Ok v -> "loop before its group is filled: " ^ Value.to_string v
    | Error e -> "loop before its group is filled: error: " ^ Eval_error.to_string e
  in
  Ok
    [ lookup "c" (1, 2)
    ; lookup "x" (2, 0)
    ; lookup "y" (0, 0)
    ; lookup "y" (2, 1)
    ; unassigned
    ]
;;
