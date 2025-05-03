use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rust_calculator::{evaluate_expression, set_variable};

fn calculator_benchmark(c: &mut Criterion) {
  c.bench_function("simple addition", |b| {
    b.iter(|| evaluate_expression(black_box("2 + 3")))
  });

  c.bench_function("complex expression", |b| {
    b.iter(|| evaluate_expression(black_box("(2 + 3) * 4 ^ 2 / (1 + 1)")))
  });

  c.bench_function("trigonometric", |b| {
    b.iter(|| evaluate_expression(black_box("sin(30) + cos(60) + tan(45)")))
  });

  c.bench_function("nested functions", |b| {
    b.iter(|| evaluate_expression(black_box("sqrt(abs(sin(45) - cos(45)) + 1)")))
  });

  c.bench_function("factorial", |b| {
    b.iter(|| evaluate_expression(black_box("fact 10")))
  });

  c.bench_function("constants", |b| {
    b.iter(|| evaluate_expression(black_box("pi * 2 * 5 + e ^ 2")))
  });

  // Benchmark for memory functions
  c.bench_function("memory operations", |b| {
    b.iter(|| {
      evaluate_expression(black_box("5 m+")).unwrap();
      evaluate_expression(black_box("mr")).unwrap();
      evaluate_expression(black_box("3 m-")).unwrap();
      evaluate_expression(black_box("mc")).unwrap();
    })
  });

  // Benchmark for unit conversions
  c.bench_function("unit conversions", |b| {
    b.iter(|| {
      evaluate_expression(black_box("10 km_to_mi")).unwrap();
      evaluate_expression(black_box("100 c_to_f")).unwrap();
      evaluate_expression(black_box("50 kg_to_lb")).unwrap();
      evaluate_expression(black_box("pi rad_to_deg")).unwrap();
    })
  });

  // Benchmark for variable operations
  c.bench_function("variable operations", |b| {
    // Setup variables first
    set_variable("x", 10.0).unwrap();
    set_variable("y", 5.0).unwrap();

    b.iter(|| {
      evaluate_expression(black_box("x + y")).unwrap();
      evaluate_expression(black_box("x * y")).unwrap();
      evaluate_expression(black_box("(x + y) / 2")).unwrap();
      evaluate_expression(black_box("sqrt(x^2 + y^2)")).unwrap();
    })
  });
}

criterion_group!(benches, calculator_benchmark);
criterion_main!(benches);
