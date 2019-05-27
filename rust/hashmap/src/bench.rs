
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