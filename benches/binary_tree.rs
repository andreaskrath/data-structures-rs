use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ds_rs::binary_tree::BinaryTree;

pub fn insert(c: &mut Criterion) {
    let mut tree = BinaryTree::new();

    c.bench_function("insert single element and clear", |b| {
        b.iter(|| {
            tree.insert(black_box(50));
            tree.clear();
        })
    });

    c.bench_function("create 3 layered balanced tree and clear", |b| {
        b.iter(|| {
            tree.insert(black_box(50));
            tree.insert(black_box(25));
            tree.insert(black_box(75));
            tree.insert(black_box(13));
            tree.insert(black_box(37));
            tree.insert(black_box(63));
            tree.insert(black_box(87));
            tree.clear();
        })
    });

    c.bench_function(
        "create 3 layered balanced tree, then duplicate all, and clear",
        |b| {
            b.iter(|| {
                tree.insert(black_box(50));
                tree.insert(black_box(25));
                tree.insert(black_box(75));
                tree.insert(black_box(13));
                tree.insert(black_box(37));
                tree.insert(black_box(63));
                tree.insert(black_box(87));
                tree.insert(black_box(50));
                tree.insert(black_box(25));
                tree.insert(black_box(75));
                tree.insert(black_box(13));
                tree.insert(black_box(37));
                tree.insert(black_box(63));
                tree.insert(black_box(87));
                tree.clear();
            })
        },
    );

    c.bench_function("create 10 element worst case tree and clear", |b| {
        b.iter(|| {
            tree.insert(black_box(0));
            tree.insert(black_box(1));
            tree.insert(black_box(2));
            tree.insert(black_box(3));
            tree.insert(black_box(4));
            tree.insert(black_box(5));
            tree.insert(black_box(6));
            tree.insert(black_box(7));
            tree.insert(black_box(8));
            tree.insert(black_box(9));
            tree.clear();
        })
    });
}

criterion_group!(benches, insert);
criterion_main!(benches);
