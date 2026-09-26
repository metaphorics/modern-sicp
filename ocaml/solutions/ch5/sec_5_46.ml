(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.46: the fib analysis of 5.45's method.  For the
    tree-recursive fib the ratios do not converge: every n doubles the
    call tree, so the compiled, interpreted, and special-purpose runs
    all grow in constant proportion per n -- and, as 5.29 measured for
    the interpreter, each level of the tree adds its constant.

    The special-purpose machine is Figure 5.12, run on the 5.2
    simulator with the monitored stack, exactly as 5.14 measured the
    factorial machine. *)

module C = Sicp_ch5.Sec_5_5
module Machine = Sicp_ch5.Sec_5_2

let ( >>= ) = Result.bind

let fib_source =
  {|(define (fib n)
  (if (< n 2)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))|}
;;

let monitored_controller =
  let monitored_driver =
    ";; branches if flag is set:\n(branch (label external-entry))\n"
    ^ {|read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))|}
  in
  String.concat
    "\n"
    (List.map
       (fun (nm, text) -> if nm = "driver" then monitored_driver else text)
       C.eceval_fragments)
;;

(** The Figure 5.12 machine with the measurement prologue and
    epilogue. *)
let special_controller =
  {|(controller
   (perform (op initialize-stack))
   (assign continue (label fib-done))
 fib-loop
   (test (op <) (reg n) (const 2))
   (branch (label immediate-answer))
   (save continue)
   (assign continue (label afterfib-n-1))
   (save n)
   (assign n (op -) (reg n) (const 1))
   (goto (label fib-loop))
 afterfib-n-1
   (restore n)
   (restore continue)
   (assign n (op -) (reg n) (const 2))
   (save continue)
   (assign continue (label afterfib-n-2))
   (save val)
   (goto (label fib-loop))
 afterfib-n-2
   (assign n (reg val))
   (restore val)
   (restore continue)
   (assign val (op +) (reg val) (reg n))
   (goto (reg continue))
 immediate-answer
   (assign val (reg n))
   (goto (reg continue))
 fib-done
   (perform (op print-stack-statistics)))|}
;;

(** [special_at n] runs the special-purpose machine at [n] and answers
    its counters. *)
let special_at n =
  Machine.make_machine
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:special_controller
  >>= fun m ->
  Machine.set_register m "n" (Machine.Int n)
  >>= fun () ->
  Machine.start m
  >>= fun () ->
  match List.rev (List.filter_map Sec_5_45.parse_stats (Machine.transcript m)) with
  | p :: _ -> Ok p
  | [] -> Error (C.Op_failed "no statistics")
;;

(** [compiled_at n] and [interpreted_at n] mirror 5.45's harness for
    fib. *)
let compiled_at n =
  let state = C.new_state () in
  C.compile_block state fib_source
  >>= fun (entry, block) ->
  C.make_compiled_evaluator
    ~controller:(monitored_controller ^ "\n" ^ block)
    ~source:(Printf.sprintf "(fib %d)" n)
    ~state
    ()
  >>= fun m ->
  C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab entry)
  >>= fun () ->
  C.set_flag m true;
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () ->
  let stats = List.filter_map Sec_5_45.parse_stats (C.transcript m) in
  match List.rev stats with
  | p :: _ -> Ok p
  | [] -> Error (C.Op_failed "no statistics")
;;

let interpreted_at n =
  Sec_5_29.measure_fib [ n ]
  >>= function
  | [ s ] -> Ok (Sec_5_26.pushes_of s, Sec_5_26.depth_of s)
  | _ -> Error (C.Op_failed "no statistics")
;;

(** [ex_5_46 ()] measures n = 5, 6, 7 on all three machines; the ratios
    per n stay near-constant (no convergence, the tree doubles), and
    the special-purpose machine keeps its large constant-factor lead. *)
let ex_5_46 () =
  let run_one n =
    interpreted_at n
    >>= fun (ip, id) ->
    compiled_at n
    >>= fun (cp, cd) ->
    special_at n
    >>= fun (sp, sd) ->
    Ok
      (Printf.sprintf
         "n = %d: interpreted %d/%d, compiled %d/%d, special %d/%d;           ratios \
          compiled %.3f/%.3f, special %.3f/%.3f"
         n
         ip
         id
         cp
         cd
         sp
         sd
         (float cp /. float ip)
         (float cd /. float id)
         (float sp /. float ip)
         (float sd /. float id))
  in
  run_one 5
  >>= fun l5 -> run_one 6 >>= fun l6 -> run_one 7 >>= fun l7 -> Ok [ l5; l6; l7 ]
;;
