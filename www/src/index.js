import "./styles";

import * as wasm from "alphanumeric";

import { debounceTime, map, tap } from "rxjs/operators";

import { fromEvent } from "rxjs";

console.log(wasm.to_string("A18720928B30D38F"));

const from = document.getElementById("from");
const to = document.getElementById("to");
const swap = document.getElementById("swap");
const left = document.getElementById("left");
const right = document.getElementById("right");

let isAlpha = true;

// remove "0x" from the begin if exists
const normalizeAlpha = (value) => {
    if (value.startsWith("0x")) {
        return value.slice(2);
    }
    return value;
}


fromEvent(from, "input")
  .pipe(
    tap(() => (to.value = "Loading...")),
    debounceTime(200),
    map((event) => {
      if (isAlpha) {
        to.value = wasm.to_string(normalizeAlpha(event.target.value));
      } else {
        to.value = wasm.from_string(event.target.value);
      }
    })
  )
  .subscribe();

fromEvent(swap, "click")
  .pipe(
    map(() => {
      isAlpha = !isAlpha;

      const temp = left.innerHTML;
      left.innerHTML = right.innerHTML;
      right.innerHTML = temp;

      to.value = "";
      from.dispatchEvent(new Event("input"));
    })
  )
  .subscribe();
