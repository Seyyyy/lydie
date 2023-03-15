use wasm_bindgen::prelude::*;
mod core;
mod formatter;

#[wasm_bindgen]
pub struct Image {
    hsv: Vec<u32>,
    width: u32,
    height: u32,
    usage_rate: core::simplify::UsageRate,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(hsv: Vec<u32>, width: u32, height: u32) -> Image {
        Image {
            hsv,
            width,
            height,
            usage_rate: core::simplify::UsageRate::new(),
        }
    }

    #[wasm_bindgen]
    pub fn calc_usage_rate(&mut self) {
        let arry = formatter::array::convert_2d_array(
            &self.hsv,
            self.width as usize,
            self.height as usize,
        );
        self.usage_rate = core::simplify::get_usage_rate_per_color(&arry);
    }

    #[wasm_bindgen]
    pub fn get_usage_rate_hue(&self) -> Vec<u32> {
        self.usage_rate.get_hue()
    }

    #[wasm_bindgen]
    pub fn get_usage_rate_gray_scale(&self) -> Vec<u32> {
        self.usage_rate.get_hue_gray()
    }

    #[wasm_bindgen]
    pub fn get_usage_rate_saturation(&self) -> Vec<u32> {
        self.usage_rate.get_saturation()
    }

    #[wasm_bindgen]
    pub fn get_usage_rate_brightness(&self) -> Vec<u32> {
        self.usage_rate.get_brightness()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hue() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_hue(),
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn test_get_gray_scale() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1);
        image.calc_usage_rate();
        assert_eq!(image.get_usage_rate_gray_scale(), vec![1, 0, 0])
    }

    #[test]
    fn test_get_saturation() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_saturation(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_get_brightness() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_brightness(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        )
    }
}
