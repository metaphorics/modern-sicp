(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.24 *)

let ( let* ) = Result.bind

module S = Sicp_ch4.Sec_4_1

let benchmark = "let rec fib n = if n < 2 then n else fib (n - 1) + fib (n - 2) in fib 22"
let runs = 5

(* Processor time of one call, the best of [runs] after one warm-up
   call, so a collection or a scheduling pause in one run does not
   decide the answer. A call faster than the process-time resolution
   reads 0; in that case amortize repeated batches until the clock
   ticks so the reported time stays a per-call measurement rather
   than a whole-batch total. *)
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
  let* best = go runs Float.infinity in
  if best > 0.0
  then Ok best
  else (
    let batch = 1000 in
    let rec run n =
      if n = 0
      then Ok ()
      else
        let* _ = f () in
        run (n - 1)
    in
    let start = Sys.time () in
    let rec measure calls =
      let* _ = run batch in
      let elapsed = Sys.time () -. start in
      if elapsed > 0.0
      then Ok (elapsed /. float_of_int calls)
      else measure (calls + batch)
    in
    measure batch)
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
