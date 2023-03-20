use wasm_bindgen::prelude::*;
mod core;
mod formatter;

#[wasm_bindgen]
pub enum ColorModel {
    RGB,
    HSV,
}

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
    pub fn new(hsv: Vec<u32>, width: u32, height: u32, color_model: ColorModel) -> Image {
        match color_model {
            ColorModel::RGB => {
                let array2d =
                    formatter::array::convert_2d_array(&hsv, width as usize, height as usize);
                let hsv2darray: Vec<Vec<u32>> = array2d
                    .iter()
                    .map(|v| formatter::color_model::rgb2hsv(v[0], v[1], v[2]))
                    .collect();
                let hsv = formatter::array::convert_flat_2d_array(&hsv2darray);
                return Image {
                    hsv,
                    width,
                    height,
                    usage_rate: core::simplify::UsageRate::new(),
                };
            }
            ColorModel::HSV => {
                return Image {
                    hsv,
                    width,
                    height,
                    usage_rate: core::simplify::UsageRate::new(),
                };
            }
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
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1, ColorModel::HSV);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_hue(),
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn test_get_gray_scale() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1, ColorModel::HSV);
        image.calc_usage_rate();
        assert_eq!(image.get_usage_rate_gray_scale(), vec![1, 0, 0])
    }

    #[test]
    fn test_get_saturation() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1, ColorModel::HSV);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_saturation(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_get_brightness() {
        let mut image = Image::new(vec![100, 100, 100, 0, 0, 0], 2, 1, ColorModel::HSV);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_brightness(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        )
    }

    #[test]
    fn test_get_hue_rgb() {
        let mut image = Image::new(vec![85, 255, 0, 0, 0, 0], 2, 1, ColorModel::RGB);
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_hue(),
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }
}
