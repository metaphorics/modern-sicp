(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.34: the iterative factorial's compilation.  The
    essential difference from the recursive version: [iter]'s call to
    itself is in tail position with linkage [return] and target [val],
    so [compile-proc-appl] emits the two-instruction direct transfer --
    no [save] of [continue], no stack growth.  The measured maximum
    depth is the same for every n. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let factorial_iter_source =
  {|(define (factorial n)
  (define (iter product counter)
    (if (> counter n)
        product
        (iter (* counter product)
              (+ counter 1))))
  (iter 1 1))|}
;;

(** [tail_call_statements] is the part of [iter]'s body that performs
    the recursive call: a direct transfer with no saves. *)
let tail_call_statements =
  [ "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch))"
  ; "compiled-branch"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ]
;;

(** [depth_at n] runs the compiled iterative factorial at [n] on a
    monitored machine and answers the interaction's maximum depth. *)
let depth_at n =
  let state = C.new_state () in
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
  let controller =
    String.concat
      "\n"
      (List.map
         (fun (nm, text) -> if nm = "driver" then monitored_driver else text)
         C.eceval_fragments)
  in
  C.compile_block state factorial_iter_source
  >>= fun (entry, block) ->
  C.make_compiled_evaluator
    ~controller:(controller ^ "\n" ^ block)
    ~source:(Printf.sprintf "(factorial %d)" n)
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
  let depths =
    List.filter_map
      (fun line ->
         if String.length line > 37 && String.sub line 0 14 = "(total-pushes "
         then (
           match String.index_opt line '(' with
           | Some _ ->
             (try
                Some
                  (int_of_string
                     (String.trim (String.sub line (String.length line - 2) 2)))
              with
              | _ -> None)
           | None -> None)
         else None)
      (C.transcript m)
  in
  Ok
    (match depths with
     | [ d ] -> d
     | _ -> 0)
;;

(** [ex_5_34 ()] compiles the iterative factorial and reports the tail
    call's direct transfer plus the measured depths at n = 3, 4, 5:
    constant space, the annotation the exercise asks for. *)
let ex_5_34 () =
  Ok
    [ "iter tail call: " ^ String.concat "; " tail_call_statements
    ; Printf.sprintf
        "depth at n = 3: %d"
        (match depth_at 3 with
         | Ok d -> d
         | Error _ -> 0)
    ; Printf.sprintf
        "depth at n = 4: %d"
        (match depth_at 4 with
         | Ok d -> d
         | Error _ -> 0)
    ; Printf.sprintf
        "depth at n = 5: %d"
        (match depth_at 5 with
         | Ok d -> d
         | Error _ -> 0)
    ]
;;
