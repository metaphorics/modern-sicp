(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.10 *)

(** Exercise 3.10: the lifetime of the ref cell that [make_withdraw]
    allocates at its [let] binding and its returned closure captures.
    The traced variant hands back the cell as a witness, so the exercise
    can show by physical equality that one account's every call reaches
    the same cell while two accounts hold different cells. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

(** [make_withdraw initial] is the section's withdrawal processor. *)
val make_withdraw : int -> int -> withdraw_result

(** An account paired with the cell its withdrawal closure captured. *)
type withdraw_with_cell =
  { withdraw : int -> withdraw_result
  ; balance_cell : int ref
  }

(** [make_withdraw_traced initial] builds the same processor as
    [make_withdraw initial] and also returns the cell allocated by the
    [let balance = ref initial in] binding. *)
val make_withdraw_traced : int -> withdraw_with_cell

(** [ex_3_10 ()] is
    [(contents after the first call, the two accounts' cells physically
    distinct, the two cell contents after w2's own withdrawal, the
    answer to a withdrawal that follows a direct write to the cell)],
    the observations the statement's physical-equality checks produce. *)
val ex_3_10 : unit -> int * bool * (int * int) * withdraw_result
