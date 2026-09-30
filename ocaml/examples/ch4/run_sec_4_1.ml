(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: guest programs of 4.1 run through the direct and
   the analyzed evaluators, and every transcript the section shows is
   proved with [expect]. *)

module Replay = Sicp_ch1.Replay
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Eval = Sicp_ch4.Sec_4_1

(* [transcript eval source] is the guest output of [source], followed by
   the runtime error when the run stops on one. *)
let transcript eval source =
  match Check.check ~filename:"replay.ml" source with
  | Error d -> "rejected: " ^ Check.kind_to_string d.kind
  | Ok program ->
    let out = Buffer.create 64 in
    (match eval ~emit:(Buffer.add_string out) program with
     | Ok _ -> Buffer.contents out
     | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e)
;;

let both source expected =
  Replay.expect (transcript Eval.run source) expected;
  Replay.expect (transcript Eval.run_analyzed source) expected
;;

let () =
  (* 4.1.4: the driver's sample, list append. *)
  both
    "let rec append x y = match x with [] -> y | h :: t -> h :: append t y\n\
     let rec show xs = match xs with [] -> \"\" | [ x ] -> x | x :: rest -> x ^ \" \" ^ \
     show rest\n\
     let () = print_endline (show (append [ \"a\"; \"b\"; \"c\" ] [ \"d\"; \"e\"; \"f\" \
     ]))"
    "a b c d e f\n";
  (* 4.1.5: the factorial program is data for the evaluator. *)
  both
    "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n\n\
     let () = print_int (factorial 5); print_newline (); print_int (factorial 10)"
    "120\n3628800";
  (* 4.1.3: closures keep their defining environment and share captured
     references. *)
  both
    "let make_counter start = let c = ref start in function () -> c := !c + 1; !c\n\
     let () = let a = make_counter 0 in let b = make_counter 0 in\n\
     let _ = a () in print_int (a () * 10 + b ())"
    "21";
  (* A runtime failure stops the program after the output before it. *)
  both
    "let () = print_string \"before \"; print_int (1 / 0)"
    "before error: division by zero"
;;
