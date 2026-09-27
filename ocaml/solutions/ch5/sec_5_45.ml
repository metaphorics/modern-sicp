(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.45: the compiler's stack use against the interpreter's
    and the special-purpose machine's, for the recursive factorial.

    All three run under the same monitored stack: the interpreted
    session on the 5.4 machine (5.27's), the compiled session on the
    5.5.7 machine with the monitored driver, and the special-purpose
    machine of Figure 5.11 as 5.14 measured it.  The ratios of pushes
    and depths approach constants; the special-purpose machine is far
    ahead of the compiled code, which in turn is far ahead of the
    interpreter -- the compiler's saves are general-purpose ones, the
    hand-tailored controller needs only [n] and [continue]. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

(** The monitored driver: statistics printed before each value. *)
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

(** [int_after prefix line] is the integer the digits right after
    [prefix] spell, searching anywhere in the line: both the 5.4
    machine's [(total-pushes = P maximum-depth = D)] and the 5.2
    machine's prefixed [total-pushes = P maximum-depth = D] carry the
    two counters behind the same markers. *)
let int_after prefix line =
  let plen = String.length prefix in
  let len = String.length line in
  let rec from i =
    if i + plen > len
    then None
    else if String.sub line i plen = prefix
    then (
      let rec digits j =
        if j < len && line.[j] >= '0' && line.[j] <= '9' then digits (j + 1) else j
      in
      let stop = digits (i + plen) in
      if stop = i + plen
      then None
      else int_of_string_opt (String.sub line (i + plen) (stop - i - plen)))
    else from (i + 1)
  in
  from 0
;;

(** [parse_stats line] reads the counters of a
    [(total-pushes = P maximum-depth = D)] line. *)
let parse_stats line =
  match int_after "total-pushes = " line, int_after "maximum-depth = " line with
  | Some p, Some d -> Some (p, d)
  | _ -> None
;;

(** [compiled_at n] runs the compiled factorial at [n] and answers the
    session's counters. *)
let compiled_at n =
  let state = C.new_state () in
  C.compile_block state factorial_source
  >>= fun (entry, block) ->
  C.make_compiled_evaluator
    ~controller:(monitored_controller ^ "\n" ^ block)
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
  let stats = List.filter_map parse_stats (C.transcript m) in
  match List.rev stats with
  | p :: _ -> Ok p
  | [] -> Error (C.Op_failed "no statistics")
;;

(** [interpreted_at n] runs the interpreted factorial at [n] on the
    monitored 5.4 driver (5.26's and 5.27's harness) and answers the
    counters. *)
let interpreted_at n =
  Sec_5_26.run (Sec_5_27.recursive_source ^ "\n(factorial " ^ string_of_int n ^ ")")
  >>= fun transcript ->
  let stats = Sec_5_26.stats_of transcript in
  match List.rev stats with
  | s :: _ -> Ok (Sec_5_26.pushes_of s, Sec_5_26.depth_of s)
  | [] -> Error (C.Op_failed "no statistics")
;;

(** [special_at n] runs the special-purpose machine of Figure 5.11 (as
    5.14 measured it) at [n]. *)
let special_at n =
  Sec_5_14.measure n
  >>= fun line ->
  match parse_stats line with
  | Some p -> Ok p
  | None -> Error (C.Op_failed "no statistics")
;;

(** [ex_5_45 ()] measures n = 5 and 10 on all three machines and
    answers the ratios: compiled over interpreted, special-purpose
    over interpreted.  The measured constants at n = 10: the compiled
    code uses about a fifth of the interpreter's pushes and about half
    its depth; the special-purpose machine about a tenth of the
    pushes and a fixed depth of 2n -- the compiler is closer to the
    interpreter than to the hand-tailored controller, and (b)'s
    improvements (open coding, which 5.38 measured, and direct calls
    for known procedures) all shrink the general machinery around
    each recursive step. *)
let ex_5_45 () =
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
  run_one 5 >>= fun line5 -> run_one 10 >>= fun line10 -> Ok [ line5; line10 ]
;;
