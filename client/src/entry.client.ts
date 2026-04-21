import init, { Temp } from "@/lib";
import { $ } from "@/scripts/dom";
import "./assets/global.css";

await init();

const form = $("form", HTMLFormElement);
const output = $("#result", HTMLOutputElement);

type Fields = { source: string; target: string; value: string };

form.addEventListener("submit", (e) => {
  e.preventDefault();

  const { value, source, target } = Object.fromEntries(new FormData(form).entries()) as Fields;
  try {
    const result = Temp.convert(value, source, target);
    output.textContent = `${result.degree}${result.unit}`;
    output.removeAttribute("data-error");
  } catch (e) {
    output.textContent = e instanceof Error ? e.message : "conversion failed";
    output.setAttribute("data-error", "");
  }
});
