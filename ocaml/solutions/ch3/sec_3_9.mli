(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.9 *)

(** Exercise 3.9: stack growth of the two factorials of @ref{1.2.1}
    under OCaml's guaranteed tail calls. The recursive version keeps one
    frame per pending multiplication; the iterative version's own call
    is its last action, so the compiler turns it into a loop. *)

(** [factorial n] is the book's recursive factorial; its stack depth
    grows linearly with [n]. *)
val factorial : int -> int

(** [factorial_iter n] is the book's iterative factorial carried by
    [fact_iter]; it runs in constant stack space. *)
val factorial_iter : int -> int

(** [factorial_traced n] is [(factorial n, d)] where [d] is the depth
    of the deepest call frame live at any point of the call. *)
val factorial_traced : int -> int * int

(** [factorial_iter_traced n] is [(factorial_iter n, d)] where [d] is
    the depth of the deepest call frame live at any point of the call. *)
val factorial_iter_traced : int -> int * int

(** [ex_3_09 ()] is [(factorial_traced 6, factorial_iter_traced 6)],
    the instrumented runs the statement checks against the drawing. *)
val ex_3_09 : unit -> (int * int) * (int * int)
