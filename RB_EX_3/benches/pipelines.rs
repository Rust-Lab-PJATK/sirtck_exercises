use std::num::NonZeroU32;

use criterion::{criterion_group, criterion_main, Criterion};
use rb_ex_3::{profile_strategies, InstantTimer, Status, TelemetrySample};

fn sample_data() -> Vec<TelemetrySample> {
    vec![
        TelemetrySample {
            endpoint: "/api/users".into(),
            latency_ms: 180,
            payload_bytes: 512,
            status: Status::Ok,
        },
        TelemetrySample {
            endpoint: "/api/users".into(),
            latency_ms: 340,
            payload_bytes: 128,
            status: Status::ServerError,
        },
        TelemetrySample {
            endpoint: "/checkout".into(),
            latency_ms: 420,
            payload_bytes: 2048,
            status: Status::Ok,
        },
        TelemetrySample {
            endpoint: "/checkout".into(),
            latency_ms: 95,
            payload_bytes: 1024,
            status: Status::ClientError,
        },
    ]
}

fn benchmark_pipelines(c: &mut Criterion) {
    let samples = sample_data();
    let iterations = NonZeroU32::new(25).expect("constant is non-zero");
    c.bench_function("telemetry_profile", |b| {
        b.iter(|| {
            let mut timer = InstantTimer::default();
            criterion::black_box(profile_strategies(
                &samples,
                iterations,
                &mut timer,
                250,
            ))
            .expect("profiling should succeed");
        });
    });
}

criterion_group!(benches, benchmark_pipelines);
criterion_main!(benches);
