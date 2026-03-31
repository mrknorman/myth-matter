use crate::{
    MaterialCommonProperties, MaterialDerivationState, MaterialElement, MaterialIceForm,
    MaterialIceParameters, MaterialMatterState, MaterialRarityContext, MaterialRegolithOrigin,
    MaterialRegolithParameters, MaterialSoilParameters, MaterialStoneGenesis,
    MaterialStoneLithology, MaterialStoneParameters, MaterialVariant, MaterialWaterParameters,
    derive_water_base_common_properties, material_absolute_property, material_ice_anchor_weights,
    material_lerp_common_properties,
};

use crate::rarity::{
    material_apply_aspects, material_aspect_budget, material_aspect_weights_for_state,
    material_effective_rarity_bits_for_state, material_rarity_budget_for_state,
};

pub fn derive_soil_base_common_properties(
    soil: MaterialSoilParameters,
) -> MaterialCommonProperties {
    derive_soil_base_common_properties_with_origin(soil, MaterialRegolithOrigin::default())
}

pub fn derive_soil_base_common_properties_with_origin(
    soil: MaterialSoilParameters,
    origin: MaterialRegolithOrigin,
) -> MaterialCommonProperties {
    let coarse = soil.gravel_pct + soil.pebble_pct;
    let fines = soil.silt_pct + soil.clay_pct + soil.tephra_pct * 0.55;
    let saturation = soil.water_saturation();
    let regolith_density_bias = match origin {
        MaterialRegolithOrigin::Alluvial | MaterialRegolithOrigin::Colluvial => 40.0,
        MaterialRegolithOrigin::Aeolian => -35.0,
        MaterialRegolithOrigin::Lacustrine | MaterialRegolithOrigin::MarinePelagic => 25.0,
        MaterialRegolithOrigin::MarineShelf | MaterialRegolithOrigin::EstuarineDeltaic => 18.0,
        MaterialRegolithOrigin::GlacialTill => 60.0,
        MaterialRegolithOrigin::Tephric => -24.0,
        MaterialRegolithOrigin::OrganicPeat => -80.0,
        MaterialRegolithOrigin::ShallowBedrock => 75.0,
        MaterialRegolithOrigin::Residual => 0.0,
    };
    let regolith_porosity_bias = match origin {
        MaterialRegolithOrigin::Aeolian | MaterialRegolithOrigin::Tephric => 45.0,
        MaterialRegolithOrigin::OrganicPeat => 120.0,
        MaterialRegolithOrigin::ShallowBedrock => -90.0,
        MaterialRegolithOrigin::GlacialTill => -36.0,
        MaterialRegolithOrigin::Alluvial | MaterialRegolithOrigin::EstuarineDeltaic => 24.0,
        _ => 0.0,
    };
    let tephra = soil.tephra_pct;

    let toughness = material_absolute_property(
        120.0
            + fines * 360.0
            + coarse * 180.0
            + tephra * 130.0
            + soil.organic_pct * 220.0
            + saturation * 140.0
            - soil.sand_pct * 120.0,
    );
    let hardness = material_absolute_property(
        55.0 + soil.sand_pct * 220.0
            + coarse * 320.0
            + soil.clay_pct * 120.0
            + tephra * 260.0
            + soil.iron_oxide_pct * 180.0
            - saturation * 140.0
            - soil.organic_pct * 60.0,
    );
    let compressive_strength = material_absolute_property(
        100.0
            + soil.clay_pct * 260.0
            + soil.silt_pct * 180.0
            + coarse * 260.0
            + tephra * 140.0
            + soil.sand_pct * 140.0
            + saturation * 160.0
            - soil.organic_pct * 120.0,
    );
    let tensile_strength = material_absolute_property(
        70.0 + soil.clay_pct * 200.0
            + soil.silt_pct * 120.0
            + tephra * 60.0
            + soil.organic_pct * 140.0
            + saturation * 60.0
            - coarse * 90.0,
    );
    let elasticity = material_absolute_property(
        110.0 + soil.organic_pct * 220.0 + saturation * 160.0 + soil.clay_pct * 60.0
            - coarse * 80.0,
    );
    let heat_capacity = material_absolute_property(
        320.0
            + soil.water_pct * 420.0
            + soil.organic_pct * 160.0
            + soil.clay_pct * 80.0
            + soil.silt_pct * 60.0,
    );
    let melting_point = material_absolute_property(
        260.0
            + (1.0 - saturation) * 280.0
            + soil.sand_pct * 60.0
            + coarse * 120.0
            + tephra * 220.0
            + soil.iron_oxide_pct * 80.0
            - soil.organic_pct * 100.0,
    );
    let boiling_point =
        material_absolute_property(420.0 + soil.water_pct * 160.0 + fines * 60.0 + coarse * 40.0);
    let thermal_conductivity = material_absolute_property(
        90.0 + saturation * 260.0
            + fines * 110.0
            + soil.sand_pct * 90.0
            + tephra * 70.0
            + soil.iron_oxide_pct * 80.0
            - soil.organic_pct * 120.0
            - coarse * 70.0,
    );
    let electrical_conductivity = material_absolute_property(
        20.0 + saturation * 300.0
            + fines * 90.0
            + tephra * 40.0
            + soil.iron_oxide_pct * 140.0
            + soil.organic_pct * 80.0
            - coarse * 40.0,
    );
    let density = material_absolute_property(
        280.0
            + soil.sand_pct * 180.0
            + fines * 160.0
            + coarse * 240.0
            + tephra * 120.0
            + soil.organic_pct * 40.0
            + soil.water_pct * 200.0
            + regolith_density_bias,
    );
    let porosity = material_absolute_property(
        260.0
            + soil.organic_pct * 240.0
            + soil.clay_pct * 160.0
            + soil.silt_pct * 90.0
            + tephra * 80.0
            - coarse * 160.0
            - soil.sand_pct * 60.0
            - saturation * 100.0
            + regolith_porosity_bias,
    );
    let ph = material_absolute_property(
        (7.0 + soil.sand_pct * 1.2 + soil.silt_pct * 0.4 + tephra * 0.2 + soil.gravel_pct * 0.6
            - soil.organic_pct * 1.8
            - soil.iron_oxide_pct * 1.0
            - saturation * 0.8)
            .clamp(3.0, 10.0),
    );
    let state = if saturation >= 0.90 && (fines + soil.organic_pct) >= 0.45 {
        MaterialMatterState::Liquid
    } else if saturation >= 0.20 {
        MaterialMatterState::Solid
    } else {
        MaterialMatterState::Powder
    };

    MaterialCommonProperties::new(
        toughness,
        hardness,
        compressive_strength,
        tensile_strength,
        elasticity,
        heat_capacity,
        melting_point,
        boiling_point,
        thermal_conductivity,
        electrical_conductivity,
        density,
        porosity,
        ph,
        state,
    )
}

