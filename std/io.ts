import {Option} from "./uncertain";
import * as stdout from "./io/writer";

function writer(): Option<object> {
  return Option.init<object>().Some(stdout);
}

const type = "Module";
export { type, writer };