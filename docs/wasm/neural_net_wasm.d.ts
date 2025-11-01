/* tslint:disable */
/* eslint-disable */
/**
 * Initialize for WASM execution
 */
export function init(): void;
/**
 * List all available examples
 */
export function listExamples(): any;
/**
 * Get details about a specific example
 */
export function getExampleInfo(name: string): any;
/**
 * Example information for JavaScript
 */
export class ExampleInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  name: string;
  description: string;
  architecture: Uint32Array;
}
/**
 * WASM-friendly neural network wrapper
 */
export class NeuralNetwork {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Create a new neural network with specified architecture
   */
  constructor(layers: Uint32Array, learning_rate: number);
  /**
   * Create a network from a built-in example
   */
  static fromExample(example_name: string, learning_rate: number): NeuralNetwork;
  /**
   * Train the network on a built-in example
   * Accepts an optional JavaScript callback for progress updates
   */
  train(example_name: string, epochs: number, progress_callback?: Function | null): void;
  /**
   * Train with custom inputs and targets
   */
  trainCustom(inputs_flat: Float64Array, targets_flat: Float64Array, input_size: number, target_size: number, epochs: number): void;
  /**
   * Evaluate the network on a single input
   */
  evaluate(input: Float64Array): Float64Array;
  /**
   * Get the network architecture
   */
  get_architecture(): Uint32Array;
  /**
   * Get the total number of parameters (weights + biases)
   */
  getParameterCount(): number;
  /**
   * Serialize the network to JSON string
   */
  toJSON(): string;
  /**
   * Deserialize a network from JSON string
   */
  static fromJSON(json: string): NeuralNetwork;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly __wbg_exampleinfo_free: (a: number, b: number) => void;
  readonly __wbg_get_exampleinfo_name: (a: number) => [number, number];
  readonly __wbg_set_exampleinfo_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_exampleinfo_description: (a: number) => [number, number];
  readonly __wbg_set_exampleinfo_description: (a: number, b: number, c: number) => void;
  readonly __wbg_get_exampleinfo_architecture: (a: number) => [number, number];
  readonly __wbg_set_exampleinfo_architecture: (a: number, b: number, c: number) => void;
  readonly __wbg_neuralnetwork_free: (a: number, b: number) => void;
  readonly neuralnetwork_new: (a: number, b: number, c: number) => [number, number, number];
  readonly neuralnetwork_fromExample: (a: number, b: number, c: number) => [number, number, number];
  readonly neuralnetwork_train: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly neuralnetwork_trainCustom: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly neuralnetwork_evaluate: (a: number, b: number, c: number) => [number, number, number, number];
  readonly neuralnetwork_get_architecture: (a: number) => [number, number];
  readonly neuralnetwork_getParameterCount: (a: number) => number;
  readonly neuralnetwork_toJSON: (a: number) => [number, number, number, number];
  readonly neuralnetwork_fromJSON: (a: number, b: number) => [number, number, number];
  readonly listExamples: () => [number, number, number];
  readonly getExampleInfo: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