pub fn derive_material_common_properties(
    variant: MaterialVariant,
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialCommonProperties {
    derive_material_common_properties_for_state(MaterialDerivationState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        soil_params: soil,
        rarity_context,
    })
}

pub fn derive_material_common_properties_for_state(
    viewer_state: MaterialDerivationState,
) -> MaterialCommonProperties {
    let variant = viewer_state.selected_variant;
    let soil = viewer_state.soil_params;
    let rarity_context = viewer_state.rarity_context;
    let base_properties = match variant {
        MaterialVariant::Stone => derive_stone_base_common_properties(
            viewer_state.selected_stone_genesis,
            viewer_state.stone_params,
        ),
        MaterialVariant::Soil => derive_soil_base_common_properties_with_origin(
            soil,
            viewer_state.derived_regolith_origin(),
        ),
        MaterialVariant::Ice => derive_ice_base_common_properties(viewer_state.ice_params),
        MaterialVariant::Water => derive_water_base_common_properties(viewer_state.water_params),
        MaterialVariant::Ceramic => derive_ceramic_base_common_properties_with_origin(
            soil,
            rarity_context,
            viewer_state.derived_regolith_origin(),
        ),
        MaterialVariant::Glass => derive_glass_base_common_properties(rarity_context),
        _ => variant.base_common_properties(),
    };

    let effective_rarity_bits = material_effective_rarity_bits_for_state(viewer_state);
    let rarity_budget = material_rarity_budget_for_state(viewer_state);
    if rarity_budget <= 1.0e-6 {
        return base_properties;
    }

    let weights = material_aspect_weights_for_state(viewer_state);
    let aspects = material_aspect_budget(weights, rarity_budget, effective_rarity_bits);
    material_apply_aspects(base_properties, aspects)
}

