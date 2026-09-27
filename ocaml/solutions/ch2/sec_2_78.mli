(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.78 *)

(** A bare [Num] needs no wrapper: OCaml's own constructor already is
    the "scheme-number" tag, the way Lisp's internal [number?] already
    distinguishes a number from every other representation. *)
type value =
  | Num of float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

(** [type_tag v] reads "scheme-number" off a bare [Num] with no
    lookup, and [t.tag] otherwise. *)
val type_tag : value -> string

(** [contents_of v] is [v] itself for a bare [Num] (a number's
    contents, per the book's plan, is the number itself) and
    [t.contents] otherwise. *)
val contents_of : value -> value

(** [attach_tag tag contents] is the identity when [tag] is
    "scheme-number" -- attaching that tag to a [Num] cannot add a
    wrapper, because [contents] must already be a [Num] -- and builds
    a [Tagged] record otherwise. *)
val attach_tag : string -> value -> value

(** [ex_2_78 ()] is [(type_tag (Num 5.), contents_of (Num 5.) = Num
    5., type_tag (attach_tag "rational" (Num 0.)))], evidence that a
    bare number needs no wrapper while every other tag still gets
    one. *)
val ex_2_78 : unit -> string * bool * string
