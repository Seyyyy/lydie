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
    hsv: Vec<u16>,
    width: u16,
    height: u16,
    usage_rate: core::simplify::UsageRate,
    pub hsv_pointer: *const u16,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::uninit_vec)]
    pub fn new(size: u32, width: u16, height: u16) -> Image {
        let mut hsv_arr = Vec::with_capacity(size as usize);
        unsafe {
            hsv_arr.set_len(size as usize);
        }
        let hsv_pointer = hsv_arr.as_ptr();

        Image {
            hsv: hsv_arr,
            width,
            height,
            usage_rate: core::simplify::UsageRate::new(),
            hsv_pointer,
        }
    }

    #[wasm_bindgen]
    pub fn calc_usage_rate(&mut self) {
        self.usage_rate =
            core::simplify::get_usage_rate_per_color(&self.hsv, self.width, self.height);
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
        let mut image = Image::new(6, 2, 1);
        image.hsv = vec![100, 100, 100, 0, 0, 0];
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_hue(),
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    fn test_get_gray_scale() {
        let mut image = Image::new(6, 2, 1);
        image.hsv = vec![100, 100, 100, 0, 0, 0];
        image.calc_usage_rate();
        assert_eq!(image.get_usage_rate_gray_scale(), vec![1, 0, 0])
    }

    #[test]
    fn test_get_saturation() {
        let mut image = Image::new(6, 2, 1);
        image.hsv = vec![100, 100, 100, 0, 0, 0];
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_saturation(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_get_brightness() {
        let mut image = Image::new(6, 2, 1);
        image.hsv = vec![100, 100, 100, 0, 0, 0];
        image.calc_usage_rate();
        assert_eq!(
            image.get_usage_rate_brightness(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        )
    }
}
