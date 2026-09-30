use std::collections::HashMap;

#[derive(Clone)]
enum Expr {
    Integer(i64),
    Boolean(bool),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    If { test: Box<Expr>, yes: Box<Expr>, no: Box<Expr> },
    Lambda { parameters: Vec<String>, body: Box<Expr> },
    Call { function: Box<Expr>, arguments: Vec<Expr> },
    Let { name: String, value: Box<Expr>, body: Box<Expr> },
}

#[derive(Clone)]
enum Value {
    Integer(i64),
    Boolean(bool),
    Function(usize),
    Closure { parameters: Vec<String>, body: Box<Expr>, environment: usize },
}

struct FunctionDef { name: String, parameters: Vec<String>, body: Expr }
struct Program { functions: Vec<FunctionDef>, factorial: Expr, captured: Expr }
struct Frame { bindings: HashMap<String, Value>, parent: Option<usize> }
struct Store { frames: Vec<Frame> }
enum Fault { Unbound, Type, Arity, NotCallable }

fn boxed(expression: Expr) -> Box<Expr> { Box::new(expression) }
fn integer(value: i64) -> Expr { Expr::Integer(value) }
fn variable(name: &str) -> Expr { Expr::Variable(String::from(name)) }
fn one(expression: Expr) -> Vec<Expr> { vec![expression] }

fn factorial_body() -> Expr {
    Expr::If {
        test: boxed(Expr::Equal(boxed(variable("n")), boxed(integer(0)))),
        yes: boxed(integer(1)),
        no: boxed(Expr::Multiply(
            boxed(variable("n")),
            boxed(Expr::Call {
                function: boxed(variable("factorial")),
                arguments: one(Expr::Subtract(boxed(variable("n")), boxed(integer(1)))),
            }),
        )),
    }
}

fn make_program() -> Program {
    let factorial = Expr::Call {
        function: boxed(variable("factorial")),
        arguments: one(integer(5)),
    };
    let captured = Expr::Let {
        name: String::from("offset"),
        value: boxed(integer(3)),
        body: boxed(Expr::Call {
            function: boxed(Expr::Lambda {
                parameters: vec![String::from("x")],
                body: boxed(Expr::Add(boxed(variable("offset")), boxed(variable("x")))),
            }),
            arguments: one(integer(39)),
        }),
    };
    Program {
        functions: vec![FunctionDef {
            name: String::from("factorial"),
            parameters: vec![String::from("n")],
            body: factorial_body(),
        }],
        factorial,
        captured,
    }
}

fn new_frame(store: &mut Store, parent: Option<usize>) -> usize {
    let index = store.frames.len();
    store.frames.push(Frame { bindings: HashMap::new(), parent });
    index
}

fn lookup(store: &Store, mut frame: Option<usize>, name: &str) -> Option<Value> {
    while let Some(index) = frame {
        let current = &store.frames[index];
        if let Some(value) = current.bindings.get(name) {
            return Some(value.clone());
        }
        frame = current.parent;
    }
    None
}

fn evaluate(
    expression: &Expr,
    environment: usize,
    global: usize,
    program: &Program,
    store: &mut Store,
) -> Result<Value, Fault> {
    match expression {
        Expr::Integer(value) => Ok(Value::Integer(*value)),
        Expr::Boolean(value) => Ok(Value::Boolean(*value)),
        Expr::Variable(name) => match lookup(store, Some(environment), name) {
            Some(value) => Ok(value),
            None => Err(Fault::Unbound),
        },
        Expr::Add(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Subtract(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Multiply(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::Equal(left, right) => {
            let first = evaluate(left, environment, global, program, store)?;
            let second = evaluate(right, environment, global, program, store)?;
            match (first, second) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
                (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
                _ => Err(Fault::Type),
            }
        }
        Expr::If { test, yes, no } => match evaluate(test, environment, global, program, store)? {
            Value::Boolean(true) => evaluate(yes, environment, global, program, store),
            Value::Boolean(false) => evaluate(no, environment, global, program, store),
            _ => Err(Fault::Type),
        },
        Expr::Lambda { parameters, body } => Ok(Value::Closure {
            parameters: parameters.clone(),
            body: body.clone(),
            environment,
        }),
        Expr::Call { function, arguments } => {
            let callable = evaluate(function, environment, global, program, store)?;
            let mut values = Vec::new();
            for argument in arguments.iter() {
                values.push(evaluate(argument, environment, global, program, store)?);
            }
            apply(callable, values, global, program, store)
        }
        Expr::Let { name, value, body } => {
            let initial = evaluate(value, environment, global, program, store)?;
            let child = new_frame(store, Some(environment));
            store.frames[child].bindings.insert(name.clone(), initial);
            evaluate(body, child, global, program, store)
        }
    }
}

fn apply(
    callable: Value,
    arguments: Vec<Value>,
    global: usize,
    program: &Program,
    store: &mut Store,
) -> Result<Value, Fault> {
    match callable {
        Value::Function(index) => {
            let definition = match program.functions.get(index) {
                Some(definition) => definition,
                None => return Err(Fault::Unbound),
            };
            if definition.parameters.len() != arguments.len() { return Err(Fault::Arity); }
            let child = new_frame(store, Some(global));
            for (name, value) in definition.parameters.iter().zip(arguments.into_iter()) {
                store.frames[child].bindings.insert(name.clone(), value);
            }
            evaluate(&definition.body, child, global, program, store)
        }
        Value::Closure { parameters, body, environment } => {
            if parameters.len() != arguments.len() { return Err(Fault::Arity); }
            let child = new_frame(store, Some(environment));
            for (name, value) in parameters.into_iter().zip(arguments.into_iter()) {
                store.frames[child].bindings.insert(name, value);
            }
            evaluate(&body, child, global, program, store)
        }
        _ => Err(Fault::NotCallable),
    }
}

fn execute_program(program: &Program) -> Result<(Value, Value), Fault> {
    let mut store = Store { frames: Vec::new() };
    let global = new_frame(&mut store, None);
    for (index, definition) in program.functions.iter().enumerate() {
        store.frames[global].bindings.insert(definition.name.clone(), Value::Function(index));
    }
    let factorial = evaluate(&program.factorial, global, global, program, &mut store)?;
    let captured = evaluate(&program.captured, global, global, program, &mut store)?;
    Ok((factorial, captured))
}

fn make_adder(addend: i64) -> Box<dyn Fn(i64) -> i64 + 'static> {
    Box::new(move |value: i64| value + addend)
}

fn make_counter() -> Box<dyn FnMut() -> i64 + 'static> {
    let mut value = 0;
    Box::new(move || {
        value += 1;
        value
    })
}

fn main() {
    match execute_program(&make_program()) {
        Ok((Value::Integer(factorial), Value::Integer(captured))) => {
            println!("{}", factorial);
            println!("{}", captured);
        }
        _ => println!("evaluator error"),
    }
    let add = make_adder(2);
    println!("{}", add(40));
    let mut counter = make_counter();
    println!("{}", counter());
    println!("{}", counter());
}
