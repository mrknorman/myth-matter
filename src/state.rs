use crate::{
    MaterialIceParameters, MaterialRarityContext, MaterialRegolithOrigin,
    MaterialRegolithParameters, MaterialSoilElementBindings, MaterialSoilParameters,
    MaterialStoneGenesis, MaterialStoneLithology, MaterialStoneParameters, MaterialVariant,
    MaterialWaterParameters, material_derive_regolith_origin, material_derive_stone_lithology,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialDerivationState {
    pub selected_variant: MaterialVariant,
    pub ice_params: MaterialIceParameters,
    pub water_params: MaterialWaterParameters,
    pub selected_stone_genesis: MaterialStoneGenesis,
    pub stone_params: MaterialStoneParameters,
    pub regolith_params: MaterialRegolithParameters,
    pub soil_params: MaterialSoilParameters,
    pub rarity_context: MaterialRarityContext,
}

impl Default for MaterialDerivationState {
    fn default() -> Self {
        Self {
            selected_variant: MaterialVariant::Wood,
            ice_params: MaterialIceParameters::default(),
            water_params: MaterialWaterParameters::default(),
            selected_stone_genesis: MaterialStoneGenesis::default(),
            stone_params: MaterialStoneParameters::default(),
            regolith_params: MaterialRegolithParameters::default(),
            soil_params: MaterialSoilParameters::default(),
            rarity_context: MaterialRarityContext::default(),
        }
    }
}

impl MaterialDerivationState {
    pub fn derived_regolith_origin(self) -> MaterialRegolithOrigin {
        material_derive_regolith_origin(self.soil_params, self.regolith_params)
    }

    pub fn soil_element_bindings(self) -> MaterialSoilElementBindings {
        MaterialSoilElementBindings::from_regolith_origin(self.derived_regolith_origin())
    }

    pub fn derived_stone_lithology(self) -> MaterialStoneLithology {
        material_derive_stone_lithology(self.selected_stone_genesis, self.stone_params)
    }
}
