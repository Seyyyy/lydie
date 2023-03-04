mod core;

// struct Analysys {
//     usage_rate: core::simplify::UsageRate,
// }

pub fn sample_fn() {
    let image_vec = vec![
        vec![0, 6, 100],
        vec![180, 55, 55],
        vec![360, 100, 6],
        vec![180, 0, 100],
    ];
    let test = core::simplify::get_usage_rate_per_color(&image_vec);
}
