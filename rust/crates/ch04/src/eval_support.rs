// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Shared helpers for the section 4.1 solution tests: the runners that
//! answer values plus displayed text, the print helper, and the pieces
//! several exercises share (the scan-out evaluators of 4.16 to 4.19 and
//! the counting analyzers of 4.23 and 4.23a). Re-exports the substrate
//! surface and the few std handles the exercises name so each solution
//! file stays one import away from everything it needs.

pub use crate::sec_4_1;
pub use crate::sec_4_1::*;
pub use sicp_runtime::{
    Closure, Env, Key, OpTable, SchemeError, Symbol, Value, cons_cell, print_value, read,
    read_program,
};
pub use std::cell::Cell;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;

/// The analyzed-body table an analyzer carries: the book stores the
/// execution procedure inside the procedure object, and the shared
/// `Closure` has no slot, so the table is keyed by procedure identity.
pub type Bodies = Rc<RefCell<HashMap<usize, Exec>>>;

/// Evaluates every form of `program` with `ev` in a fresh global
/// environment whose `display` writes into a captured buffer, answering
/// the values and the displayed text.
///
/// # Errors
/// The reader's parse errors and the first evaluation error.
pub fn run_with(ev: &impl Evaluator, program: &str) -> Result<(Vec<Value>, String), SchemeError> {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_environment_in(&sink);
    let forms = read_program(program)?;
    let mut values = Vec::with_capacity(forms.len());
    for form in &forms {
        values.push(ev.eval(form, &env)?);
    }
    Ok((values, cell.borrow().clone()))
}

/// Evaluates a program with an analyzed evaluator: every form is
/// analyzed and executed in order.
///
/// # Errors
/// The reader's parse errors and the first evaluation error.
pub fn run_analyzed(
    ev: &impl Analyzer,
    program: &str,
) -> Result<(Vec<Value>, String), SchemeError> {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_environment_in(&sink);
    let forms = read_program(program)?;
    let mut values = Vec::with_capacity(forms.len());
    for form in &forms {
        values.push(ev.eval_exp(form, &env)?);
    }
    Ok((values, cell.borrow().clone()))
}

/// The printed form of each value, the answer the book shows.
#[must_use]
pub fn printed(values: &[Value]) -> Vec<String> {
    values.iter().map(print_value).collect()
}

/// Reads and evaluates a program with the base evaluator, answering the
/// printed values. The program must evaluate without raising.
///
/// # Panics
/// Panics via [`Base::eval`] when the program raises, which only a
/// solution bug causes: every `run_base` program is expected to run.
#[must_use]
pub fn run_base(program: &str) -> Vec<String> {
    let env = setup_environment();
    let values = eval_program(&env, program).expect("the base evaluator runs the program");
    printed(&values)
}

/// The scan result: the defined names, their value expressions, and the
/// rest of the body.
pub type Scan = (Vec<Value>, Vec<Value>, Vec<Value>);

/// The book's `scan-out-defines`: the names and values of a body's
/// internal defines, then the rest of the body; an empty scan means
/// nothing to transform. Defines nested inside inner lambdas stay where
/// they are.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] on a malformed define.
pub fn scan_out_defines(body: &[Value]) -> Result<Option<Scan>, SchemeError> {
    let mut names = Vec::new();
    let mut inits = Vec::new();
    let mut rest = Vec::new();
    for form in body {
        if !is_tagged_list(form, "define") {
            rest.push(form.clone());
            continue;
        }
        let items = form.list_items()?;
        let target = items.get(1).cloned().unwrap_or(Value::Nil);
        match target {
            Value::Sym(name) => {
                names.push(Value::Sym(name));
                inits.push(items.get(2).cloned().unwrap_or(Value::Nil));
            }
            Value::Pair(_) => {
                let name = sec_4_1::definition_variable(form)?;
                let (params, param_rest) = sec_4_1::split_params(&sec_4_1::rest_of(&target)?)?;
                let procedure_body: Vec<Value> = items.into_iter().skip(2).collect();
                let lambda = make_lambda(&params, param_rest.as_ref(), &procedure_body);
                names.push(name);
                inits.push(lambda);
            }
            other => {
                return Err(SchemeError::TypeMismatch(format!(
                    "not a define target: {other}"
                )));
            }
        }
    }
    if names.is_empty() {
        return Ok(None);
    }
    Ok(Some((names, inits, rest)))
}

/// The binding `(name (quote *unassigned*))` of the scanned `let`.
fn unassigned_binding(name: &Value) -> Value {
    Value::list(vec![
        name.clone(),
        Value::list(vec![Value::sym("quote"), unassigned()]),
    ])
}

