use std::fmt;

// =============================================================================
// 基本的なResult型の実装（標準ライブラリに近い形）
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum MyResult<T, E> {
Ok(T),
Err(E),
}

impl<T, E> MyResult<T, E> {
// Monadの基本操作: unit (return)
pub fn ok(value: T) -> Self {
MyResult::Ok(value)
}

```
pub fn err(error: E) -> Self {
    MyResult::Err(error)
}

// Monadの基本操作: bind (flatmap)
pub fn and_then<U, F>(self, f: F) -> MyResult<U, E>
where
    F: FnOnce(T) -> MyResult<U, E>,
{
    match self {
        MyResult::Ok(value) => f(value),
        MyResult::Err(e) => MyResult::Err(e),
    }
}

// Functorの操作: map
pub fn map<U, F>(self, f: F) -> MyResult<U, E>
where
    F: FnOnce(T) -> U,
{
    match self {
        MyResult::Ok(value) => MyResult::Ok(f(value)),
        MyResult::Err(e) => MyResult::Err(e),
    }
}

// エラー型を変換するmap
pub fn map_err<F, O>(self, f: F) -> MyResult<T, O>
where
    F: FnOnce(E) -> O,
{
    match self {
        MyResult::Ok(value) => MyResult::Ok(value),
        MyResult::Err(e) => MyResult::Err(f(e)),
    }
}

// unwrap系の操作
pub fn unwrap(self) -> T
where
    E: fmt::Debug,
{
    match self {
        MyResult::Ok(value) => value,
        MyResult::Err(e) => panic!("called `MyResult::unwrap()` on an `Err` value: {:?}", e),
    }
}

pub fn unwrap_or(self, default: T) -> T {
    match self {
        MyResult::Ok(value) => value,
        MyResult::Err(_) => default,
    }
}

pub fn unwrap_or_else<F>(self, f: F) -> T
where
    F: FnOnce(E) -> T,
{
    match self {
        MyResult::Ok(value) => value,
        MyResult::Err(e) => f(e),
    }
}

// パターンマッチング用のヘルパー
pub fn is_ok(&self) -> bool {
    matches!(self, MyResult::Ok(_))
}

pub fn is_err(&self) -> bool {
    matches!(self, MyResult::Err(_))
}

// Optionへの変換
pub fn ok_value(self) -> Option<T> {
    match self {
        MyResult::Ok(value) => Some(value),
        MyResult::Err(_) => None,
    }
}

pub fn err_value(self) -> Option<E> {
    match self {
        MyResult::Ok(_) => None,
        MyResult::Err(e) => Some(e),
    }
}

// より高度な操作
pub fn or<F>(self, res: MyResult<T, F>) -> MyResult<T, F> {
    match self {
        MyResult::Ok(value) => MyResult::Ok(value),
        MyResult::Err(_) => res,
    }
}

pub fn or_else<F, O>(self, f: F) -> MyResult<T, O>
where
    F: FnOnce(E) -> MyResult<T, O>,
{
    match self {
        MyResult::Ok(value) => MyResult::Ok(value),
        MyResult::Err(e) => f(e),
    }
}

// 両方の値がOkの場合のみ成功
pub fn and<U>(self, res: MyResult<U, E>) -> MyResult<U, E> {
    match self {
        MyResult::Ok(_) => res,
        MyResult::Err(e) => MyResult::Err(e),
    }
}

// コールバック関数による条件分岐
pub fn match_result<R, F, G>(self, on_ok: F, on_err: G) -> R
where
    F: FnOnce(T) -> R,
    G: FnOnce(E) -> R,
{
    match self {
        MyResult::Ok(value) => on_ok(value),
        MyResult::Err(error) => on_err(error),
    }
}

// 副作用を伴うコールバック（デバッグやロギング用）
pub fn inspect<F>(self, f: F) -> Self
where
    F: FnOnce(&T),
{
    if let MyResult::Ok(ref value) = self {
        f(value);
    }
    self
}

pub fn inspect_err<F>(self, f: F) -> Self
where
    F: FnOnce(&E),
{
    if let MyResult::Err(ref error) = self {
        f(error);
    }
    self
}
```

}

// =============================================================================
// 関数型スタイルでのMonad則検証
// =============================================================================

