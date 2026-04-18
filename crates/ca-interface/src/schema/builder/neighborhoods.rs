use std::any::TypeId;

use crate::schema::{NeighborhoodRef, NeighborhoodSet, NeighborhoodSpec};
use crate::NeighborhoodId;

use super::BuildError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct NeighborhoodRegistry {
    pub(super) owner: TypeId,
    pub(super) default_neighborhood: NeighborhoodId,
    pub(super) expected_names: Vec<&'static str>,
}

/// Erase one enum-backed neighborhood set into builder-owned metadata.
pub(super) fn register_neighborhoods<N: NeighborhoodSet>(
) -> Result<NeighborhoodRegistry, BuildError> {
    Ok(NeighborhoodRegistry {
        owner: TypeId::of::<N>(),
        default_neighborhood: N::default_neighborhood().map_err(BuildError::from)?,
        expected_names: N::variants().iter().map(|value| value.label()).collect(),
    })
}

/// Match declarative neighborhood specs against the declared neighborhood set.
///
/// The builder stores neighborhood ids and names separately, so this pass
/// confirms that every declared enum entry has exactly one matching spec.
pub(super) fn validate_neighborhoods(
    registry: &NeighborhoodRegistry,
    specs: &[NeighborhoodSpec],
    default_override: Option<NeighborhoodRef>,
) -> Result<(Vec<NeighborhoodSpec>, NeighborhoodId), BuildError> {
    let mut resolved = vec![None; registry.expected_names.len()];

    for spec in specs {
        if spec.id().index() >= resolved.len() {
            return Err(BuildError::MismatchedNeighborhood(spec.name()));
        }

        let expected_name = registry.expected_names[spec.id().index()];
        if expected_name != spec.name() {
            return Err(BuildError::MismatchedNeighborhood(spec.name()));
        }
        if resolved[spec.id().index()].is_some() {
            return Err(BuildError::DuplicateNeighborhoodSpec(spec.name()));
        }
        resolved[spec.id().index()] = Some(*spec);
    }

    let mut neighborhoods = Vec::with_capacity(resolved.len());
    for (index, spec) in resolved.into_iter().enumerate() {
        let spec = spec.ok_or(BuildError::MissingNeighborhoodSpec(
            registry.expected_names[index],
        ))?;
        neighborhoods.push(spec);
    }

    let default_neighborhood = match default_override {
        Some(reference) => {
            if reference.owner() != registry.owner {
                return Err(BuildError::UnknownRuleNeighborhood(reference.label()));
            }
            reference.id().map_err(BuildError::from)?
        }
        None => registry.default_neighborhood,
    };

    Ok((neighborhoods, default_neighborhood))
}
