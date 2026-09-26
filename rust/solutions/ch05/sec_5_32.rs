// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.32: the explicit-control
//! evaluator's symbol-operator fast path.

use ch05::sec_5_2::{Fault, OpHandler};
use ch05::sec_5_4::{compose_controller, operation};
use sicp_runtime::Value;

mod ex_5_32 {
    //! Exercise 5.32: symbol operators can be looked up in place;
    //! compile-time analysis still avoids a run-time test for every
    //! general expression.

    use super::*;

    const FAST_APPLICATION: &str = "ev-application
  (save continue)
  (assign unev (op operands) (reg exp))
  (test (op symbol-operator?) (reg exp))
  (branch (label ev-appl-symbol-operator))
  (save env)
  (save unev)
  (assign exp (op operator) (reg exp))
  (assign continue (label ev-appl-did-operator))
  (goto (label eval-dispatch))
ev-appl-symbol-operator
  (assign exp (op operator) (reg exp))
  (assign val (op lookup-variable-value) (reg exp) (reg env))
  (assign argl (op empty-arglist))
  (assign proc (reg val))
  (test (op no-operands?) (reg unev))
  (branch (label apply-dispatch))
  (save proc)
  (goto (label ev-appl-operand-loop))";

    fn symbol_operator() -> (&'static str, OpHandler) {
        operation("symbol-operator?", |args| {
            let Some(exp) = args.first() else {
                return Err(Fault::Parse(
                    "symbol-operator? needs an expression".to_owned(),
                ));
            };
            let is_symbol = exp
                .list_items()
                .ok()
                .and_then(|items| items.first().cloned())
                .is_some_and(|operator| matches!(operator, Value::Sym(_)));
            Ok(Value::boolean(is_symbol))
        })
    }

    fn monitored_driver() -> &'static str {
        "read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input) (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output) (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))"
    }

    fn measured(controller: &str, source: &str) -> Result<(u64, u64), Fault> {
        let mut evaluator =
            ch05::sec_5_4::make_evaluator(controller, &[symbol_operator()], source)?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        let stats = transcript
            .iter()
            .rev()
            .find_map(|line| line.strip_prefix("(total-pushes = "))
            .ok_or_else(|| Fault::Parse("no stack statistics printed".to_owned()))?;
        let mut fields = stats.split_whitespace();
        let pushes = fields
            .next()
            .ok_or_else(|| Fault::Parse("push count missing".to_owned()))?
            .parse()
            .map_err(|_| Fault::Parse("push count malformed".to_owned()))?;
        let depth = stats
            .split("maximum-depth = ")
            .nth(1)
            .and_then(|tail| tail.strip_suffix(')'))
            .ok_or_else(|| Fault::Parse("maximum depth missing".to_owned()))?
            .parse()
            .map_err(|_| Fault::Parse("maximum depth malformed".to_owned()))?;
        Ok((pushes, depth))
    }

    fn fast_controller() -> String {
        compose_controller(&[
            ("ev-application", FAST_APPLICATION),
            ("driver", monitored_driver()),
        ])
    }
    /// This controller answers symbol calls and compound operators
    /// without evaluating the operator expression first. It preserves
    /// the same machine stack depth while reducing pushes for factorial.
    pub fn ex_5_32() -> Result<Vec<String>, Fault> {
        let source =
            "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial 5)";
        let base = compose_controller(&[("driver", monitored_driver())]);
        let base_stats = measured(&base, source)?;
        let fast_stats = measured(&fast_controller(), source)?;
        let session_controller = fast_controller();
        let mut evaluator = ch05::sec_5_4::make_evaluator(
            &session_controller,
            &[symbol_operator()],
            "(define (f x) (* x x))\n(f 6)\n((lambda (y) (+ y 1)) 41)",
        )?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"36".to_owned()), "{transcript:?}");
        assert!(transcript.contains(&"42".to_owned()), "{transcript:?}");
        assert!(
            fast_stats.0 < base_stats.0,
            "{fast_stats:?} must beat {base_stats:?}"
        );
        assert_eq!(fast_stats.1, base_stats.1);
        Ok(vec![
            format!(
                "symbol and compound-operator answers: {}",
                transcript.join(" ")
            ),
            format!(
                "base monitored factorial pushes/depth: {}/{}",
                base_stats.0, base_stats.1
            ),
            format!(
                "fast-path factorial pushes/depth: {}/{}",
                fast_stats.0, fast_stats.1
            ),
            "design: symbol calls avoid a run-time operator-evaluation path".to_owned(),
        ])
    }

    #[test]
    fn ex_5_32_check() -> Result<(), Fault> {
        let answers = ex_5_32()?;
        assert!(answers[0].contains("42"));
        assert!(answers[1].contains("144"));
        Ok(())
    }
}
