import test from 'ava';

import { compile, JsxRuntime, Processor } from '../index.js';

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
    jsxRuntime: JsxRuntime.Automatic,
  });
  t.true(result.includes('preact'));
});

test('processor', t => {
  const processor = new Processor({});
  const res = processor.process(`# Hello`);
  t.true(res.includes('h1') && res.includes('Hello'))
})
