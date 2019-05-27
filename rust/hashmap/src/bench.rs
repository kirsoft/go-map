
extern crate test;

use test::Bencher;


#[bench]
fn write_bench(b: &mut Bencher) {
    let mut m = super::crete_hashmap();

    b.iter(|| {
        let _i = super::write(&mut m);
    });
}

#[bench]
fn read_bench(b: &mut Bencher) {
    let mut m = super::crete_hashmap();
    super::write(&mut m);

    b.iter(|| {
        let _r = super::read(&mut m);
    });
}

#[bench]
fn read_write_bench(b: &mut Bencher) {
    let mut m = super::crete_hashmap();

    b.iter(|| {
        super::write(&mut m);
        let _r = super::read(&mut m);
    });
}

#[bench]
fn fx_write_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();

    b.iter(|| {
        let _i = super::fx_write(&mut m);
    });
}

#[bench]
fn fx_read_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();
    super::fx_write(&mut m);

    b.iter(|| {
        let _r = super::fx_read(&mut m);
    });
}

#[bench]
fn fx_read_write_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();

    b.iter(|| {
        super::fx_write(&mut m);
        let _r = super::fx_read(&mut m);
    });
}

#[bench]
fn hashbrown_write_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();

    b.iter(|| {
        let _i = super::fx_write(&mut m);
    });
}

#[bench]
fn hashbrown_read_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();
    super::fx_write(&mut m);

    b.iter(|| {
        let _r = super::fx_read(&mut m);
    });
}

#[bench]
fn hashbrown_read_write_bench(b: &mut Bencher) {
    let mut m = super::fx_crete_hashmap();

    b.iter(|| {
        super::fx_write(&mut m);
        let _r = super::fx_read(&mut m);
    });
}