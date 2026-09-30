(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.18 *)

(** Exercise 4.18: an alternative scan-out strategy.

    The alternative evaluates every right-hand side into a temporary
    first and assigns the internal names only afterward.  A right-hand
    side may then use another internal name only inside a procedure it
    delays; one that reads the name's value while it runs finds the
    unassigned marker.  The book's [solve] is such a group: [dy] applies
    [stream_map] to [y] at once, so it works under the text's strategy,
    which assigns [y] before [dy] is evaluated, and fails under the
    alternative.  [solve] keeps that shape in admitted OCaml: [y] uses
    [dy] only inside a procedure, and [dy] is a pair holding [y] itself,
    a statically constructive value that OCaml builds by backpatching. *)

(** [scan_out_alternative e] is the alternative scanned-out form of the
    [let rec] group [e], or [e] itself when [e] is not one. *)
val scan_out_alternative : Sicp_common.Ast.expr -> Sicp_common.Ast.expr

(** [eval_alternative] is the evaluator of [Sec_4_16.scanning] with
    [scan_out_alternative]. *)
val eval_alternative : Sicp_ch4.Sec_4_1.eval_t

(** [solve] is the source of a procedure whose internal definitions have
    the shape of the book's [solve]. *)
val solve : string

(** [ex_4_18 ()] runs [solve] under the text's strategy, under the
    alternative, and under the standard evaluator, then runs under the
    alternative a group whose uses of each other are all delayed. *)
val ex_4_18 : unit -> string list
