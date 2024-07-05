import { Temperature, Unit } from "quicktemp";
import { getElement } from "./dom";

// Construct the temperature, and get its value, from and to.
const temperature = Temperature.new();

const form = getElement("form", HTMLFormElement);
const output = getElement("#result", HTMLOutputElement);

type Values = { from_scale: string; to_scale: string; degree: string };

form.addEventListener("submit", function (e) {
  e.preventDefault();

  const formData = Object.fromEntries(new FormData(form).entries()) as Values;

  try {
    temperature.set_from_unit(to_unit(formData.from_scale));
    temperature.set_to_unit(to_unit(formData.to_scale));
    temperature.set_degree(parseFloat(formData.degree));

    output.textContent = `${temperature.convert()}°${to_string(
      temperature.get_to_unit()
    )}`;
  } catch (error) {
    temperature.set_from_unit(to_unit("Celsius"));
    temperature.set_to_unit(to_unit("Fahrenheit"));
    temperature.set_degree(parseFloat("100"));

    output.textContent = `${temperature.convert()}°${to_string(
      temperature.get_to_unit()
    )}`;
  }
});

function to_unit(value: string) {
  switch (value.toLowerCase()) {
    case "celsius":
      return Unit.Celsius;
    case "fahrenheit":
      return Unit.Fahrenheit;
    case "kelvin":
      return Unit.Kelvin;
    default:
      throw new Error("Unknown unit");
  }
}

function to_string(unit: Unit) {
  switch (unit) {
    case Unit.Celsius:
      return "C";
    case Unit.Fahrenheit:
      return "F";
    case Unit.Kelvin:
      return "K";
    default:
      throw new Error("Unknown unit");
  }
}
