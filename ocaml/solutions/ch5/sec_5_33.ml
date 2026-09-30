(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let program ~filename source =
  Result.map_error
    (fun d -> Eval_error.Invalid_form (Check.diagnostic_to_string d))
    (Check.check ~filename source)
;;

type run =
  { value : Value.t
  ; output : string
  ; steps : int
  ; pushes : int
  ; depth : int
  }

let run_code ?operations code =
  let out = Buffer.create 16 in
  let* m = C.load ?operations ~emit:(Buffer.add_string out) code in
  M.initialize_stack m;
  let* () = M.start m in
  let* w = M.get_register m "val" in
  let* value =
    match w with
    | W.V v -> Ok v
    | w -> Error (Eval_error.Bad_instruction ("val holds " ^ W.word_to_string w))
  in
  let pushes, depth = M.stack_statistics m in
  Ok { value; output = Buffer.contents out; steps = M.executed m; pushes; depth }
;;

let run_program ?operations program =
  run_code ?operations (C.compile_program (C.new_state ()) (Check.items program))
;;

let factorial = "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n"

let factorial_alt =
  "let rec factorial_alt n = if n = 1 then 1 else n * factorial_alt (n - 1)"
;;

let definition_code source =
  let* p = program ~filename:"ex_5_33.ml" source in
  match Check.items p with
  | [ Ast.Value_item (true, [ binding ]) ] ->
    Ok (C.compile (C.new_state ()) binding.rhs "val" C.Next)
  | _ -> Error (Eval_error.Invalid_form "one recursive definition")
;;

let stack_operations (seq : C.seq) =
  String.concat
    "; "
    (List.filter_map
       (function
         | M.Save r -> Some ("save " ^ r)
         | M.Restore r -> Some ("restore " ^ r)
         | _ -> None)
       seq.statements)
;;

let measured name definition =
  let* p =
    program ~filename:"ex_5_33.ml" (definition ^ "\nlet result = " ^ name ^ " 5\n")
  in
  run_program p
;;

let ex_5_33 () =
  let* plain = definition_code factorial in
  let* alt = definition_code factorial_alt in
  let* plain_run = measured "factorial" factorial in
  let* alt_run = measured "factorial_alt" factorial_alt in
  let line name code (r : run) =
    Printf.sprintf
      "%s: %d statements; %s; answers %s in %d steps"
      name
      (List.length code.C.statements)
      (stack_operations code)
      (Value.to_string r.value)
      r.steps
  in
  Ok [ line "factorial" plain plain_run; line "factorial_alt" alt alt_run ]
;;

(* The hand-optimized body of [factorial_alt].  It replaces the span of
   the naive compilation from [entry1] to [after-if5]; every other
   statement of the unit is the compiler's.  The labels it names are the
   ones the naive compilation of [factorial_alt_program] generates. *)
let v w = M.Const (W.V w)
let n = M.Const (W.Exp (Ast.var "n"))
let binary op = M.Const (W.Exp (Ast.arith op (Ast.var "a") (Ast.var "b")))
let equal = M.Const (W.Exp (Ast.compare_ Ast.Eq (Ast.var "a") (Ast.var "b")))

let optimized_body =
  [ M.Label "entry1"
  ; M.Assign_op ("env", "compiled-procedure-bind", [ M.Reg "proc"; M.Reg "argl" ])
  ; M.Assign_op ("arg1", "lookup-variable-value", [ n; M.Reg "env" ])
  ; M.Assign ("arg2", v (Value.int 1))
  ; M.Assign_op ("val", "apply-binary", [ equal; M.Reg "arg1"; M.Reg "arg2" ])
  ; M.Test ("false?", [ M.Reg "val" ])
  ; M.Branch "false-branch4"
  ; M.Assign ("val", v (Value.int 1))
  ; M.Goto_reg "continue"
  ; M.Label "false-branch4"
  ; M.Save "continue"
  ; M.Save "arg1"
  ; M.Assign_op ("val", "apply-binary", [ binary Ast.Sub; M.Reg "arg1"; M.Reg "arg2" ])
  ; M.Assign_op ("argl", "adjoin-arg", [ M.Reg "val"; M.Const (W.Args []) ])
  ; M.Assign ("continue", M.Label_ref "after-call8")
  ; M.Goto "entry1"
  ; M.Label "after-call8"
  ; M.Restore "arg1"
  ; M.Assign ("arg2", M.Reg "val")
  ; M.Assign_op ("val", "apply-binary", [ binary Ast.Mul; M.Reg "arg1"; M.Reg "arg2" ])
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ]
;;

let factorial_alt_program = factorial_alt ^ "\nlet result = factorial_alt 5\n"

let index_of label statements =
  let rec go i = function
    | [] -> Error (Eval_error.Unknown_label label)
    | M.Label l :: _ when l = label -> Ok i
    | _ :: rest -> go (i + 1) rest
  in
  go 0 statements
;;

let splice (naive : C.seq) =
  let* first = index_of "entry1" naive.statements in
  let* last = index_of "after-if5" naive.statements in
  let before = List.filteri (fun i _ -> i < first) naive.statements in
  let after = List.filteri (fun i _ -> i > last) naive.statements in
  Ok { naive with statements = before @ optimized_body @ after }
;;

let ex_5_33a () =
  let* p = program ~filename:"ex_5_33a.ml" factorial_alt_program in
  let naive = C.compile_program (C.new_state ()) (Check.items p) in
  let* optimized = splice naive in
  let* naive_run = run_code naive in
  let* optimized_run = run_code optimized in
  let win = naive_run.steps - optimized_run.steps in
  Ok
    [ Printf.sprintf "naive steps: %d" naive_run.steps
    ; Printf.sprintf "optimized steps: %d" optimized_run.steps
    ; "naive answer: " ^ Value.to_string naive_run.value
    ; "optimized answer: " ^ Value.to_string optimized_run.value
    ; Printf.sprintf
        "instruction win: %d of %d (%.1f%%)"
        win
        naive_run.steps
        (100.0 *. float_of_int win /. float_of_int naive_run.steps)
    ]
;;
