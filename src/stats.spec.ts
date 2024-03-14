import { expect, it, describe } from 'vitest'
import { get_entropy, get_hsb_rate } from './stats'

it('各要素(HSB, グレースケール)のエントロピーを取得する', () => {
    const result = get_entropy({
        hue_chromatic: [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        hue_gray: [2, 1, 5],
        saturation: [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        brightness: [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
    })
    expect(result).toEqual({
        hue_chromatic: 0.4421141086977403,
        hue_gray: 0.8194483718728035,
        saturation: 0,
        brightness: 1,
    })
})

describe('各要素(HSB, グレースケール)の割合を取得する', () => {
    it('ケース1', () => {
        const result = get_hsb_rate({
            hue_chromatic: [3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            hue_gray: [2, 2, 6],
            saturation: [0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            brightness: [0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1],
        })
        expect(result).toEqual({
            hue_chromatic: [0.75, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0],
            hue_gray: [0.2, 0.2, 0.6],
            saturation: [0, 0, 0.75, 0, 0, 0, 0.25, 0, 0, 0, 0, 0],
            brightness: [0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, 0, 0.25],
        })
    })

    it('ケース2', () => {
        const result = get_hsb_rate({
            hue_chromatic: [3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            hue_gray: [0, 0, 0],
            saturation: [0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            brightness: [0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1],
        })
        expect(result).toEqual({
            hue_chromatic: [0.75, 0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0],
            hue_gray: [0, 0, 0],
            saturation: [0, 0, 0.75, 0, 0, 0, 0.25, 0, 0, 0, 0, 0],
            brightness: [0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, 0, 0.25],
        })
    })
})
