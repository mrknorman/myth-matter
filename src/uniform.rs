use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::ShaderType},
};

use crate::{
    MaterialIceParameters, MaterialRegolithOrigin, MaterialSoilElementBindings,
    MaterialSoilParameters, MaterialStoneGenesis, MaterialStoneParameters, MaterialViewerState,
};

pub struct MaterialUniformPlugin;

#[derive(Resource, Clone, Debug, PartialEq, ExtractResource, ShaderType)]
pub struct MaterialUniform {
    pub soil_primary: Vec4,
    pub soil_secondary: Vec4,
    pub soil_modifiers: Vec4,
    pub stone_common: Vec4,
    pub stone_specific: Vec4,
    pub soil_element_primary: UVec4,
    pub soil_element_secondary: UVec4,
    pub material_metadata: UVec4,
}

impl Default for MaterialUniform {
    fn default() -> Self {
        Self::from_viewer_state(MaterialViewerState::default())
    }
}

impl MaterialUniform {
    pub fn from_soil_configuration(
        parameters: MaterialSoilParameters,
        bindings: MaterialSoilElementBindings,
        stone_genesis: MaterialStoneGenesis,
        stone_params: MaterialStoneParameters,
        derived_stone_lithology_id: u32,
        regolith_origin: MaterialRegolithOrigin,
        ice_params: MaterialIceParameters,
    ) -> Self {
        Self {
            soil_primary: Vec4::new(
                parameters.sand_pct,
                parameters.silt_pct,
                parameters.clay_pct,
                parameters.gravel_pct,
            ),
            soil_secondary: Vec4::new(
                parameters.pebble_pct,
                parameters.tephra_pct,
                parameters.organic_pct,
                parameters.water_pct,
            ),
            soil_modifiers: Vec4::new(
                parameters.iron_oxide_pct,
                ice_params.compaction(),
                ice_params.air_content(),
                ice_params.impurity(),
            ),
            stone_common: Vec4::new(
                stone_params.vein_content,
                stone_params.fracture_density,
                stone_params.oxide_staining,
                stone_params.weathering,
            ),
            stone_specific: Vec4::new(
                stone_params.primary,
                stone_params.secondary,
                stone_params.tertiary,
                stone_params.quaternary,
            ),
            soil_element_primary: UVec4::new(
                bindings.sand.shader_id(),
                bindings.silt.shader_id(),
                bindings.clay.shader_id(),
                bindings.gravel.shader_id(),
            ),
            soil_element_secondary: UVec4::new(
                bindings.pebble.shader_id(),
                bindings.tephra.shader_id(),
                bindings.organic.shader_id(),
                bindings.iron_oxide.shader_id(),
            ),
            material_metadata: UVec4::new(
                stone_genesis.shader_id(),
                regolith_origin.shader_id(),
                derived_stone_lithology_id,
                ice_params.impurity_element.shader_id(),
            ),
        }
    }

    pub fn from_viewer_state(viewer_state: MaterialViewerState) -> Self {
        Self::from_soil_configuration(
            viewer_state.soil_params,
            viewer_state.soil_element_bindings(),
            viewer_state.selected_stone_genesis,
            viewer_state.stone_params,
            viewer_state.derived_stone_lithology().shader_id(),
            viewer_state.derived_regolith_origin(),
            viewer_state.ice_params,
        )
    }
}

impl Plugin for MaterialUniformPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialUniform>()
            .add_systems(PostUpdate, update_material_uniform);
    }
}

fn update_material_uniform(
    mut uniform: ResMut<MaterialUniform>,
    viewer_state: Option<Res<MaterialViewerState>>,
) {
    let next = viewer_state
        .as_deref()
        .copied()
        .map(MaterialUniform::from_viewer_state)
        .unwrap_or_default();
    if *uniform != next {
        *uniform = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_uniform_packs_soil_parameters_and_element_bindings() {
        let uniform = MaterialUniform::from_soil_configuration(
            MaterialSoilParameters {
                sand_pct: 0.30,
                silt_pct: 0.25,
                clay_pct: 0.20,
                gravel_pct: 0.15,
                pebble_pct: 0.07,
                tephra_pct: 0.05,
                organic_pct: 0.03,
                water_pct: 0.08,
                iron_oxide_pct: 0.05,
            },
            MaterialSoilElementBindings::default(),
            MaterialStoneGenesis::Metamorphic,
            MaterialStoneParameters {
                vein_content: 0.42,
                fracture_density: 0.18,
                oxide_staining: 0.11,
                weathering: 0.08,
                primary: 0.72,
                secondary: 0.58,
                tertiary: 0.46,
                quaternary: 0.83,
            },
            1,
            MaterialRegolithOrigin::Tephric,
            MaterialIceParameters::new(0.18, 0.88, 0.04, crate::MaterialElement::Tephra),
        );
        assert_eq!(uniform.soil_primary, Vec4::new(0.30, 0.25, 0.20, 0.15));
        assert_eq!(uniform.soil_secondary, Vec4::new(0.07, 0.05, 0.03, 0.08));
        assert_eq!(uniform.soil_modifiers, Vec4::new(0.05, 0.18, 0.88, 0.04));
        assert_eq!(uniform.stone_common, Vec4::new(0.42, 0.18, 0.11, 0.08));
        assert_eq!(uniform.stone_specific, Vec4::new(0.72, 0.58, 0.46, 0.83));
        assert_eq!(uniform.soil_element_primary, UVec4::new(0, 0, 0, 0));
        assert_eq!(uniform.soil_element_secondary, UVec4::new(0, 9, 10, 11));
        assert_eq!(uniform.material_metadata, UVec4::new(2, 9, 1, 9));
    }
}
