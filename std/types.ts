import { Option } from "./uncertain";

function from(input: any): Option<string> {
  if(typeof input != "object") return Option.init<string>().None();
  const next = (obj: any): string => {
    const type = obj?.type || typeof obj || "Unknown";
    if( type != "string" && typeof type == "string" ) return type;
    switch(type) {
      case "Option": {
        const t = obj.isSome() ? `Some(${next(obj.unwrap())})` 
                               : "None";
        return `Option[T] { ${t} }`;
      }
      case "Result": {
        const t = obj.isOk() ? `Ok(${next(obj.unwrap())})` 
                             : `Err(${next(obj.unwrap())})`;
        return `Option[T, E] { ${t} }`;
      }
      case "Stdout": {
        return "Stdout";
      }
      case "string": {
        return obj;
      }
      default: {
        return type;
      }
    }
  };
  
  const entry = next;
  const type = input?.type || "Unknown";
  if( type == "Unknown" ) return Option.init<string>().None();
  else return Option.init<string>().Some(`${entry(input)}`);
}

const type: string = "impl Type";
export { type, from }