# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Fictional exact two-line order contracts; never provider/authority evidence."""
from .rp01_resources import (
    TEXT, INTEGER, AMOUNT, record, option, variant, literal, field, binary,
    negate, every, nonempty, encoded, pin, nominal,
)

TYPES_ID = 'urn:choreoform:benchmarks:rp03:types:0.1.0'
PEOPLE = ['Customer', 'Commerce operator', 'Warehouse operator', 'Support agent', 'Clock observer']
ROLES = ['customer', 'commerce', 'warehouse', 'support', 'clock']
# Each consequential operation has its own declared provider/effect boundary.
SPECS = [
    ('reserve_a', 'Reserve first line', 'Reservation A evidence', ['reserved','unavailable'], 'compensatable'),
    ('reserve_b', 'Reserve second line', 'Reservation B evidence', ['reserved','unavailable'], 'compensatable'),
    ('authorize', 'Authorize order payment', 'Authorization evidence', ['authorized','declined'], 'compensatable'),
    ('release_a', 'Release first reservation', 'Release A evidence', ['released','problem'], 'irreversible'),
    ('release_b', 'Release second reservation', 'Release B evidence', ['released','problem'], 'irreversible'),
    ('void', 'Void authorization', 'Void evidence', ['voided','expired','problem'], 'irreversible'),
    ('pack_a', 'Pack first line', 'Packing A evidence', ['packed','declined'], 'compensatable'),
    ('pack_b', 'Pack second line', 'Packing B evidence', ['packed','declined'], 'compensatable'),
    ('label_a', 'Purchase first label', 'Label A evidence', ['labelled','declined'], 'compensatable'),
    ('label_b', 'Purchase second label', 'Label B evidence', ['labelled','declined'], 'compensatable'),
    ('handover_a', 'Hand over first parcel', 'Handover A evidence', ['handed','declined'], 'irreversible'),
    ('handover_b', 'Hand over second parcel', 'Handover B evidence', ['handed','declined'], 'irreversible'),
    ('deliver_a', 'Observe first delivery', 'Delivery A evidence', ['delivered','exception'], 'read-only'),
    ('deliver_b', 'Observe second delivery', 'Delivery B evidence', ['delivered','exception'], 'read-only'),
    ('stop_a', 'Stop first warehouse work', 'Stop A evidence', ['stopped','cutoff','problem'], 'irreversible'),
    ('stop_b', 'Stop second warehouse work', 'Stop B evidence', ['stopped','cutoff','problem'], 'irreversible'),
    ('void_label_a', 'Void first label', 'Label void A evidence', ['voided','problem'], 'irreversible'),
    ('void_label_b', 'Void second label', 'Label void B evidence', ['voided','problem'], 'irreversible'),
    ('capture', 'Capture committed amount', 'Capture evidence', ['captured','problem'], 'irreversible'),
    ('refund', 'Refund captured amount', 'Refund evidence', ['refunded','problem'], 'irreversible'),
    ('intercept_a', 'Request first intercept', 'Intercept A evidence', ['intercepted','unstoppable','problem'], 'irreversible'),
    ('intercept_b', 'Request second intercept', 'Intercept B evidence', ['intercepted','unstoppable','problem'], 'irreversible'),
    ('notify', 'Notify customer of order', 'Notification evidence', ['notified','problem'], 'irreversible'),
]
CAPABILITIES = [name for _,name,*_ in SPECS]


def protection(capabilities=None):
    return {'policy':'order_access', 'sensitivity':'order', 'purposes':{'fulfillment':True},
            'participants':{r+'_actor':True for r in ROLES},
            'capabilities':{identity:True for identity in (capabilities if capabilities is not None else [key+'_capability' for key,*_ in SPECS])}}


