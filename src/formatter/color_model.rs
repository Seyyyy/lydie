pub fn rgb2hsv(r: u32, g: u32, b: u32) -> Vec<u32> {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut h = 0.0;

    let mut s = 0.0;
    let d = max - min;
    if max != 0.0 {
        s = d / max;
    }

    let v = max;

    if max != min {
        if min == r {
            h = 60.0 * (b - g) / d + 180.0;
        } else if min == g {
            h = 60.0 * (r - b) / d + 300.0;
        } else if min == b {
            h = 60.0 * (g - r) / d + 60.0;
        }
    }

    vec![
        h.round() as u32,
        (s * 100.0).round() as u32,
        (v * 100.0).round() as u32,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hsv1() {
        let result = rgb2hsv(168, 50, 82);
        assert_eq!(result, vec![344, 70, 66]);
    }

    #[test]
    fn test_rgb2hsv2() {
        let result = rgb2hsv(0, 0, 0);
        assert_eq!(result, vec![0, 0, 0]);
    }

    #[test]
    fn test_rgb2hsv3() {
        let result = rgb2hsv(255, 255, 255);
        assert_eq!(result, vec![0, 0, 100]);
    }

    #[test]
    fn test_rgb2hsv4() {
        let result = rgb2hsv(250, 12, 1);
        assert_eq!(result, vec![3, 100, 98]);
    }
}
