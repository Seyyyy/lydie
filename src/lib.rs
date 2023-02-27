pub fn sampling(v: f64, before_range: f64, after_range: f64) -> i32 {
    let normalize_value = v / (before_range / after_range);
    if normalize_value > after_range {
        normalize_value.floor() as i32
    } else {
        normalize_value.round() as i32
    }
}

pub fn trim_gray_scale(v: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    (
        vec![vec![0, 5, 100], vec![0, 100, 5]],
        vec![vec![0, 6, 100], vec![0, 100, 6]],
    )
}

pub fn normalize_gray_scale(v: f64, w: f64) -> i32 {
    2
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
        assert_eq!(expect, trim_gray_scale(&original))
    }

    // グレースケールを0~2の範囲の離散値に正規化する
    // 0:黒, 1:グレー, 2:白
    #[test]
    fn case_normalize_gray_scale() {
        assert_eq!(2, normalize_gray_scale(5., 100.))
    }
}
