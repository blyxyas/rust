use rustc_data_structures::unord::UnordSet;
use rustc_hir::find_attr;
use rustc_middle::bug;
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::sym;

pub(crate) fn inferred_outlives(tcx: TyCtxt<'_>) {
    let live_symbols = if let Ok(live_symbols) = tcx.live_symbols_and_ignored_derived_traits(()) {
        &live_symbols.final_result.live_symbols
    } else {
        &UnordSet::default()
    };

    for id in tcx.hir_free_items() {
        if !live_symbols.is_empty() && live_symbols.contains(&id.owner_id.def_id) {
            continue;
        }

        if !find_attr!(tcx, id.owner_id, RustcDumpInferredOutlives) {
            continue;
        }

        let preds = tcx.inferred_outlives_of(id.owner_id);
        let mut preds: Vec<_> = preds
            .iter()
            .map(|(pred, _)| match pred.kind().skip_binder() {
                ty::ClauseKind::RegionOutlives(p) => p.to_string(),
                ty::ClauseKind::TypeOutlives(p) => p.to_string(),
                err => bug!("unexpected clause {:?}", err),
            })
            .collect();
        preds.sort();

        let span = tcx.def_span(id.owner_id);
        let mut err = tcx.dcx().struct_span_err(span, sym::rustc_dump_inferred_outlives.as_str());
        for pred in preds {
            err.note(pred);
        }
        err.emit();
    }
}
