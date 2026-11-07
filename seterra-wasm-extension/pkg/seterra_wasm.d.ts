/* tslint:disable */
/* eslint-disable */

export function create_old_header(): void;

export function init_style_sheets(): void;

export function set_center_game(enabled: boolean): void;

export function set_darkmode(enabled: boolean): void;

export function set_dim_screen(enabled: boolean): void;

export function set_label(enabled: boolean): void;

export function set_map_padding(enabled: boolean): void;

export function set_old_header(enabled: boolean): void;

export function set_old_header_flag(enabled: boolean): void;

export function set_type_easy_text_on_top(enabled: boolean): void;

export function set_url_parameter(parameter: string, to_enable: boolean): void;

export function set_water(enabled: boolean): void;

export function update_old_header(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly create_old_header: () => void;
    readonly init_style_sheets: () => void;
    readonly set_center_game: (a: number) => void;
    readonly set_darkmode: (a: number) => void;
    readonly set_dim_screen: (a: number) => void;
    readonly set_label: (a: number) => void;
    readonly set_map_padding: (a: number) => void;
    readonly set_old_header: (a: number) => void;
    readonly set_old_header_flag: (a: number) => void;
    readonly set_type_easy_text_on_top: (a: number) => void;
    readonly set_url_parameter: (a: number, b: number, c: number) => void;
    readonly set_water: (a: number) => void;
    readonly update_old_header: () => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
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
