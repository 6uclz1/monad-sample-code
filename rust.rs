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
pub fn ok(self) -> Option<T> {
    match self {
        MyResult::Ok(value) => Some(value),
        MyResult::Err(_) => None,
    }
}

pub fn err(self) -> Option<E> {
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
```

}

// =============================================================================
// Monad則の検証用ヘルパー関数
// =============================================================================

// Left Identity: unit(a).bind(f) == f(a)
// Right Identity: m.bind(unit) == m
// Associativity: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))

pub fn verify_monad_laws() {
println!(”=== Monad Laws Verification ===”);

```
// Left Identity
let value = 42;
let f = |x: i32| MyResult::ok(x * 2);
let left = MyResult::ok(value).and_then(f);
let right = f(value);
println!("Left Identity: {:?} == {:?} -> {}", left, right, left == right);

// Right Identity
let m = MyResult::ok(42);
let left = m.clone().and_then(MyResult::ok);
let right = m;
println!("Right Identity: {:?} == {:?} -> {}", left, right, left == right);

// Associativity
let m = MyResult::ok(5);
let f = |x: i32| MyResult::ok(x + 1);
let g = |x: i32| MyResult::ok(x * 2);

let left = m.clone().and_then(f).and_then(g);
let right = m.and_then(|x| f(x).and_then(g));
println!("Associativity: {:?} == {:?} -> {}", left, right, left == right);
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
// 実用的なMonad操作の例
// =============================================================================

type CalcResult<T> = MyResult<T, CalculationError>;

// 安全な除算
pub fn safe_divide(a: f64, b: f64) -> CalcResult<f64> {
if b == 0.0 {
CalcResult::err(CalculationError::DivisionByZero)
} else {
CalcResult::ok(a / b)
}
}

// 安全な平方根
pub fn safe_sqrt(x: f64) -> CalcResult<f64> {
if x < 0.0 {
CalcResult::err(CalculationError::NegativeSquareRoot)
} else {
CalcResult::ok(x.sqrt())
}
}

// 安全な対数
pub fn safe_log(x: f64) -> CalcResult<f64> {
if x <= 0.0 {
CalcResult::err(CalculationError::InvalidInput(“Log argument must be positive”.to_string()))
} else {
CalcResult::ok(x.ln())
}
}

// 複雑な計算のチェーン
pub fn complex_calculation(a: f64, b: f64, c: f64) -> CalcResult<f64> {
safe_divide(a, b)
.and_then(|result1| {
safe_sqrt(result1)
.and_then(|result2| {
safe_log(result2 + c)
})
})
}

// より関数型スタイルでの実装
pub fn complex_calculation_v2(a: f64, b: f64, c: f64) -> CalcResult<f64> {
safe_divide(a, b)
.and_then(safe_sqrt)
.map(|x| x + c)
.and_then(safe_log)
}

// =============================================================================
// 高階関数とMonadの組み合わせ
// =============================================================================

// 複数のResultを処理するヘルパー関数
pub fn sequence_results<T, E>(results: Vec<MyResult<T, E>>) -> MyResult<Vec<T>, E> {
results.into_iter().fold(MyResult::ok(Vec::new()), |acc, result| {
acc.and_then(|mut vec| {
result.map(|value| {
vec.push(value);
vec
})
})
})
}

// traverse操作（mapとsequenceの組み合わせ）
pub fn traverse<T, U, E, F>(items: Vec<T>, f: F) -> MyResult<Vec<U>, E>
where
F: Fn(T) -> MyResult<U, E>,
{
items.into_iter()
.map(f)
.collect::<Vec<_>>()
.into_iter()
.fold(MyResult::ok(Vec::new()), |acc, result| {
acc.and_then(|mut vec| {
result.map(|value| {
vec.push(value);
vec
})
})
})
}

// =============================================================================
// パフォーマンステストとベンチマーク用コード
// =============================================================================

pub fn benchmark_monad_operations() {
use std::time::Instant;

```
println!("\n=== Performance Benchmark ===");

// 大量のand_then操作
let start = Instant::now();
let mut result = MyResult::ok(0);
for i in 0..1_000_000 {
    result = result.and_then(|x| MyResult::ok(x + 1));
}
let duration = start.elapsed();
println!("1M and_then operations: {:?}, result: {:?}", duration, result.map(|x| x % 1000));

// 大量のmap操作
let start = Instant::now();
let mut result = MyResult::ok(0);
for i in 0..1_000_000 {
    result = result.map(|x| x + 1);
}
let duration = start.elapsed();
println!("1M map operations: {:?}, result: {:?}", duration, result.map(|x| x % 1000));
```

}

// =============================================================================
// 実用例のデモンストレーション
// =============================================================================

pub fn demonstrate_practical_usage() {
println!(”\n=== Practical Usage Examples ===”);

```
// 1. 基本的な計算チェーン
println!("1. Basic calculation chain:");
let result1 = complex_calculation(100.0, 4.0, 1.0);
match result1 {
    MyResult::Ok(value) => println!("   Success: {}", value),
    MyResult::Err(e) => println!("   Error: {}", e),
}

// 2. エラーが発生する例
println!("2. Error case (division by zero):");
let result2 = complex_calculation(100.0, 0.0, 1.0);
match result2 {
    MyResult::Ok(value) => println!("   Success: {}", value),
    MyResult::Err(e) => println!("   Error: {}", e),
}

// 3. 複数の値を処理
println!("3. Processing multiple values:");
let inputs = vec![
    (16.0, 4.0, 0.1),
    (25.0, 5.0, 0.2),
    (36.0, 6.0, 0.3),
];

let results: Vec<_> = inputs.into_iter()
    .map(|(a, b, c)| complex_calculation_v2(a, b, c))
    .collect();

for (i, result) in results.iter().enumerate() {
    match result {
        MyResult::Ok(value) => println!("   Calculation {}: {:.6}", i + 1, value),
        MyResult::Err(e) => println!("   Calculation {}: Error - {}", i + 1, e),
    }
}

// 4. traverse操作の例
println!("4. Traverse operation:");
let numbers = vec![4.0, 9.0, 16.0, 25.0];
let sqrt_results = traverse(numbers, safe_sqrt);
match sqrt_results {
    MyResult::Ok(values) => println!("   Square roots: {:?}", values),
    MyResult::Err(e) => println!("   Error: {}", e),
}

// 5. エラー回復の例
println!("5. Error recovery:");
let risky_calculation = safe_divide(10.0, 0.0)
    .or_else(|_| {
        println!("   Division failed, using alternative calculation");
        MyResult::ok(42.0)
    })
    .and_then(safe_sqrt);

match risky_calculation {
    MyResult::Ok(value) => println!("   Recovered result: {}", value),
    MyResult::Err(e) => println!("   Final error: {}", e),
}
```

}

// =============================================================================
// メイン関数
// =============================================================================

fn main() {
println!(“Rust Result Monad Implementation Demo”);
println!(”=====================================”);

```
// Monad則の検証
verify_monad_laws();

// 実用例のデモ
demonstrate_practical_usage();

// パフォーマンステスト
benchmark_monad_operations();

println!("\n=== Advanced Monad Patterns ===");

// より高度なパターンの例
let chained_operation = MyResult::ok(10)
    .and_then(|x| {
        if x > 5 { MyResult::ok(x * 2) } else { MyResult::err("Too small") }
    })
    .map(|x| x + 1)
    .and_then(|x| {
        if x < 100 { MyResult::ok(format!("Result: {}", x)) } else { MyResult::err("Too large") }
    });

println!("Chained operation result: {:?}", chained_operation);

// エラーの合成
let multiple_errors = vec![
    MyResult::err(CalculationError::DivisionByZero),
    MyResult::err(CalculationError::NegativeSquareRoot),
    MyResult::ok(42),
    MyResult::err(CalculationError::Overflow),
];

let first_success = multiple_errors.into_iter()
    .fold(MyResult::err(CalculationError::InvalidInput("No valid result".to_string())), |acc, curr| {
        acc.or(curr)
    });

println!("First success from multiple results: {:?}", first_success);
```

}
