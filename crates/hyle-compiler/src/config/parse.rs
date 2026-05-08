use kdl::{KdlDocument, KdlNode, KdlValue};

use crate::config::ast::{
    BoundsConfig, ConfigAst, FieldConfig, HyleDirective, InputConfig, LatticeConfig, ModelConfig,
    NeighborhoodConfig, PipelineConfig, RunConfig, SimulationConfig, SpacingConfig, StageConfig,
    WorldConfig,
};
use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::source::SourceFile;

/// Parses a KDL config source into a typed config AST.
///
/// This currently supports the scaffolded Hyle configuration layout used by the
/// repository reset and extracts the structured information needed by later
/// compiler stages.
pub fn parse_config(source: &SourceFile) -> Result<ConfigAst, DiagnosticReport> {
    if source.contents.trim().is_empty() {
        let mut report = DiagnosticReport::new();
        report.push(Diagnostic::error(
            Some(source.path.clone()),
            "config source is empty",
        ));
        return Err(report);
    }

    let document = source
        .contents
        .parse::<KdlDocument>()
        .map_err(|error| report_error(source, error.to_string()))?;

    parse_document(source, &document)
}

fn parse_document(
    source: &SourceFile,
    document: &KdlDocument,
) -> Result<ConfigAst, DiagnosticReport> {
    let hyle_node = required_single_node(document, "hyle", "document")
        .map_err(|message| report_error(source, message))?;
    let world_node = required_single_node(document, "world", "document")
        .map_err(|message| report_error(source, message))?;

    let hyle = parse_hyle(hyle_node).map_err(|message| report_error(source, message))?;
    let world = parse_world(world_node).map_err(|message| report_error(source, message))?;
    let lattices = named_nodes(document, "lattice")
        .map(parse_lattice)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|message| report_error(source, message))?;
    let neighborhoods = named_nodes(document, "neighborhood")
        .map(parse_neighborhood)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|message| report_error(source, message))?;
    let models = named_nodes(document, "model")
        .map(parse_model)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|message| report_error(source, message))?;
    let simulations = named_nodes(document, "simulation")
        .map(parse_simulation)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|message| report_error(source, message))?;

    Ok(ConfigAst {
        source_path: source.path.clone(),
        hyle,
        world,
        lattices,
        neighborhoods,
        models,
        simulations,
    })
}

fn parse_hyle(node: &KdlNode) -> Result<HyleDirective, String> {
    Ok(HyleDirective {
        version: property_string(node, "version")?,
    })
}

fn parse_world(node: &KdlNode) -> Result<WorldConfig, String> {
    let dimensions = required_single_node(children(node, "world")?, "dimensions", "world")?;
    Ok(WorldConfig {
        dimensions: argument_u32(dimensions, 0, "dimensions")?,
    })
}

fn parse_lattice(node: &KdlNode) -> Result<LatticeConfig, String> {
    let spacing = required_single_node(children(node, "lattice")?, "spacing", "lattice")?;

    Ok(LatticeConfig {
        name: argument_string(node, 0, "lattice")?,
        cell: property_string(node, "cell")?,
        spacing: SpacingConfig {
            x: argument_f64(spacing, 0, "spacing")?,
            y: argument_f64(spacing, 1, "spacing")?,
            z: argument_f64(spacing, 2, "spacing")?,
        },
    })
}

fn parse_neighborhood(node: &KdlNode) -> Result<NeighborhoodConfig, String> {
    let child_document = children(node, "neighborhood")?;
    let radius = required_single_node(child_document, "radius", "neighborhood")?;
    let center = required_single_node(child_document, "center", "neighborhood")?;
    let metric = required_single_node(child_document, "metric", "neighborhood")?;

    Ok(NeighborhoodConfig {
        name: argument_string(node, 0, "neighborhood")?,
        radius: argument_u32(radius, 0, "radius")?,
        center: argument_bool(center, 0, "center")?,
        metric: argument_string(metric, 0, "metric")?,
    })
}

fn parse_model(node: &KdlNode) -> Result<ModelConfig, String> {
    let fields = named_nodes(children(node, "model")?, "field")
        .map(parse_field)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ModelConfig {
        name: argument_string(node, 0, "model")?,
        lattice: property_string(node, "on")?,
        fields,
    })
}

fn parse_field(node: &KdlNode) -> Result<FieldConfig, String> {
    let child_document = children(node, "field")?;
    let default_node = required_single_node(child_document, "default", "field")?;
    let bounds_node = required_single_node(child_document, "bounds", "field")?;
    let storage_node = required_single_node(child_document, "storage", "field")?;

    Ok(FieldConfig {
        name: argument_string(node, 0, "field")?,
        field_type: property_string(node, "type")?,
        default: argument_f64(default_node, 0, "default")?,
        bounds: BoundsConfig {
            min: argument_f64(bounds_node, 0, "bounds")?,
            max: argument_f64(bounds_node, 1, "bounds")?,
        },
        storage: argument_string(storage_node, 0, "storage")?,
    })
}