pub fn verify_monad_laws() {
println!(”=== Monad Laws Verification ===”);

```
let test_left_identity = || {
    let value = 42;
    let f = |x: i32| MyResult::ok(x * 2);
    let left = MyResult::ok(value).and_then(f);
    let right = f(value);
    (left, right, left == right)
};

let test_right_identity = || {
    let m = MyResult::ok(42);
    let left = m.clone().and_then(MyResult::ok);
    let right = m;
    (left, right, left == right)
};

let test_associativity = || {
    let m = MyResult::ok(5);
    let f = |x: i32| MyResult::ok(x + 1);
    let g = |x: i32| MyResult::ok(x * 2);
    
    let left = m.clone().and_then(f).and_then(g);
    let right = m.and_then(|x| f(x).and_then(g));
    (left, right, left == right)
};

// コールバック関数を使用した結果処理
let process_law_result = |name: &str, test_fn: fn() -> (MyResult<i32, &str>, MyResult<i32, &str>, bool)| {
    let (left, right, is_equal) = test_fn();
    println!("{}: {:?} == {:?} -> {}", name, left, right, is_equal);
};

process_law_result("Left Identity", test_left_identity);
process_law_result("Right Identity", test_right_identity);
process_law_result("Associativity", test_associativity);
```

}

// =============================================================================
// カスタムエラー型の定義
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum CalculationError {
DivisionByZero,
NegativeSquareRoot,
Overflow,
InvalidInput(String),
}

impl fmt::Display for CalculationError {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
match self {
CalculationError::DivisionByZero => write!(f, “Division by zero”),
CalculationError::NegativeSquareRoot => write!(f, “Cannot calculate square root of negative number”),
CalculationError::Overflow => write!(f, “Calculation overflow”),
CalculationError::InvalidInput(msg) => write!(f, “Invalid input: {}”, msg),
}
}
}

impl std::error::Error for CalculationError {}

// =============================================================================
// 実用的なMonad操作の例（全て関数型スタイル）
// =============================================================================

type CalcResult<T> = MyResult<T, CalculationError>;

// 安全な除算
pub fn safe_divide(a: f64, b: f64) -> CalcResult<f64> {
let check_zero = |divisor: f64| {
if divisor == 0.0 {
CalcResult::err(CalculationError::DivisionByZero)
} else {
CalcResult::ok(divisor)
}
};

```
check_zero(b).map(|_| a / b)
```

}

// 安全な平方根
pub fn safe_sqrt(x: f64) -> CalcResult<f64> {
let validate_positive = |value: f64| {
if value < 0.0 {
CalcResult::err(CalculationError::NegativeSquareRoot)
} else {
CalcResult::ok(value)
}
};

```
validate_positive(x).map(|v| v.sqrt())
```

}

// 安全な対数
pub fn safe_log(x: f64) -> CalcResult<f64> {
let validate_log_domain = |value: f64| {
if value <= 0.0 {
CalcResult::err(CalculationError::InvalidInput(“Log argument must be positive”.to_string()))
} else {
CalcResult::ok(value)
}
};

```
validate_log_domain(x).map(|v| v.ln())
```

}

// 複雑な計算のチェーン（完全に関数型）
pub fn complex_calculation(a: f64, b: f64, c: f64) -> CalcResult<f64> {
safe_divide(a, b)
.and_then(safe_sqrt)
.map(|x| x + c)
.and_then(safe_log)
}

// 条件付き計算のチェーン
pub fn conditional_calculation<F1, F2>(
input: f64,
condition: F1,
on_true: F2,
on_false: F2,
) -> CalcResult<f64>
where
F1: FnOnce(f64) -> bool,
F2: FnOnce(f64) -> CalcResult<f64>,
{
CalcResult::ok(input).and_then(|x| {
if condition(x) {
on_true(x)
} else {
on_false(x)
}
})
}

// =============================================================================
// 高階関数とMonadの組み合わせ（完全にコールバック駆動）
// =============================================================================

// 複数のResultを処理するヘルパー関数（fold使用）
pub fn sequence_results<T, E>(results: Vec<MyResult<T, E>>) -> MyResult<Vec<T>, E> {
results.into_iter().fold(
MyResult::ok(Vec::new()),
|acc, result| {
acc.and_then(|mut vec| {
result.map(|value| {
vec.push(value);
vec
})
})
}
)
}

