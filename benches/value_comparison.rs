use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use datavalue_rs::{from_json, Bump, DataValue};
use serde_json::Value as JsonValue;

// Helper to create a complex nested JSON structure with serde_json
fn create_complex_json() -> JsonValue {
    serde_json::json!({
        "id": 123456,
        "name": "Complex Object",
        "active": true,
        "score": 98.6,
        "data": {
            "items": [
                { "id": 1, "value": "first" },
                { "id": 2, "value": "second" },
                { "id": 3, "value": "third" },
                { "id": 4, "value": "fourth" },
                { "id": 5, "value": "fifth" }
            ],
            "counts": [10, 20, 30, 40, 50],
            "metadata": {
                "created": "2023-01-01",
                "updated": "2023-06-15",
                "tags": ["important", "featured", "new"]
            }
        }
    })
}

// Helper to convert serde_json Value to DataValue
fn create_complex_data_value<'a>(arena: &'a Bump, json: &JsonValue) -> DataValue<'a> {
    from_json(arena, json).unwrap()
}

// Benchmark: accessing primitive values
fn bench_access_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("AccessPrimitives");

    // Setup test data
    let json_value = create_complex_json();
    let arena = Bump::new();
    let data_value = create_complex_data_value(&arena, &json_value);

    // Benchmark access for serde_json::Value
    group.bench_function(BenchmarkId::new("serde_json", "get_id"), |b| {
        b.iter(|| {
            black_box(json_value["id"].as_i64().unwrap());
        })
    });

    // Benchmark access for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "get_id"), |b| {
        b.iter(|| {
            black_box(data_value["id"].as_i64().unwrap());
        })
    });

    // Add more primitive access benchmarks
    group.bench_function(BenchmarkId::new("serde_json", "get_name"), |b| {
        b.iter(|| {
            black_box(json_value["name"].as_str().unwrap());
        })
    });

    group.bench_function(BenchmarkId::new("datavalue", "get_name"), |b| {
        b.iter(|| {
            black_box(data_value["name"].as_str().unwrap());
        })
    });

    group.bench_function(BenchmarkId::new("serde_json", "get_active"), |b| {
        b.iter(|| {
            black_box(json_value["active"].as_bool().unwrap());
        })
    });

    group.bench_function(BenchmarkId::new("datavalue", "get_active"), |b| {
        b.iter(|| {
            black_box(data_value["active"].as_bool().unwrap());
        })
    });

    group.finish();
}

// Benchmark: deep nested access
fn bench_deep_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("DeepAccess");

    // Setup test data
    let json_value = create_complex_json();
    let arena = Bump::new();
    let data_value = create_complex_data_value(&arena, &json_value);

    // Benchmark deep access for serde_json::Value
    group.bench_function(BenchmarkId::new("serde_json", "deep_access"), |b| {
        b.iter(|| {
            black_box(json_value["data"]["items"][2]["value"].as_str().unwrap());
        })
    });

    // Benchmark deep access for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "deep_access"), |b| {
        b.iter(|| {
            black_box(data_value["data"]["items"][2]["value"].as_str().unwrap());
        })
    });

    // Deep array access
    group.bench_function(BenchmarkId::new("serde_json", "array_access"), |b| {
        b.iter(|| {
            black_box(json_value["data"]["metadata"]["tags"][1].as_str().unwrap());
        })
    });

    group.bench_function(BenchmarkId::new("datavalue", "array_access"), |b| {
        b.iter(|| {
            black_box(data_value["data"]["metadata"]["tags"][1].as_str().unwrap());
        })
    });

    group.finish();
}

// Benchmark: iteration over arrays
fn bench_array_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArrayIteration");

    // Setup test data
    let json_value = create_complex_json();
    let arena = Bump::new();
    let data_value = create_complex_data_value(&arena, &json_value);

    // Benchmark array iteration for serde_json::Value
    group.bench_function(BenchmarkId::new("serde_json", "array_iteration"), |b| {
        b.iter(|| {
            let array = json_value["data"]["counts"].as_array().unwrap();
            let mut sum = 0;
            for item in array {
                sum += item.as_i64().unwrap();
            }
            black_box(sum);
        })
    });

    // Benchmark array iteration for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "array_iteration"), |b| {
        b.iter(|| {
            let array = data_value["data"]["counts"].as_array().unwrap();
            let mut sum = 0;
            for item in array {
                sum += item.as_i64().unwrap();
            }
            black_box(sum);
        })
    });

    group.finish();
}

// Benchmark: object iteration
fn bench_object_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("ObjectIteration");

    // Setup test data
    let json_value = create_complex_json();
    let arena = Bump::new();
    let data_value = create_complex_data_value(&arena, &json_value);

    // Benchmark object iteration for serde_json::Value
    group.bench_function(BenchmarkId::new("serde_json", "object_iteration"), |b| {
        b.iter(|| {
            let obj = json_value["data"]["metadata"].as_object().unwrap();
            let mut result = String::new();
            for (key, _) in obj {
                result.push_str(key);
            }
            black_box(result);
        })
    });

    // Benchmark object iteration for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "object_iteration"), |b| {
        b.iter(|| {
            let obj = data_value["data"]["metadata"].as_object().unwrap();
            let mut result = String::new();
            for (key, _) in obj {
                result.push_str(key);
            }
            black_box(result);
        })
    });

    group.finish();
}

