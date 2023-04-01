// 0 ~ 11 (Level 12)
const SAMPLING_CHROMATIC_LEVEL: usize = 12;
// 0 ~ 2 (Level 3)
const SAMPLING_GRAY_LEVEL: usize = 3;

#[derive(Debug, PartialEq)]
pub struct UsageRate {
    hue_chromatic: Vec<u32>,
    hue_gray: Vec<u32>,
    saturation: Vec<u32>,
    brightness: Vec<u32>,
}

impl UsageRate {
    pub fn new() -> UsageRate {
        UsageRate {
            hue_chromatic: vec![0; SAMPLING_CHROMATIC_LEVEL],
            hue_gray: vec![0; SAMPLING_GRAY_LEVEL],
            saturation: vec![0; SAMPLING_CHROMATIC_LEVEL],
            brightness: vec![0; SAMPLING_CHROMATIC_LEVEL],
        }
    }

    pub fn get_hue(&self) -> Vec<u32> {
        self.hue_chromatic.clone()
    }

    pub fn get_hue_gray(&self) -> Vec<u32> {
        self.hue_gray.clone()
    }

    pub fn get_saturation(&self) -> Vec<u32> {
        self.saturation.clone()
    }

    pub fn get_brightness(&self) -> Vec<u32> {
        self.brightness.clone()
    }
}

pub fn get_usage_rate_per_color(hsv_vec: &Vec<Vec<u16>>) -> UsageRate {
    let arr = trim_gray_scale(&hsv_vec, 5, 5);

    let mut hc_arr = vec![0; SAMPLING_CHROMATIC_LEVEL];
    let mut hg_arr = vec![0; SAMPLING_GRAY_LEVEL];
    let mut s_arr = vec![0; SAMPLING_CHROMATIC_LEVEL];
    let mut b_arr = vec![0; SAMPLING_CHROMATIC_LEVEL];

    // analisys chromatic array
    for i in arr.1 {
        let hi = sampling(i[0] as f64, 360., (SAMPLING_CHROMATIC_LEVEL - 1) as f64);
        let si = sampling(
            (i[1] - 5) as f64,
            95.,
            (SAMPLING_CHROMATIC_LEVEL - 1) as f64,
        );
        let bi = sampling(
            (i[2] - 5) as f64,
            95.,
            (SAMPLING_CHROMATIC_LEVEL - 1) as f64,
        );
        hc_arr[hi as usize] += 1;
        s_arr[si as usize] += 1;
        b_arr[bi as usize] += 1;
    }

    // analisys gray scale array
    for i in arr.0 {
        let hi = sampling(i[2] as f64, 100., (SAMPLING_GRAY_LEVEL - 1) as f64);
        hg_arr[hi as usize] += 1;
    }

    let expect = UsageRate {
        hue_chromatic: hc_arr,
        hue_gray: hg_arr,
        saturation: s_arr,
        brightness: b_arr,
    };
    return expect;
}

fn sampling(v: f64, before_range: f64, after_range: f64) -> u16 {
    let normalize_value = v / (before_range / after_range);
    if normalize_value > after_range {
        normalize_value.floor() as u16
    } else {
        normalize_value.round() as u16
    }
}

fn trim_gray_scale(
    v: &Vec<Vec<u16>>,
    threshold_saturation: u16,
    threshold_brightness: u16,
) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let mut chromatic: Vec<Vec<u16>> = Vec::new();
    let mut gray: Vec<Vec<u16>> = Vec::new();

    for j in v {
        if j[1] <= threshold_saturation || j[2] <= threshold_brightness {
            gray.push(j.to_vec());
        } else {
            chromatic.push(j.to_vec());
        }
    }

    (gray, chromatic)
}

#[cfg(test)]
mod test {
    use super::*;

    // Hue(0 ~ 360deg)を0~11(11段階)の範囲の離散値に正規化する
    #[test]
    fn case_normalize_hue() {
        assert_eq!(11, sampling(360., 360., 11.))
    }

    // Saturation(5 ~ 100%)を0~11(11段階)の範囲の離散値に正規化する
    // 5%以下は無彩色として除外する予定
    #[test]
    fn case_normalize_saturation() {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        let offset = 5.;
        assert_eq!(11, sampling(100. - offset, 95., 11.))
    }

    // Brightness(5 ~ 100%)を0~11(11段階)の範囲の離散値に正規化する
    // 5%以下は無彩色として除外する予定
    #[test]
    fn case_normalize_brightness() {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        let offset = 5.;
        assert_eq!(0, sampling(6. - offset, 95., 11.))
    }

    // グレースケールを0~2(3段階)の範囲の離散値に正規化する
    // 0:黒, 1:グレー, 2:白
    #[test]
    fn case_normalize_gray_scale() {
        // 無彩色であるという前提があるなら、グレースケールはbrightnessのみで計測できる。
        assert_eq!(1, sampling(40., 100., 2.))
    }

    // 無彩色を削除した配列と無彩色のみの配列を生成
    #[test]
    fn case_trim_gray_scale() {
        let original = vec![
            vec![0, 5, 100],
            vec![0, 100, 5],
            vec![0, 6, 100],
            vec![0, 100, 6],
        ];
        let expect = (
            vec![vec![0, 5, 100], vec![0, 100, 5]],
            vec![vec![0, 6, 100], vec![0, 100, 6]],
        );
        // sが5%以下またはbが5%以下
        assert_eq!(expect, trim_gray_scale(&original, 5, 5))
    }

    // hsv配列を受け取って分析結果を返却する
    // 各段階ごとのピクセル数を返却する
    // 割合の計算は親モジュールの責務
    #[test]
    fn case_usage_rate_per_color() {
        let image_vec = vec![
            vec![0, 6, 100],
            vec![180, 55, 55],
            vec![360, 100, 6],
            vec![180, 0, 100],
        ];
        let expect = UsageRate {
            hue_chromatic: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            hue_gray: vec![0, 0, 1],
            saturation: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            brightness: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        };
        assert_eq!(expect, get_usage_rate_per_color(&image_vec))
    }
}