pub fn material_derive_stone_lithology(
    genesis: MaterialStoneGenesis,
    params: MaterialStoneParameters,
) -> MaterialStoneLithology {
    match genesis {
        MaterialStoneGenesis::Igneous => {
            if params.quaternary >= 0.58 {
                MaterialStoneLithology::Volcaniclastic
            } else if params.primary >= 0.74 && params.secondary >= 0.48 {
                if params.tertiary <= 0.28 {
                    MaterialStoneLithology::OceanicBasalt
                } else {
                    MaterialStoneLithology::BasalticVolcanic
                }
            } else if params.primary >= 0.56 {
                MaterialStoneLithology::BasalticVolcanic
            } else {
                MaterialStoneLithology::Crystalline
            }
        }
        MaterialStoneGenesis::Sedimentary => {
            if params.tertiary >= 0.68 {
                if params.primary >= 0.56 && params.secondary >= 0.44 {
                    MaterialStoneLithology::CarbonatePlatform
                } else {
                    MaterialStoneLithology::Carbonate
                }
            } else if params.primary >= 0.56
                && params.secondary >= 0.44
                && params.quaternary >= 0.42
            {
                MaterialStoneLithology::Sandstone
            } else if params.primary <= 0.26 {
                MaterialStoneLithology::MudstoneShale
            } else if params.quaternary <= 0.24 {
                MaterialStoneLithology::BasinFill
            } else {
                MaterialStoneLithology::PassiveMarginSediment
            }
        }
        MaterialStoneGenesis::Metamorphic => {
            if params.quaternary >= 0.78 && params.secondary <= 0.38 && params.tertiary <= 0.36 {
                MaterialStoneLithology::Crystalline
            } else {
                MaterialStoneLithology::Metamorphic
            }
        }
    }
}