// Benchmark: containment checks
fn bench_contains_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("ContainsKey");

    // Setup test data
    let json_value = create_complex_json();
    let arena = Bump::new();
    let data_value = create_complex_data_value(&arena, &json_value);

    // Benchmark containment check for serde_json::Value
    group.bench_function(BenchmarkId::new("serde_json", "contains_key"), |b| {
        b.iter(|| {
            black_box(
                json_value["data"]
                    .as_object()
                    .unwrap()
                    .contains_key("items"),
            );
        })
    });

    // Benchmark containment check for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "contains_key"), |b| {
        b.iter(|| {
            black_box(data_value["data"].contains_key("items"));
        })
    });

    group.finish();
}

// Benchmark: complex data processing - extracting and aggregating data
fn bench_complex_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("ComplexProcessing");

    // Setup test data - larger dataset
    let mut json_value = JsonValue::Object(serde_json::Map::new());

    // Create a complex nested structure
    let mut records = Vec::with_capacity(100);
    for i in 0..100 {
        records.push(serde_json::json!({
            "id": i,
            "type": if i % 3 == 0 { "A" } else if i % 3 == 1 { "B" } else { "C" },
            "values": [i, i*2, i*3, i*4, i*5],
            "metadata": {
                "region": if i % 4 == 0 { "North" } else if i % 4 == 1 { "South" } else if i % 4 == 2 { "East" } else { "West" },
                "active": i % 5 != 0,
                "rating": (i % 5) as f64 + 1.0
            }
        }));
    }

    json_value["records"] = JsonValue::Array(records);

    // Convert to DataValue
    let arena = Bump::new();
    let data_value = from_json(&arena, &json_value).unwrap();

    // Benchmark complex aggregation for serde_json::Value
    group.bench_function(
        BenchmarkId::new("serde_json", "filter_and_aggregate"),
        |b| {
            b.iter(|| {
                let records = json_value["records"].as_array().unwrap();

                // Filter type A records and calculate average rating
                let mut type_a_count = 0;
                let mut type_a_rating_sum = 0.0;

                for record in records {
                    if record["type"].as_str().unwrap() == "A" {
                        if record["metadata"]["active"].as_bool().unwrap() {
                            type_a_count += 1;
                            type_a_rating_sum += record["metadata"]["rating"].as_f64().unwrap();
                        }
                    }
                }

                black_box(if type_a_count > 0 {
                    type_a_rating_sum / type_a_count as f64
                } else {
                    0.0
                });
            })
        },
    );

    // Benchmark complex aggregation for DataValue
    group.bench_function(BenchmarkId::new("datavalue", "filter_and_aggregate"), |b| {
        b.iter(|| {
            let records = data_value["records"].as_array().unwrap();

            // Filter type A records and calculate average rating
            let mut type_a_count = 0;
            let mut type_a_rating_sum = 0.0;

            for record in records {
                if record["type"].as_str().unwrap() == "A" {
                    if record["metadata"]["active"].as_bool().unwrap() {
                        type_a_count += 1;
                        type_a_rating_sum += record["metadata"]["rating"].as_f64().unwrap();
                    }
                }
            }

            black_box(if type_a_count > 0 {
                type_a_rating_sum / type_a_count as f64
            } else {
                0.0
            });
        })
    });

    // Benchmark multiple key access pattern
    group.bench_function(BenchmarkId::new("serde_json", "multiple_key_access"), |b| {
        b.iter(|| {
            let records = json_value["records"].as_array().unwrap();
            let mut result = Vec::new();

            for record in records {
                if record["id"].as_i64().unwrap() % 10 == 0 {
                    result.push((
                        record["id"].as_i64().unwrap(),
                        record["metadata"]["region"].as_str().unwrap(),
                        record["values"].as_array().unwrap()[2].as_i64().unwrap(),
                    ));
                }
            }

            black_box(result);
        })
    });

    group.bench_function(BenchmarkId::new("datavalue", "multiple_key_access"), |b| {
        b.iter(|| {
            let records = data_value["records"].as_array().unwrap();
            let mut result = Vec::new();

            for record in records {
                if record["id"].as_i64().unwrap() % 10 == 0 {
                    result.push((
                        record["id"].as_i64().unwrap(),
                        record["metadata"]["region"].as_str().unwrap(),
                        record["values"].as_array().unwrap()[2].as_i64().unwrap(),
                    ));
                }
            }

            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_access_primitives,
    bench_deep_access,
    bench_array_iteration,
    bench_object_iteration,
    bench_contains_key,
    bench_complex_processing
);
criterion_main!(benches);
