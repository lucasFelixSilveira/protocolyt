import asciz from "../asciz";
import {Option} from "../uncertain";
import * as types from "../types";

function println(fmt: asciz) {
  if(typeof fmt != "string") return;
  console.write(`${fmt}\x0a`);
}

async function print(fmt: asciz) {
  if(typeof fmt != "string") return;
  console.write(fmt);
}

function debug(obj: object | string) {
  if( typeof obj == "string" ) return console.log("Debug:", obj);
  if( typeof obj != "object" ) return;
  const type = obj?.type || "Unknown";
  if( type == "Unknown" ) return console.log("Debug: Unkown");
  const resp: Option<string> = types.from(obj);
  return console.log("Debug:", resp.isNone() ? "Invalid input" : resp.unwrap())
}


const type: string = "Stdout";
export { type, print, println, debug };