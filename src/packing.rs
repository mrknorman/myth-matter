use crate::{
    MaterialDerivationState, MaterialIceParameters, MaterialRegolithOrigin,
    MaterialSoilElementBindings, MaterialSoilParameters, MaterialStoneGenesis,
    MaterialStoneParameters,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialPackedFields {
    pub soil_primary: [f32; 4],
    pub soil_secondary: [f32; 4],
    pub soil_modifiers: [f32; 4],
    pub stone_common: [f32; 4],
    pub stone_specific: [f32; 4],
    pub soil_element_primary: [u32; 4],
    pub soil_element_secondary: [u32; 4],
    pub material_metadata: [u32; 4],
}

impl MaterialPackedFields {
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
            soil_primary: [
                parameters.sand_pct,
                parameters.silt_pct,
                parameters.clay_pct,
                parameters.gravel_pct,
            ],
            soil_secondary: [
                parameters.pebble_pct,
                parameters.tephra_pct,
                parameters.organic_pct,
                parameters.water_pct,
            ],
            soil_modifiers: [
                parameters.iron_oxide_pct,
                ice_params.compaction(),
                ice_params.air_content(),
                ice_params.impurity(),
            ],
            stone_common: [
                stone_params.vein_content,
                stone_params.fracture_density,
                stone_params.oxide_staining,
                stone_params.weathering,
            ],
            stone_specific: [
                stone_params.primary,
                stone_params.secondary,
                stone_params.tertiary,
                stone_params.quaternary,
            ],
            soil_element_primary: [
                bindings.sand.shader_id(),
                bindings.silt.shader_id(),
                bindings.clay.shader_id(),
                bindings.gravel.shader_id(),
            ],
            soil_element_secondary: [
                bindings.pebble.shader_id(),
                bindings.tephra.shader_id(),
                bindings.organic.shader_id(),
                bindings.iron_oxide.shader_id(),
            ],
            material_metadata: [
                stone_genesis.shader_id(),
                regolith_origin.shader_id(),
                derived_stone_lithology_id,
                ice_params.impurity_element.shader_id(),
            ],
        }
    }

    pub fn from_derivation_state(viewer_state: MaterialDerivationState) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MaterialElement;

    #[test]
    fn material_packed_fields_pack_soil_parameters_and_element_bindings() {
        let fields = MaterialPackedFields::from_soil_configuration(
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
            MaterialIceParameters::new(0.18, 0.88, 0.04, MaterialElement::Tephra),
        );

        assert_eq!(fields.soil_primary, [0.30, 0.25, 0.20, 0.15]);
        assert_eq!(fields.soil_secondary, [0.07, 0.05, 0.03, 0.08]);
        assert_eq!(fields.soil_modifiers, [0.05, 0.18, 0.88, 0.04]);
        assert_eq!(fields.stone_common, [0.42, 0.18, 0.11, 0.08]);
        assert_eq!(fields.stone_specific, [0.72, 0.58, 0.46, 0.83]);
        assert_eq!(fields.soil_element_primary, [0, 0, 0, 0]);
        assert_eq!(fields.soil_element_secondary, [0, 9, 10, 11]);
        assert_eq!(fields.material_metadata, [2, 9, 1, 9]);
    }
}
