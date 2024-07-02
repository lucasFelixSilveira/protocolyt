import { Option } from "./uncertain";

class ASCIz {
  type: string =  "ASCIz";
  bytes: Uint8Array | null = null;
  private constructor() {
    this.bytes = new Uint8Array();
  }

  public len(): number {
    try {
      return (this.bytes as Uint8Array).length
    } catch {
      return 0;
    }
  }

  static init(): ASCIz {
    return new ASCIz();
  }

  public from(fmt: any): Option<ASCIz> {
    const resp = Option.init<ASCIz>();
    const str = new ASCIz();

    if( typeof fmt == "string" ) {
      str.bytes = new TextEncoder().encode(fmt as string)
      return resp.Some(str);
    } 

    return resp.None()
  }
}

const type = "impl ASCIz" 
export { type, ASCIz }