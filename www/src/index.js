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

fromEvent(from, "input")
  .pipe(
    tap(() => (to.value = "Loading...")),
    debounceTime(200),
    map((event) => {
      if (isAlpha) {
        to.value = wasm.to_string(event.target.value);
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
