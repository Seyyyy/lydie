import { expect, it, describe } from 'vitest'
import { sampling, is_gray_scale, get_usage_rate_per_color } from './simplify'

describe('正規化', () => {
    it('Hue(0 ~ 360deg)を0~11(11段階)の範囲の離散値に正規化する', () => {
        const result = sampling(360, 360, 11)
        expect(result).toEqual(11)
    })

    it('Saturation(5 ~ 100%)を0~11(11段階)の範囲の離散値に正規化する(5%以下は無彩色として除外する予定)', () => {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        const offset = 5
        const result = sampling(100 - offset, 95, 11)
        expect(result).toEqual(11)
    })

    it('Brightness(5 ~ 100%)を0~11(11段階)の範囲の離散値に正規化する(5%以下は無彩色として除外する予定)', () => {
        // 「5 < 100」の値だと正規化が難しいので[0 < 95]にずらす
        const offset = 5
        const result = sampling(100 - offset, 95, 11)
        expect(result).toEqual(11)
    })

    it('グレースケールを0~2(3段階)の範囲の離散値に正規化する(0:黒, 1:グレー, 2:白)', () => {
        // 無彩色であるという前提があるなら、グレースケールはbrightnessのみで計測できる。
        const result = sampling(40, 100, 2)
        expect(result).toEqual(1)
    })
})

it.each([
    {
        params: [0, 5, 100],
        expected: true,
    },
    {
        params: [0, 100, 5],
        expected: true,
    },
    {
        params: [0, 6, 100],
        expected: false,
    },
    {
        params: [0, 100, 6],
        expected: false,
    },
])('グレースケールの判定', (args) => {
    const result = is_gray_scale(args.params)
    expect(result).toEqual(args.expected)
})

it('分析', () => {
    // hsv配列を受け取って分析結果を返却する
    // 各段階ごとのピクセル数を返却する
    // 割合の計算は親モジュールの責務
    const result = get_usage_rate_per_color(
        [0, 6, 100, 180, 55, 55, 360, 100, 6, 180, 0, 100],
        2,
        2
    )

    expect(result).toEqual({
        hue_chromatic: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        hue_gray: [0, 0, 1],
        saturation: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        brightness: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
    })
})