/// The scanned body as one form: the names bound to the unassigned
/// marker by a `let`, then one `set!` per name, then the rest.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] on a malformed body.
pub fn scanned_body(body: &[Value]) -> Result<Value, SchemeError> {
    let Some((names, inits, rest)) = scan_out_defines(body)? else {
        return match body.len() {
            1 => Ok(body[0].clone()),
            _ => Ok(sec_4_1::make_begin(body)),
        };
    };
    let bindings = Value::list(names.iter().map(unassigned_binding).collect());
    let mut forms = Vec::new();
    for (name, init) in names.iter().zip(&inits) {
        forms.push(Value::list(vec![
            Value::sym("set!"),
            name.clone(),
            init.clone(),
        ]));
    }
    forms.extend(rest);
    let inner = if forms.len() == 1 {
        forms.remove(0)
    } else {
        sec_4_1::make_begin(&forms)
    };
    Ok(Value::list(vec![Value::sym("let"), bindings, inner]))
}

/// The error a read of the unassigned marker raises.
fn unassigned_error(name: &str) -> SchemeError {
    SchemeError::TypeMismatch(format!(
        "the variable {name} is read before its define runs"
    ))
}

fn checked_lookup(exp: &Value, env: &Rc<Env>) -> EvalResult {
    let Value::Sym(name) = exp else {
        return Err(SchemeError::TypeMismatch("not a variable".to_owned()));
    };
    let value = lookup_variable_value(name, env)?;
    if value == unassigned() {
        return Err(unassigned_error(name));
    }
    Ok(value)
}

fn scanned_closure(
    scan: fn(&[Value]) -> Result<Option<Scan>, SchemeError>,
    name: Option<Symbol>,
    params: Vec<Symbol>,
    rest: Option<Symbol>,
    body: &[Value],
    env: &Rc<Env>,
) -> EvalResult {
    let scanned = match scan(body)? {
        Some(_) => scanned_body(body)?,
        None => {
            return Ok(Value::Closure(Rc::new(Closure {
                name,
                params,
                rest,
                body: body.to_vec(),
                env: Rc::clone(env),
            })));
        }
    };
    Ok(Value::Closure(Rc::new(Closure {
        name,
        params,
        rest,
        body: vec![scanned],
        env: Rc::clone(env),
    })))
}

/// The evaluator of exercise 4.16 that scans out defines when it builds
/// a procedure and raises on a read of an unassigned name.
pub struct WithScanOut;

impl Evaluator for WithScanOut {
    fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
        if is_variable(exp) {
            return Ok(Step::Done(checked_lookup(exp, env)?));
        }
        if is_lambda(exp) {
            let (params, rest) = sec_4_1::lambda_parameters(exp)?;
            let body = sec_4_1::lambda_body(exp)?;
            return Ok(Step::Done(scanned_closure(
                scan_out_defines,
                None,
                params,
                rest,
                &body,
                env,
            )?));
        }
        self.base_step(exp, env)
    }

    fn make_named_procedure(
        &self,
        name: &str,
        params: &[Symbol],
        rest: Option<&Symbol>,
        body: &[Value],
        env: &Rc<Env>,
    ) -> EvalResult {
        // Installed where the procedure is made: the scan runs once per
        // procedure, not once per call.
        scanned_closure(
            scan_out_defines,
            Some(Rc::from(name)),
            params.to_vec(),
            rest.cloned(),
            body,
            env,
        )
    }
}

/// The alternative strategy of exercise 4.18: the initializers evaluate
/// in an inner `let` while every defined name is still unassigned.
pub struct WithScanOutAlt;

impl Evaluator for WithScanOutAlt {
    fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
        if is_variable(exp) {
            return Ok(Step::Done(checked_lookup(exp, env)?));
        }
        if is_lambda(exp) {
            return Ok(Step::Done(scanned_alt_lambda(exp, env)?));
        }
        self.base_step(exp, env)
    }

    fn make_named_procedure(
        &self,
        name: &str,
        params: &[Symbol],
        rest: Option<&Symbol>,
        body: &[Value],
        env: &Rc<Env>,
    ) -> EvalResult {
        let params = params.to_vec();
        let rest = rest.cloned();
        scanned_alt_named(name, &params, rest.as_ref(), body, env)
    }
}

fn scanned_alt_lambda(exp: &Value, env: &Rc<Env>) -> EvalResult {
    let (params, rest) = sec_4_1::lambda_parameters(exp)?;
    let body = sec_4_1::lambda_body(exp)?;
    scanned_alt_closure(None, params, rest, &body, env)
}

fn scanned_alt_named(
    name: &str,
    params: &[Symbol],
    rest: Option<&Symbol>,
    body: &[Value],
    env: &Rc<Env>,
) -> EvalResult {
    scanned_alt_closure(
        Some(Rc::from(name)),
        params.to_vec(),
        rest.cloned(),
        body,
        env,
    )
}

