# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Controlled RP-01 fixture adoption/baseline only, not authoring migration."""
from copy import deepcopy
from .rp01_resources import encoded, pin
from .rp01_source import CORE3

CORE4={'contract':'urn:choreoform:contracts:core-profile:0.4.0','digest':'sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3','dialect':'core'}
PROFILE={'id':'urn:choreoform:fault-source-profile:provider-observations:0.1.0','revision':'sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0'}
BINDING='payment_faults_binding'
OLD='For retries use clock "Expense clock", basis "Payable request".'
NEW=OLD[:-1]+', fault source "Payment faults".'


def adopt(source,ledger,provider):
    assert (ledger['profile'],ledger['version'],ledger['root'])==('0.6.0','0.5.0','expense')
    assert source.count(OLD)==1 and source.count('authoring "0.6.0"')==1
    assert not any(d['id']==BINDING for d in ledger['declarations'])
    ledger=deepcopy(ledger)
    artifact={'kind':'provider-fault-source','version':'0.1.0','id':'urn:choreoform:benchmarks:rp01:payment-faults:0.1.0','profile':PROFILE,'provider':provider,'emissions':{'confirmed no effect':{'observations':{'no-effect':True},'evidence':'definitive-no-effect'},'provider unknown':{'observations':{'unknown':True},'evidence':'unresolved'}}}
    raw=encoded(artifact);source_pin=pin(artifact['id'],raw)
    ledger.update(profile='0.7.0',version='0.6.0');ledger['contracts']['core']=CORE4.copy()
    ledger['declarations'].append({'id':BINDING,'kind':'resource','scope':'expense','name':'Payment faults'})
    ledger['resources'].append({'binding':BINDING,'kind':'provider-fault-source','contract':source_pin['id'],'digest':source_pin['revision']})
    return source.replace('authoring "0.6.0"','authoring "0.7.0"',1).replace(OLD,NEW,1),ledger,raw,source_pin


def historical_v6(source,ledger):
    """Exact test-only pre-adoption counterpart; never an exported migration."""
    assert (ledger['profile'],ledger['version'],ledger['root'])==('0.7.0','0.6.0','expense')
    assert source.count(NEW)==1
    ledger=deepcopy(ledger)
    ledger.update(profile='0.6.0',version='0.5.0');ledger['contracts']['core']=CORE3.copy()
    assert sum(d['id']==BINDING for d in ledger['declarations'])==1
    assert sum(r['binding']==BINDING for r in ledger['resources'])==1
    ledger['declarations']=[d for d in ledger['declarations'] if d['id']!=BINDING]
    ledger['resources']=[r for r in ledger['resources'] if r['binding']!=BINDING]
    return source.replace('authoring "0.7.0"','authoring "0.6.0"',1).replace(NEW,OLD,1),ledger