pub fn derive_stone_base_common_properties(
    genesis: MaterialStoneGenesis,
    params: MaterialStoneParameters,
) -> MaterialCommonProperties {
    let base = MaterialVariant::Stone.base_common_properties();
    let lithology = material_derive_stone_lithology(genesis, params);
    let (mut hardness, mut compressive, mut tensile, mut density, mut porosity, mut melt) =
        match lithology {
            MaterialStoneLithology::Crystalline => (80.0, 100.0, 20.0, 90.0, -50.0, 90.0),
            MaterialStoneLithology::Metamorphic => (90.0, 110.0, 28.0, 80.0, -40.0, 110.0),
            MaterialStoneLithology::BasalticVolcanic => (120.0, 130.0, 16.0, 100.0, -70.0, 160.0),
            MaterialStoneLithology::Volcaniclastic => (10.0, -20.0, -30.0, -20.0, 80.0, 25.0),
            MaterialStoneLithology::OceanicBasalt => (130.0, 150.0, 12.0, 120.0, -65.0, 180.0),
            MaterialStoneLithology::PassiveMarginSediment => {
                (-40.0, -30.0, -25.0, -10.0, 70.0, -10.0)
            }
            MaterialStoneLithology::CarbonatePlatform => (20.0, 30.0, -10.0, 10.0, 25.0, 20.0),
            MaterialStoneLithology::Sandstone => (-10.0, 20.0, -20.0, -5.0, 35.0, 0.0),
            MaterialStoneLithology::MudstoneShale => (-60.0, -50.0, -45.0, -30.0, 90.0, -20.0),
            MaterialStoneLithology::Carbonate => (10.0, 15.0, -15.0, 0.0, 30.0, 10.0),
            MaterialStoneLithology::BasinFill => (-70.0, -80.0, -60.0, -60.0, 120.0, -35.0),
        };

    hardness += params.vein_content * 12.0 - params.fracture_density * 120.0
        + params.oxide_staining * 18.0
        - params.weathering * 140.0;
    compressive +=
        params.vein_content * 10.0 - params.fracture_density * 150.0 - params.weathering * 180.0;
    tensile +=
        params.vein_content * 8.0 - params.fracture_density * 110.0 - params.weathering * 90.0;
    density += params.oxide_staining * 18.0 - params.weathering * 28.0;
    porosity += params.fracture_density * 140.0 + params.weathering * 160.0;
    melt += params.oxide_staining * 10.0 - params.weathering * 60.0;

    match genesis {
        MaterialStoneGenesis::Igneous => {
            let maficity = params.primary;
            let cooling_rate = params.secondary;
            let volatile_content = params.tertiary;
            let fragmental_content = params.quaternary;
            hardness += maficity * 70.0 + cooling_rate * 18.0 - volatile_content * 10.0;
            compressive += maficity * 80.0 - fragmental_content * 100.0;
            tensile += cooling_rate * 20.0 - fragmental_content * 70.0;
            density += maficity * 110.0 - volatile_content * 45.0 - fragmental_content * 25.0;
            porosity += volatile_content * 80.0 + fragmental_content * 140.0;
            melt += maficity * 120.0 + cooling_rate * 25.0 - volatile_content * 70.0;
        }
        MaterialStoneGenesis::Sedimentary => {
            let grain_size = params.primary;
            let sorting = params.secondary;
            let carbonate_content = params.tertiary;
            let cementation = params.quaternary;
            hardness += grain_size * 25.0 + cementation * 45.0 - carbonate_content * 16.0;
            compressive += sorting * 30.0 + cementation * 110.0 - grain_size * 12.0;
            tensile += cementation * 80.0 - grain_size * 20.0;
            density += carbonate_content * 38.0 + cementation * 22.0 - grain_size * 10.0;
            porosity += grain_size * 36.0 - cementation * 160.0 - sorting * 45.0;
            melt += carbonate_content * 24.0 + cementation * 10.0;
        }
        MaterialStoneGenesis::Metamorphic => {
            let grade = params.primary;
            let foliation = params.secondary;
            let banding = params.tertiary;
            let recrystallization = params.quaternary;
            hardness += grade * 90.0 + recrystallization * 72.0 - params.weathering * 30.0;
            compressive += grade * 110.0 + recrystallization * 50.0 - foliation * 28.0;
            tensile += grade * 28.0 + recrystallization * 55.0 - foliation * 60.0;
            density += grade * 55.0 + banding * 18.0;
            porosity += foliation * 26.0 - recrystallization * 100.0;
            melt += grade * 115.0 + recrystallization * 80.0;
        }
    };
    MaterialCommonProperties::new(
        material_absolute_property(base.toughness as f32 + compressive * 0.35 + tensile * 0.20),
        material_absolute_property(base.hardness as f32 + hardness),
        material_absolute_property(base.compressive_strength as f32 + compressive),
        material_absolute_property(base.tensile_strength as f32 + tensile),
        base.elasticity,
        base.heat_capacity,
        material_absolute_property(base.melting_point as f32 + melt),
        material_absolute_property(base.boiling_point as f32 + melt * 0.55),
        base.thermal_conductivity,
        base.electrical_conductivity,
        material_absolute_property(base.density as f32 + density),
        material_absolute_property((base.porosity as f32 + porosity).max(0.0)),
        base.ph,
        base.state,
    )
}

