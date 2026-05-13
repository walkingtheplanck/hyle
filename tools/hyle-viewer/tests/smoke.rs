use hyle_ir::ModuleIr;
use hyle_viewer::ViewerScaffold;

#[test]
fn viewer_tracks_attached_module() {
    let mut viewer = ViewerScaffold::default();

    viewer.attach_module(ModuleIr::default());

    assert!(viewer.has_module());
}
