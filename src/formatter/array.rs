pub fn convert_2d_array(arr: &Vec<u32>, width: usize, height: usize) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0; 3]; width * height];
    for i in 0..height {
        for j in 0..width {
            let index = i * width + j;
            let h = arr[index * 3];
            let s = arr[index * 3 + 1];
            let v = arr[index * 3 + 2];
            result[index] = vec![h, s, v];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_2d_array() {
        let arr = vec![100, 100, 100, 0, 0, 0];
        let result = convert_2d_array(&arr, 2, 1);
        assert_eq!(result, vec![vec![100, 100, 100], vec![0, 0, 0]]);
    }
}
