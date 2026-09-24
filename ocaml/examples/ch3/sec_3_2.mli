(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.2, grouped by
    subsection. Each value a listing displays is asserted by
    [run_sec_3_2] through [Replay], so the book's result comments are
    true by construction. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

(** 3.2.1: a definition is sugar for binding the name to a [fun] value;
    both spellings bind [square] to the same kind of closure in the
    global frame. *)
module Square : sig
  val square : int -> int
  val square_from_fun : int -> int
end

(** 3.2.2: the three closures of the [f 5] walkthrough, each created in
    the global environment. *)
module F_and_sum_of_squares : sig
  val square : int -> int
  val sum_of_squares : int -> int -> int
  val f : int -> int
end

(** 3.2.3: [make_withdraw] and the closure-plus-cell shapes the section
    draws: one cell shared by [bump] and [peek], and one cell per
    one-closure counter. *)
module Local_state : sig
  val make_withdraw : int -> int -> withdraw_result
  val make_shared_counter : unit -> (unit -> int) * (unit -> int)
  val make_counter : unit -> unit -> int
end

(** 3.2.4: the block-structured square root whose internal definitions
    Figure 3.11 places in the frame of one [sqrt] call. *)
module Internal_definitions : sig
  val sqrt : float -> float
end
