// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
import init, {canonical_document, change_document, view_document} from './generated/choreoform_authoring_prototype.js';

const $ = id => document.getElementById(id);
const encoder = new TextEncoder();
const decoder = new TextDecoder('utf-8', {fatal: true});
const MiB = 1024 * 1024;
const pageSize = 100;
const session = new URLSearchParams(location.hash.slice(1)).get('session') || '';
history.replaceState(null, '', location.pathname);
// Reopening a launcher link in this same tab must initialize its new session.
addEventListener('hashchange', () => { if (location.hash) location.reload(); });
let permissions = {}, model = null, selected = null, scope = null, page = 0, diagramPage = 0, busy = false, download = null;
let irText = '', irStarts = [], irPage = 0;
const evidence = {experiment: 'paired-prototype-0.1.0', userAgent: navigator.userAgent, representativeUsers: false, processExecution: false, snapshots: []};
const roleLabels = {'type-alias': 'Type definition', 'data-protection': 'Protection definition', 'predicate': 'Pure test', 'predicate-input': 'Test input', 'provider-policy': 'Provider requirements', 'initial': 'Initial value', 'assignment': 'Assignment', 'flow': 'Outcome route', 'scope': 'Scope', 'node': 'Step', 'data': 'Data', 'expression': 'Expression', 'actor': 'Actor requirement', 'capability': 'Capability requirement', 'policy': 'Policy', 'resource': 'Pinned resource', 'outcome': 'Outcome', 'branch': 'Branch', 'input': 'Input', 'output': 'Output', 'purpose': 'Purpose'};
const nodeLabels = {activity: 'Activity', compute: 'Calculation', invoke: 'Call', decision: 'Decision', split: 'Split', join: 'Join', wait: 'Wait', repeat: 'Repeat', fanout: 'Collection', finish: 'Finish'};
function status(message, error = false) { $('status').textContent = message; $('status').classList.toggle('error', error); }
function record() { $('evidence').textContent = JSON.stringify(evidence, null, 2); }
function bytes(value, maximum) {
  if (!(value instanceof Uint8Array) || value.byteLength > maximum) throw new Error('Input exceeds this profile’s byte limit.');
  return value;
}
// These guards run BEFORE wasm-bindgen copies an argument into Wasm memory.
function canonical(raw, header, blob) { return canonical_document(bytes(raw, 16 * MiB), bytes(header, 4 * MiB), bytes(blob, 8 * MiB)); }
function view(raw, header, blob) { return view_document(bytes(raw, 16 * MiB), bytes(header, 4 * MiB), bytes(blob, 8 * MiB)); }
function changed(raw, request, header, blob) { return change_document(bytes(raw, 16 * MiB), bytes(request, MiB), bytes(header, 4 * MiB), bytes(blob, 8 * MiB)); }
async function readLimited(response, maximum) {
  const length = response.headers.get('Content-Length');
  if (length && (!/^\d+$/.test(length) || Number(length) > maximum)) { await response.body?.cancel(); throw new Error('Response exceeds this profile’s byte limit.'); }
  if (!response.body) return new Uint8Array();
  const reader = response.body.getReader(), chunks = []; let size = 0;
  try {
    for (;;) {
      const {value, done} = await reader.read(); if (done) break;
      size += value.byteLength;
      if (size > maximum) throw new Error('Response exceeds this profile’s byte limit.');
      chunks.push(value);
    }
  } catch (error) { await reader.cancel(); throw error; }
  const result = new Uint8Array(size); let offset = 0;
  for (const chunk of chunks) { result.set(chunk, offset); offset += chunk.byteLength; }
  return result;
}
async function api(path, maximum, options = {}) {
  const response = await fetch(path, {...options, credentials: 'omit', headers: {'X-Choreoform-Session': session, ...options.headers}});
  if (!response.ok) {
    const raw = await readLimited(response, 4096); let message = 'Request refused.';
    try { const error = JSON.parse(decoder.decode(raw)); message = ({access: 'This session does not permit that action.', conflict: 'The snapshot changed. Choose the process again to reload.', 'not-found': 'Process unavailable.', 'native-refusal': 'The shared core refused this change.', size: 'This request exceeds the profile limit.', io: 'The local adapter could not complete this request.', request: 'Malformed request.'})[error.code] || message; if (error.diagnostic) message += ` ${error.diagnostic}`; } catch { /* Never echo HTTP/body data as a fallback. */ }
    throw new Error(message);
  }
  return readLimited(response, maximum);
}
function controls() {
  $('app').setAttribute('aria-busy', String(busy));
  $('process').disabled = busy;
  for (const id of ['format', 'export', 'verify']) $(id).disabled = busy || !model || (id === 'format' && !permissions.edit) || (id === 'export' && !permissions.export);
  $('rename').disabled = busy || !permissions.edit || !selected || !model?.byId.get(selected)?.renameAllowed;
  $('name').disabled = $('rename').disabled;
  $('place').disabled = busy || !permissions.edit || !selected;
  for (const id of ['x', 'y', 'collapsed']) $(id).disabled = $('place').disabled;
  for (const id of ['search', 'scope', 'source', 'diagram-scale']) $(id).disabled = busy || !model;
  $('save').hidden = !download || busy;
  if (busy) for (const id of ['previous', 'next', 'diagram-previous', 'diagram-next']) $(id).disabled = true;
}
async function work(action) {
  if (busy) return;
  busy = true; controls(); status('Checking the shared Rust model…');
  try { await new Promise(requestAnimationFrame); await action(); }
  catch (error) { status(typeof error === 'string' ? `The shared core refused this operation. ${error}` : error.message || 'Operation refused.', true); }
  finally { busy = false; if (model) { $('process').value = model.key; renderList(); renderDiagram(); } controls(); }
}
function clearDownload() { if (download) URL.revokeObjectURL(download); download = null; $('save').removeAttribute('href'); $('save').hidden = true; }
function kind(entity) {
  if (!entity.nodeKind) return roleLabels[entity.role];
  const node = model.view.ir.body.nodes[entity.id];
  return entity.nodeKind === 'activity' ? (node.mode === 'human' ? 'Human activity' : 'Capability activity') : nodeLabels[entity.nodeKind];
}
function option(value, label) { const o = document.createElement('option'); o.value = value; o.textContent = label; return o; }
function button(label, action) { const b = document.createElement('button'); b.type = 'button'; b.textContent = label; b.addEventListener('click', action); return b; }
function nameOf(id) { return model.byId.get(id)?.caption || 'Unavailable reference'; }
function renderList() {
  if (!model) return;
  const focused = $('items').contains(document.activeElement) ? document.activeElement.dataset.entity : null;
  const query = $('search').value.toLocaleLowerCase();
  const rows = model.view.entities.filter(e => !query || e.caption.toLocaleLowerCase().includes(query) || kind(e).toLocaleLowerCase().includes(query));
  page = Math.min(page, Math.max(0, Math.ceil(rows.length / pageSize) - 1));
  const start = page * pageSize;
  $('count').textContent = `${rows.length} matching items of ${model.view.entities.length}. ${rows.length ? `${start + 1}–${Math.min(start + pageSize, rows.length)} shown.` : ''}`;
  $('items').replaceChildren(...rows.slice(start, start + pageSize).map(entity => {
    const b = button(`${entity.caption} · ${kind(entity)}`, () => select(entity.id, true)); b.dataset.entity = entity.id;
    b.setAttribute('aria-current', String(entity.id === selected)); return b;
  }));
  if (focused) [...$('items').children].find(b => b.dataset.entity === focused)?.focus({preventScroll: true});
  $('previous').disabled = page === 0 || busy; $('next').disabled = start + pageSize >= rows.length || busy;
}
const SVG = 'http://www.w3.org/2000/svg';
function svg(tag, attributes = {}, text) {
  const element = document.createElementNS(SVG, tag);
  for (const [key, value] of Object.entries(attributes)) element.setAttribute(key, String(value));
  if (text !== undefined) element.textContent = text;
  return element;
}
function renderDiagram() {
  if (!model) return;
  const focused = $('diagram').contains(document.activeElement) ? document.activeElement.dataset.entity : null;
  const all = model.view.entities.filter(e => e.owner === scope);
  diagramPage = Math.min(diagramPage, Math.max(0, Math.ceil(all.length / pageSize) - 1));
  const rows = all.slice(diagramPage * pageSize, (diagramPage + 1) * pageSize);
  const positions = new Map(rows.map(e => [e.id, e.placement]));
  const layer = svg('g');
  for (const flow of model.view.flows) {
    const a = positions.get(flow.source), b = positions.get(flow.target);
    if (!a || !b) continue;
    const group = svg('g');
    group.append(svg('path', {class: 'route', d: `M ${a.x + 108} ${a.y + 72} L ${b.x + 108} ${b.y}`, 'marker-end': 'url(#arrow)'}));
    group.append(svg('title', {}, `${nameOf(flow.source)} — ${flow.caption} → ${nameOf(flow.target)}`));
    group.append(svg('text', {x: (a.x + b.x) / 2 + 108, y: (a.y + b.y) / 2 + 32}, flow.caption.slice(0, 30)));
    layer.append(group);
  }
  for (const entity of rows) {
    const p = entity.placement;
    const group = svg('g', {transform: `translate(${p.x} ${p.y})`, role: 'button', tabindex: '0', 'aria-label': `${entity.caption}, ${kind(entity)}${p.collapsed ? ', collapsed' : ''}`});
    group.dataset.entity = entity.id;
    group.append(svg('title', {}, `${entity.caption} · ${kind(entity)}`));
    group.append(svg('rect', {width: 216, height: p.collapsed ? 48 : 80, rx: 7, class: `card ${entity.role} ${entity.id === selected ? 'selected' : ''}`}));
    group.append(svg('text', {x: 10, y: 23}, entity.caption.length > 27 ? entity.caption.slice(0, 26) + '…' : entity.caption));
    group.append(svg('text', {x: 10, y: 41, class: 'kind'}, kind(entity)));
    if (!p.collapsed) group.append(svg('text', {x: 10, y: 64, class: 'kind'}, entity.role === 'scope' ? 'Open through item details' : 'Select for complete detail'));
    group.addEventListener('click', () => select(entity.id, true));
    group.addEventListener('keydown', event => { if (['Enter', ' '].includes(event.key)) { event.preventDefault(); select(entity.id, true); } });
    layer.append(group);
  }
  const defs = svg('defs'), marker = svg('marker', {id: 'arrow', viewBox: '0 0 10 10', refX: 9, refY: 5, markerWidth: 7, markerHeight: 7, orient: 'auto-start-reverse'});
  marker.append(svg('path', {d: 'M 0 0 L 10 5 L 0 10 z', fill: '#487780'})); defs.append(marker);
  const xs = rows.map(e => e.placement.x), ys = rows.map(e => e.placement.y);
  const minX = Math.min(0, ...xs) - 20, minY = Math.min(0, ...ys) - 20;
  const width = Math.max(480, ...xs.map(x => x + 236 - minX)), height = Math.max(260, ...ys.map(y => y + 100 - minY));
  $('diagram').setAttribute('viewBox', `${minX} ${minY} ${width} ${height}`);
  $('diagram').setAttribute('width', Math.min(width, 4000)); $('diagram').setAttribute('height', Math.min(height, 4000));
  $('diagram').classList.toggle('fit', $('diagram-scale').value === 'fit');
  $('diagram').replaceChildren(defs, layer);
  if (focused) [...layer.children].find(g => g.dataset.entity === focused)?.focus({preventScroll: true});
  $('diagram-note').textContent = `${nameOf(scope)} · items owned directly by this scope. ${rows.length} of ${all.length} shown on page ${diagramPage + 1}. Arrows appear when both endpoints are on this page; all routes remain in the item list and details.`;
  $('diagram-previous').disabled = diagramPage === 0 || busy;
  $('diagram-next').disabled = (diagramPage + 1) * pageSize >= all.length || busy;
}
function fact(label, value) { const p = document.createElement('p'); p.textContent = `${label}: ${value}`; $('facts').append(p); }
function ref(label, id) {
  if (!id || !model.byId.has(id)) return;
  const p = document.createElement('p'); p.append(document.createTextNode(`${label}: `), button(nameOf(id), () => select(id, true))); $('facts').append(p);
}
function inspect() {
  const entity = model.byId.get(selected); if (!entity) return;
  $('selected').textContent = entity.caption;
  $('owner').textContent = entity.owner ? `Owned by ${nameOf(entity.owner)}` : 'Root declaration';
  $('facts').replaceChildren(); fact('Kind', kind(entity));
  if (entity.owner) ref('Owner', entity.owner);
  if (entity.role === 'scope') $('facts').append(button('Open this scope in the diagram', () => { if (busy) return; scope = entity.id; diagramPage = 0; $('scope').value = scope; renderDiagram(); }));
  const body = model.view.ir.body, record = entity.semanticMap ? body[entity.semanticMap][entity.id] : null;
  if (record) {
    for (const field of ['entry', 'parent', 'cancellation', 'faults', 'closure', 'race', 'actor', 'capability', 'body', 'condition', 'join', 'source', 'target', 'policy']) {
      // Only declared, top-level reference slots; literal/type payloads are opaque.
      if (typeof record[field] === 'string') ref(field[0].toUpperCase() + field.slice(1), record[field]);
    }
    for (const field of ['reads', 'writes']) {
      if (record[field] && typeof record[field] === 'object') {
        const ids = Object.keys(record[field]); fact(field === 'reads' ? 'Static reads' : 'Static writes', ids.length ? ids.map(nameOf).join(', ') : 'None');
      }
    }
    if (entity.nodeKind === 'finish') fact('Declared outcome', record.outcome);
  }
  const flow = model.view.flows.find(f => f.id === entity.id);
  if (flow) { ref('From', flow.source); fact('Outcome', flow.caption); ref('To', flow.target); }
  const location = entity.location;
  $('provenance').textContent = ({declaration: 'Complete native declaration location.', slot: 'Native generated-slot location.', owner: 'Owner declaration shown; this item has no separate token location.', companion: 'Companion declaration; this resource has no source clause.'})[location.basis];
  $('clause').textContent = location.start === null ? 'Exact pinned resource supplied by the local catalog. No automatic fetch or provider authorization.' : decoder.decode(model.sourceBytes.subarray(location.start, location.end));
  if (entity.pin) { fact('Resource kind', entity.pin.kind); fact('Pinned contract', entity.pin.contract); fact('Exact byte digest', entity.pin.digest); }
  $('name').value = entity.caption;
  $('rename').textContent = entity.renameAllowed ? 'Rename definition' : 'Rename unavailable in this profile';
  $('x').value = entity.placement.x; $('y').value = entity.placement.y; $('collapsed').checked = entity.placement.collapsed;
  controls();
}
function select(id, jump = false) {
  if (busy || !model.byId.has(id)) return;
  selected = id; const entity = model.byId.get(id);
  inspect(); renderList(); renderDiagram();
  if (jump && entity.location.start !== null) {
    const start = decoder.decode(model.sourceBytes.subarray(0, entity.location.start)).length;
    const end = decoder.decode(model.sourceBytes.subarray(0, entity.location.end)).length;
    $('source').setSelectionRange(start, end);
  }
}
async function checkedSnapshot(key, raw, header, blob) {
  const text = canonical(raw, header, blob);
  if (text !== decoder.decode(raw)) throw new Error('Native and browser document bytes differ.');
  const projected = view(raw, header, blob), result = JSON.parse(projected);
  const reference = await api(`/api/reference/${key}`, 16 * MiB, {headers: {'X-Authoring-Base': result.binding.authoring}});
  if (decoder.decode(reference) !== projected) throw new Error('Native and browser views differ.');
  return {key, raw, header, blob, document: JSON.parse(text), view: result, viewText: projected, sourceBytes: encoder.encode(result.source), byId: new Map(result.entities.map(e => [e.id, e]))};
}
function install(next) {
  model = next; clearDownload();
  if (!model.byId.has(selected)) selected = model.view.binding.root;
  if (!model.view.entities.some(e => e.id === scope && e.role === 'scope')) scope = model.view.binding.root;
  $('scope').replaceChildren(...model.view.entities.filter(e => e.role === 'scope').map(e => option(e.id, e.caption)));
  $('scope').value = scope; $('source').value = model.view.source;
  renderList(); renderDiagram(); inspect();
  irText = ''; irStarts = []; irPage = 0; renderIR();
  if (evidence.snapshots.length === 100) evidence.snapshots.shift();
  evidence.snapshots.push({key: model.key, authoring: model.view.binding.authoring, semanticRevision: model.view.binding.semanticRevision, entities: model.view.entities.length, flows: model.view.flows.length, documentBytes: model.raw.byteLength, nativeDocumentParity: true, nativeViewParity: true});
  record(); status('Text and diagram match. Full validity and execution permission are not established.');
}
async function load() {
  const key = $('process').value;
  const [raw, header, blob] = await Promise.all([api(`/api/process/${key}`, 16 * MiB), api(`/api/registry/${key}`, 4 * MiB), api(`/api/resources/${key}`, 8 * MiB)]);
  const next = await checkedSnapshot(key, raw, header, blob);
  selected = null; scope = null; page = 0; diagramPage = 0; install(next);
}
async function change(operation) {
  const request = bytes(encoder.encode(JSON.stringify({format: 'choreoform-authoring-change', version: '0.1.0', base: model.view.binding.authoring, operation})), MiB);
  const expected = changed(model.raw, request, model.header, model.blob);
  const actual = await api(`/api/change/${model.key}`, 16 * MiB, {method: 'POST', headers: {'Content-Type': 'application/json'}, body: request});
  if (decoder.decode(actual) !== expected) throw new Error('Native and browser change bytes differ. Reload this process.');
  const next = await checkedSnapshot(model.key, actual, model.header, model.blob);
  evidence.lastChange = {kind: operation.kind, exactNativeParity: true}; install(next);
}
async function verify() {
  const repeat = await checkedSnapshot(model.key, model.raw, model.header, model.blob);
  if (repeat.viewText !== model.viewText) throw new Error('Repeated view differs.');
  const invalid = encoder.encode('{"SECRET":0,"SECRET":1}');
  let privateRefusal = false;
  try { canonical(invalid, model.header, model.blob); } catch (error) { privateRefusal = !String(error).includes('SECRET'); }
  if (!privateRefusal) throw new Error('Private refusal check failed.');
  evidence.lastVerification = {key: model.key, deterministic: true, nativeParity: true, malformedDocumentRefusedPrivately: true};
  record(); status('This browser repeated the shared Rust view and matched the native snapshot. Malformed input was refused privately.');
}
function renderIR() {
  if (!$('ir-details').open || !model) {
    $('ir-detail').textContent = ''; $('ir-page').textContent = '';
    irText = ''; irStarts = []; return;
  }
  if (!irText) {
    irText = JSON.stringify(model.view.ir); irStarts = [0]; irPage = 0;
    // Bound rendered accessibility text; retain every character and scalar boundary.
    for (let start = 0; start < irText.length;) {
      let end = Math.min(start + 32768, irText.length);
      const last = irText.charCodeAt(end - 1), next = irText.charCodeAt(end);
      if (last >= 0xd800 && last <= 0xdbff && next >= 0xdc00 && next <= 0xdfff) end--;
      irStarts.push(end); start = end;
    }
  }
  const count = irStarts.length - 1;
  irPage = Math.max(0, Math.min(irPage, count - 1));
  $('ir-detail').textContent = irText.slice(irStarts[irPage], irStarts[irPage + 1]);
  $('ir-page').textContent = `Page ${irPage + 1} of ${count}. The complete exact IR is available across these pages; this is a read-only inspection view.`;
  $('ir-previous').disabled = irPage === 0;
  $('ir-next').disabled = irPage + 1 === count;
}
$('ir-details').addEventListener('toggle', renderIR);
$('ir-previous').addEventListener('click', () => { irPage--; renderIR(); });
$('ir-next').addEventListener('click', () => { irPage++; renderIR(); });
$('process').addEventListener('change', () => work(load));
$('search').addEventListener('input', () => { page = 0; renderList(); });
$('previous').addEventListener('click', () => { page--; renderList(); });
$('next').addEventListener('click', () => { page++; renderList(); });
$('diagram-previous').addEventListener('click', () => { diagramPage--; renderDiagram(); });
$('diagram-next').addEventListener('click', () => { diagramPage++; renderDiagram(); });
$('diagram-scale').addEventListener('change', renderDiagram);
$('scope').addEventListener('change', () => { scope = $('scope').value; diagramPage = 0; renderDiagram(); });
function sourceSelection() {
  if (!model || busy) return;
  const offset = encoder.encode($('source').value.slice(0, $('source').selectionStart)).byteLength;
  const rows = model.view.entities.filter(e => e.location.start !== null && e.location.start <= offset && offset < e.location.end);
  rows.sort((a, b) => (a.location.end - a.location.start) - (b.location.end - b.location.start) || Number(b.category === 'declaration') - Number(a.category === 'declaration'));
  if (rows.length) select(rows[0].id);
}
$('source').addEventListener('click', sourceSelection);
$('source').addEventListener('keyup', sourceSelection);
$('rename-form').addEventListener('submit', event => {
  event.preventDefault(); if (!selected || !permissions.edit || !model.byId.get(selected)?.renameAllowed) return;
  const name = $('name').value;
  if (!name || [...name].length > 4096) { status('Use between 1 and 4096 Unicode characters.', true); return; }
  work(() => change({kind: 'rename', target: selected, name}));
});
$('place').addEventListener('click', () => work(async () => {
  if (!selected || !permissions.edit) return;
  if (!$('x').value || !$('y').value) throw new Error('Enter both coordinates.');
  const x = Number($('x').value), y = Number($('y').value);
  if (![x, y].every(n => Number.isInteger(n) && Math.abs(n) <= 1000000)) throw new Error('Coordinates must be whole numbers between −1000000 and 1000000.');
  const entries = model.document.layout.entries.filter(e => e.entity !== selected);
  entries.push({entity: selected, x, y, collapsed: $('collapsed').checked});
  entries.sort((a, b) => a.entity < b.entity ? -1 : a.entity > b.entity ? 1 : 0);
  await change({kind: 'replace-layout', layout: {format: 'choreoform-authoring-layout', version: '0.1.0', entries}});
}));
$('format').addEventListener('click', () => work(() => change({kind: 'format'})));
$('verify').addEventListener('click', () => work(verify));
$('export').addEventListener('click', () => work(async () => {
  const raw = await api(`/api/export/${model.key}`, 16 * MiB, {headers: {'X-Authoring-Base': model.view.binding.authoring}});
  if (decoder.decode(raw) !== canonical(raw, model.header, model.blob) || decoder.decode(raw) !== decoder.decode(model.raw)) throw new Error('Export differs from the checked complete snapshot.');
  clearDownload(); download = URL.createObjectURL(new Blob([raw], {type: 'application/json'}));
  $('save').href = download; $('save').download = `process-${model.key}.choreoform.json`;
  evidence.lastExport = {key: model.key, completeSnapshot: true, nativeCanonicalReimport: true, bytes: raw.byteLength}; record();
  status('Complete process export prepared and readmitted by the shared Rust core. Use “Save complete process” to save it.');
}));

