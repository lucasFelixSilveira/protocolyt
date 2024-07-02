class Result<T, E> {
  type: string = "Option";
  state: boolean | null = null;
  val: T | E | null = null;

  private constructor() {}

  static init<U, Y>(): Result<U, Y> {
    return new Result<U, Y>();
  }

  public Ok(val: T): this {
    this.state = true;
    this.val = val;
    return this;
  }

  public Err(val: E): this {
    this.state = false;
    this.val = val;
    return this;
  }

  public isOk(): boolean {
    return this.state == null ? false : this.state;
  }

  public isErr(): boolean {
    return this.state == null ? false : !this.state;
  }

  public unwrap(): any {
    return this.val;
  }

  public unwrapOrElse(callback: () => T): any {
    return this.isErr() ? callback() : this.val;
  }
}

class Option<T> {
  type: string = "Option";
  state: boolean | null = null;
  val: T | null = null;

  private constructor() {}

  static init<U>(): Option<U> {
    return new Option<U>();
  }

  public Some(val: T): this {
    this.state = true;
    this.val = val;
    return this;
  }

  public None(): this {
    this.state = false;
    return this;
  }

  public isSome(): boolean {
    return this.state == null ? false : this.state;
  }

  public isNone(): boolean {
    return this.state == null ? false : !this.state;
  }

  public unwrap(): any {
    if( this.isNone() ) {
      console.log("Invalid value. The value is not valid for the UNWRAP method to be used.");
      return null;
    }
    return this.val;
  }

  public unwrapOrElse(callback: () => T): any {
    return this.isNone() ? callback() : this.val;
  }
}

const type: string = "Module";
export {type, Option, Result };