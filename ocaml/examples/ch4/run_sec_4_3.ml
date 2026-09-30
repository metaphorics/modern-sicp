(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the 4.3 search programs under the named search
   experiment, whose transcript carries the successful branches' output
   and then the answer, choice, and failure counts. *)

module Replay = Sicp_ch1.Replay
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Ast = Sicp_common.Ast
module S = Sicp_ch4.Sec_4_3

let ( let* ) = Result.bind

(* The exercise's evaluator: [permanent_set] and [if_fail] clauses over
   the section's open dispatch; every other node falls back on
   [open_eval], which routes subexpressions through this same [eval]. *)
let rec eval : S.eval_t =
  fun search env e ->
  match Ast.view e with
  | Ast.Apply (f, args) ->
    (match Ast.view f, args with
     | Ast.Var "permanent_set", [ target; rhs ] ->
       (match Ast.view target with
        | Ast.Var name ->
          let* v = eval search env rhs in
          S.permanent_assign search name v
        | _ -> S.open_eval ~self:eval search env e)
     | Ast.Var "if_fail", [ a; b ] ->
       let* i = S.choose search 2 in
       eval search env (if i = 0 then a else b)
     | _ -> S.open_eval ~self:eval search env e)
  | _ -> S.open_eval ~self:eval search env e
;;

let forms =
  [ "permanent_set", "let permanent_set r v = r := v"; "if_fail", "let if_fail x _y = x" ]
;;

let extended source = S.run_with ~eval ~forms source

let search source =
  match Check.check_experiment ~experiment:Check.Search ~filename:"replay.ml" source with
  | Error d -> "rejected: " ^ Check.kind_to_string d.kind
  | Ok program ->
    let out = Buffer.create 64 in
    (match Sicp_ch4.Sec_4_3.run ~emit:(Buffer.add_string out) program with
     | Ok _ -> Buffer.contents out
     | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e)
;;

(* The generators of 4.3.1: a choice over a list, and a primality test. *)
let preamble =
  "let rec an_element_of items =\n\
  \  match items with\n\
  \  | [] -> require false; 0\n\
  \  | x :: rest -> amb x (an_element_of rest)\n\
   let rec divides_none n d = d * d > n || (n mod d <> 0 && divides_none n (d + 1))\n\
   let prime n = n >= 2 && divides_none n 2\n"
;;

let () =
  (* 4.3.1: the prime-sum pairs of two lists, every answer in order. *)
  Replay.expect
    (search
       (preamble
        ^ "let () =\n\
          \  let a = an_element_of [ 1; 3; 5; 8 ] in\n\
          \  let b = an_element_of [ 20; 35; 110 ] in\n\
          \  require (prime (a + b));\n\
          \  print_endline (string_of_int a ^ \" \" ^ string_of_int b)"))
    "3 20\n3 110\n8 35\nanswers: 3\nchoices: 16\nfailures: 14\n";
  (* A search whose every branch fails answers nothing. *)
  Replay.expect
    (search
       (preamble
        ^ "let () = let x = an_element_of [ 4; 6 ] in require (prime x); print_int x"))
    "answers: 0\nchoices: 2\nfailures: 3\n";
  Replay.expect
    (extended
       (preamble
        ^ "let count = ref 0\n\
           let () =\n\
          \  let x = an_element_of [ 1; 2; 3 ] in\n\
          \  let y = an_element_of [ 1; 2; 3 ] in\n\
          \  permanent_set count (!count + 1);\n\
          \  require (x <> y);\n\
          \  print_endline (string_of_int x ^ \" \" ^ string_of_int y ^ \" \" ^ \
           string_of_int !count)"))
    "1 2 2\n1 3 3\n2 1 4\n2 3 6\n3 1 7\n3 2 8\nanswers: 6\nchoices: 12\nfailures: 7\n";
  (* [if_fail] is a two-alternative choice point: the first branch's
     whole subtree fails before the second runs. *)
  Replay.expect
    (extended
       (preamble
        ^ "let () = print_int (if_fail (let x = an_element_of [ 1; 3; 5 ] in require (x \
           mod 2 = 0); x) 0)"))
    "0answers: 1\nchoices: 4\nfailures: 4\n";
  (* A [permanent_set] beneath a primitive applies once per call even
     though every attempt replays the prefix. *)
  Replay.expect
    (extended
       (preamble
        ^ "let count = ref 0\n\
           let _ = List.map (fun x -> permanent_set count (!count + x - x + 1)) [ 1; 2 ]\n\
           let x = an_element_of [ 1; 2; 3 ]\n\
           let () = require (x = 3)\n\
           let () = print_int !count"))
    "2answers: 1\nchoices: 3\nfailures: 3\n"
;;
