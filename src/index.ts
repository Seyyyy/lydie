import { UsageRate, Entropy } from './constants'
import { get_usage_rate_per_color } from './simplify'
import { get_entropy, get_hsb_rate } from './stats'

export class Image {
    hsv: number[]
    width: number
    height: number

    /**
     *
     * @param hsv 画像をhsv配列に変換したもの
     * @param width 画像の横幅の画素数
     * @param height 画像の縦幅の画素数
     */
    constructor(hsv: number[], width: number, height: number) {
        this.hsv = hsv
        this.width = width
        this.height = height
    }

    /**
     * @returns 色相、彩度、明度、グレースケールのそれぞれを正規化後、各段階ごとのピクセル数を取得する
     */
    public get_usage_quantity(): UsageRate {
        return get_usage_rate_per_color(this.hsv, this.width, this.height)
    }

    /**
     * @returns 色相、彩度、明度、グレースケールの平均情報量を取得する
     */
    public get_entropy(): Entropy {
        return get_entropy(get_usage_rate_per_color(this.hsv, this.width, this.height))
    }

    /**
     * @returns 色相、彩度、明度、グレースケールの使用率を取得する
     */
    public get_usage_rate(): UsageRate {
        return get_hsb_rate(get_usage_rate_per_color(this.hsv, this.width, this.height))
    }
}
