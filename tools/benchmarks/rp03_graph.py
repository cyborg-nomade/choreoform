# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Complete declared two-line order graph; includes unresolved proof boundaries."""
from .rp03_source import (
    Scope, new_package, foundations, service, instruction_expression,
    record, variant, option, nominal, literal, BOOLEAN, INTEGER, TEXT,
    read, raw, field, fact, text, eq, tag, payload, present, exists,
    every, negate, binary, conditional, rec, zero, quote, expr,
)
from .rp03_resources import SPECS, PEOPLE, CAPABILITIES


def build(type_pin,providers,pins,*,historical=False):
    p=new_package(historical=historical);p.guard_rules={}
    capabilities=CAPABILITIES+['Release first reservation during settlement','Release second reservation during settlement']
    root=Scope(p,'order','Order fulfillment',None,'Start acceptance',
        inputs=[('order','Order','Original order')],outputs=[('first','First line','First line disposition'),('second','Second line','Second line disposition')],
        outcomes=['recorded'],closure='Preserve order obligations',race='Order commitments',cancellation='Customer cancellation')
    p.resource('order_types','Order types','named-type',type_pin)
    for key,name,*_ in SPECS:
        p.resource(key+'_provider_binding',name+' provider','provider',pins[key])
        p.resource(key+'_terminal_binding',name+' terminal facts','provider-terminal-observation-source',pins[key+'_terminal'])
    p.resource('order_clock_binding','Order clock','clock',pins['clock'])
    foundations(root)
    order_type=nominal(type_pin,'Order')
    root.data('original_order','Original order',order_type,participants=PEOPLE,capabilities=capabilities)
    # All results are optional because a non-dispatched operation has no evidence.
    # None is not no-effect, rejection or a settled obligation.
    names_by_key={key:name for key,name,*_ in SPECS}
    for key,name,*_ in SPECS:
        out=providers[key]['output']
        root.data(key+'_record',name+' record',option(out),{'op':'none','element':out},participants=PEOPLE,capabilities=capabilities)
    instantiated={};p.instances=[]
    def instantiate(scope,identity,key):
        prior={'release_a':'reserve_a','release_b':'reserve_b','void':'authorize','refund':'capture',
            'void_label_a':'label_a','void_label_b':'label_b'}.get(key)
        previous=next(row[1] for row in SPECS if row[0]==prior) if prior else None
        compensation=None
        deadline=None
        if key in ['pack_a','pack_b','label_a','label_b','handover_a','handover_b','capture']:
            auth_expiry=field(raw(payload(present(read('Authorize order payment record')),'authorized')),'expiry')
            deadline=auth_expiry
            if key!='capture':
                reserve_name='Reserve first line' if key.endswith('_a') else 'Reserve second line'
                stock_expiry=field(raw(payload(present(read(reserve_name+' record')),'reserved')),'expiry')
                deadline=conditional(binary('lt',stock_expiry,auth_expiry),stock_expiry,auth_expiry)
        instance=key if key not in instantiated else identity.removesuffix('_call')
        capability=names_by_key[key] if instance==key else names_by_key[key]+' during settlement'
        if previous:
            # This finite graph has exactly one declaration for each prior effect.
            # Refuse an ambiguous future expansion rather than selecting by order.
            assert len(instantiated[prior])==1, prior
            prior_owner=instantiated[prior][0]
            chain=[];cursor=prior_owner
            while cursor:chain.append(cursor.name);cursor=cursor.parent
            compensation=quote('Request '+previous)+''.join(' in '+quote(part) for part in chain)
        owner=service(root,scope,key,providers[key],compensates=compensation,deadline=deadline,
            instance=instance,capability=capability,capabilities=capabilities)
        instantiated.setdefault(key,[]).append(owner)
        p.instances.append({'creator':identity,'operation':key,'instance':instance,'body':owner.id,
            'parent':scope.id,'capability':instance+'_capability'})
    dispositions=variant({'pending':record(), 'fulfilled':record(reference=TEXT),
        'cancelled':record(reason=TEXT), 'exception':record(reason=TEXT)})
    for key,name in [('first','First line'),('second','Second line')]:
        root.data(key+'_disposition',name+' disposition',dispositions,
            {'op':'variant','type':dispositions,'tag':'pending','value':rec()},participants=PEOPLE,capabilities=capabilities)
    O=read('Original order'); R=raw(O)
    def result(key): return read(names_by_key[key]+' record')
    def success(key,case): return every([exists(result(key)),eq(tag(present(result(key))),text(case))])
    def evidence(key,case): return raw(payload(present(result(key)),case))
    def reference(key,case): return present(field(evidence(key,case),'subject reference'))
    def line(which):return field(R,'first line' if which=='a' else 'second line')
    def scope_key(which):return field(line(which),'line ID') if which else field(R,'order ID')
    def scope_revision(which):return field(line(which),'revision') if which else field(R,'revision')
    def amount(which):return field(line(which),'amount') if which else field(R,'amount')
    p.guard_types={'Original order':order_type, 'First line disposition':dispositions, 'Second line disposition':dispositions,
        **{name+' record':option(providers[key]['output']) for key,name,*_ in SPECS}}
    known_type=record(**{key:option(providers[key]['output']) for key,*_ in SPECS})
    known_values=rec(**{key:result(key) for key,*_ in SPECS})
    def decision(scope,identity,name,condition,yes,no):
        p.guard_rules[identity]=condition
        scope.expression(identity+'_condition',name+' condition',BOOLEAN,condition)
        scope.step(identity,name,f'Choose by ["yes" when {quote(name+" condition")}]; otherwise "no".',[('yes',yes),('no',no)])
    def call(scope,identity,key,next_step,*,which=None,basis=None,amount_value=None):
        instantiate(scope,identity,key)
        name=names_by_key[key];local=identity+'_value';local_name=identity.replace('_',' ')+' result'
        scope.data(local,local_name,providers[key]['output'],participants=PEOPLE,capabilities=capabilities)
        instruction_value=instruction_expression(O,scope_key(which),scope_revision(which),amount_value if amount_value is not None else amount(which),
            basis if basis is not None else field(R,'payment reference'),identity+'-instruction-v1',
            {'op':'some','value':field(line(which),'quantity')} if which else {'op':'none','element':INTEGER},
            {'op':'some','value':field(line(which),'warehouse')} if which else {'op':'none','element':TEXT})
        scope.expression(identity+'_instruction',identity.replace('_',' ')+' instruction',providers[key]['input'],instruction_value)
        record_name='Record '+identity.replace('_',' ')
        scope.step(identity,identity.replace('_',' '),f'Run {quote(name+" occurrence")} with inputs ["Request" uses {quote(identity.replace("_"," ")+" instruction")}] and outputs ["Result" uses {quote(local_name)}].',[('complete',record_name)])
        scope.compute(identity+'_record',record_name,[(key+'_record',name+' record',{'op':'some','value':read(local_name)})],next_step)
    def parallel(scope,identity,name,branches,next_step):
        for label,_,_ in branches:p.symbol(identity,'branch',label)
        scope.step(identity,name,'Start together ['+', '.join(quote(label)+' runs '+quote(child.name) for label,child,_ in branches)+'] and gather at '+quote(name+' gathered')+'.')
        outcomes=', '.join('outcome '+quote(outcome)+' of '+quote(child.name) for _,child,outcome in branches)
        scope.step(identity+'_gather',name+' gathered',f'Gather from {quote(name)} when all report [{outcomes}]; for unfinished work use "Settle children".',[('gathered',next_step)])
    # Original A: reserve each distinct warehouse line and authorize independently.
    acceptance=[]
    for which in ['a','b']:
        scope=Scope(p,'accept_'+which,'Reserve '+which+' branch',root,'Reserve '+which+' instruction',outcomes=[which+'-recorded'])
        call(scope,'reserve_'+which+'_call','reserve_'+which,'Finish reservation '+which,which=which,basis=field(R,'order ID'))
        # The entry caption is the actual invocation, not its expression.
        scope.entry='reserve '+which+' call'
        scope.step('accept_'+which+'_done','Finish reservation '+which,'Finish as '+quote(which+'-recorded')+'.')
        root.child(scope);acceptance.append((which,scope,which+'-recorded'))
    auth=Scope(p,'accept_payment','Authorize branch',root,'authorize call',outcomes=['authorization-recorded'])
    call(auth,'authorize_call','authorize','Finish authorization')
    auth.step('accept_payment_done','Finish authorization','Finish as "authorization recorded".' if historical else 'Finish as "authorization-recorded".')
    root.child(auth);acceptance.append(('payment',auth,'authorization-recorded'))
    parallel(root,'start_acceptance','Start acceptance',acceptance,'Check authorization')
    authorized=success('authorize','authorized');a_reserved=success('reserve_a','reserved');b_reserved=success('reserve_b','reserved')
    decision(root,'check_authorization','Check authorization',authorized,'Check complete stock','Release and close')
    decision(root,'check_stock','Check complete stock',every([a_reserved,b_reserved]),'Start fulfillment','Check partial stock')
    decision(root,'check_partial','Check partial stock',every([binary('or',a_reserved,b_reserved),eq(field(R,'delivery choice'),text('offer partial'))]),'Ask partial stock choice','Release and close')
    # A bounded partial offer preserves the accepted original order and records a
    # separate customer decision; it does not mutate its priced lines/revisions.
    offer_type=record(order=order_type,first=BOOLEAN,second=BOOLEAN)
    decision_type=variant({case:nominal(type_pin,'Partial stock decision') for case in ['ship','cancel']})
    root.data('partial_decision','Partial stock decision',decision_type,participants=PEOPLE,capabilities=capabilities)
    root.expression('partial_offer','Partial stock offer',offer_type,rec(order=O,first=a_reserved,second=b_reserved))
    matching=literal(BOOLEAN,False)
    for case,choice in [('cancel','cancel all'),('ship','ship available')]:
        v=raw(payload(fact('result'),case))
        matching=conditional(eq(tag(fact('result')),text(case)),every([
            eq(field(v,'order ID'),field(R,'order ID')),eq(field(v,'order revision'),field(R,'revision')),eq(field(v,'choice'),text(choice)),eq(field(v,'principal'),fact('principal'))]),matching)
    root.policy('partial_work','work','Partial stock rules',f'Instructions "Offer only the unchanged available lines or cancellation; preserve original commercial terms and the new attributable choice.". Authority "Customer". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when ({expr(matching)}). Compensates none.')
    root.step('partial_choice','Ask partial stock choice','Ask "Customer" to decide using "Partial stock offer" under "Partial stock rules" and record in "Partial stock decision".', [('ship','Start fulfillment'),('cancel','Release and close')])
    # Two fixed no-port branches retain independent per-line records. No generic
    # fan-out result collection or arbitrary line count is claimed.
    preparation=[]
    for which,reserved in [('a',a_reserved),('b',b_reserved)]:
        scope=Scope(p,'prepare_'+which,'Prepare '+which+' branch',root,'Check reservation '+which,outcomes=[which+'-prepared'])
        decision(scope,'prepare_'+which+'_guard','Check reservation '+which,reserved,'pack '+which+' call','Finish preparation '+which)
        call(scope,'pack_'+which+'_call','pack_'+which,'Check packing '+which,which=which,basis=reference('reserve_'+which,'reserved'))
        decision(scope,'pack_'+which+'_guard','Check packing '+which,success('pack_'+which,'packed'),'label '+which+' call','Finish preparation '+which)
        call(scope,'label_'+which+'_call','label_'+which,'Check label '+which,which=which,basis=reference('pack_'+which,'packed'))
        decision(scope,'label_'+which+'_guard','Check label '+which,success('label_'+which,'labelled'),'handover '+which+' call','Finish preparation '+which)
        call(scope,'handover_'+which+'_call','handover_'+which,'Finish preparation '+which,which=which,basis=reference('label_'+which,'labelled'))
        scope.step('prepare_'+which+'_done','Finish preparation '+which,'Finish as '+quote(which+'-prepared')+'.')
        root.child(scope);preparation.append((which,scope,which+'-prepared'))
    parallel(root,'start_fulfillment','Start fulfillment',preparation,'Check capture commitment')
    committed=binary('add',conditional(success('handover_a','handed'),amount('a'),zero()),conditional(success('handover_b','handed'),amount('b'),zero()))
    capture_guard=every([authorized,binary('lt',zero(),committed),negate(binary('lt',field(evidence('authorize','authorized'),'amount'),committed)),
        eq(field(evidence('authorize','authorized'),'currency'),field(R,'currency'))])
    decision(root,'capture_guard','Check capture commitment',capture_guard,'capture call','Start delivery tracking')
    call(root,'capture_call','capture','Start delivery tracking',basis=reference('authorize','authorized'),amount_value=committed)
    # Actual requested handovers stay observable even if capture later returns a
    # business problem. A failed/unknown capture never hides a physical parcel.
    deliveries=[]
    for which in ['a','b']:
        scope=Scope(p,'delivery_'+which,'Delivery '+which+' branch',root,'Check handover '+which,outcomes=[which+'-delivery-recorded'])
        decision(scope,'deliver_'+which+'_guard','Check handover '+which,success('handover_'+which,'handed'),'deliver '+which+' call','Finish delivery '+which)
        call(scope,'deliver_'+which+'_call','deliver_'+which,'Finish delivery '+which,which=which,basis=reference('handover_'+which,'handed'))
        scope.step('delivery_'+which+'_branch_done','Finish delivery '+which,'Finish as '+quote(which+'-delivery-recorded')+'.')
        root.child(scope);deliveries.append((which,scope,which+'-delivery-recorded'))
    parallel(root,'start_delivery','Start delivery tracking',deliveries,'Record line dispositions')
    def disposition(which):
        return conditional(every([success('deliver_'+which,'delivered'),success('capture','captured')]),
            {'op':'variant','type':dispositions,'tag':'fulfilled','value':rec(reference=reference('deliver_'+which,'delivered'))},
            {'op':'variant','type':dispositions,'tag':'exception','value':rec(reason=text('Per-line effect/delivery/payment evidence or remaining settlement requires accountable review.'))})
    root.compute('record_dispositions','Record line dispositions',[(key+'_disposition',name+' disposition',disposition(which)) for key,name,which in [('first','First line','a'),('second','Second line','b')]],'Check exception ownership')
    # Original B and explicit customer refusal: no path here requests shipment.
    cleanup=Scope(p,'release_normal','Release order effects',root,'Check first release',outcomes=['released'])
    for which,next_name in [('a','Check second release'),('b','Check authorization release')]:
        decision(cleanup,'normal_release_'+which+'_guard','Check '+('first' if which=='a' else 'second')+' release',success('reserve_'+which,'reserved'),'normal release '+which+' call',next_name)
        call(cleanup,'normal_release_'+which+'_call','release_'+which,next_name,which=which,basis=reference('reserve_'+which,'reserved'))
    decision(cleanup,'normal_void_guard','Check authorization release',authorized,'normal void call','Finish normal release')
    call(cleanup,'normal_void_call','void','Finish normal release',basis=reference('authorize','authorized'))
    cleanup.step('normal_release_done','Finish normal release','Finish as "released".')
    root.child(cleanup)
    # Flow nodes invoke the cleanup scope; its caption is distinct from the
    # invocation node, so both identity and typed ordinary outcome stay explicit.
    root.step('run_release','Release and close','Run "Release order effects" with inputs empty and outputs empty.',[('released','Record release dispositions')])
    financial_release=binary('or',success('authorize','declined'),binary('or',success('void','voided'),success('void','expired')))
    release_assignments=[]
    for key,name,which in [('first','First line','a'),('second','Second line','b')]:
        released=binary('or',success('reserve_'+which,'unavailable'),every([success('reserve_'+which,'reserved'),success('release_'+which,'released')]))
        value=conditional(every([released,financial_release]),
            {'op':'variant','type':dispositions,'tag':'cancelled','value':rec(reason=text('No fulfillment was started on this route; the matching stock and authorization release evidence is recorded.'))},
            {'op':'variant','type':dispositions,'tag':'exception','value':rec(reason=text('Release or authorization evidence still needs accountable settlement.'))})
        release_assignments.append((key+'_disposition',name+' disposition',value))
    root.compute('record_releases','Record release dispositions',release_assignments,'Check exception ownership')
    settled=every([binary('or',eq(tag(read(name+' disposition')),text('fulfilled')),eq(tag(read(name+' disposition')),text('cancelled'))) for name in ['First line','Second line']])
    decision(root,'check_exception','Check exception ownership',settled,'notify call','Ask exception owner')
    # Original C: cancellation remains a scoped monotonic request; this is the
    # surviving settlement body, not an ordinary continuation of the cancelled root.
    settlement=Scope(p,'order_settlement','Order settlement',root,'Ask settlement direction')
    direction_type=variant({case:nominal(type_pin,'Settlement direction') for case in ['stop','retain']})
    settlement.data('settlement_direction','Settlement direction',direction_type,participants=PEOPLE,capabilities=capabilities)
    settlement.expression('settlement_direction_input','Settlement direction input',record(order=order_type,known=known_type),rec(order=O,known=known_values))
    direction=literal(BOOLEAN,False)
    for case,action in [('stop','request stop'),('retain','retain')]:
        item=raw(payload(fact('result'),case))
        direction=conditional(eq(tag(fact('result')),text(case)),every([
            eq(field(item,'order ID'),field(R,'order ID')),eq(field(item,'order revision'),field(R,'revision')),
            eq(field(item,'action'),text(action)),eq(field(item,'principal'),fact('principal'))]),direction)
    settlement.policy('settlement_direction_work','work','Settlement direction rules',f'Instructions "Choose and attribute the next recovery action; receiver entry alone neither requests cancellation nor grants new dispatch.". Authority "Support agent". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when ({expr(direction)}). Compensates none.')
    settlement.step('settlement_direction_ask','Ask settlement direction','Ask "Support agent" to decide using "Settlement direction input" under "Settlement direction rules" and record in "Settlement direction".',[('stop','Check first stop'),('retain','Ask surviving exception owner')])
    for which,next_name in [('a','Check second stop'),('b','Check refund')]:
        first='first' if which=='a' else 'second'
        decision(settlement,'stop_'+which+'_guard','Check '+first+' stop',success('pack_'+which,'packed'),'stop '+which+' call',next_name)
        call(settlement,'stop_'+which+'_call','stop_'+which,'Check '+first+' cutoff',which=which,basis=reference('pack_'+which,'packed'))
        decision(settlement,'cutoff_'+which+'_guard','Check '+first+' cutoff',success('stop_'+which,'cutoff'),'intercept '+which+' call','Check '+first+' label release')
        call(settlement,'intercept_'+which+'_call','intercept_'+which,'Check '+first+' label release',which=which,basis=reference('stop_'+which,'cutoff'))
        decision(settlement,'void_label_'+which+'_guard','Check '+first+' label release',every([success('label_'+which,'labelled'),success('stop_'+which,'stopped')]),'cancel label '+which+' call','Check '+first+' reservation release')
        call(settlement,'cancel_label_'+which+'_call','void_label_'+which,'Check '+first+' reservation release',which=which,basis=reference('label_'+which,'labelled'))
        decision(settlement,'cancel_release_'+which+'_guard','Check '+first+' reservation release',every([success('reserve_'+which,'reserved'),success('stop_'+which,'stopped')]),'cancel release '+which+' call',next_name)
        call(settlement,'cancel_release_'+which+'_call','release_'+which,next_name,which=which,basis=reference('reserve_'+which,'reserved'))
    refund_ready=every([success('capture','captured'),
        binary('lt',zero(),field(evidence('capture','captured'),'amount')),
        eq(field(evidence('capture','captured'),'amount'),committed),
        conditional(success('handover_a','handed'),success('intercept_a','intercepted'),literal(BOOLEAN,True)),
        conditional(success('handover_b','handed'),success('intercept_b','intercepted'),literal(BOOLEAN,True))])
    decision(settlement,'refund_guard','Check refund',refund_ready,'refund call','Ask surviving exception owner')
    call(settlement,'refund_call','refund','Ask surviving exception owner',basis=reference('capture','captured'),amount_value=field(evidence('capture','captured'),'amount'))
    # Caller-visible ownership claims are not actual receiver acceptance. All
    # closure gates must still check accepted ownership/all transitive obligations.
    ownership_type=variant({'owned':nominal(type_pin,'Exception ownership evidence')})
    for scope,identity,caption,next_name in [(root,'exception','Ask exception owner','notify call'),(settlement,'surviving_exception','Ask surviving exception owner','Finish surviving settlement')]:
        summary_type=record(order=order_type,first=dispositions,second=dispositions,known=known_type)
        name=caption+' input';scope.expression(identity+'_input',name,summary_type,rec(order=O,first=read('First line disposition'),second=read('Second line disposition'),known=known_values))
        scope.data(identity+'_claim',caption+' evidence',ownership_type,participants=PEOPLE,capabilities=capabilities)
        claim=raw(payload(fact('result'),'owned'))
        predicate=every([eq(field(claim,'order ID'),field(R,'order ID')),eq(field(claim,'order revision'),field(R,'revision')),eq(field(claim,'principal'),fact('principal'))])
        scope.policy(identity+'_work','work',caption+' rules',f'Instructions "Record current accountable support ownership and all outstanding obligations; a claim alone does not settle or transfer them.". Authority "Support agent". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when ({expr(predicate)}). Compensates none.')
        scope.step(identity+'_ask',caption,f'Ask "Support agent" to decide using {quote(name)} under {quote(caption+" rules")} and record in {quote(caption+" evidence")}.',[('owned',next_name)])
    settlement.step('settlement_done','Finish surviving settlement','Finish as "complete".')
    root.child(settlement)
    call(root,'notify_call','notify','Finish order record',basis=field(R,'order ID'))
    root.step('order_done','Finish order record','Finish as "recorded".')
    # Every constructed capability request contains O; each human request/completion
    # also depends on O. A revised original order withdraws those decisions/work.
    # This explicit recipe dependency does not prove any private inherited context.
    declarations={r['id']:r for r in p.ledger['declarations']}
    original_dependents=[]
    for _,(scope,name,action) in p.actions.items():
        if not action.startswith(('Request ','Ask ')):continue
        chain=[]
        while scope is not None:
            row=declarations[scope];chain.append(row['name']);scope=row['scope']
        original_dependents.append(quote(name)+''.join(' in '+quote(part) for part in chain))
    assert len(original_dependents)==29
    index=next(i for i,line in enumerate(root.declarations) if line.startswith('Keep "Original order"'))
    root.declarations[index]=root.declarations[index].replace('Invalidate empty.',
        'Invalidate ['+', '.join(original_dependents)+'].')
    source='# SPDX-FileCopyrightText: 2026 Choreoform contributors\n# SPDX-License-Identifier: MPL-2.0\n# Synthetic full two-line draft; not admitted or executable.\n'+f'Choreoform authoring "{p.ledger["profile"]}".\n'+root.render()+'\n'
    return source,p
