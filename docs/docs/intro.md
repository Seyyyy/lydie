---
sidebar_position: 1
---

# Introduction

## Backgroud and Motivation

I think, Aesthetic image will uplift the emotion.
But if it is only human subjectivity, it is difficult to understand what factors contribute to the beauty of the work.
And it requires a long-term effort by trial and error due to subjectivity.

To solve this issue, might be used application such as a color picker(such as getting RGB or HSV parameter in pixel), models generated using machine learning. But it is difficult.

The reasons that make this difficult by following.

- Color picker: Parameter getting by application is hard to handle. Because most people can't solder a color just by looking at parameter. Therefore, a lot of parameter level is hard to control.
- Models generated using machine learning: This practice can find aesthetic, but it can't find "why aesthetic". It is insufficient for people to learn "why aesthetic".

To solve this issue, **this project supply hint for to improve as a image creator**.

## Who use this ?

This project is analysys tool for created image creator(e.g. Photographer, Web designer and Illustrator).
You can recognize aesthetic image by using this application.

## Approach

To describe "why aesthetic", supply information by following.

### Color

#### Abstracted HSB Color

Abstracted HSB Color is 12 level Hue, Saturation and Brightness.
Possible to understand aesthetic color combination by this parameter.

- The abstraction to 12 level, because it is easy to memorize and create color combinations for reasons of a lot of divisor.
- Using HSB parameter, because it is near by human recognition.

[Models](/docs/model/abstract_color)

#### Color complexity

Color complexity is variance of ratio that each HSB level used by image.
Possible to understand aesthetic color ratio by this parameter.

[Models](/docs/model/color_complexity)

## Hypothesis

Using this application, you will understand how to improve created image.

### Possible to Understand aesthetic color combination

Expected following situation.

- If blue color ratio is biggest, aesthetic image may have blue colors.
- If Hue parameter is biased, aesthetic image may have fewer colors.