fn derive_ice_base_common_properties(ice: MaterialIceParameters) -> MaterialCommonProperties {
    let dense = MaterialIceForm::GlacialIce.base_common_properties();
    let lake = MaterialCommonProperties::new(
        140,
        180,
        220,
        80,
        95,
        290,
        78,
        300,
        180,
        12,
        170,
        70,
        7,
        MaterialMatterState::Solid,
    );
    let snow = MaterialIceForm::Snow.base_common_properties();
    let (dense_weight, lake_weight, snow_weight) = material_ice_anchor_weights(ice);
    let compaction = ice.compaction();
    let air = ice.air_content();
    let impurity = ice.impurity();
    let dense_mix = material_lerp_common_properties(dense, lake, lake_weight);
    let snow_mix = material_lerp_common_properties(lake, snow, snow_weight);
    let blended = material_lerp_common_properties(
        dense_mix,
        snow_mix,
        snow_weight / (snow_weight + dense_weight + 1.0e-6),
    );
    let impurity_density = match ice.impurity_element {
        MaterialElement::Stone
        | MaterialElement::Carbonate
        | MaterialElement::Tephra
        | MaterialElement::IronOxide => 1.0,
        MaterialElement::Metal | MaterialElement::Crystal => 1.2,
        MaterialElement::Humus | MaterialElement::Wood => 0.75,
        _ => 0.9,
    };

    MaterialCommonProperties::new(
        material_absolute_property(
            blended.toughness as f32 + compaction * 18.0 - air * 12.0 - impurity * 8.0,
        ),
        material_absolute_property(
            blended.hardness as f32 + compaction * 26.0 - air * 14.0 - impurity * 12.0,
        ),
        material_absolute_property(
            blended.compressive_strength as f32 + compaction * 30.0 - air * 18.0,
        ),
        material_absolute_property(
            blended.tensile_strength as f32 + compaction * 14.0 - air * 10.0 - impurity * 4.0,
        ),
        material_absolute_property(blended.elasticity as f32 + air * 12.0 - compaction * 6.0),
        material_absolute_property(blended.heat_capacity as f32 + air * 10.0 + impurity * 12.0),
        material_absolute_property(
            (blended.melting_point as f32 - impurity * 6.0 + compaction * 2.0).max(0.0),
        ),
        material_absolute_property(
            (blended.boiling_point as f32 - impurity * 4.0 + compaction * 3.0).max(0.0),
        ),
        material_absolute_property(
            (blended.thermal_conductivity as f32 + compaction * 28.0 - air * 34.0).max(0.0),
        ),
        material_absolute_property(
            blended.electrical_conductivity as f32 + impurity * 32.0 * impurity_density,
        ),
        material_absolute_property(
            (blended.density as f32 + compaction * 34.0 - air * 56.0
                + impurity * 18.0 * impurity_density)
                .max(0.0),
        ),
        material_absolute_property(
            (blended.porosity as f32 + air * 64.0 - compaction * 22.0 + impurity * 8.0).max(0.0),
        ),
        blended.ph,
        MaterialMatterState::Solid,
    )
}

