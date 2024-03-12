import { expect, it, describe } from 'vitest'
import { Image } from './index'

describe('画像の分析結果に対して各パラメータの取得ができる', () => {
    it('画像の各要素(色相、グレースケール、彩度、明度)ごとに使用数を取得する', () => {
        const image = new Image([100, 100, 100, 0, 0, 0], 2, 1)
        expect(image.get_usage_quantity()).toEqual({
            hue_chromatic: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            hue_gray: [1, 0, 0],
            saturation: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            brightness: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        })
    })

    it('画像の各要素(色相、グレースケール、彩度、明度)ごとに平均情報量を取得する', () => {
        const image = new Image([0, 6, 100, 180, 55, 55, 360, 100, 6, 180, 0, 100], 2, 2)
        expect(image.get_entropy()).toEqual({
            hue_chromatic: 0.4421141086977403,
            hue_gray: 0,
            saturation: 0.4421141086977403,
            brightness: 0.4421141086977403,
        })
    })

    it('画像の各要素(色相、グレースケール、彩度、明度)ごとに使用率を取得する', () => {
        const image = new Image([100, 100, 100, 0, 0, 0, 100, 50, 100], 2, 2)
        expect(image.get_usage_rate()).toEqual({
            hue_chromatic: [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            hue_gray: [1, 0, 0],
            saturation: [0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0.5],
            brightness: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        })
    })
})
