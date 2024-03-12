import { SAMPLING_CHROMATIC_LEVEL, SAMPLING_GRAY_LEVEL, UsageRate, Entropy } from './constants'

/**
 * @param level 正規化した後の色相、彩度、明度、グレースケールの最大値
 * @returns 平均情報量の最大値
 * @description
 * 平均情報量の最大値
 * すべての事象が等確率で起こる場合の平均情報量
 * 12段階に色を分割した場合は1/12の確率
 */
const max_entropy = (level: number): number => {
    let maxEntropy = 0
    let eachPixelProbability = 1 / level
    for (let i = 0; i < level; i++) {
        maxEntropy -= eachPixelProbability * Math.log2(eachPixelProbability)
    }
    return maxEntropy
}

/**
 * @param usage_rate 色相, 彩度, 明度における各画素の使用数
 * @returns 情報量の計算結果(情報量の正規化はnormalize_entropyで行う)
 */
const calc_entropy = (usage_rate: UsageRate): Entropy => {
    let entropy: Entropy = {
        hue_chromatic: 0,
        hue_gray: 0,
        saturation: 0,
        brightness: 0,
    }
    let sum = 0

    // Calculate entropy for hue_chromatic
    sum = usage_rate.hue_chromatic.reduce((a, b) => a + b, 0)
    usage_rate.hue_chromatic.forEach((rate) => {
        if (rate !== 0) {
            const probability = rate / sum
            entropy.hue_chromatic -= probability * Math.log2(probability)
        }
    })

    // Calculate entropy for hue_gray
    sum = usage_rate.hue_gray.reduce((a, b) => a + b, 0)
    usage_rate.hue_gray.forEach((rate) => {
        if (rate !== 0) {
            const probability = rate / sum
            entropy.hue_gray -= probability * Math.log2(probability)
        }
    })

    // Calculate entropy for saturation
    sum = usage_rate.saturation.reduce((a, b) => a + b, 0)
    usage_rate.saturation.forEach((rate) => {
        if (rate !== 0) {
            const probability = rate / sum
            entropy.saturation -= probability * Math.log2(probability)
        }
    })

    // Calculate entropy for brightness
    sum = usage_rate.brightness.reduce((a, b) => a + b, 0)
    usage_rate.brightness.forEach((rate) => {
        if (rate !== 0) {
            const probability = rate / sum
            entropy.brightness -= probability * Math.log2(probability)
        }
    })

    return entropy
}

/**
 * @param entropy 色相, 彩度, 明度の情報量の計算結果
 * @param chromaticLevel 色相, 彩度, 明度の正規化後の最大値
 * @param grayLevel グレースケールの正規化後の最大値
 * @returns 平均情報量の計算結果
 */
const normalize_entropy = (
    entropy: Entropy,
    chromaticLevel: number,
    grayLevel: number
): Entropy => {
    let normalizedEntropy: Entropy = {
        hue_chromatic: 0,
        hue_gray: 0,
        saturation: 0,
        brightness: 0,
    }
    normalizedEntropy.hue_chromatic = entropy.hue_chromatic / max_entropy(chromaticLevel)
    normalizedEntropy.hue_gray = entropy.hue_gray / max_entropy(grayLevel)
    normalizedEntropy.saturation = entropy.saturation / max_entropy(chromaticLevel)
    normalizedEntropy.brightness = entropy.brightness / max_entropy(chromaticLevel)
    return normalizedEntropy
}

/**
 * @param usage_rate 色相, 彩度, 明度における各画素の使用数
 * @returns 平均情報量の計算結果
 */
export const get_entropy = (usage_rate: UsageRate): Entropy => {
    let entropy = calc_entropy(usage_rate)
    let normalizedEntropy = normalize_entropy(
        entropy,
        usage_rate.hue_chromatic.length,
        usage_rate.hue_gray.length
    )
    return normalizedEntropy
}

/**
 * @param usageRate 色相, 彩度, 明度における各画素の使用数
 * @returns 色相, 彩度, 明度の使用率
 */
export const get_hsb_rate = (usageRate: UsageRate): UsageRate => {
    // for hue_chromatic each rate
    let hue_chromatic: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)
    let sum = usageRate.hue_chromatic.reduce((a, b) => a + b, 0)
    usageRate.hue_chromatic.forEach((rate, i) => {
        let rateValue = rate / sum
        if (isNaN(rateValue)) {
            rateValue = 0
        }
        hue_chromatic[i] = rateValue
    })

    // for hue_gray each rate
    let hue_gray: number[] = Array(SAMPLING_GRAY_LEVEL).fill(0)
    sum = usageRate.hue_gray.reduce((a, b) => a + b, 0)
    usageRate.hue_gray.forEach((rate, i) => {
        let rateValue = rate / sum
        if (isNaN(rateValue)) {
            rateValue = 0
        }
        hue_gray[i] = rateValue
    })

    // for saturation each rate
    let saturation: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)
    sum = usageRate.saturation.reduce((a, b) => a + b, 0)
    usageRate.saturation.forEach((rate, i) => {
        let rateValue = rate / sum
        if (isNaN(rateValue)) {
            rateValue = 0
        }
        saturation[i] = rateValue
    })

    // for brightness each rate
    let brightness: number[] = Array(SAMPLING_CHROMATIC_LEVEL).fill(0)
    sum = usageRate.brightness.reduce((a, b) => a + b, 0)
    usageRate.brightness.forEach((rate, i) => {
        let rateValue = rate / sum
        if (isNaN(rateValue)) {
            rateValue = 0
        }
        brightness[i] = rateValue
    })

    return {
        hue_chromatic,
        hue_gray,
        saturation,
        brightness,
    }
}
