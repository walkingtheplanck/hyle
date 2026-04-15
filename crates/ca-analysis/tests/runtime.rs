use hyle_ca_analysis::analyze_step_report;
use hyle_ca_interface::{MaterialId, StepReport, TransitionCount};

#[test]
fn runtime_analysis_tracks_living_birth_and_death_counts() {
    let step_report = StepReport::new(
        3,
        5,
        vec![8, 4, 2],
        vec![
            TransitionCount {
                from: MaterialId::new(0),
                to: MaterialId::new(1),
                count: 3,
            },
            TransitionCount {
                from: MaterialId::new(1),
                to: MaterialId::new(0),
                count: 1,
            },
            TransitionCount {
                from: MaterialId::new(2),
                to: MaterialId::new(1),
                count: 1,
            },
        ],
    );

    let report = analyze_step_report(&step_report, &[MaterialId::new(1)]);

    assert_eq!(report.step, 3);
    assert_eq!(report.total_cells, 14);
    assert_eq!(report.changed_cells, 5);
    assert_eq!(report.stable_cells, 9);
    assert_eq!(report.living_cells, 4);
    assert_eq!(report.born_cells, 4);
    assert_eq!(report.died_cells, 1);
    assert_eq!(report.populations.len(), 3);
}
