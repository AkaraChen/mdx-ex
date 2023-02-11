/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum JsxRuntime {
  Automatic = 0,
  Classic = 1
}
export interface Opts {
  providerImportSource?: string
  jsxRuntime?: JsxRuntime
  jsxImportSource?: string
  pragma?: string
  pragmaFrag?: string
  pragmaImportSource?: string
  filepath?: string
}
export function compile(mdx: string, userOpts?: Opts | undefined | null): string
export class Processor {
  opts: Opts
  constructor(opts: Opts)
  process(mdx: string): string
}
