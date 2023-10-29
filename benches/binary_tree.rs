use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ds_rs::binary_tree::BinaryTree;
use rand::{thread_rng, Rng};

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

pub fn into_iter(c: &mut Criterion) {
    let mut tree = BinaryTree::new();

    for _ in 0..10000 {
        let rand_num = thread_rng().gen_range(i32::MIN..i32::MAX);
        tree.insert(rand_num);
    }

    c.bench_function("create iterator from 10.000 element tree", |b| {
        b.iter(|| {
            let iter = tree.clone().into_iter();
            iter.for_each(|_| {});
        })
    });
}

criterion_group!(benches, insert, into_iter);
criterion_main!(benches);
