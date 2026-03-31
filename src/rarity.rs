use crate::{
    MaterialCommonProperties, MaterialDerivationState, MaterialIceParameters,
    MaterialRarityContext, MaterialRegolithOrigin, MaterialRegolithParameters,
    MaterialSoilParameters, MaterialStoneGenesis, MaterialStoneLithology, MaterialStoneParameters,
    MaterialVariant, material_absolute_property, material_ice_anchor_weights,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MaterialHiddenAspect {
    Cohesion,
    Refractoriness,
    Elasticity,
    Conductivity,
    DensityBias,
    Purity,
    Crystallinity,
    Vitality,
    Fertility,
    Ceramicity,
}

impl MaterialHiddenAspect {
    const COUNT: usize = 10;

    const fn index(self) -> usize {
        match self {
            Self::Cohesion => 0,
            Self::Refractoriness => 1,
            Self::Elasticity => 2,
            Self::Conductivity => 3,
            Self::DensityBias => 4,
            Self::Purity => 5,
            Self::Crystallinity => 6,
            Self::Vitality => 7,
            Self::Fertility => 8,
            Self::Ceramicity => 9,
        }
    }
}

type MaterialAspectVector = [f64; MaterialHiddenAspect::COUNT];

pub fn material_effective_rarity_bits_for_state(viewer_state: MaterialDerivationState) -> f64 {
    let rarity_context = viewer_state.rarity_context;
    let family_bias = match viewer_state.selected_variant {
        MaterialVariant::Metal => rarity_context.depth * 4.2,
        MaterialVariant::Stone => rarity_context.depth * 3.6,
        MaterialVariant::Wood => rarity_context.northness() * 4.0,
        MaterialVariant::Foliage => {
            rarity_context.center_proximity * 3.2 + rarity_context.northness() * 2.0
        }
        MaterialVariant::Glass => rarity_context.craft_quality * 2.0,
        MaterialVariant::Ice => {
            let snowiness = f64::from(viewer_state.ice_params.snowiness());
            rarity_context.height * (3.8 - 0.5 * snowiness)
        }
        MaterialVariant::Ceramic => {
            material_ceramic_feedstock_score_with_origin(
                viewer_state.soil_params,
                rarity_context,
                viewer_state.derived_regolith_origin(),
            ) * 4.0
                + rarity_context.craft_quality * 2.4
        }
        MaterialVariant::Crystal => rarity_context.radial_distance * 4.4,
        MaterialVariant::Soil => {
            rarity_context.center_proximity * 2.8 + rarity_context.southness() * 2.5
        }
    };
    (rarity_context.rarity_bits + family_bias).max(0.0)
}

pub fn material_effective_rarity_bits(
    variant: MaterialVariant,
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> f64 {
    material_effective_rarity_bits_for_state(MaterialDerivationState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        soil_params: soil,
        rarity_context,
    })
}

pub fn material_rarity_budget_for_state(viewer_state: MaterialDerivationState) -> f64 {
    let scale = match viewer_state.selected_variant {
        MaterialVariant::Metal => 14.0,
        MaterialVariant::Stone => 13.0,
        MaterialVariant::Wood => 12.0,
        MaterialVariant::Foliage => 11.5,
        MaterialVariant::Glass => 10.5,
        MaterialVariant::Ice => 10.0 - f64::from(viewer_state.ice_params.snowiness()) * 0.9,
        MaterialVariant::Ceramic => 13.0,
        MaterialVariant::Crystal => 12.5,
        MaterialVariant::Soil => 11.0,
    };
    let effective_rarity_bits = material_effective_rarity_bits_for_state(viewer_state);
    scale * (((1.0 + effective_rarity_bits).powf(1.7)) - 1.0)
}

#[cfg(test)]
#[allow(dead_code)]
fn material_rarity_budget(variant: MaterialVariant, effective_rarity_bits: f64) -> f64 {
    let scale = match variant {
        MaterialVariant::Metal => 14.0,
        MaterialVariant::Stone => 13.0,
        MaterialVariant::Wood => 12.0,
        MaterialVariant::Foliage => 11.5,
        MaterialVariant::Glass => 10.5,
        MaterialVariant::Ice => 10.0,
        MaterialVariant::Ceramic => 13.0,
        MaterialVariant::Crystal => 12.5,
        MaterialVariant::Soil => 11.0,
    };
    scale * (((1.0 + effective_rarity_bits).powf(1.7)) - 1.0)
}

pub(crate) fn material_aspect_weights_for_state(
    viewer_state: MaterialDerivationState,
) -> MaterialAspectVector {
    let variant = viewer_state.selected_variant;
    let soil = viewer_state.soil_params;
    let rarity_context = viewer_state.rarity_context;
    let mut weights = [0.04; MaterialHiddenAspect::COUNT];
    let north = rarity_context.northness();
    let south = rarity_context.southness();

    match variant {
        MaterialVariant::Metal => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Cohesion,
                1.00 + rarity_context.depth * 0.65,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Conductivity,
                0.95 + rarity_context.depth * 0.55,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.65 + rarity_context.depth * 0.30,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::DensityBias,
                0.25 + rarity_context.depth * 0.10,
            );
        }
        MaterialVariant::Stone => {
            let lithology = viewer_state.derived_stone_lithology();
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Cohesion,
                0.95 + rarity_context.depth * 0.45,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.85 + rarity_context.depth * 0.55,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::DensityBias,
                0.45 + rarity_context.depth * 0.18,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.15 + rarity_context.depth * 0.10,
            );
            if matches!(
                lithology,
                MaterialStoneLithology::Crystalline
                    | MaterialStoneLithology::Metamorphic
                    | MaterialStoneLithology::Carbonate
            ) {
                material_add_weight(&mut weights, MaterialHiddenAspect::Purity, 0.22);
            }
            if matches!(
                lithology,
                MaterialStoneLithology::OceanicBasalt | MaterialStoneLithology::BasalticVolcanic
            ) {
                material_add_weight(&mut weights, MaterialHiddenAspect::DensityBias, 0.24);
                material_add_weight(&mut weights, MaterialHiddenAspect::Refractoriness, 0.18);
            }
            if matches!(
                lithology,
                MaterialStoneLithology::Volcaniclastic
                    | MaterialStoneLithology::BasinFill
                    | MaterialStoneLithology::PassiveMarginSediment
            ) {
                material_add_weight(&mut weights, MaterialHiddenAspect::Fertility, 0.10);
            }
            if viewer_state.selected_stone_genesis == MaterialStoneGenesis::Metamorphic {
                material_add_weight(
                    &mut weights,
                    MaterialHiddenAspect::Purity,
                    0.14 + f64::from(viewer_state.stone_params.quaternary) * 0.18,
                );
            }
        }
        MaterialVariant::Wood => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Elasticity,
                0.95 + north * 0.70,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Cohesion,
                0.75 + north * 0.50,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Vitality,
                0.65 + north * 0.40,
            );
            material_add_weight(&mut weights, MaterialHiddenAspect::DensityBias, 0.15);
        }
        MaterialVariant::Foliage => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Elasticity,
                0.92 + north * 0.42 + rarity_context.center_proximity * 0.18,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Vitality,
                1.02 + rarity_context.center_proximity * 0.78 + north * 0.18,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Fertility,
                0.84 + rarity_context.center_proximity * 0.62 + south * 0.14,
            );
            material_add_weight(&mut weights, MaterialHiddenAspect::DensityBias, 0.10);
        }
        MaterialVariant::Glass => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.85 + rarity_context.craft_quality * 0.65,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.45 + rarity_context.craft_quality * 0.25,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Crystallinity,
                0.15 + rarity_context.craft_quality * 0.10,
            );
        }
        MaterialVariant::Ice => {
            let snowiness = f64::from(viewer_state.ice_params.snowiness());
            let air = f64::from(viewer_state.ice_params.air_content());
            let lake_mix = f64::from(material_ice_anchor_weights(viewer_state.ice_params).1);
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.82 + rarity_context.height * (0.55 - 0.10 * snowiness) + lake_mix * 0.18,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Crystallinity,
                0.55 + rarity_context.height * 0.30 + lake_mix * 0.30,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Cohesion,
                0.28 + rarity_context.height * (0.12 - 0.04 * snowiness),
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Vitality,
                0.10 + snowiness * 0.24 + air * 0.10,
            );
        }
        MaterialVariant::Ceramic => {
            let feedstock = material_ceramic_feedstock_score_with_origin(
                soil,
                rarity_context,
                viewer_state.derived_regolith_origin(),
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Ceramicity,
                0.95 + feedstock * 0.75 + rarity_context.craft_quality * 0.35,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.75 + feedstock * 0.45 + south * 0.20,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.60 + feedstock * 0.30 + rarity_context.craft_quality * 0.20,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Cohesion,
                0.35 + feedstock * 0.20,
            );
        }
        MaterialVariant::Crystal => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Crystallinity,
                1.00 + rarity_context.radial_distance * 0.70,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.85 + rarity_context.radial_distance * 0.45,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.55 + rarity_context.radial_distance * 0.22,
            );
        }
        MaterialVariant::Soil => {
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Fertility,
                0.35 + rarity_context.center_proximity * 0.80
                    + f64::from(soil.organic_pct) * 1.10
                    + f64::from(soil.water_capacity()) * 0.25,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Vitality,
                0.30 + rarity_context.center_proximity * 0.55 + f64::from(soil.organic_pct) * 0.65,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Ceramicity,
                0.20 + south * 0.75
                    + f64::from(soil.clay_pct) * 0.60
                    + f64::from(soil.silt_pct) * 0.22
                    + f64::from(soil.tephra_pct) * 0.42,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Purity,
                0.12 + south * 0.30 + f64::from(soil.sand_pct) * 0.18
                    - f64::from(soil.organic_pct) * 0.10
                    + f64::from(soil.tephra_pct) * 0.12,
            );
            material_add_weight(
                &mut weights,
                MaterialHiddenAspect::Refractoriness,
                0.16 + south * 0.38
                    + f64::from(soil.clay_pct) * 0.28
                    + f64::from(soil.iron_oxide_pct) * 1.10
                    + f64::from(soil.tephra_pct) * 0.55,
            );
        }
    }

    material_apply_seed_variation(&mut weights, rarity_context.seed, variant);
    weights
}

