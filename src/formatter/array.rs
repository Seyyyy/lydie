pub fn convert_2d_array(arr: &Vec<u16>, width: usize, height: usize) -> Vec<Vec<u16>> {
    let mut result = Vec::with_capacity(width * height);
    for i in 0..height {
        for j in 0..width {
            let index = i * width + j;
            let h = arr[index * 3];
            let s = arr[index * 3 + 1];
            let v = arr[index * 3 + 2];
            result.push(vec![h, s, v]);
        }
    }
    result
}

pub fn convert_flat_2d_array(arr: &Vec<Vec<u16>>) -> Vec<u16> {
    let mut result = Vec::with_capacity(arr.len() * 3);
    for i in 0..arr.len() {
        result.push(arr[i][0]);
        result.push(arr[i][1]);
        result.push(arr[i][2]);
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

    #[test]
    fn test_convert_flat_2d_array() {
        let arr = vec![vec![100, 100, 100], vec![0, 0, 0]];
        let result = convert_flat_2d_array(&arr);
        assert_eq!(result, vec![100, 100, 100, 0, 0, 0]);
    }
}
