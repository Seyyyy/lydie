import {
    SAMPLING_CHROMATIC_LEVEL,
    SAMPLING_GRAY_LEVEL,
    GRAYSCALE_THRESHOLD,
    UsageRate,
} from './constants'

/**
 * @param v 色相, 彩度, 明度の値
 * @param beforeRange 色相, 彩度, 明度の変換前の最大値
 * @param afterRange 色相, 彩度, 明度の変換後の最大値
 * @returns 正規化した値
 * @description
 * 色相, 彩度, 明度の値を下記に正規化する
 * 色相: 0 ~ 11 (12段階)
 * 彩度: 0 ~ 11 (12段階)
 * 明度: 0 ~ 11 (12段階)
 * グレースケール: 0 ~ 2 (3段階)
 */
export const sampling = (v: number, beforeRange: number, afterRange: number): number => {
    let normalizeValue = v / (beforeRange / afterRange)
    if (normalizeValue > afterRange) {
        return Math.floor(normalizeValue)
    } else {
        return Math.round(normalizeValue)
    }
}

/**
 * @param v 色相, 彩度, 明度の値
 * @param thresholdSaturation 彩度のグレースケール判定の閾値
 * @param thresholdValue 明度のグレースケール判定の閾値
 * @returns true: グレースケール, false: グレースケールではない
 * @description
 * 彩度または明度が閾値以下の場合はグレースケールと判定する(現状は5以下)
 */
export const is_gray_scale = (v: number[]): boolean => {
    if (v[1] <= GRAYSCALE_THRESHOLD.SATURATION || v[2] <= GRAYSCALE_THRESHOLD.VALUE) {
        return true
    }
    return false
}

/**
 * @param v 色相, 彩度, 明度の値
 * @param width 画像の横幅の画素数
 * @param height 画像の縦幅の画素数
 * @returns 色相、彩度、明度のそれぞれを正規化した値のリスト
 */
export const get_usage_rate_per_color = (v: number[], width: number, height: number): UsageRate => {
    let hcArr: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)
    let hgArr: number[] = Array(SAMPLING_GRAY_LEVEL).fill(0)
    let sArr: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)
    let bArr: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)

    let size = width * height * 3

    for (let i = 0; i < size; i += 3) {
        if (is_gray_scale([v[i], v[i + 1], v[i + 2]])) {
            let hi = sampling(v[i + 2], 100, SAMPLING_GRAY_LEVEL - 1)
            hgArr[hi] += 1
        } else {
            let hi = sampling(v[i], 360, SAMPLING_CHROMATIC_LEVEL - 1)
            let si = sampling(v[i + 1] - 5, 95, SAMPLING_CHROMATIC_LEVEL - 1)
            let bi = sampling(v[i + 2] - 5, 95, SAMPLING_CHROMATIC_LEVEL - 1)
            hcArr[hi] += 1
            sArr[si] += 1
            bArr[bi] += 1
        }
    }

    return {
        hue_chromatic: hcArr,
        hue_gray: hgArr,
        saturation: sArr,
        brightness: bArr,
    }
}
