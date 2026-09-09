use rustc_crate_store::ForeignModule;
use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::unord::UnordSet;
use rustc_hir as hir;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::query::LocalCrate;
use rustc_middle::ty::TyCtxt;

pub(crate) fn collect(tcx: TyCtxt<'_>, LocalCrate: LocalCrate) -> FxIndexMap<DefId, ForeignModule> {
    let mut modules = FxIndexMap::default();

    let live_symbols = if let Ok(live_symbols) = tcx.live_symbols_and_ignored_derived_traits(()) {
        &live_symbols.final_result.live_symbols
    } else {
        &UnordSet::default()
    };

    // We need to collect all the `ForeignMod`, even if they are empty.
    for id in tcx.hir_free_items() {
        if !live_symbols.is_empty() && live_symbols.contains(&id.owner_id.def_id) {
            continue;
        }

        if !matches!(tcx.def_kind(id.owner_id), DefKind::ForeignMod) {
            continue;
        }

        let def_id = id.owner_id.to_def_id();
        let item = tcx.hir_item(id);

        if let hir::ItemKind::ForeignMod { abi, items } = item.kind {
            let foreign_items = items.iter().map(|it| it.owner_id.to_def_id()).collect();
            modules.insert(def_id, ForeignModule { def_id, abi, foreign_items });
        }
    }

    modules
}
