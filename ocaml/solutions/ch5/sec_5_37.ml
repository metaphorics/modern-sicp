(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind
let union a b = a @ List.filter (fun r -> not (List.mem r a)) b
let difference a b = List.filter (fun r -> not (List.mem r b)) a

let rec always_preserving regs (first : C.seq) second =
  match regs with
  | [] -> C.append_sequences [ first; second ]
  | r :: rest ->
    always_preserving
      rest
      (C.make_instruction_sequence
         (union [ r ] first.needs)
         (difference first.modifies [ r ])
         ((M.Save r :: first.statements) @ [ M.Restore r ]))
      second
;;

let rec compile_without_preserving state e target linkage =
  C.compile_open
    ~preserving:always_preserving
    ~self:compile_without_preserving
    state
    e
    target
    linkage
;;

let stack_operations (s : C.seq) =
  List.filter
    (function
      | M.Save _ | M.Restore _ -> true
      | _ -> false)
    s.statements
;;

let rendered s =
  String.concat "; " (List.map Sec_5_35.statement_to_string (stack_operations s))
;;

let last_rhs source =
  let* p = Sec_5_33.program ~filename:"ex_5_37.ml" source in
  match List.rev (Check.items p) with
  | Ast.Value_item (_, [ b ]) :: _ -> Ok (p, b.rhs)
  | _ -> Error (Eval_error.Invalid_form "the unit ends in one binding")
;;

let factorial_program = Sec_5_33.factorial ^ "\nlet result = factorial 5\n"
let simple = "let f a b = a + b\nlet g a = a\nlet combination = f (g 1) 2\n"

let ex_5_37 () =
  let* _, combination = last_rhs simple in
  let* p, _ = last_rhs factorial_program in
  let* factorial_rhs =
    match Check.items p with
    | Ast.Value_item (true, [ b ]) :: _ -> Ok b.rhs
    | _ -> Error (Eval_error.Invalid_form "factorial first")
  in
  let with_ = C.compile (C.new_state ()) factorial_rhs "val" C.Next in
  let without = compile_without_preserving (C.new_state ()) factorial_rhs "val" C.Next in
  let items = Check.items p in
  let* run_with = Sec_5_33.run_code (C.compile_program (C.new_state ()) items) in
  let* run_without =
    Sec_5_33.run_code
      (C.compile_program_with ~compile:compile_without_preserving (C.new_state ()) items)
  in
  let size (s : C.seq) =
    Printf.sprintf
      "%d statements, %d saves/restores"
      (List.length s.statements)
      (List.length (stack_operations s))
  in
  let run (r : Sec_5_33.run) =
    Printf.sprintf
      "answers %s with %d pushes, depth %d"
      (Sicp_common.Value.to_string r.value)
      r.pushes
      r.depth
  in
  Ok
    [ "f (g 1) 2 with preserving: "
      ^ rendered (C.compile (C.new_state ()) combination "val" C.Next)
    ; "f (g 1) 2 without: "
      ^ rendered (compile_without_preserving (C.new_state ()) combination "val" C.Next)
    ; "factorial with preserving: " ^ size with_
    ; "factorial without: " ^ size without
    ; "factorial 5 with preserving: " ^ run run_with
    ; "factorial 5 without: " ^ run run_without
    ]
;;
