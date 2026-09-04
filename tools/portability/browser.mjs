// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
import init, { run_suite, inspect_bytes } from './generated/choreoform_portability.js';

const status = document.querySelector('#status');
const summary = document.querySelector('#summary');
const button = document.querySelector('#run');
const sha256 = async bytes => [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))]
  .map(byte => byte.toString(16).padStart(2, '0')).join('');

async function checkedFetch(path) {
  const response = await fetch(path, { cache: 'no-store' });
  if (!response.ok) throw new Error(`${path}: HTTP ${response.status}`);
  return response;
}

button.addEventListener('click', async () => {
  button.disabled = true;
  status.textContent = 'Running the Rust suite inside browser WebAssembly…';
  document.querySelector('#cases').replaceChildren();
  summary.textContent = '';
  try {
    const [wasmBytes, nativeText] = await Promise.all([
      checkedFetch('./generated/choreoform_portability_bg.wasm').then(r => r.arrayBuffer()),
      checkedFetch('./generated/native.json').then(r => r.text()),
    ]);
    const module = await WebAssembly.compile(wasmBytes);
    const imports = WebAssembly.Module.imports(module);
    // Only string-return/panic ABI support. No WASI or ambient core host APIs.
    const importsAllowed = imports.length === 1 && imports.every(entry => entry.kind === 'function'
      && entry.module === './choreoform_portability_bg.js'
      && entry.name === '__wbindgen_init_externref_table');
    if (!importsAllowed) throw new Error(`Unreviewed Wasm imports: ${JSON.stringify(imports)}`);
    await init({ module_or_path: module });
    const browserText = run_suite();
    const repeatText = run_suite();
    const native = JSON.parse(nativeText);
    const actual = JSON.parse(browserText);
    const parity = nativeText === browserText;
    const repeatable = browserText === repeatText;
    const encoder = new TextEncoder();
    const fixtureTransfers = native.cases.slice(0, 3).every(row =>
      inspect_bytes(encoder.encode(JSON.stringify(row.actual.document))) === JSON.stringify(row.actual));
    const transportTransfers = [
      [new Uint8Array([0xff]), 'utf8'],
      [encoder.encode('{"x":0,"x":1}'), 'duplicate-key'],
      [encoder.encode('1e0'), 'number-token'],
    ].every(([bytes, category]) => JSON.parse(inspect_bytes(bytes)).category === category);
    const byteBoundaryParity = fixtureTransfers && transportTransfers;
    const passed = parity && repeatable && byteBoundaryParity && native.passed && actual.passed;
    for (let index = 0; index < actual.cases.length; index++) {
      const row = actual.cases[index];
      const tr = document.createElement('tr');
      for (const value of [row.name, row.expected, row.actual.category,
        JSON.stringify(row) === JSON.stringify(native.cases[index]) && row.passed ? 'PASS' : 'FAIL']) {
        const td = document.createElement('td'); td.textContent = value; tr.append(td);
      }
      document.querySelector('#cases').append(tr);
    }
    const evidence = {
      passed, reportVersion: actual.reportVersion, cases: actual.cases.length,
      exactReportParity: parity, repeatable, byteBoundaryParity,
      byteBoundaryCases: 6, importsAllowed, imports,
      userAgent: navigator.userAgent,
      wasmSha256: await sha256(wasmBytes),
      nativeReportSha256: await sha256(new TextEncoder().encode(nativeText)),
      browserReportSha256: await sha256(new TextEncoder().encode(browserText)),
    };
    summary.textContent = JSON.stringify(evidence, null, 2);
    status.textContent = passed ? `PASS — ${actual.cases.length} cases; exact native/browser report parity.` : 'FAIL — inspect case results.';
  } catch (error) {
    status.textContent = 'FAIL — browser verification did not complete.';
    summary.textContent = String(error);
  } finally { button.disabled = false; }
});
