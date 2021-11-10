import "./styles";

import { debounceTime, map, tap } from "rxjs/operators";
import { from_string, to_string } from "alphanumeric";

import { fromEvent } from "rxjs";

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
      to.value = isAlpha
        ? to_string(event.target.value, 10)
        : from_string(event.target.value);
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
