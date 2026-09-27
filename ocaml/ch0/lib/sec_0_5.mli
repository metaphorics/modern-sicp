(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Lists and higher-order functions: the named definitions behind the
    listings of section 0.5. *)

(** [squares] is the squares of [1; 2; 3; 4] under [List.map]: [[1; 4; 9;
    16]]. *)
val squares : int list

(** [evens] is the even integers of [1] to [10] under [List.filter]: [[2;
    4; 6; 8; 10]]. *)
val evens : int list

(** [my_map f xs] is [List.map f xs], derived by hand. Teaching form: not
    tail recursive, unlike the library version. *)
val my_map : ('a -> 'b) -> 'a list -> 'b list

(** [my_filter p xs] is [List.filter p xs], derived by hand. Teaching
    form: not tail recursive. *)
val my_filter : ('a -> bool) -> 'a list -> 'a list

(** [my_fold_left f acc xs] is [List.fold_left f acc xs], derived by hand
    with a tail-recursive helper. *)
val my_fold_left : ('acc -> 'a -> 'acc) -> 'acc -> 'a list -> 'acc
