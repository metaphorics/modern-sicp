(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Bindings, functions, and modules: the named definitions behind the
    listings of section 0.3. *)

(** [square x] is [x] multiplied by itself, the translation of
    [(define (square x) ( * x x))]. *)
val square : int -> int

(** [sum_of_squares x y] is [square x + square y], the translation of
    [(define (sum-of-squares x y) (+ (square x) (square y)))]. *)
val sum_of_squares : int -> int -> int

(** [f x] binds a helper name for one expression, the way Scheme's [let]
    does: [f 3] is [20]. *)
val f : int -> int

(** [factorial n] is [n!], defined with [let rec]: [factorial 6] is [720]. *)
val factorial : int -> int

(** [inc] is the addition operator partially applied to [1]. *)
val inc : int -> int

(** [double] is the multiplication operator partially applied to [2]. *)
val double : int -> int

(** [pad_to ~width s] is [s] left-padded with dots to [width] characters,
    or [s] itself when it is already [width] long or longer. *)
val pad_to : width:int -> string -> string
