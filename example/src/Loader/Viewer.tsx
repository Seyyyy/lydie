import { Image as LImage } from "lydie";
import { hue, saturation, brightness, gray } from "./colorList";
import "./Viewer.css";

type Props = {
  image: LImage;
};

export function Viewer(props: Props) {
  const sum = (arr: number[]): number => {
    let sum = 0;
    arr.forEach((value) => {
      sum += value;
    });
    return sum;
  };

  const getElement = (arr: number[], subTitle: string[]): JSX.Element => {
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

  console.log(props.image.get_usage_rate());

  return (
    <div>
      <h3 className="border">hue</h3>
      <p>
        {"Pixel count is : " +
          sum(props.image.get_usage_quantity().hue_chromatic)}
      </p>
      {getElement(props.image.get_usage_rate().hue_chromatic, hue)}

      <h3 className="border">saturation</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_quantity().saturation)}
      </p>
      {getElement(props.image.get_usage_rate().saturation, saturation)}

      <h3 className="border">value</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_quantity().brightness)}
      </p>
      {getElement(props.image.get_usage_rate().brightness, brightness)}

      <h3 className="border">gray_scale</h3>
      <p>
        {"Pixel count is : " + sum(props.image.get_usage_quantity().hue_gray)}
      </p>
      {getElement(props.image.get_usage_rate().hue_gray, gray)}

      <h3 className="border">entropy</h3>
      <p>{"Hue Entropy is : " + props.image.get_entropy().hue_chromatic}</p>
      <p>{"Saturation Entropy is : " + props.image.get_entropy().saturation}</p>
      <p>{"Brightness Entropy is : " + props.image.get_entropy().brightness}</p>
      <p>{"Gray Entropy is : " + props.image.get_entropy().hue_gray}</p>
    </div>
  );

  return (
    <div>
      {/* <h3 className="border">hue</h3>
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
      <h3 className="border">entropy</h3>
      <p>{"Hue Entropy is : " + props.image.get_entropy()[0]}</p>
      <p>{"Saturation Entropy is : " + props.image.get_entropy()[2]}</p>
      <p>{"Brightness Entropy is : " + props.image.get_entropy()[3]}</p>
      <p>{"Gray Entropy is : " + props.image.get_entropy()[1]}</p> */}
    </div>
  );
}