// reduce操作でのエラー処理
pub fn reduce_results<T, F, E>(
results: Vec<MyResult<T, E>>,
initial: T,
reducer: F,
) -> MyResult<T, E>
where
F: Fn(T, T) -> T,
T: Clone,
{
sequence_results(results)
.map(|values| {
values.into_iter().fold(initial, reducer)
})
}

// traverse操作（mapとsequenceの組み合わせ）
pub fn traverse<T, U, E, F>(items: Vec<T>, f: F) -> MyResult<Vec<U>, E>
where
F: Fn(T) -> MyResult<U, E>,
{
items.into_iter()
.map(f)
.fold(
MyResult::ok(Vec::new()),
|acc, result| {
acc.and_then(|mut vec| {
result.map(|value| {
vec.push(value);
vec
})
})
}
)
}

// filter_map的な操作
pub fn filter_map_results<T, U, E, F>(
items: Vec<T>,
predicate_mapper: F,
) -> MyResult<Vec<U>, E>
where
F: Fn(T) -> Option<MyResult<U, E>>,
{
items.into_iter()
.filter_map(predicate_mapper)
.fold(
MyResult::ok(Vec::new()),
|acc, result| {
acc.and_then(|mut vec| {
result.map(|value| {
vec.push(value);
vec
})
})
}
)
}

// =============================================================================
// パフォーマンステストとベンチマーク用コード（再帰的実装）
// =============================================================================

pub fn benchmark_monad_operations() {
use std::time::Instant;

```
println!("\n=== Performance Benchmark ===");

// 再帰的なand_then操作
let recursive_and_then = |n: u32| -> MyResult<u32, &'static str> {
    fn helper(current: MyResult<u32, &'static str>, remaining: u32) -> MyResult<u32, &'static str> {
        if remaining == 0 {
            current
        } else {
            helper(current.and_then(|x| MyResult::ok(x + 1)), remaining - 1)
        }
    }
    helper(MyResult::ok(0), n)
};

let start = Instant::now();
let result = recursive_and_then(100_000);
let duration = start.elapsed();
println!("100K recursive and_then operations: {:?}, result: {:?}", 
         duration, result.map(|x| x % 1000));

// fold操作でのmap
let fold_map = |n: u32| -> MyResult<u32, &'static str> {
    (0..n).fold(MyResult::ok(0), |acc, _| acc.map(|x| x + 1))
};

let start = Instant::now();
let result = fold_map(100_000);
let duration = start.elapsed();
println!("100K fold map operations: {:?}, result: {:?}", 
         duration, result.map(|x| x % 1000));
```

}

// =============================================================================
// 実用例のデモンストレーション（コールバック中心）
// =============================================================================

pub fn demonstrate_practical_usage() {
println!(”\n=== Practical Usage Examples ===”);

```
// 1. コールバック関数を使った結果処理
let process_calculation = |desc: &str, calc_fn: fn() -> CalcResult<f64>| {
    println!("{}:", desc);
    calc_fn().match_result(
        |value| println!("   Success: {:.6}", value),
        |error| println!("   Error: {}", error)
    );
};

process_calculation("1. Basic calculation chain", || {
    complex_calculation(100.0, 4.0, 1.0)
});

process_calculation("2. Error case (division by zero)", || {
    complex_calculation(100.0, 0.0, 1.0)
});

// 3. 複数の値を処理（map + コールバック）
println!("3. Processing multiple values:");
let inputs = vec![
    (16.0, 4.0, 0.1),
    (25.0, 5.0, 0.2),
    (36.0, 6.0, 0.3),
];

inputs.into_iter()
    .map(|(a, b, c)| complex_calculation(a, b, c))
    .enumerate()
    .for_each(|(i, result)| {
        result.match_result(
            |value| println!("   Calculation {}: {:.6}", i + 1, value),
            |error| println!("   Calculation {}: Error - {}", i + 1, error)
        );
    });

// 4. traverse操作の例
println!("4. Traverse operation:");
let numbers = vec![4.0, 9.0, 16.0, 25.0];
traverse(numbers, safe_sqrt).match_result(
    |values| println!("   Square roots: {:?}", values),
    |error| println!("   Error: {}", error)
);

// 5. エラー回復の例（完全にコールバック駆動）
println!("5. Error recovery:");
safe_divide(10.0, 0.0)
    .or_else(|_| {
        println!("   Division failed, using alternative calculation");
        MyResult::ok(42.0)
    })
    .and_then(safe_sqrt)
    .match_result(
        |value| println!("   Recovered result: {}", value),
        |error| println!("   Final error: {}", error)
    );

// 6. 条件付き計算の例
println!("6. Conditional calculation:");
let input_value = 10.0;
conditional_calculation(
    input_value,
    |x| x > 5.0,
    |x| safe_sqrt(x).map(|v| v * 2.0),
    |x| safe_log(x + 1.0)
).match_result(
    |value| println!("   Conditional result: {:.6}", value),
    |error| println!("   Conditional error: {}", error)
);
```

}

