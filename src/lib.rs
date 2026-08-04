pub mod derivation;
pub mod ice;
pub mod identity;
pub mod packing;
pub mod properties;
mod rarity;
pub mod recipes;
pub mod soil;
pub mod state;
pub mod stone;
pub mod taxonomy;
pub mod water;

pub use derivation::{
    derive_material_common_properties, derive_material_common_properties_for_state,
    derive_soil_base_common_properties, derive_soil_base_common_properties_with_origin,
    derive_stone_base_common_properties, material_derive_stone_lithology,
};
pub use ice::{
    MaterialIceForm, MaterialIceParameter, MaterialIceParameters, material_ice_anchor_weights,
};
pub use identity::{
    MATERIAL_PARAMETER_QUANTIZATION_STEPS_V1, MATERIAL_SEMANTIC_KEY_PROFILE_V1,
    MaterialContentKeyV1, MaterialSemanticKeyV1, MaterialSemanticParametersV1,
    material_quantize_unit_lane, material_snap_unit_lane,
};
pub use packing::MaterialPackedFields;
pub use properties::{
    MaterialCommonProperties, MaterialCommonProperty, MaterialContextParameter,
    MaterialDebugPropertyOverrides, MaterialMatterState, MaterialRarityContext,
};
pub use rarity::{
    material_effective_rarity_bits, material_effective_rarity_bits_for_state,
    material_rarity_budget_for_state,
};
pub use recipes::{
    MaterialOpticalClass, MaterialRecipe, MaterialRecipeParameters, MaterialRepresentativeColor,
    TERRAIN_MATERIAL_INPUT_SCHEMA_VERSION, TERRAIN_MATERIAL_RECIPE_SCHEMA_VERSION,
    TerrainBedrockClass, TerrainBedrockInputs, TerrainCoverClass, TerrainCoverInputs,
    TerrainMaterialColumnRecipe, TerrainMaterialInputs, TerrainRegolithInputs, TerrainSurfaceClass,
    TerrainSurfaceInputs, derive_ceramic_recipe, derive_material_recipe,
    derive_terrain_material_column_recipe, derive_terrain_material_column_recipe_from_surface,
};
pub use soil::{
    MaterialRegolithOrigin, MaterialRegolithParameter, MaterialRegolithParameters,
    MaterialSoilElementBindings, MaterialSoilParameter, MaterialSoilParameters,
    material_derive_regolith_origin, material_regolith_parameters_for_origin,
};
pub use state::MaterialDerivationState;
pub use stone::{
    MaterialStoneGenesis, MaterialStoneLithology, MaterialStoneParameter, MaterialStoneParameters,
};
pub use taxonomy::{
    MaterialClass, MaterialElement, MaterialOrientationAxis, MaterialPreviewLod, MaterialVariant,
};
pub use water::{
    MATERIAL_WATER_MAX_MAX_PATH_LENGTH_M, MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M,
    MaterialWaterParameter, MaterialWaterParameters, derive_water_base_common_properties,
};

pub const MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT: usize = 7;
pub const MATERIAL_SOIL_IRON_OXIDE_MAX: f32 = 0.12;
pub const MATERIAL_SOIL_MIN_WATER_CAPACITY: f32 = 0.04;
pub const MATERIAL_SOIL_MAX_WATER_CAPACITY: f32 = 0.34;

fn material_lerp_u64(a: u64, b: u64, t: f32) -> u64 {
    let t = t.clamp(0.0, 1.0);
    ((a as f32) + ((b as f32) - (a as f32)) * t).round() as u64
}

fn material_lerp_common_properties(
    a: MaterialCommonProperties,
    b: MaterialCommonProperties,
    t: f32,
) -> MaterialCommonProperties {
    MaterialCommonProperties::new(
        material_lerp_u64(a.toughness, b.toughness, t),
        material_lerp_u64(a.hardness, b.hardness, t),
        material_lerp_u64(a.compressive_strength, b.compressive_strength, t),
        material_lerp_u64(a.tensile_strength, b.tensile_strength, t),
        material_lerp_u64(a.elasticity, b.elasticity, t),
        material_lerp_u64(a.heat_capacity, b.heat_capacity, t),
        material_lerp_u64(a.melting_point, b.melting_point, t),
        material_lerp_u64(a.boiling_point, b.boiling_point, t),
        material_lerp_u64(a.thermal_conductivity, b.thermal_conductivity, t),
        material_lerp_u64(a.electrical_conductivity, b.electrical_conductivity, t),
        material_lerp_u64(a.density, b.density, t),
        material_lerp_u64(a.porosity, b.porosity, t),
        material_lerp_u64(a.ph, b.ph, t),
        if t < 0.5 { a.state } else { b.state },
    )
}

fn material_absolute_property(value: f32) -> u64 {
    value.max(0.0).round() as u64
}

fn material_add_signed_u64(value: u64, delta: i64) -> u64 {
    if delta >= 0 {
        value.saturating_add(delta as u64)
    } else {
        value.saturating_sub(delta.unsigned_abs())
    }
}

pub fn material_debug_override_abs_max(property: MaterialCommonProperty) -> i64 {
    match property {
        MaterialCommonProperty::Ph => 7,
        MaterialCommonProperty::ElectricalConductivity | MaterialCommonProperty::Porosity => 1000,
        _ => 2000,
    }
}
