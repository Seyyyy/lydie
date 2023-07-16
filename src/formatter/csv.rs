use crate::core::simplify::UsageRate;
use crate::core::stats::get_entropy;
use crate::core::stats::get_hsb_rate;

/// Convert UsageRate to CSV output.
///
/// # Arguments
///
/// * `usage_rate` - UsageRate
///
/// # Returns
///
/// * `csv` - CSV output(Array meaning are the following).
///     * hue_rate-red
///     * hue_rate-red_yellow
///     * hue_rate-yellow
///     * hue_rate-yellow_green
///     * hue_rate-green
///     * hue_rate-green_cyan
///     * hue_rate-cyan
///     * hue_rate-cyan_blue
///     * hue_rate-blue
///     * hue_rate-blue_purple
///     * hue_rate-purple
///     * hue_rate-purple_red
///     * hue_rate-black
///     * hue_rate-gray
///     * hue_rate-white
///     * saturation_rate-0
///     * saturation_rate-1
///     * saturation_rate-2
///     * saturation_rate-3
///     * saturation_rate-4
///     * saturation_rate-5
///     * saturation_rate-6
///     * saturation_rate-7
///     * saturation_rate-8
///     * saturation_rate-9
///     * saturation_rate-10
///     * saturation_rate-11
///     * brightness_rate-0
///     * brightness_rate-1
///     * brightness_rate-2
///     * brightness_rate-3
///     * brightness_rate-4
///     * brightness_rate-5
///     * brightness_rate-6
///     * brightness_rate-7
///     * brightness_rate-8
///     * brightness_rate-9
///     * brightness_rate-10
///     * brightness_rate-11
///     * entropy-hue_chromatic
///     * entropy-hue_gray
///     * entropy-saturation
///     * entropy-brightness
pub fn struct2csv(usage_rate: &UsageRate) -> [f64; 43] {
    let entropy = get_entropy(usage_rate);
    let hsb_rate = get_hsb_rate(usage_rate);
    [
        hsb_rate.hue_chromatic[0],
        hsb_rate.hue_chromatic[1],
        hsb_rate.hue_chromatic[2],
        hsb_rate.hue_chromatic[3],
        hsb_rate.hue_chromatic[4],
        hsb_rate.hue_chromatic[5],
        hsb_rate.hue_chromatic[6],
        hsb_rate.hue_chromatic[7],
        hsb_rate.hue_chromatic[8],
        hsb_rate.hue_chromatic[9],
        hsb_rate.hue_chromatic[10],
        hsb_rate.hue_chromatic[11],
        hsb_rate.hue_gray[0],
        hsb_rate.hue_gray[1],
        hsb_rate.hue_gray[2],
        hsb_rate.saturation[0],
        hsb_rate.saturation[1],
        hsb_rate.saturation[2],
        hsb_rate.saturation[3],
        hsb_rate.saturation[4],
        hsb_rate.saturation[5],
        hsb_rate.saturation[6],
        hsb_rate.saturation[7],
        hsb_rate.saturation[8],
        hsb_rate.saturation[9],
        hsb_rate.saturation[10],
        hsb_rate.saturation[11],
        hsb_rate.brightness[0],
        hsb_rate.brightness[1],
        hsb_rate.brightness[2],
        hsb_rate.brightness[3],
        hsb_rate.brightness[4],
        hsb_rate.brightness[5],
        hsb_rate.brightness[6],
        hsb_rate.brightness[7],
        hsb_rate.brightness[8],
        hsb_rate.brightness[9],
        hsb_rate.brightness[10],
        hsb_rate.brightness[11],
        entropy[0],
        entropy[1],
        entropy[2],
        entropy[3],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_struct2csv() {
        let usage_rate = UsageRate {
            hue_chromatic: vec![3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            hue_gray: vec![2, 2, 6],
            saturation: vec![0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            brightness: vec![0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1],
        };
        let csv = struct2csv(&usage_rate);
        let expect = [
            // hue
            0.75,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.25,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.2,
            0.2,
            0.6,
            // saturation
            0.0,
            0.0,
            0.75,
            0.0,
            0.0,
            0.0,
            0.25,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            // brightness
            0.0,
            0.0,
            0.0,
            0.75,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.25,
            // entropy
            0.22630030977895446,
            0.8649735207179273,
            0.22630030977895446,
            0.22630030977895446,
        ];
        assert_eq!(expect, csv);
    }
}
