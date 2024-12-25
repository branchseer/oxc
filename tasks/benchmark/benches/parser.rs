use oxc_allocator::Allocator;
use oxc_benchmark::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oxc_parser::{Handler, ParseOptions, Parser};
use oxc_span::{ast_alloc::VoidAllocator, SourceType};
use oxc_tasks_common::TestFiles;
use std::hint::black_box;

fn bench_parser(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("parser");

    for file in TestFiles::complicated().files() {
        let source_type = SourceType::from_path(&file.file_name).unwrap();
        for allow_skip_ambient in [false, true] {
            for no_ast_alloc in [false, true] {
                let mut parameter = file.file_name.clone();
                if allow_skip_ambient || no_ast_alloc {
                    parameter.push_str(&format!(
                        "(allow_skip_ambient: {allow_skip_ambient}, no_ast_alloc: {no_ast_alloc})"
                    ));
                }
                group.bench_with_input(
                    BenchmarkId::from_parameter(&parameter),
                    &file.source_text,
                    |b, source_text| {
                        // Do not include initializing allocator in benchmark.
                        // User code would likely reuse the same allocator over and over to parse multiple files,
                        // so we do the same here.
                        let mut allocator = Allocator::default();
                        if no_ast_alloc {
                            b.iter(|| {
                                black_box(
                                    Parser::new(&allocator, source_text, source_type)
                                        .with_options(ParseOptions {
                                            parse_regular_expression: true,
                                            allow_skip_ambient,
                                            ..ParseOptions::default()
                                        })
                                        .parse_with(&VoidAllocator::new(), ()),
                                );
                                allocator.reset();
                            });
                        } else {
                            b.iter(|| {
                                black_box(
                                    Parser::new(&allocator, source_text, source_type)
                                        .with_options(ParseOptions {
                                            parse_regular_expression: true,
                                            allow_skip_ambient,
                                            ..ParseOptions::default()
                                        })
                                        .parse(),
                                );
                                allocator.reset();
                            });
                        }
                    },
                );
            }
        }
    }
    group.finish();
}

criterion_group!(parser, bench_parser);
criterion_main!(parser);