#[cfg(test)]
#[allow(dead_code)]
fn material_aspect_weights(
    variant: MaterialVariant,
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialAspectVector {
    material_aspect_weights_for_state(MaterialDerivationState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        soil_params: soil,
        rarity_context,
    })
}

pub(crate) fn material_aspect_budget(
    weights: MaterialAspectVector,
    rarity_budget: f64,
    effective_rarity_bits: f64,
) -> MaterialAspectVector {
    let mut shares = [0.0; MaterialHiddenAspect::COUNT];
    let sharpness = 1.0 + 0.35 * effective_rarity_bits;
    let mut total = 0.0;
    for (index, weight) in weights.iter().enumerate() {
        let share = weight.max(0.0001).powf(sharpness);
        shares[index] = share;
        total += share;
    }
    if total <= 1.0e-9 {
        return shares;
    }
    for share in &mut shares {
        *share = (*share / total) * rarity_budget;
    }
    shares
}

pub(crate) fn material_apply_aspects(
    base: MaterialCommonProperties,
    aspects: MaterialAspectVector,
) -> MaterialCommonProperties {
    let cohesion = aspects[MaterialHiddenAspect::Cohesion.index()];
    let refractoriness = aspects[MaterialHiddenAspect::Refractoriness.index()];
    let elasticity = aspects[MaterialHiddenAspect::Elasticity.index()];
    let conductivity = aspects[MaterialHiddenAspect::Conductivity.index()];
    let density_bias = aspects[MaterialHiddenAspect::DensityBias.index()];
    let purity = aspects[MaterialHiddenAspect::Purity.index()];
    let crystallinity = aspects[MaterialHiddenAspect::Crystallinity.index()];
    let vitality = aspects[MaterialHiddenAspect::Vitality.index()];
    let fertility = aspects[MaterialHiddenAspect::Fertility.index()];
    let ceramicity = aspects[MaterialHiddenAspect::Ceramicity.index()];

    let toughness_delta = 0.95 * cohesion + 0.40 * elasticity + 0.15 * purity + 0.25 * vitality;
    let hardness_delta = 0.22 * cohesion
        + 0.28 * refractoriness
        + 0.62 * crystallinity
        + 0.12 * purity
        + 0.75 * ceramicity
        - 0.18 * fertility;
    let compressive_delta = 0.72 * cohesion
        + 0.25 * refractoriness
        + 0.42 * density_bias
        + 0.58 * ceramicity
        + 0.32 * crystallinity;
    let tensile_delta =
        0.58 * cohesion + 0.50 * elasticity + 0.15 * crystallinity + 0.12 * vitality;
    let elasticity_delta =
        0.96 * elasticity + 0.28 * vitality - 0.22 * ceramicity - 0.12 * crystallinity;
    let heat_capacity_delta =
        0.24 * vitality + 0.36 * fertility + 0.18 * cohesion + 0.12 * refractoriness;
    let melting_delta =
        0.98 * refractoriness + 0.24 * purity + 0.76 * ceramicity + 0.20 * crystallinity;
    let boiling_delta = 0.82 * refractoriness + 0.22 * purity + 0.42 * ceramicity;
    let thermal_delta = 0.72 * conductivity + 0.22 * refractoriness + 0.18 * density_bias;
    let electrical_delta = 1.00 * conductivity + 0.10 * purity;
    let density_delta = 0.55 * density_bias + 0.22 * cohesion + 0.24 * ceramicity
        - 0.28 * elasticity
        - 0.18 * vitality;
    let porosity_delta = 0.42 * fertility + 0.18 * vitality
        - 0.42 * density_bias
        - 0.34 * ceramicity
        - 0.10 * purity;
    let ph_delta = 0.22 * fertility - 0.18 * purity - 0.12 * ceramicity + 0.08 * vitality;

    MaterialCommonProperties::new(
        material_absolute_property(base.toughness as f32 + toughness_delta as f32),
        material_absolute_property(base.hardness as f32 + hardness_delta as f32),
        material_absolute_property(base.compressive_strength as f32 + compressive_delta as f32),
        material_absolute_property(base.tensile_strength as f32 + tensile_delta as f32),
        material_absolute_property(base.elasticity as f32 + elasticity_delta as f32),
        material_absolute_property(base.heat_capacity as f32 + heat_capacity_delta as f32),
        material_absolute_property(base.melting_point as f32 + melting_delta as f32),
        material_absolute_property(base.boiling_point as f32 + boiling_delta as f32),
        material_absolute_property(base.thermal_conductivity as f32 + thermal_delta as f32),
        material_absolute_property(base.electrical_conductivity as f32 + electrical_delta as f32),
        material_absolute_property(base.density as f32 + density_delta as f32),
        material_absolute_property((base.porosity as f32 + porosity_delta as f32).max(0.0)),
        material_absolute_property((base.ph as f32 + ph_delta as f32).clamp(0.0, 14.0)),
        base.state,
    )
}

