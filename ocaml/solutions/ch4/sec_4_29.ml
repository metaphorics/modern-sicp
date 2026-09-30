(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2

let definitions =
  "let count = ref 0\n\
   let id x = count := !count + 1; x\n\
   let square x = x * x\n\
   let cube x = x * x * x\n"
;;

let interaction =
  definitions
  ^ "let () = print_int (square (id 10)); print_newline ()\n\
     let () = print_int !count; print_newline ()\n\
     let () = print_int (cube (id 10)); print_newline ()\n\
     let () = print_int !count; print_newline ()\n"
;;

let section st ~emit program =
  Lazy_eval.run_with
    ~self:(Lazy_eval.fix (fun ~self -> Lazy_eval.open_eval ~self st))
    st
    ~emit
    program
;;

let ex_4_29 () =
  List.map
    (fun memoize ->
       Sicp_ch4.Sec_4_1.transcript
         ~experiment:Check.Lazy
         (section (Lazy_eval.state ~memoize ()))
         interaction)
    [ true; false ]
;;

let render_counts (c : Lazy_eval.counts) =
  Printf.sprintf
    "allocations=%d forces=%d recomputations=%d memo_hits=%d"
    c.allocations
    c.forces
    c.recomputations
    c.memo_hits
;;

(* One run of [square (id 10)] under one mode: the answer is the value
   of the program's last binding, and the counts are read from the
   run's state rather than from its transcript. *)
let counted memoize =
  let st = Lazy_eval.state ~memoize () in
  let source = definitions ^ "let answer = square (id 10)\n" in
  match Check.check_experiment ~experiment:Check.Lazy ~filename:"ex_4_29a.ml" source with
  | Error d -> [ "rejected: " ^ Check.diagnostic_to_string d ]
  | Ok program ->
    (match section st ~emit:ignore program with
     | Ok answer -> [ Value.to_string answer; render_counts (Lazy_eval.counts st) ]
     | Error e -> [ "error: " ^ Eval_error.to_string e ])
;;

let ex_4_29a () = counted true @ counted false
