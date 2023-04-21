use super::simplify::UsageRate;

#[derive(Debug, PartialEq)]
pub struct Entropy {
    pub hue_chromatic: f64,
    pub hue_gray: f64,
    pub saturation: f64,
    pub brightness: f64,
}

fn calc_entropy(usage_rate: &UsageRate) -> Entropy {
    let mut entropy = Entropy {
        hue_chromatic: 0.,
        hue_gray: 0.,
        saturation: 0.,
        brightness: 0.,
    };
    let mut sum = 0.;

    // 平均情報量 -ΣP(A)log2P(A)
    // -Σ(N(each level sum)/N(sum) * log2(N(each level sum)/N(sum)))
    for i in 0..usage_rate.hue_chromatic.len() {
        sum += usage_rate.hue_chromatic[i] as f64;
    }
    for i in 0..usage_rate.hue_chromatic.len() {
        if usage_rate.hue_chromatic[i] != 0 {
            entropy.hue_chromatic -= (usage_rate.hue_chromatic[i] as f64 / sum)
                * (usage_rate.hue_chromatic[i] as f64 / sum).log2();
        }
    }
    sum = 0.;
    for i in 0..usage_rate.hue_gray.len() {
        sum += usage_rate.hue_gray[i] as f64;
    }
    for i in 0..usage_rate.hue_gray.len() {
        if usage_rate.hue_gray[i] != 0 {
            entropy.hue_gray -= (usage_rate.hue_gray[i] as f64 / sum)
                * (usage_rate.hue_gray[i] as f64 / sum).log2();
        }
    }
    sum = 0.;
    for i in 0..usage_rate.saturation.len() {
        sum += usage_rate.saturation[i] as f64;
    }
    for i in 0..usage_rate.saturation.len() {
        if usage_rate.saturation[i] != 0 {
            entropy.saturation -= (usage_rate.saturation[i] as f64 / sum)
                * (usage_rate.saturation[i] as f64 / sum).log2();
        }
    }
    sum = 0.;
    for i in 0..usage_rate.brightness.len() {
        sum += usage_rate.brightness[i] as f64;
    }
    for i in 0..usage_rate.brightness.len() {
        if usage_rate.brightness[i] != 0 {
            entropy.brightness -= (usage_rate.brightness[i] as f64 / sum)
                * (usage_rate.brightness[i] as f64 / sum).log2();
        }
    }
    entropy
}

// 平均情報量の最大値
// すべての事象が等確率で起こる場合の平均情報量
// 12段階に色を分割した場合は1/12の確率
fn max_entropy(level: u32) -> f64 {
    let mut max_entropy = 0.;
    let each_pixel_probability = 1. / level as f64;
    for _i in 0..level {
        max_entropy -= (each_pixel_probability) * each_pixel_probability.log2();
    }
    max_entropy
}

fn normalize_entropy(entropy: &Entropy, chromatic_level: u32, gray_level: u32) -> Entropy {
    let mut normalized_entropy = Entropy {
        hue_chromatic: 0.,
        hue_gray: 0.,
        saturation: 0.,
        brightness: 0.,
    };
    normalized_entropy.hue_chromatic = entropy.hue_chromatic / max_entropy(chromatic_level);
    normalized_entropy.hue_gray = entropy.hue_gray / max_entropy(gray_level);
    normalized_entropy.saturation = entropy.saturation / max_entropy(chromatic_level);
    normalized_entropy.brightness = entropy.brightness / max_entropy(chromatic_level);
    normalized_entropy
}

pub fn get_entropy(usage_rate: &UsageRate) -> Vec<f64> {
    let entropy = calc_entropy(usage_rate);
    let normalize_entropy = normalize_entropy(
        &entropy,
        usage_rate.hue_chromatic.len() as u32,
        usage_rate.hue_gray.len() as u32,
    );
    vec![
        normalize_entropy.hue_chromatic,
        normalize_entropy.hue_gray,
        normalize_entropy.saturation,
        normalize_entropy.brightness,
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_get_entropy() {
        let usage_rate = UsageRate {
            hue_chromatic: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            hue_gray: vec![2, 1, 5],
            saturation: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            brightness: vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        };
        let entropy = get_entropy(&usage_rate);
        let expect = vec![0.4421141086977403, 0.8194483718728035, 0.0, 1.0];
        assert_eq!(expect, entropy)
    }

    #[test]
    fn case_max_entropy() {
        assert_eq!(3.584962500721156, max_entropy(12));
    }
}