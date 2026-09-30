(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.41: the lexical address of a variable in a compile-time
    environment. *)

(** [find_variable name frames] is the lexical address of [name] in the
    compile-time environment [frames], innermost frame first: the frame
    number and the displacement within that frame of the nearest
    binding, or [None] when no frame binds [name]. *)
val find_variable : string -> string list list -> (int * int) option

(** [address_to_string a] is [a] as [(frame, displacement)], or
    [not found]. *)
val address_to_string : (int * int) option -> string

(** [book_environment] is the exercise's compile-time environment
    [[y; z]; [a; b; c; d; e]; [x; y]]. *)
val book_environment : string list list

(** [ex_5_41 ()] is the addresses of [c], [x], and [w] in
    [book_environment]. *)
val ex_5_41 : unit -> (string list, Sicp_common.Eval_error.t) result
