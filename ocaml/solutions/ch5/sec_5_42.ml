(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.42: [compile-variable] and [compile-assignment] emit
    lexical-address instructions when [find-variable] locates the name
    and fall back to the evaluator's global search when it does not
    (the only name a compile-time environment can miss is a global).
    The compiler's [lexical] configuration is exactly this rewrite; a
    miss is visible in the output as the plain
    [lookup-variable-value] instruction.  The test is the nested
    [lambda] combination at the start of 5.5.6, run to a value on the
    lexical machine. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let nested_example =
  {|(define (f x y)
  (lambda (a b c d e)
    (lambda (y z) (* x y z))))|}
;;

(** [compiled_statements src] is the lexical compilation of [src]. *)
let compiled_statements src =
  let state = C.new_state () in
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile { C.default_config with lexical = true } state [] exp "val" C.Next
    >>= fun seq -> Ok seq.stmts
;;

(** [lexical_accesses stmts] is the lexical-address instructions the
    compilation emitted. *)
let lexical_accesses stmts =
  List.filter
    (fun s ->
       let pat = "(op lexical-address-lookup)" in
       String.length s >= String.length pat
       &&
       try String.sub s 8 (String.length pat) = pat with
       | _ -> false)
    stmts
;;

(** [ex_5_42 ()] compiles the example lexically, shows the emitted
    accesses -- [x] at (2 0), [y] at (0 0), [z] at (0 1) inside the
    innermost lambda -- and runs the applied example on the lexical
    machine: [(f 3 4)] answers a procedure, and applying it to five
    arguments computes [* 3 6 10] = 180 through lexical lookups. *)
let ex_5_42 () =
  compiled_statements nested_example
  >>= fun stmts ->
  let applied =
    {|(define (f x y)
  (lambda (a b c d e)
    ((lambda (y z) (* x y z))
     (* a b x)
     (+ c d x))))
(define (run f) (f 1 2 3 4 5))
(run (f 3 4))|}
  in
  Sec_5_39.run_lexical applied
  >>= fun transcript ->
  Ok
    [ String.concat "\n" (lexical_accesses stmts)
    ; "lexical run: " ^ String.concat " " transcript
    ]
;;