#[cfg(test)]
#[allow(dead_code)]
fn derive_ceramic_base_common_properties(
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialCommonProperties {
    derive_ceramic_base_common_properties_with_origin(
        soil,
        rarity_context,
        MaterialRegolithOrigin::default(),
    )
}

fn derive_ceramic_base_common_properties_with_origin(
    soil: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
    origin: MaterialRegolithOrigin,
) -> MaterialCommonProperties {
    let clay_matrix = soil.clay_pct * 0.95 + soil.silt_pct * 0.40 + soil.sand_pct * 0.18;
    let feedstock_bonus = soil.tephra_pct * 0.34;
    let organic_penalty = soil.organic_pct * 0.60;
    let oxide_bonus = soil.iron_oxide_pct * 6.0;
    let craft = rarity_context.craft_quality;
    let south_bonus = rarity_context.southness();
    let origin_bonus = match origin {
        MaterialRegolithOrigin::Residual => (0.0, 0.0, 0.0, 0.0),
        MaterialRegolithOrigin::Alluvial => (12.0, 8.0, -12.0, 8.0),
        MaterialRegolithOrigin::Colluvial => (8.0, 6.0, -18.0, 4.0),
        MaterialRegolithOrigin::Aeolian => (10.0, 18.0, 14.0, -18.0),
        MaterialRegolithOrigin::Lacustrine => (6.0, 20.0, 12.0, -10.0),
        MaterialRegolithOrigin::MarineShelf => (18.0, 40.0, 18.0, -24.0),
        MaterialRegolithOrigin::MarinePelagic => (10.0, 54.0, 24.0, -30.0),
        MaterialRegolithOrigin::EstuarineDeltaic => (14.0, 26.0, 10.0, -12.0),
        MaterialRegolithOrigin::GlacialTill => (30.0, -12.0, -14.0, 18.0),
        MaterialRegolithOrigin::Tephric => (48.0, 34.0, 6.0, -4.0),
        MaterialRegolithOrigin::OrganicPeat => (-28.0, -24.0, -44.0, 32.0),
        MaterialRegolithOrigin::ShallowBedrock => (34.0, 10.0, -20.0, 6.0),
    };

    let toughness = material_absolute_property(
        220.0 + clay_matrix * 90.0 + feedstock_bonus * 70.0 + craft as f32 * 120.0 + origin_bonus.0
            - organic_penalty * 100.0,
    );
    let hardness = material_absolute_property(
        760.0
            + clay_matrix * 240.0
            + feedstock_bonus * 90.0
            + craft as f32 * 190.0
            + oxide_bonus * 42.0
            + origin_bonus.1
            - organic_penalty * 120.0,
    );
    let compressive_strength = material_absolute_property(
        840.0
            + clay_matrix * 210.0
            + feedstock_bonus * 84.0
            + craft as f32 * 220.0
            + south_bonus as f32 * 90.0
            + origin_bonus.1,
    );
    let tensile_strength = material_absolute_property(
        170.0
            + craft as f32 * 110.0
            + clay_matrix * 55.0
            + feedstock_bonus * 34.0
            + origin_bonus.0 * 0.35
            - organic_penalty * 60.0,
    );
    let elasticity = material_absolute_property(
        75.0 + soil.silt_pct * 40.0 - soil.clay_pct * 30.0
            + feedstock_bonus * 16.0
            + craft as f32 * 25.0,
    );
    let heat_capacity = material_absolute_property(
        360.0 + soil.clay_pct * 120.0 + soil.silt_pct * 80.0 + craft as f32 * 40.0
            - feedstock_bonus * 18.0
            + origin_bonus.3,
    );
    let melting_point = material_absolute_property(
        920.0
            + clay_matrix * 260.0
            + feedstock_bonus * 160.0
            + craft as f32 * 180.0
            + south_bonus as f32 * 130.0
            + oxide_bonus * 36.0,
    );
    let boiling_point = material_absolute_property(
        980.0
            + clay_matrix * 180.0
            + feedstock_bonus * 110.0
            + craft as f32 * 140.0
            + south_bonus as f32 * 90.0
            + origin_bonus.2,
    );
    let thermal_conductivity = material_absolute_property(
        190.0
            + craft as f32 * 65.0
            + soil.sand_pct * 50.0
            + oxide_bonus * 18.0
            + feedstock_bonus * 34.0
            + origin_bonus.0 * 0.20,
    );
    let electrical_conductivity = material_absolute_property(
        10.0 + craft as f32 * 12.0 + oxide_bonus * 10.0 + feedstock_bonus * 10.0,
    );
    let density = material_absolute_property(
        600.0
            + clay_matrix * 190.0
            + feedstock_bonus * 60.0
            + south_bonus as f32 * 75.0
            + origin_bonus.0
            - organic_penalty * 140.0,
    );
    let porosity = material_absolute_property(
        95.0 + soil.silt_pct * 55.0 + soil.organic_pct * 90.0
            - craft as f32 * 70.0
            - clay_matrix * 40.0,
    );
    let ph = material_absolute_property(
        (7.0 + soil.sand_pct * 0.6 - soil.organic_pct * 1.2 - soil.iron_oxide_pct * 2.0)
            .clamp(4.0, 9.0),
    );

    MaterialCommonProperties::new(
        toughness,
        hardness,
        compressive_strength,
        tensile_strength,
        elasticity,
        heat_capacity,
        melting_point,
        boiling_point,
        thermal_conductivity,
        electrical_conductivity,
        density,
        porosity,
        ph,
        MaterialMatterState::Solid,
    )
}

fn derive_glass_base_common_properties(
    rarity_context: MaterialRarityContext,
) -> MaterialCommonProperties {
    let base = MaterialVariant::Glass.base_common_properties();
    let craft = rarity_context.craft_quality as f32;
    MaterialCommonProperties::new(
        material_absolute_property(base.toughness as f32 + craft * 40.0),
        material_absolute_property(base.hardness as f32 + craft * 55.0),
        material_absolute_property(base.compressive_strength as f32 + craft * 45.0),
        material_absolute_property(base.tensile_strength as f32 + craft * 18.0),
        base.elasticity,
        material_absolute_property(base.heat_capacity as f32 + craft * 20.0),
        material_absolute_property(base.melting_point as f32 + craft * 65.0),
        material_absolute_property(base.boiling_point as f32 + craft * 55.0),
        material_absolute_property(base.thermal_conductivity as f32 + craft * 18.0),
        base.electrical_conductivity,
        base.density,
        material_absolute_property((base.porosity as f32 - craft * 8.0).max(0.0)),
        base.ph,
        MaterialMatterState::Solid,
    )
}
