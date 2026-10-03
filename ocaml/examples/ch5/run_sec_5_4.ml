(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the explicit-control evaluator runs the 5.4
   programs, and its monitored stack shows the tail-call lesson of
   5.4.2: an iterative procedure runs in constant stack depth while a
   recursive one grows with its argument. *)

module Eval = Sicp_ch5.Sec_5_4
module Check = Sicp_common.Check
module Replay = Sicp_ch1.Replay

let program source =
  match Check.check ~filename:"replay.ml" source with
  | Ok p -> p
  | Error d -> failwith (Check.diagnostic_to_string d)
;;

let output source =
  let out = Buffer.create 64 in
  match Eval.run ~emit:(Buffer.add_string out) (program source) with
  | Ok _ -> Buffer.contents out
  | Error e -> Buffer.contents out ^ "error: " ^ Sicp_common.Eval_error.to_string e
;;

let depth source =
  match Eval.stack_statistics_after (program source) with
  | Ok (_, depth) -> depth
  | Error e -> failwith (Sicp_common.Eval_error.to_string e)
;;

let recursive n =
  Printf.sprintf
    "let rec factorial n = if n = 1 then 1 else n * factorial (n - 1)\n\
     let v = factorial %d"
    n
;;

let iterative n =
  Printf.sprintf
    "let rec fact_iter product counter max = if counter > max then product else \
     fact_iter (counter * product) (counter + 1) max\n\
     let v = fact_iter 1 1 %d"
    n
;;

let () =
  Replay.expect
    (output
       "let rec factorial n = if n = 1 then 1 else n * factorial (n - 1)\n\
        let () = print_int (factorial 5)")
    "120";
  Replay.expect
    (output
       "let rec append x y = match x with [] -> y | h :: t -> h :: append t y\n\
        let () = print_int (List.length (append [ 1; 2; 3 ] [ 4; 5; 6 ]))")
    "6";
  (* 5.4.2: tail calls leave no saved frames behind. *)
  Replay.expect (string_of_bool (depth (iterative 5) = depth (iterative 10))) "true";
  let d5 = depth (recursive 5) in
  let d6 = depth (recursive 6) in
  let d10 = depth (recursive 10) in
  Replay.expect (string_of_bool (d6 > d5 && d10 - d5 = 5 * (d6 - d5))) "true"
;;
