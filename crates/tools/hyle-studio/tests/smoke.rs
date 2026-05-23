use hyle_sole::{SoleModule, SoleWorld};
use hyle_studio::ViewerScaffold;

#[test]
fn viewer_tracks_attached_module() {
    let mut viewer = ViewerScaffold::default();

    viewer.attach_module(SoleModule {
        version: "0.1".to_owned(),
        world: SoleWorld {
            dimensions: 2,
            cell: "Square".to_owned(),
        },
        ranges: Vec::new(),
        models: Vec::new(),
        inputs: Vec::new(),
        rules: Vec::new(),
    });

    assert!(viewer.has_module());
}
