// 0 ~ 11 (Level 12)
const SAMPLING_CHROMATIC_LEVEL: usize = 12;
// 0 ~ 2 (Level 3)
const SAMPLING_GRAY_LEVEL: usize = 3;

#[derive(Debug, PartialEq)]
pub struct UsageRate {
    pub hue_chromatic: Vec<u32>,
    pub hue_gray: Vec<u32>,
    pub saturation: Vec<u32>,
    pub brightness: Vec<u32>,
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

pub fn get_usage_rate_per_color(v: &Vec<u16>, width: u16, height: u16) -> UsageRate {
    let mut hc_arr: [u32; SAMPLING_CHROMATIC_LEVEL] = [0; SAMPLING_CHROMATIC_LEVEL];
    let mut hg_arr: [u32; SAMPLING_GRAY_LEVEL] = [0; SAMPLING_GRAY_LEVEL];
    let mut s_arr: [u32; SAMPLING_CHROMATIC_LEVEL] = [0; SAMPLING_CHROMATIC_LEVEL];
    let mut b_arr: [u32; SAMPLING_CHROMATIC_LEVEL] = [0; SAMPLING_CHROMATIC_LEVEL];

    let size = (width as u32) * (height as u32) * 3;

    for i in (0..size).step_by(3) {
        if is_gray_scale(
            [v[i as usize], v[(i + 1) as usize], v[(i + 2) as usize]],
            5,
            5,
        ) {
            let hi = sampling(
                v[(i + 2) as usize] as f64,
                100.,
                (SAMPLING_GRAY_LEVEL - 1) as f64,
            );
            hg_arr[hi as usize] += 1;
        } else {
            let hi = sampling(
                v[i as usize] as f64,
                360.,
                (SAMPLING_CHROMATIC_LEVEL - 1) as f64,
            );
            let si = sampling(
                (v[(i + 1) as usize] - 5) as f64,
                95.,
                (SAMPLING_CHROMATIC_LEVEL - 1) as f64,
            );
            let bi = sampling(
                (v[(i + 2) as usize] - 5) as f64,
                95.,
                (SAMPLING_CHROMATIC_LEVEL - 1) as f64,
            );
            hc_arr[hi as usize] += 1;
            s_arr[si as usize] += 1;
            b_arr[bi as usize] += 1;
        }
    }

    let result = UsageRate {
        hue_chromatic: hc_arr.to_vec(),
        hue_gray: hg_arr.to_vec(),
        saturation: s_arr.to_vec(),
        brightness: b_arr.to_vec(),
    };
    return result;
}

fn sampling(v: f64, before_range: f64, after_range: f64) -> u16 {
    let normalize_value = v / (before_range / after_range);
    if normalize_value > after_range {
        normalize_value.floor() as u16
    } else {
        normalize_value.round() as u16
    }
}

fn is_gray_scale(v: [u16; 3], threshold_saturation: u16, threshold_value: u16) -> bool {
    if v[1] <= threshold_saturation || v[2] <= threshold_value {
        return true;
    }
    false
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

    // 無彩色の判定
    #[test]
    fn case_is_gray_scale() {
        assert_eq!(true, is_gray_scale([0, 5, 100], 5, 5));
        assert_eq!(true, is_gray_scale([0, 100, 5], 5, 5));
        assert_eq!(false, is_gray_scale([0, 6, 100], 5, 5));
        assert_eq!(false, is_gray_scale([0, 100, 6], 5, 5));
    }

    // hsv配列を受け取って分析結果を返却する
    // 各段階ごとのピクセル数を返却する
    // 割合の計算は親モジュールの責務
    #[test]
    fn case_get_usage_rate_per_color() {
        let image_vec = vec![0, 6, 100, 180, 55, 55, 360, 100, 6, 180, 0, 100];
        let width = 2;
        let height = 2;
        let expect = UsageRate {
            hue_chromatic: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            hue_gray: vec![0, 0, 1],
            saturation: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            brightness: vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        };
        assert_eq!(expect, get_usage_rate_per_color(&image_vec, width, height))
    }
}
