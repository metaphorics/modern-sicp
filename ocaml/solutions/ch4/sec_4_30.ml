(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Lazy_eval = Sicp_ch4.Sec_4_2

let ( let* ) = Result.bind

(* The text's original sequence: the discarded left expression is
   evaluated and never forced. *)
let original st ~self e env =
  match Ast.view e with
  | Ast.Sequence (first, second) ->
    let* _ = self first env in
    self second env
  | _ -> Lazy_eval.open_eval ~self st e env
;;

(* Cy's sequence is the section's own: every discarded left expression
   is forced. *)
let cy st ~self e env = Lazy_eval.open_eval ~self st e env

let run open_ source =
  let st = Lazy_eval.state () in
  Sicp_ch4.Sec_4_1.transcript
    ~experiment:Check.Lazy
    (Lazy_eval.run_with ~self:(Lazy_eval.fix (open_ st)) st)
    source
;;

let for_each =
  "let rec for_each proc items =\n\
  \  match items with\n\
  \  | [] -> print_endline \"done\"\n\
  \  | x :: rest -> proc x; for_each proc rest\n\
   let () = for_each (fun x -> print_int x; print_newline ()) [ 57; 321; 88 ]\n"
;;

let p1_p2 =
  "let rec show l =\n\
  \  match l with\n\
  \  | [] -> print_newline ()\n\
  \  | x :: rest -> print_int x; print_string \" \"; show rest\n\
   let p1 x = x := List.append !x [ 2 ]; !x\n\
   let p2 x =\n\
  \  let p e = e; !x in\n\
  \  p (x := List.append !x [ 2 ])\n\
   let () = show (p1 (ref [ 1 ]))\n\
   let () = show (p2 (ref [ 1 ]))\n"
;;

let ex_4_30 () =
  [ run original for_each; run cy for_each; run original p1_p2; run cy p1_p2 ]
;;
