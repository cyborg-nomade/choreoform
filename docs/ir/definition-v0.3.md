<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Definition IR 0.3.0: explicit handler-local history reference

This version inherits Definition IR 0.2 except for one required nullable scope
field. It is a closed wire representation, not an execution or initialized-value
admission. [ADR-0052](../decisions/0052-typed-handler-entry.md) governs the new
semantic binding; implementation and conditional proof are separate requirements.
Earlier versions, fields, meaning and revision domains remain fixed.

Every scope adds `faultInput`, exactly one of:

```
null
{history:Ref,result:DataID}
```

Ref is the inherited closed external artifact reference, with nonempty ID and
canonical sha256 revision. Result references a data declaration owned by the same
scope. A typed projection retains history ArtifactRef and result DataId separately
from ordinary inputs. No ordinary port or binding inference is added. Old scope
records without faultInput do not satisfy 0.3; older wires reject it even when null.
Wait capture remains required nullable exactly as in 0.2. All other records remain
unchanged. Unknown wire versions refuse.

The schema is [definition-0.3.schema.json](../../schemas/ir/definition-0.3.schema.json).
Schema validation checks closed shape, not reference existence, normal visibility,
local role ownership, exact resource bytes, actual source correlation or protection.
The native structural projection additionally checks data role and same-scope
ownership. Neither check can initialize a value or turn an ordinary scope into a
valid handler. Semantic binding requires exact core 0.7 and all inherited artifacts;
older selected dialects cannot silently ignore this field.

The version participates in the existing semantic hash domain along with format,
kind and complete body. Keep from_parts defaulting to 0.1; use the explicit versioned
constructor to create 0.3 transport. Constructor/decode/canonical hashes do not imply
structural, semantic, conditional entry or execution admission.
