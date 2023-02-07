import { Construct } from "constructs";
import { Polycons } from "polycons";
import { Code, Resource } from "../core";
import { TracingContext } from "../target-sim";

/**
 * Global identifier for `Counter`.
 */
export const COUNTER_TYPE = "wingsdk.cloud.Counter";

/**
 * Properties for `Counter`.
 */
export interface CounterProps {
  /**
   * The initial value of the counter.
   * @default 0
   */
  readonly initial?: number;
}

/**
 * Functionality shared between all `Counter` implementations.
 */
export abstract class CounterBase extends Resource {
  public readonly stateful = true;

  /**
   * The initial value of the counter.
   */
  public readonly initial: number;

  constructor(scope: Construct, id: string, props: CounterProps = {}) {
    super(scope, id);

    this.display.title = "Counter";
    this.display.description = "A distributed atomic counter";

    if (!scope) {
      this.initial = -1; // not used
      return;
    }

    this.initial = props.initial ?? 0;
  }
}

/**
 * Represents a distributed atomic counter.
 *
 * @inflight `@winglang/sdk.cloud.ICounterClient`
 */
export class Counter extends CounterBase {
  constructor(scope: Construct, id: string, props: CounterProps = {}) {
    super(null as any, id, props);
    return Polycons.newInstance(COUNTER_TYPE, scope, id, props) as Counter;
  }

  /** @internal */
  public _toInflight(): Code {
    throw new Error("Method not implemented.");
  }
}

/**
 * Inflight interface for `Counter`.
 */
export interface ICounterClient {
  /**
   * Increments the counter atomically by a certain amount and returns the previous value.
   * @param amount amount to increment (default is 1).
   * @param ctx Context of the tracing
   * @returns the previous value of the counter.
   * @inflight
   */
  inc(amount?: number, ctx?: TracingContext): Promise<number>;

  /**
   * Decrement the counter, returning the previous value.
   * @param amount amount to decrement (default is 1).
   * @param ctx Context of the tracing
   * @returns the previous value of the counter.
   * @inflight
   */
  dec(amount?: number, ctx?: TracingContext): Promise<number>;

  /**
   * Get the current value of the counter.
   * @param ctx Context of the tracing
   * Using this API may introduce race conditions since the value can change between
   * the time it is read and the time it is used in your code.
   * @returns current value
   * @inflight
   */
  peek(ctx?: TracingContext): Promise<number>;
}

/**
 * Functionality shared between all `CounterClient` implementations regardless of the target.
 */
export abstract class CounterClientBase implements ICounterClient {
  inc(amount?: number, _ctx?: TracingContext): Promise<number> {
    amount;
    throw new Error("Method not implemented.");
  }
  dec(amount?: number, _ctx?: TracingContext): Promise<number> {
    return this.inc(-1 * (amount ?? 1));
  }
  peek(_ctx?: TracingContext): Promise<number> {
    throw new Error("Method not implemented.");
  }
}

/**
 * List of inflight operations available for `Counter`.
 * @internal
 */
export enum CounterInflightMethods {
  /** `Counter.inc` */
  INC = "inc",
  /** `Counter.dec` */
  DEC = "dec",
  /** `Counter.peek` */
  PEEK = "peek",
}
