(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.35: the expression compiled to Figure 5.18 is
    [(define (f x) (+ x (g (+ x 2))))].  The figure's numbering is a
    session artifact: the book's label counter had generated fourteen
    labels before this compilation.  Seeding this compiler's counter
    at 14 reproduces the figure exactly, with the edition's two
    spellings -- the entry name rides in a [(const entry16)] and the
    parameter list is one [(const x)] input. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind
let source = "(define (f x) (+ x (g (+ x 2))))"

(** [figure_statements] is Figure 5.18 in the edition's spelling, one
    controller line per instruction or label. *)
let figure_statements =
  [ "(assign val (op make-compiled-procedure) (const entry16) (reg env))"
  ; "(goto (label after-lambda15))"
  ; "entry16"
  ; "(assign env (op compiled-procedure-env) (reg proc))"
  ; "(assign env (op extend-environment) (const x) (reg argl) (reg env))"
  ; "(assign proc (op lookup-variable-value) (const +) (reg env))"
  ; "(save continue)"
  ; "(save proc)"
  ; "(save env)"
  ; "(assign proc (op lookup-variable-value) (const g) (reg env))"
  ; "(save proc)"
  ; "(assign proc (op lookup-variable-value) (const +) (reg env))"
  ; "(assign val (const 2))"
  ; "(assign argl (op list) (reg val))"
  ; "(assign val (op lookup-variable-value) (const x) (reg env))"
  ; "(assign argl (op cons) (reg val) (reg argl))"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch19))"
  ; "compiled-branch18"
  ; "(assign continue (label after-call17))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch19"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "after-call17"
  ; "(assign argl (op list) (reg val))"
  ; "(restore proc)"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch22))"
  ; "compiled-branch21"
  ; "(assign continue (label after-call20))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch22"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "after-call20"
  ; "(assign argl (op list) (reg val))"
  ; "(restore env)"
  ; "(assign val (op lookup-variable-value) (const x) (reg env))"
  ; "(assign argl (op cons) (reg val) (reg argl))"
  ; "(restore proc)"
  ; "(restore continue)"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch25))"
  ; "compiled-branch24"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch25"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "(goto (reg continue))"
  ; "after-call23"
  ; "after-lambda15"
  ; "(perform (op define-variable!) (const f) (reg val) (reg env))"
  ; "(assign val (const ok))"
  ]
;;

(** [ex_5_35 ()] compiles [source] with the counter seeded at 14 and
    answers the compiler's own output beside the figure it matches. *)
let ex_5_35 () =
  let state = C.new_state_seeded 14 in
  match Sicp_common.Reader.read source with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile C.default_config state [] exp "val" C.Next
    >>= fun seq ->
    Ok
      [ "compiled to the figure: " ^ source
      ; String.concat "\n" seq.stmts
      ; "figure matches: " ^ string_of_bool (seq.stmts = figure_statements)
      ]
;;
