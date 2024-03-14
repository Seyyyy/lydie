// 0 ~ 11 (Level 12)
export const SAMPLING_CHROMATIC_LEVEL = 12
// 0 ~ 2 (Level 3)
export const SAMPLING_GRAY_LEVEL = 3

export const GRAYSCALE_THRESHOLD = {
    SATURATION: 5,
    VALUE: 5,
}

export type UsageRate = {
    hue_chromatic: number[]
    hue_gray: number[]
    saturation: number[]
    brightness: number[]
}

export type Entropy = {
    hue_chromatic: number
    hue_gray: number
    saturation: number
    brightness: number
}
