# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Reviewed RP-01 fixture declarations; never infer or apply user edits."""
import re

from .rp01_source import quote

# ADR-0038 native requirements at revision 559d82d...: exactly 72 missing pairs.
MISSING_LINKS = {
    'checks_request': ('checks_request_step',),
    'correction_reasons': ('correct_claim',),
    'cycle_checks': ('choose_check', 'correct_claim', 'duplicate_review', 'finance_review', 'manager_review'),
    'disposition': ('check_rejection', 'choose_paid', 'choose_payment', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
    'disposition_reasons': ('check_rejection', 'choose_paid', 'choose_payment', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
    'duplicate_result': ('correct_claim', 'finance_review', 'manager_review'),
    'finance_evidence': ('check_rejection', 'choose_paid', 'choose_payment', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
    'finance_result': ('correct_claim',),
    'manager_evidence': ('check_rejection', 'choose_paid', 'choose_payment', 'correct_claim', 'finance_review', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
    'manager_result': ('correct_claim', 'finance_review'),
    'notice_request': ('notice_request_step',),
    'payable_reference': ('notice_request_step', 'publish_outcome'),
    'payment_deadline': ('payment_request_step',),
    'payment_due': ('check_rejection', 'checks_request_step', 'choose_check', 'choose_paid', 'choose_payment', 'correct_claim', 'duplicate_review', 'finance_review', 'manager_review', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
    'payment_request': ('payment_request_step',),
    'recorded_notice': ('publish_outcome',),
    'recorded_payment': ('choose_paid', 'notice_request_step', 'publish_outcome'),
    'review_needed': ('check_rejection', 'checks_request_step', 'choose_check', 'choose_paid', 'choose_payment', 'correct_claim', 'duplicate_review', 'finance_review', 'manager_review', 'notice_request_step', 'payment_request_step', 'publish_outcome'),
}


def declare_links(source, ledger):
    """Add the frozen links to the generated fixture's uniquely named cells.

    The ledger supplies qualified readable references without changing identities.
    This deliberately refuses a changed baseline instead of guessing an edit.
    """
    declarations = {d['id']: d for d in ledger['declarations']}
    for cell, sites in MISSING_LINKS.items():
        assert declarations[cell]['kind'] == 'data'
        references = []
        for site in sites:
            node = declarations[site]
            assert node['kind'] == 'node'
            chain = []
            scope = node['scope']
            while scope:
                chain.append(scope)
                scope = declarations[scope]['scope']
            assert declarations[cell]['scope'] in chain
            references.append(quote(node['name']) + ''.join(' in ' + quote(declarations[s]['name']) for s in chain))
        pattern = r'(^Keep ' + re.escape(quote(declarations[cell]['name'])) + r' as [^\n]*\nSensitivity [^\n]*\nCapabilities [^\n]*\. Invalidate )empty(\. Initially [^\n]* End data\.)'
        source, count = re.subn(pattern, lambda m: m[1] + '[' + ', '.join(references) + ']' + m[2], source, flags=re.MULTILINE)
        assert count == 1, ('fixture invalidation baseline changed', cell, count)
    return source
