class Vector<T> {
  row: T[];

  private constructor() {
    this.row = [];
  }

  static init<U>(): Vector<U> {
    return new Vector<U>();
  }

  public push(val: T): void {
    this.row.push(val)
  }
  public pop(): any {
    return this.row.pop()
  }
}

export default Vector;