fn scanned_alt_closure(
    name: Option<Symbol>,
    params: Vec<Symbol>,
    rest: Option<Symbol>,
    body: &[Value],
    env: &Rc<Env>,
) -> EvalResult {
    let Some((names, inits, rest_forms)) = scan_out_defines(body)? else {
        return Ok(Value::Closure(Rc::new(Closure {
            name,
            params,
            rest,
            body: body.to_vec(),
            env: Rc::clone(env),
        })));
    };
    let outer_bindings = Value::list(names.iter().map(unassigned_binding).collect());
    // Fresh names a, b, ... hold the initializer values while every
    // defined name is still unassigned.
    let fresh: Vec<Value> = (0..inits.len())
        .map(|index| Value::sym(&format!("init-{index}")))
        .collect();
    let inner_bindings = Value::list(
        fresh
            .iter()
            .zip(&inits)
            .map(|(fresh_name, init)| Value::list(vec![fresh_name.clone(), init.clone()]))
            .collect(),
    );
    let mut sets = Vec::new();
    for (name, fresh_name) in names.iter().zip(&fresh) {
        sets.push(Value::list(vec![
            Value::sym("set!"),
            name.clone(),
            fresh_name.clone(),
        ]));
    }
    let mut tail = sets;
    tail.extend(rest_forms);
    let tail_form = if tail.len() == 1 {
        tail.remove(0)
    } else {
        sec_4_1::make_begin(&tail)
    };
    let scanned = Value::list(vec![
        Value::sym("let"),
        outer_bindings,
        Value::list(vec![
            Value::sym("let"),
            inner_bindings,
            Value::list(vec![Value::sym("begin"), tail_form]),
        ]),
    ]);
    Ok(Value::Closure(Rc::new(Closure {
        name,
        params,
        rest,
        body: vec![scanned],
        env: Rc::clone(env),
    })))
}

/// Alyssa's `analyze_sequence` of exercise 4.23: the expressions are
/// analyzed but the sequence itself is not; a wrapper loops through the
/// procedures at execution time.
pub struct AlyssaAnalyzer {
    /// The analyzed bodies of the procedures this analyzer builds.
    pub bodies: Bodies,
}

impl Default for AlyssaAnalyzer {
    fn default() -> Self {
        Self {
            bodies: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Analyzer for AlyssaAnalyzer {
    fn analyze(&self, exp: &Value) -> Result<Exec, SchemeError> {
        self.base_analyze(exp)
    }

    fn bodies(&self) -> &Bodies {
        &self.bodies
    }

    fn analyze_sequence(&self, exps: &[Value]) -> Result<Exec, SchemeError> {
        let procs: Vec<Exec> = exps
            .iter()
            .map(|exp| self.analyze(exp))
            .collect::<Result<_, _>>()?;
        Ok(Rc::new(move |env| {
            let Some((last, head)) = procs.split_last() else {
                return Err(SchemeError::TypeMismatch(
                    "Empty sequence: ANALYZE".to_owned(),
                ));
            };
            for proc in head {
                proc(env)?;
            }
            last(env)
        }))
    }
}

/// The counting analyzers of exercise 4.23a: analysis invocations, and
/// the runs of the sequence execution procedure each style produces.
pub struct Counting {
    /// The analyzed bodies of the procedures this analyzer builds.
    pub bodies: Bodies,
    /// The analysis invocations counted so far.
    pub analyze_calls: Cell<u64>,
    /// The sequence-execution runs counted so far.
    pub sequence_execs: Rc<Cell<u64>>,
    /// Whether the sequence style is Alyssa's wrapper.
    pub alyssa_style: bool,
}

impl Counting {
    /// A counting analyzer in the text's style, or in Alyssa's.
    #[must_use]
    pub fn new(alyssa_style: bool) -> Self {
        Self {
            bodies: Rc::new(RefCell::new(HashMap::new())),
            analyze_calls: Cell::new(0),
            sequence_execs: Rc::new(Cell::new(0)),
            alyssa_style,
        }
    }

    /// The analysis invocation count.
    #[must_use]
    pub fn analyze_calls(&self) -> u64 {
        self.analyze_calls.get()
    }

    /// The sequence execution procedure invocation count.
    #[must_use]
    pub fn sequence_execs(&self) -> u64 {
        self.sequence_execs.get()
    }
}

impl Analyzer for Counting {
    fn analyze(&self, exp: &Value) -> Result<Exec, SchemeError> {
        self.analyze_calls.set(self.analyze_calls.get() + 1);
        self.base_analyze(exp)
    }

    fn bodies(&self) -> &Bodies {
        &self.bodies
    }

    fn analyze_sequence(&self, exps: &[Value]) -> Result<Exec, SchemeError> {
        let procs: Vec<Exec> = exps
            .iter()
            .map(|exp| self.analyze(exp))
            .collect::<Result<_, _>>()?;
        if !self.alyssa_style && procs.len() == 1 {
            // The text's version: no sequence execution procedure exists
            // for a one-expression body, so none is counted.
            return Ok(Rc::clone(&procs[0]));
        }
        let counter = Rc::clone(&self.sequence_execs);
        Ok(Rc::new(move |env| {
            counter.set(counter.get() + 1);
            let Some((last, head)) = procs.split_last() else {
                return Err(SchemeError::TypeMismatch(
                    "Empty sequence: ANALYZE".to_owned(),
                ));
            };
            for proc in head {
                proc(env)?;
            }
            last(env)
        }))
    }
}
