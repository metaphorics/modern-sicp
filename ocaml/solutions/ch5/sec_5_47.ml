(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1

let ( let* ) = Result.bind

let compound_branch =
  [ M.Test ("compound-procedure?", [ M.Reg "proc" ])
  ; M.Branch "ca-compound"
  ; M.Perform ("signal-not-applicable", [ M.Reg "proc" ])
  ; M.Label "ca-compound"
  ; M.Goto "apply-entry"
  ]
;;

let runtime =
  List.concat_map
    (function
      | M.Perform ("signal-not-applicable", _) -> compound_branch
      | i -> [ i ])
    Sec_5_45.runtime
;;

let source =
  "let apply_twice f x = f (f x)\nlet add6 x = x + 6\nlet result = apply_twice add6 0\n"
;;

type mode =
  | Compiled
  | Interpreted

let session ?runtime modes =
  let* p = Sec_5_33.program ~filename:"ex_5_47.ml" source in
  let state = C.new_state () in
  let items = List.combine modes (Check.items p) in
  let blocks =
    List.filter_map
      (fun (i, (mode, item)) ->
         match mode with
         | Compiled ->
           Some
             ( Printf.sprintf "item-%d" i
             , Sec_5_45.block_statements (C.compile_program state [ item ]) )
         | Interpreted -> None)
      (List.mapi (fun i x -> i, x) items)
  in
  let* ev = Sec_5_45.make_evaluator ?runtime ~emit:ignore blocks in
  let* last, _ =
    List.fold_left
      (fun acc (i, (mode, item)) ->
         let* _, env = acc in
         match mode with
         | Compiled -> Sec_5_45.run_block ev env (Printf.sprintf "item-%d" i)
         | Interpreted -> Sec_5_45.eval_item ev env item)
      (Ok (Value.unit, Sec_5_45.global_environment ~emit:ignore))
      (List.mapi (fun i x -> i, x) items)
  in
  Ok last
;;

let outcome = function
  | Ok v -> Value.to_string v
  | Error e -> "error: " ^ Eval_error.to_string e
;;

let ex_5_47 () =
  let modes = [ Compiled; Interpreted; Interpreted ] in
  Ok
    [ "compound branch: "
      ^ String.concat "; " (List.map Sec_5_35.statement_to_string compound_branch)
    ; "without the branch: " ^ outcome (session modes)
    ; "with the branch: " ^ outcome (session ~runtime modes)
    ; "all compiled with the branch: "
      ^ outcome (session ~runtime [ Compiled; Compiled; Compiled ])
    ]
;;
