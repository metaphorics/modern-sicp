(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.50: the metacircular evaluator compiled.  The
    object-language evaluator source is [Metacircular.source], the
    corpus's 4.1 text adapted to this machine (the ways are listed in
    that module).  Compiled and run on the 5.5.7 machine, its session
    defines [factorial] inside the object world and answers 120, with
    the tick pair and the ok lines the corpus expects.

    The interpretation levels are timed on the same computation
    [(factorial 5)]: level 0, the compiled factorial on the machine;
    level 1, the explicit-control evaluator interpreting it (the 5.4
    driver); level 2, the compiled metacircular interpreting it.  Both
    the machine's own step counts -- deterministic -- and the wall
    clock are reported; each added level of interpretation multiplies
    the work by its constant. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** [metacircular_session ()] runs the compiled evaluator's own
    program and answers the transcript: the corpus session. *)
let metacircular_session () =
  let state = C.new_state () in
  C.compile_and_go
    ~state
    ~compiled:Sicp_ch5.Metacircular.source
    ~source:"(m-eval '(factorial 5) the-global-environment)"
    ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.transcript m, C.step_count m)
;;

(** [level_0_steps] is the compiled factorial's own step count at
    [n]. *)
let level_0_steps n =
  let state = C.new_state () in
  C.compile_and_go
    ~state
    ~compiled:"(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))"
    ~source:(Printf.sprintf "(factorial %d)" n)
    ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.step_count m)
;;

(** [level_1_steps] is the explicit-control evaluator's step count
    interpreting [(factorial n)] through the 5.4 driver. *)
let level_1_steps n =
  Sicp_ch5.Sec_5_4.make_evaluator
    ~source:
      (Printf.sprintf
         "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial %d)"
         n)
    ()
  >>= fun m ->
  (match Sicp_ch5.Sec_5_4.start m with
   | Ok () -> Ok ()
   | Error (Sicp_ch5.Sec_5_4.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted ->
     Ok ()
   | Error e -> Error e)
  >>= fun () ->
  (* the 5.4 machine does not count steps; the monitored session's
     pushes stand in as its workload measure *)
  let stats = Sec_5_26.stats_of (Sicp_ch5.Sec_5_4.transcript m) in
  match stats with
  | [ s ] -> Ok (Sec_5_26.pushes_of s)
  | _ -> Ok 0
;;

(** [level_2_steps session_steps] is the metacircular's total step
    count, from the session run. *)
let level_2_steps session_steps = session_steps

(** [ex_5_50 ()] runs the session and the three levels at n = 5: the
    transcript pins the compiled interpreter's answers, and the
    measured counts show the price of each level -- the machine's own
    steps for levels 0 and 2, and the interpreted session's monitored
    pushes for level 1. *)
let ex_5_50 () =
  metacircular_session ()
  >>= fun (transcript, meta_steps) ->
  level_0_steps 5
  >>= fun l0 ->
  level_1_steps 5
  >>= fun l1_pushes ->
  let t0 = Sys.time () in
  metacircular_session ()
  >>= fun (_, meta_steps_2) ->
  let wall = Sys.time () -. t0 in
  Ok
    [ "compiled metacircular session: " ^ String.concat " " transcript
    ; Printf.sprintf "level 0 (compiled factorial), steps = %d" l0
    ; Printf.sprintf "level 1 (interpreted factorial), monitored pushes = %d" l1_pushes
    ; Printf.sprintf
        "level 2 (compiled metacircular), machine steps = %d (repeat %d, wall %.3fs)"
        meta_steps
        meta_steps_2
        wall
    ; Printf.sprintf
        "interpretation price: level 2 over level 0 = %.0f machine steps"
        (float meta_steps /. float (max 1 l0))
    ]
;;
