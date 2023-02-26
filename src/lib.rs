#[cfg(test)]
mod test {
    use super::*;

    // Hueを0~12の範囲の離散値に正規化する
    #[test]
    fn case_normalize_hue() {
        assert_eq!(12, normalize_hue(360))
    }

    // Saturationを0~12の範囲の離散値に正規化する
    #[test]
    fn case_normalize_saturation() {
        assert_eq!(12, normalize_saturation(100))
    }

    // Brightnessを0~12の範囲の離散値に正規化する
    #[test]
    fn case_normalize_brightness() {
        assert_eq!(12, normalize_brightness(100))
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
        assert_eq!(2, normalize_gray_scale(5, 100))
    }
}
