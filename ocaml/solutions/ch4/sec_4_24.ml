(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.24 *)

let ( let* ) = Result.bind

module S = Sicp_ch4.Sec_4_1

let benchmark = "let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2) in fib 22"
let runs = 5

(* Processor time of one call, the best of [runs] after one warm-up
   call, so a collection or a scheduling pause in one run does not
   decide the answer. *)
let best_time f =
  let* _ = f () in
  let rec go n best =
    if n = 0
    then Ok best
    else (
      let start = Sys.time () in
      let* _ = f () in
      go (n - 1) (Float.min best (Sys.time () -. start)))
  in
  go runs Float.infinity
;;

let timings () =
  let* e = Sec_4_1.open_expression [] benchmark in
  let env = S.the_global_environment () in
  let* direct = best_time (fun () -> S.eval_expr e env) in
  let execution = S.analyze e in
  let* analyzed = best_time (fun () -> execution env) in
  let* analysis = best_time (fun () -> Ok (S.analyze e)) in
  Ok (direct, analyzed, analysis)
;;

let ex_4_24 () =
  let* direct, analyzed, analysis = timings () in
  Ok
    [ Printf.sprintf "direct: %.6f s" direct
    ; Printf.sprintf "analyzed execution: %.6f s" analyzed
    ; Printf.sprintf "analysis alone: %.6f s" analysis
    ; Printf.sprintf "direct / analyzed: %.2f" (direct /. Float.max analyzed Float.epsilon)
    ]
;;