await work(async () => {
  if (!/^[a-f0-9]{64}$/.test(session)) throw new Error('Open a launcher-provided session link to read a process.');
  const [wasmResponse, expectedResponse] = await Promise.all([fetch('./generated/choreoform_authoring_prototype_bg.wasm', {credentials: 'omit'}), fetch('./imports.json', {credentials: 'omit'})]);
  if (!wasmResponse.ok || !expectedResponse.ok) throw new Error('Shared core assets unavailable. Run the prototype preparation step.');
  const wasmBytes = await readLimited(wasmResponse, 32 * MiB), expected = JSON.parse(decoder.decode(await readLimited(expectedResponse, MiB)));
  const module = await WebAssembly.compile(wasmBytes), imports = WebAssembly.Module.imports(module);
  const normalized = rows => JSON.stringify(rows.map(r => [r.module, r.name, r.kind]).sort());
  if (normalized(imports) !== normalized(expected)) throw new Error('Unreviewed shared-core imports.');
  await init({module_or_path: module});
  evidence.wasmSha256 = [...new Uint8Array(await crypto.subtle.digest('SHA-256', wasmBytes))].map(n => n.toString(16).padStart(2, '0')).join('');
  evidence.actualImports = imports;
  const catalog = JSON.parse(decoder.decode(await api('/api/catalog', MiB)));
  permissions = catalog.permissions;
  $('process').replaceChildren(...catalog.items.map(item => option(item.key, item.caption)));
  await load();
});
