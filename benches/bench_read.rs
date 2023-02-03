use bencher::{benchmark_group, benchmark_main, Bencher};
use philips_isyntax_rs::{ImageType, PhilipsEngine, Rectangle, RegionRequest};
use std::path::Path;

fn simple_isyntax() -> &'static Path {
    Path::new("tests/data/sample.isyntax")
}

fn make_request(row: u32, col: u32, width: u32, height: u32, level: u32) -> RegionRequest {
    RegionRequest {
        roi: Rectangle {
            start_x: (col * width) * 2_u32.pow(level),
            end_x: ((col + 1) * width - 1) * 2_u32.pow(level),
            start_y: (row * height) * 2_u32.pow(level),
            end_y: ((row + 1) * height - 1) * 2_u32.pow(level),
        },
        level,
    }
}

fn philips_read_region_256_lvl_0(bench: &mut Bencher) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(simple_isyntax()).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    bench.iter(|| view.read_region(&engine, &make_request(10, 10, 256, 256, 0)));
}

fn philips_read_region_512_lvl_0(bench: &mut Bencher) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(simple_isyntax()).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    bench.iter(|| view.read_region(&engine, &make_request(10, 10, 512, 512, 0)));
}

fn philips_read_region_256_lvl_1(bench: &mut Bencher) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(simple_isyntax()).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    bench.iter(|| {
        view.read_region(&engine, &make_request(10, 10, 256, 256, 1))
            .unwrap()
    });
}

fn philips_read_region_512_lvl_1(bench: &mut Bencher) {
    let engine = PhilipsEngine::new().unwrap();
    let facade = engine.facade("facade_name").unwrap();
    facade.open(simple_isyntax()).unwrap();
    let image = facade.image(&ImageType::WSI).unwrap();
    let view = image.view().unwrap();

    bench.iter(|| view.read_region(&engine, &make_request(10, 10, 512, 512, 1)));
}

benchmark_group!(
    benches_region,
    philips_read_region_256_lvl_0,
    philips_read_region_512_lvl_0,
    philips_read_region_256_lvl_1,
    philips_read_region_512_lvl_1
);

benchmark_main!(benches_region);
