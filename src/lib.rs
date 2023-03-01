#[derive(Debug, PartialEq)]
pub struct UsageRate {
    hue_chromatic: Vec<i32>,
    hue_gray_scale: Vec<i32>,
    saturation: Vec<i32>,
    brightness: Vec<i32>,
}

pub fn get_usage_rate_per_color(hsv_vec: &Vec<Vec<i32>>) -> UsageRate {
    // let arr = trim_gray_scale(&hsv_vec, 5, 5);

    let expect = UsageRate {
        hue_chromatic: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        hue_gray_scale: vec![0, 0, 1],
        saturation: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        brightness: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    };
    return expect;
}

fn sampling(v: f64, before_range: f64, after_range: f64) -> i32 {
    let normalize_value = v / (before_range / after_range);
    if normalize_value > after_range {
        normalize_value.floor() as i32
    } else {
        normalize_value.round() as i32
    }
}

fn trim_gray_scale(
    v: &Vec<Vec<i32>>,
    threshold_saturation: i32,
    threshold_brightness: i32,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut chromatic: Vec<Vec<i32>> = Vec::new();
    let mut gray: Vec<Vec<i32>> = Vec::new();

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

    // Hue(0 ~ 360deg)を0~12の範囲の離散値に正規化する
    #[test]
    fn case_normalize_hue() {
        assert_eq!(12, sampling(360., 360., 12.))
    }

    // Saturation(5 ~ 100%)を0~12の範囲の離散値に正規化する
    // 5%以下は無彩色として除外する予定
    #[test]
    fn case_normalize_saturation() {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        let offset = 5.;
        assert_eq!(12, sampling(100. - offset, 95., 12.))
    }

    // Brightness(5 ~ 100%)を0~12の範囲の離散値に正規化する
    // 5%以下は無彩色として除外する予定
    #[test]
    fn case_normalize_brightness() {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        let offset = 5.;
        assert_eq!(0, sampling(6. - offset, 95., 12.))
    }

    // グレースケールを0~2の範囲の離散値に正規化する
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
    #[test]
    fn case_usage_rate_per_color() {
        let image_vec = vec![
            vec![0, 10, 100],
            vec![180, 50, 50],
            vec![360, 100, 10],
            vec![180, 0, 100],
        ];
        let expect = UsageRate {
            hue_chromatic: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            hue_gray_scale: vec![0, 0, 1],
            saturation: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            brightness: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        };
        assert_eq!(expect, get_usage_rate_per_color(&image_vec))
    }
}
