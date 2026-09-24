(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.20 *)

(** Exercise 3.20, replaced for this edition: tracing the aliasing the
    procedural pairs create, by physical equality of the objects the
    slots designate. *)

(** The procedural pair of the section: a record of closures over two
    cells, dispatch replaced by field selection. *)
type 'a proc_pair =
  { pcar : unit -> 'a
  ; pcdr : unit -> 'a
  ; set_pcar : 'a -> unit
  ; set_pcdr : 'a -> unit
  }

(** [proc_cons x y] is the procedural pair holding [x] and [y]. *)
val proc_cons : 'a -> 'a -> 'a proc_pair

(** [ex_3_20 ()] is [(the printed value of (car x) after (set-car! (cdr
    z) 17), whether both slots of z physically hold the same object
    as x)] for x = (proc_cons 1 2) and z = (proc_cons x x). *)
val ex_3_20 : unit -> string * bool