fn parse_simulation(node: &KdlNode) -> Result<SimulationConfig, String> {
    let child_document = children(node, "simulation")?;
    let use_models = required_single_node(child_document, "use-models", "simulation")?;
    let pipeline = required_single_node(child_document, "pipeline", "simulation")?;
    let inputs = named_nodes(child_document, "input")
        .map(parse_input)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SimulationConfig {
        name: argument_string(node, 0, "simulation")?,
        use_models: string_arguments(use_models, "use-models")?,
        inputs,
        pipeline: parse_pipeline(pipeline)?,
    })
}

fn parse_input(node: &KdlNode) -> Result<InputConfig, String> {
    let default_node = required_single_node(children(node, "input")?, "default", "input")?;

    Ok(InputConfig {
        name: argument_string(node, 0, "input")?,
        input_type: property_string(node, "type")?,
        default: argument_f64(default_node, 0, "default")?,
    })
}

fn parse_pipeline(node: &KdlNode) -> Result<PipelineConfig, String> {
    let stages = named_nodes(children(node, "pipeline")?, "stage")
        .map(parse_stage)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PipelineConfig { stages })
}

fn parse_stage(node: &KdlNode) -> Result<StageConfig, String> {
    let runs = named_nodes(children(node, "stage")?, "run")
        .map(parse_run)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(StageConfig {
        name: argument_string(node, 0, "stage")?,
        runs,
    })
}

fn parse_run(node: &KdlNode) -> Result<RunConfig, String> {
    let child_document = children(node, "run")?;
    let model = required_single_node(child_document, "model", "run")?;
    let neighborhood = required_single_node(child_document, "neighborhood", "run")?;

    Ok(RunConfig {
        name: argument_string(node, 0, "run")?,
        model: argument_string(model, 0, "model")?,
        neighborhood: argument_string(neighborhood, 0, "neighborhood")?,
    })
}

fn named_nodes<'a>(
    document: &'a KdlDocument,
    name: &'a str,
) -> impl Iterator<Item = &'a KdlNode> + 'a {
    document
        .nodes()
        .iter()
        .filter(move |node| node.name().value() == name)
}

fn required_single_node<'a>(
    document: &'a KdlDocument,
    name: &'a str,
    context: &str,
) -> Result<&'a KdlNode, String> {
    let mut matches = named_nodes(document, name);
    let first = matches
        .next()
        .ok_or_else(|| format!("missing `{name}` node in {context}"))?;

    if matches.next().is_some() {
        return Err(format!("duplicate `{name}` node in {context}"));
    }

    Ok(first)
}

fn children<'a>(node: &'a KdlNode, context: &str) -> Result<&'a KdlDocument, String> {
    node.children()
        .ok_or_else(|| format!("`{}` node is missing its children block", context))
}

fn property_string(node: &KdlNode, key: &str) -> Result<String, String> {
    match node.get(key) {
        Some(KdlValue::String(value)) => Ok(value.clone()),
        Some(_) => Err(format!(
            "`{}` property on `{}` must be a string",
            key,
            node.name().value()
        )),
        None => Err(format!(
            "missing `{}` property on `{}`",
            key,
            node.name().value()
        )),
    }
}

fn argument_string(node: &KdlNode, index: usize, context: &str) -> Result<String, String> {
    match node.get(index) {
        Some(KdlValue::String(value)) => Ok(value.clone()),
        Some(_) => Err(format!("argument {index} on `{context}` must be a string")),
        None => Err(format!("missing argument {index} on `{context}`")),
    }
}

fn argument_u32(node: &KdlNode, index: usize, context: &str) -> Result<u32, String> {
    match node.get(index) {
        Some(KdlValue::Integer(value)) => u32::try_from(*value)
            .map_err(|_| format!("argument {index} on `{context}` must fit into u32")),
        Some(_) => Err(format!(
            "argument {index} on `{context}` must be an integer"
        )),
        None => Err(format!("missing argument {index} on `{context}`")),
    }
}

fn argument_f64(node: &KdlNode, index: usize, context: &str) -> Result<f64, String> {
    match node.get(index) {
        Some(KdlValue::Float(value)) => Ok(*value),
        Some(KdlValue::Integer(value)) => Ok(*value as f64),
        Some(_) => Err(format!("argument {index} on `{context}` must be numeric")),
        None => Err(format!("missing argument {index} on `{context}`")),
    }
}

fn argument_bool(node: &KdlNode, index: usize, context: &str) -> Result<bool, String> {
    match node.get(index) {
        Some(KdlValue::Bool(value)) => Ok(*value),
        Some(_) => Err(format!("argument {index} on `{context}` must be a boolean")),
        None => Err(format!("missing argument {index} on `{context}`")),
    }
}

fn string_arguments(node: &KdlNode, context: &str) -> Result<Vec<String>, String> {
    let values = node
        .entries()
        .iter()
        .enumerate()
        .map(|(index, entry)| match entry.value() {
            KdlValue::String(value) => Ok(value.clone()),
            _ => Err(format!("argument {index} on `{context}` must be a string")),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if values.is_empty() {
        return Err(format!(
            "`{context}` must contain at least one string argument"
        ));
    }

    Ok(values)
}

fn report_error(source: &SourceFile, message: String) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(Diagnostic::error(Some(source.path.clone()), message));
    report
}