def types():
    ref = record(identity=TEXT, revision=TEXT, digest=TEXT, provenance=TEXT)
    line = record(**{'line ID':TEXT, 'revision':TEXT, 'sku':TEXT, 'quantity':INTEGER,
                     'amount':AMOUNT, 'currency':TEXT, 'warehouse':TEXT})
    order = record(**{'order ID':TEXT, 'revision':TEXT, 'customer reference':TEXT,
        'contact reference':ref, 'address reference':ref, 'payment reference':TEXT,
        'terms reference':ref, 'pricing reference':ref, 'delivery choice':TEXT,
        'first line':line, 'second line':line, 'amount':AMOUNT, 'currency':TEXT})
    value={'op':'param','name':'value'}
    eq=lambda a,b:binary('eq',a,b)
    amount=lambda v:negate(binary('lt',v,literal(AMOUNT,'0')))
    checks=[nonempty(field(value,k)) for k in ['order ID','revision','customer reference','payment reference','delivery choice']]
    for key in ['contact reference','address reference','terms reference','pricing reference']:
        checks += [nonempty(field(field(value,key),k)) for k in ref['fields']]
    checks.append({'op':'contains','list':literal({'kind':'list','element':TEXT},['separate shipments','offer partial']),'value':field(value,'delivery choice')})
    currency=field(value,'currency')
    checks += [{'op':'contains','list':literal({'kind':'list','element':TEXT},['EUR','USD']), 'value':currency}, amount(field(value,'amount'))]
    for key in ['first line','second line']:
        item=field(value,key)
        checks += [nonempty(field(item,k)) for k in ['line ID','revision','sku','warehouse']]
        checks += [binary('lt',literal(INTEGER,'0'),field(item,'quantity')), amount(field(item,'amount')), eq(field(item,'currency'),currency)]
    checks += [negate(eq(field(field(value,'first line'),'line ID'),field(field(value,'second line'),'line ID'))),
               negate(eq(field(field(value,'first line'),'warehouse'),field(field(value,'second line'),'warehouse'))),
               eq(field(value,'amount'),binary('add',field(field(value,'first line'),'amount'),field(field(value,'second line'),'amount')))]
    definitions={'Order':{'representation':order,'invariant':every(checks)}}
    evidence=record(**{'operation':TEXT,'phase':TEXT,'order ID':TEXT,'order revision':TEXT,
        'scope key':TEXT,'scope revision':TEXT,'request revision':TEXT,'amount':AMOUNT,
        'currency':TEXT,'basis reference':TEXT,'reference':TEXT,'provider event':TEXT,
        'address revision':TEXT,'expiry':INTEGER,'reason':TEXT, 'subject reference':option(TEXT),'target quantity':option(INTEGER),'target warehouse':option(TEXT)})
    for key,_,name,tags,_ in SPECS:
        checks=[nonempty(field(value,k)) for k in ['order ID','order revision','scope key','scope revision','request revision','basis reference','reference','provider event','address revision']]
        checks += [eq(field(value,'operation'),literal(TEXT,key)),
                   {'op':'contains','list':literal({'kind':'list','element':TEXT},tags),'value':field(value,'phase')},
                   {'op':'contains','list':literal({'kind':'list','element':TEXT},['EUR','USD']),'value':field(value,'currency')},
                   amount(field(value,'amount')), negate(binary('lt',field(value,'expiry'),literal(INTEGER,'0')))]
        if key.endswith(('_a','_b')):
            checks += [{'op':'isSome','value':field(value,'target quantity')},
                binary('lt',literal(INTEGER,'0'),{'op':'unwrap','value':field(value,'target quantity')}),
                {'op':'isSome','value':field(value,'target warehouse')},
                nonempty({'op':'unwrap','value':field(value,'target warehouse')})]
        positive = [tag for tag in tags if tag not in ['unavailable','declined','problem','exception']]
        checks.append({'op':'if','condition':{'op':'contains','list':literal({'kind':'list','element':TEXT},positive),'value':field(value,'phase')},
            'then':every([{'op':'isSome','value':field(value,'subject reference')},
                nonempty({'op':'unwrap','value':field(value,'subject reference')})]),
            'else':nonempty(field(value,'reason'))})
        definitions[name]={'representation':evidence,'invariant':every(checks)}
    decision=record(**{'order ID':TEXT,'order revision':TEXT,'principal':TEXT,'assignment':TEXT,'choice':TEXT,'reason':TEXT})
    definitions['Partial stock decision']={'representation':decision,'invariant':every([
        *[nonempty(field(value,k)) for k in decision['fields']],
        {'op':'contains','list':literal({'kind':'list','element':TEXT},['ship available','cancel all']), 'value':field(value,'choice')},
    ])}
    direction=record(**{'order ID':TEXT,'order revision':TEXT,'principal':TEXT,'assignment':TEXT,'action':TEXT,'reason':TEXT})
    definitions['Settlement direction']={'representation':direction,'invariant':every([
        *[nonempty(field(value,k)) for k in direction['fields']],
        {'op':'contains','list':literal({'kind':'list','element':TEXT},['request stop','retain']), 'value':field(value,'action')},
    ])}
    ownership=record(**{'order ID':TEXT,'order revision':TEXT,'principal':TEXT,'assignment':TEXT,
        'receiver occurrence':TEXT,'obligations reference':ref,'reason':TEXT})
    definitions['Exception ownership evidence']={'representation':ownership,'invariant':every([
        *[nonempty(field(value,k)) for k in ['order ID','order revision','principal','assignment','receiver occurrence','reason']],
        *[nonempty(field(field(value,'obligations reference'),k)) for k in ref['fields']],
    ])}
    document={'kind':'named-types','version':'0.1.0','id':TYPES_ID,'definitions':definitions}
    raw=encoded(document)
    return document,raw,pin(TYPES_ID,raw)