#[cfg(test)]
#[allow(dead_code)]
fn material_ceramic_feedstock_score(
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> f64 {
    material_ceramic_feedstock_score_with_origin(
        soil,
        rarity_context,
        MaterialRegolithOrigin::default(),
    )
}

pub(crate) fn material_ceramic_feedstock_score_with_origin(
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
    origin: MaterialRegolithOrigin,
) -> f64 {
    let origin_bias = match origin {
        MaterialRegolithOrigin::Residual => 0.0,
        MaterialRegolithOrigin::Alluvial => 0.06,
        MaterialRegolithOrigin::Colluvial => 0.02,
        MaterialRegolithOrigin::Aeolian => 0.12,
        MaterialRegolithOrigin::Lacustrine => 0.14,
        MaterialRegolithOrigin::MarineShelf => 0.20,
        MaterialRegolithOrigin::MarinePelagic => 0.16,
        MaterialRegolithOrigin::EstuarineDeltaic => 0.12,
        MaterialRegolithOrigin::GlacialTill => -0.06,
        MaterialRegolithOrigin::Tephric => 0.24,
        MaterialRegolithOrigin::OrganicPeat => -0.24,
        MaterialRegolithOrigin::ShallowBedrock => -0.05,
    };
    (f64::from(soil.clay_pct) * 0.62
        + f64::from(soil.silt_pct) * 0.18
        + f64::from(soil.sand_pct) * 0.12
        + f64::from(soil.tephra_pct) * 0.42
        + rarity_context.southness() * 0.45
        + origin_bias
        + f64::from(soil.iron_oxide_pct) * 1.10
        - f64::from(soil.organic_pct) * 0.38
        - f64::from(soil.water_saturation()) * 0.16)
        .max(0.0)
}

fn material_add_weight(
    weights: &mut MaterialAspectVector,
    aspect: MaterialHiddenAspect,
    value: f64,
) {
    weights[aspect.index()] += value.max(0.0);
}

fn material_apply_seed_variation(
    weights: &mut MaterialAspectVector,
    seed: u64,
    variant: MaterialVariant,
) {
    for (index, weight) in weights.iter_mut().enumerate() {
        let noise = material_seed_unit(seed, variant, index);
        *weight *= 0.82 + noise * 0.36;
    }
}

fn material_seed_unit(seed: u64, variant: MaterialVariant, index: usize) -> f64 {
    let mixed =
        material_splitmix64(seed ^ ((variant.shader_family_id() as u64) << 32) ^ index as u64);
    ((mixed >> 11) as f64) / ((1u64 << 53) as f64)
}

fn material_splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
