(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the programs of 4.2 under the strict core and
   under the named lazy experiment, whose transcript ends with its thunk
   counts. *)

module Replay = Sicp_ch1.Replay
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error

let transcript ~experiment eval source =
  match Check.check_experiment ~experiment ~filename:"replay.ml" source with
  | Error d -> "rejected: " ^ Check.kind_to_string d.kind
  | Ok program ->
    let out = Buffer.create 64 in
    (match eval ~emit:(Buffer.add_string out) program with
     | Ok _ -> Buffer.contents out
     | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e)
;;

let strict = transcript ~experiment:Check.Core Sicp_ch4.Sec_4_1.run
let lazy_ = transcript ~experiment:Check.Lazy Sicp_ch4.Sec_4_2.run

let () =
  (* 4.2.1: [try] dies under applicative order and answers under normal
     order, where the division is never demanded. *)
  let try_ =
    "let try_ a b = if a = 0 then 1 else b\nlet () = print_int (try_ 0 (1 / 0))"
  in
  Replay.expect (strict try_) "error: division by zero";
  Replay.expect
    (lazy_ try_)
    "1thunk-allocations: 2\n\
     thunk-forces: 1\n\
     thunk-recomputations: 1\n\
     thunk-memo-hits: 0\n";
  (* 4.2.2: a thunk forced twice computes once. *)
  Replay.expect
    (lazy_
       "let twice x = x + x\n\
        let () = print_int (twice (let () = print_string \"computed \" in 21))")
    "computed 42thunk-allocations: 1\n\
     thunk-forces: 2\n\
     thunk-recomputations: 1\n\
     thunk-memo-hits: 1\n";
  (* A pattern forces exactly the delayed subjects whose shape it tests:
     reaching the second element forces the delayed tail. *)
  let nested =
    "let cons x y = x :: y\n\
     let () = match cons 1 (cons 2 []) with _ :: x :: _ -> print_int x | _ -> print_int 0"
  in
  Replay.expect (strict nested) "2";
  Replay.expect
    (lazy_ nested)
    "2thunk-allocations: 4\n\
     thunk-forces: 2\n\
     thunk-recomputations: 2\n\
     thunk-memo-hits: 0\n"
;;
