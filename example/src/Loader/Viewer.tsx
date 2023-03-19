import { Image as LImage } from "lydie";
import { hue, saturation, brightness, gray } from "./colorList";
import "./Viewer.css";

type Props = {
  image: LImage;
};

export function Viewer(props: Props) {
  const sum = (arr: Uint32Array): number => {
    let sum = 0;
    arr.forEach((value) => {
      sum += value;
    });
    return sum;
  };

  const getElement = (arr: Uint32Array, subTitle: string[]): JSX.Element => {
    const sumValue = sum(arr);
    let ElementArray: JSX.Element[] = [];
    arr.forEach((value, index) => {
      const per = (value / sumValue) * 100;
      ElementArray.push(
        <div key={index}>{subTitle[index] + " : " + Math.round(per) + "%"}</div>
      );
    });
    return <div>{ElementArray}</div>;
  };

  return (
    <div>
      <h3 className="border">hue</h3>
      <p>{"Pixel count is : " + sum(props.image.get_usage_rate_hue())}</p>
      {getElement(props.image.get_usage_rate_hue(), hue)}
      <h3 className="border">saturation</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_rate_saturation())}
      </p>
      {getElement(props.image.get_usage_rate_saturation(), saturation)}
      <h3 className="border">value</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_rate_brightness())}
      </p>
      {getElement(props.image.get_usage_rate_brightness(), brightness)}
      <h3 className="border">gray_scale</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_rate_gray_scale())}
      </p>
      {getElement(props.image.get_usage_rate_gray_scale(), gray)}
    </div>
  );
}
