(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let machine_code blocks =
  C.make_instruction_sequence
    []
    []
    (List.concat_map
       (fun (label, code) -> (M.Label label :: code) @ [ M.Goto "done" ])
       blocks
     @ Sec_5_45.runtime
     @ [ M.Label "done" ])
;;

let execute ~emit blocks env label =
  let* m = C.load ~emit (machine_code blocks) in
  let* () = M.set_register m "env" (W.Env env) in
  let* () = M.goto_label m label in
  let* () = M.start m in
  let* value = M.get_register m "val" in
  let* env = M.get_register m "env" in
  match value, env with
  | W.V v, W.Env env -> Ok (v, env)
  | w, W.Env _ | _, w ->
    Error (Eval_error.Bad_instruction ("the loop reads " ^ W.word_to_string w))
;;

let read_compile_execute_print ~emit items =
  let state = C.new_state () in
  let* _, _, lines =
    List.fold_left
      (fun acc item ->
         let* blocks, env, lines = acc in
         let label = Printf.sprintf "form-%d" (List.length blocks) in
         let blocks =
           blocks
           @ [ label, Sec_5_45.block_statements (C.compile_program state [ item ]) ]
         in
         let* v, env = execute ~emit blocks env label in
         Ok (blocks, env, lines @ [ Sec_5_48.describe env item v ]))
      (Ok ([], Sec_5_45.global_environment ~emit, []))
      items
  in
  Ok lines
;;

let source =
  "let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)\n\
   let a = fib 12\n\
   let double x = x + x\n\
   let b = double 441\n"
;;

let ex_5_49 () =
  let* p = Sec_5_33.program ~filename:"ex_5_49.ml" source in
  read_compile_execute_print ~emit:ignore (Check.items p)
;;
