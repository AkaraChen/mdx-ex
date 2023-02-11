import test from 'ava';

import { compile, JsxRuntime } from '../index.js';

test('compile', t => {
  const result = compile(`# Hello`);
  t.true(result.includes('h1') && result.includes('Hello'));
});

test('error', t => {
  t.throws(() => {
    compile(`import {} from ava;`);
  });
});

test('importSource', t => {
  const result = compile(`export default () => <h1>Hello</h1>`, {
    jsxImportSource: 'preact',
    jsxRuntime: JsxRuntime.Automatic
  });
  t.true(result.includes('preact'));
});
