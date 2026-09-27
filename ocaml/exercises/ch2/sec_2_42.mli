(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.42:
   the eight-queens puzzle *)

(** A board with no positions. *)
val empty_board : (int * int) list

(** [adjoin_position row column positions] adds [row, column] to the
    set of positions. *)
val adjoin_position : int -> int -> (int * int) list -> (int * int) list

(** [safe k positions] holds when the queen in column [k] checks none
    of the queens in the other columns of [positions]. *)
val safe : int -> (int * int) list -> bool

(** [ex_2_42 board_size] is the sequence of all solutions of the
    [board_size]-queens puzzle, each a list of [row, column] pairs. *)
val ex_2_42 : int -> (int * int) list list
