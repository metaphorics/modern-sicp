(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

type account =
  { deposit : float -> float
  ; withdraw : float -> float
  ; balance : unit -> float
  }

let make_account initial =
  let balance = ref initial in
  { deposit =
      (fun amount ->
        balance := !balance +. amount;
        !balance)
  ; withdraw =
      (fun amount ->
        balance := !balance -. amount;
        !balance)
  ; balance = (fun () -> !balance)
  }
;;

let make_shared_accounts initial =
  let shared = ref initial in
  let view () =
    { deposit =
        (fun amount ->
          shared := !shared +. amount;
          !shared)
    ; withdraw =
        (fun amount ->
          shared := !shared -. amount;
          !shared)
    ; balance = (fun () -> !shared)
    }
  in
  view (), view ()
;;
