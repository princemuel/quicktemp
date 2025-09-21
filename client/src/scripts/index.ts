import { convert } from "quicktemp";
import { $ } from "./dom";
import { capitalize } from "./utils";

const form = $("form", HTMLFormElement);
const output = $("#result", HTMLOutputElement);

type FormDataValues = { source: string; target: string; value: string };

form.addEventListener("submit", (e) => {
    e.preventDefault();

    const { value, source, target } = Object.fromEntries(new FormData(form).entries()) as FormDataValues;

    const result = convert(value, source, target);

    output.textContent = `${result}°${capitalize(target)}`;
});
