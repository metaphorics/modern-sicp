(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.26: [unless] as a special form versus [unless] as a
    procedure.  Ben's side is a derived expression: a rewrite of
    [unless c u e] to [if c then e else u] that runs before the
    applicative-order direct evaluator, so the unchosen arm is never
    evaluated.  Alyssa's side keeps [unless] an ordinary procedure under
    the lazy experiment, so it is a value a higher-order procedure can
    receive; the same value use finds [unless] unbound under Ben's
    rewrite, where it is only syntax. *)

(** [ex_4_26 ()] answers, in order, the armed call
    [unless (1 = 1) (1 / 0) 42] under Ben's rewrite and under the lazy
    experiment, then [unless] mapped over two armed triples under the
    lazy experiment and under Ben's rewrite. *)
val ex_4_26 : unit -> string list
