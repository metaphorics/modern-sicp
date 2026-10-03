(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Lazy_eval = Sicp_ch4.Sec_4_2

let ( let* ) = Result.bind

(* The variant's application clause evaluates the operator without
   forcing it, so a delayed operator reaches [apply] as a thunk. *)
let unforced st ~self e env =
  match Ast.view e with
  | Ast.Apply (operator, operands) ->
    let* operator = self operator env in
    Lazy_eval.apply ~self st env operator operands
  | _ -> Lazy_eval.open_eval ~self st e env
;;

let program =
  "let add a b = a + b\n\
   let run_with op = op 2 3\n\
   let () = print_int (run_with add); print_newline ()\n"
;;

let run open_ =
  let st = Lazy_eval.state () in
  Sicp_ch4.Sec_4_1.transcript
    ~experiment:Check.Lazy
    (Lazy_eval.run_with ~self:(Lazy_eval.fix (open_ st)) st)
    program
;;

let ex_4_28 () = [ run (fun st ~self -> Lazy_eval.open_eval ~self st); run unforced ]