def instruction(type_pin):
    return record(**{'order':nominal(type_pin,'Order'),'scope key':TEXT,'scope revision':TEXT,
        'request revision':TEXT,'amount':AMOUNT,'currency':TEXT,'basis reference':TEXT,'address revision':TEXT,
        'target quantity':option(INTEGER),'target warehouse':option(TEXT)})


def providers(type_pin, *, capabilities=None):
    result={}
    for key,_,name,tags,effect in SPECS:
        identity=f'urn:choreoform:benchmarks:rp03:{key.replace("_","-")}:0.1.0'
        output=variant({tag:nominal(type_pin,name) for tag in tags})
        observations={'result-'+tag:{'state':'succeeded','result':output,'outcome':tag} for tag in tags}
        observations.update({state:{'state':state,'result':None,'outcome':None} for state in ['acknowledged','rejected','cancelled','unknown','no-effect']})
        doc={'kind':'provider-contract','version':'0.1.0','id':identity,'input':instruction(type_pin),'output':output,
            'outcomes':{tag:True for tag in tags},'resultProtection':protection(capabilities),'observations':observations,
            'idempotency':'none' if effect=='read-only' else 'stable-key','effectClass':effect}
        raw=encoded(doc);result[key]=(doc,raw,pin(identity,raw))
    return result


def inputs():
    ref=lambda k:{'identity':'urn:choreoform:benchmarks:rp03:fictional-'+k,'revision':k+'-v1',
                  'digest':'sha256:'+('1' if k=='address' else '2')*64,'provenance':'synthetic reference only, not verified evidence'}
    order={'order ID':'rp03-fictional-order','revision':'order-v1','customer reference':'fictional-customer',
        'contact reference':ref('contact'),'address reference':ref('address'),'payment reference':'fictional-provider-token-reference',
        'terms reference':ref('terms'),'pricing reference':ref('pricing'),'delivery choice':'separate shipments',
        'first line':{'line ID':'line-a','revision':'line-a-v1','sku':'fictional-sku-a','quantity':'1','amount':'12000','currency':'EUR','warehouse':'fictional-warehouse-a'},
        'second line':{'line ID':'line-b','revision':'line-b-v1','sku':'fictional-sku-b','quantity':'2','amount':'8000','currency':'EUR','warehouse':'fictional-warehouse-b'},
        'amount':'20000','currency':'EUR'}
    return {'order':order}


def clock_resources(*, capabilities=None):
    source_id='urn:choreoform:benchmarks:rp03:clock-source:0.1.0'
    source={'kind':'provider-contract','version':'0.1.0','id':source_id,'input':record(),'output':INTEGER,
        'outcomes':{'tick':True},'resultProtection':protection(capabilities),
        'observations':{'tick':{'state':'succeeded','result':INTEGER,'outcome':'tick'},'unknown':{'state':'unknown','result':None,'outcome':None}},
        'idempotency':'none','effectClass':'read-only'}
    raw=encoded(source);source_pin=pin(source_id,raw)
    clock={'kind':'clock','version':'0.1.0','id':'urn:choreoform:benchmarks:rp03:clock:0.1.0',
        'profile':{'id':'urn:choreoform:clock-profile:instance-microseconds:0.1.0','revision':'sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9'},'source':source_pin}
    cr=encoded(clock)
    return {'clock-source':(source,raw,source_pin),'clock':(clock,cr,pin(clock['id'],cr))}
