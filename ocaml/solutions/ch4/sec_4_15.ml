(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.15 *)

(** Exercise 4.15: the halting argument made executable. The standard
    evaluator runs under a fuel budget, so a non-terminating object
    program answers the typed "fuel is exhausted" error instead of
    hanging. The statement's diagonal [(try try)] runs under two
    [halts?] procedures -- one that answers yes always, one that answers
    no always, each a constant oracle whose answer is probed through the
    evaluator before the diagonal runs: the yes answer sends [try] into
    [run-forever] until the fuel runs out, and the no answer produces
    [halted] for a program the yes run just proved cannot halt.
    Whichever way [halts?] decides, the diagonal makes it wrong. *)

module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** [fuel] is the depth budget left: every evaluation under [Ev.eval]
    spends one unit before it dispatches. *)
let fuel = ref 0

(** The standard dispatch under the budget: when the fuel is gone the
    evaluator answers the exhaustion error, which unwinds the whole
    evaluation the way an error unwind does. *)
module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let eval exp env =
    if !fuel = 0
    then Error (Eval_error.Invalid_form "the fuel is exhausted")
    else (
      fuel := !fuel - 1;
      C.eval exp env)
  ;;
end

(** [run_fueled budget env text] reads one form of [text] and evaluates
    it under [Ev.eval] with [budget] units of fuel. *)
let run_fueled budget env text =
  fuel := budget;
  match Reader.read text with
  | Ok exp -> Ev.eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
;;

(** [try_program oracle] is the statement's pair of definitions plus the
    oracle: [halts?] answers [oracle] whatever procedure and object it
    examines. *)
let try_program oracle =
  "(define (run-forever) (run-forever)) (define (try p) (if (halts? p p) (run-forever) \
   'halted)) (define (halts? p a) "
  ^ oracle
  ^ ")"
;;

(** [render r] is the printed outcome of one demonstration step. *)
let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [diagonal oracle] defines [run-forever], [try], and the [oracle]
    halts? in a fresh global environment, probes the oracle's answer
    through the evaluator, then evaluates [(try try)] under the fuel
    budget, answering the observation trace: the probed answer, then the
    run's outcome. *)
let diagonal oracle =
  let env = SE.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = SE.run_program env (try_program oracle) in
  let answer = run_fueled 100 env "(halts? 'x 'x)" in
  let outcome = run_fueled 200 env "(try try)" in
  [ "halts? answered " ^ render answer; render outcome ]
;;

(** [run_forever_trace ()] runs the diagonal under the always-yes
      [halts?]: the yes answer sends [try] into [run-forever], and the
      fuel runs out. *)
let run_forever_trace () = diagonal "#t"

(** [halts_trace ()] runs the diagonal under the always-no [halts?]:
      the no answer makes [try] return the symbol [halted]. *)
let halts_trace () = diagonal "#f"

(** [ex_4_15 ()] runs both demonstrations in the order the statement
    raises the two outcomes. *)
let ex_4_15 () = run_forever_trace () @ halts_trace ()