// =============================================================================
// 高度な関数型パターンの実装
// =============================================================================

// モナド変換子風の操作
pub fn lift_option<T, E>(option: Option<T>, error: E) -> MyResult<T, E> {
option.map_or_else(|| MyResult::err(error), MyResult::ok)
}

// コンビネータライブラリの実装
pub fn combine_results<T, U, R, F>(
result1: MyResult<T, CalculationError>,
result2: MyResult<U, CalculationError>,
combiner: F,
) -> MyResult<R, CalculationError>
where
F: FnOnce(T, U) -> R,
{
result1.and_then(|t| {
result2.map(|u| combiner(t, u))
})
}

// パイプライン処理の抽象化
pub fn pipeline<T>(initial: T) -> Pipeline<T, ()> {
Pipeline { value: MyResult::ok(initial) }
}

pub struct Pipeline<T, E> {
value: MyResult<T, E>,
}

impl<T, E> Pipeline<T, E> {
pub fn then<U, F>(self, f: F) -> Pipeline<U, E>
where
F: FnOnce(T) -> MyResult<U, E>,
{
Pipeline {
value: self.value.and_then(f),
}
}

```
pub fn map<U, F>(self, f: F) -> Pipeline<U, E>
where
    F: FnOnce(T) -> U,
{
    Pipeline {
        value: self.value.map(f),
    }
}

pub fn execute(self) -> MyResult<T, E> {
    self.value
}
```

}

// =============================================================================
// メイン関数
// =============================================================================

fn main() {
println!(“Rust Result Monad Implementation Demo (Callback-Driven)”);
println!(”======================================================”);

```
// Monad則の検証
verify_monad_laws();

// 実用例のデモ
demonstrate_practical_usage();

// パフォーマンステスト
benchmark_monad_operations();

println!("\n=== Advanced Monad Patterns ===");

// パイプライン処理の例
let pipeline_result = pipeline(10)
    .then(|x| if x > 5 { MyResult::ok(x * 2) } else { MyResult::err("Too small") })
    .map(|x| x + 1)
    .then(|x| if x < 100 { MyResult::ok(format!("Result: {}", x)) } else { MyResult::err("Too large") })
    .execute();

pipeline_result.match_result(
    |success| println!("Pipeline result: {}", success),
    |error| println!("Pipeline error: {}", error)
);

// コンビネータの例
let combined = combine_results(
    safe_sqrt(16.0),
    safe_log(2.71828),
    |sqrt_val, log_val| sqrt_val + log_val
);

combined.match_result(
    |result| println!("Combined calculation: {:.6}", result),
    |error| println!("Combined error: {}", error)
);

// エラーの連鎖処理
let error_chain = vec![
    || MyResult::err(CalculationError::DivisionByZero),
    || MyResult::err(CalculationError::NegativeSquareRoot),
    || MyResult::ok(42.0),
    || MyResult::err(CalculationError::Overflow),
];

let first_success = error_chain.into_iter()
    .map(|f| f())
    .fold(
        MyResult::err(CalculationError::InvalidInput("No valid result".to_string())),
        |acc, curr| acc.or(curr)
    );

first_success.match_result(
    |value| println!("First success from chain: {}", value),
    |error| println!("All failed: {}", error)
);
```

}
