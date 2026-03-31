use crate::{
    MaterialClass, MaterialDerivationState, MaterialElement, MaterialIceParameters,
    MaterialOrientationAxis, MaterialRarityContext, MaterialRegolithOrigin,
    MaterialRegolithParameters, MaterialSoilElementBindings, MaterialSoilParameters,
    MaterialStoneGenesis, MaterialStoneLithology, MaterialStoneParameters, MaterialVariant,
    MaterialWaterParameters, derive_material_common_properties_for_state,
    material_derive_stone_lithology, material_regolith_parameters_for_origin,
};

pub const TERRAIN_MATERIAL_INPUT_SCHEMA_VERSION: u32 = 1;
pub const TERRAIN_MATERIAL_RECIPE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialOpticalClass {
    Opaque,
    SpecularOpaque,
    ThinTransmissive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialRepresentativeColor {
    pub rgb: [f32; 3],
}

impl MaterialRepresentativeColor {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { rgb: [r, g, b] }
    }

    pub fn mix(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.rgb[0] + (other.rgb[0] - self.rgb[0]) * t,
            self.rgb[1] + (other.rgb[1] - self.rgb[1]) * t,
            self.rgb[2] + (other.rgb[2] - self.rgb[2]) * t,
        )
    }

    pub fn scale(self, factor: f32) -> Self {
        Self::new(
            (self.rgb[0] * factor).clamp(0.0, 1.0),
            (self.rgb[1] * factor).clamp(0.0, 1.0),
            (self.rgb[2] * factor).clamp(0.0, 1.0),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialRecipeParameters {
    None,
    Soil {
        params: MaterialSoilParameters,
        bindings: MaterialSoilElementBindings,
        regolith_origin: MaterialRegolithOrigin,
    },
    Stone {
        genesis: MaterialStoneGenesis,
        params: MaterialStoneParameters,
        lithology: MaterialStoneLithology,
    },
    Ice {
        params: MaterialIceParameters,
    },
    Water {
        params: MaterialWaterParameters,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialRecipe {
    pub class: MaterialClass,
    pub variant: MaterialVariant,
    pub orientation: MaterialOrientationAxis,
    pub optical_class: MaterialOpticalClass,
    pub representative_color: MaterialRepresentativeColor,
    pub common_properties: crate::MaterialCommonProperties,
    pub parameters: MaterialRecipeParameters,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TerrainSurfaceClass {
    Rock,
    Gravel,
    Sand,
    Silt,
    Clay,
    #[default]
    Loam,
    Peat,
    MarineSand,
    MarineMud,
    CarbonateShoal,
    SnowIce,
}

impl TerrainSurfaceClass {
    pub const ALL: [Self; 11] = [
        Self::Rock,
        Self::Gravel,
        Self::Sand,
        Self::Silt,
        Self::Clay,
        Self::Loam,
        Self::Peat,
        Self::MarineSand,
        Self::MarineMud,
        Self::CarbonateShoal,
        Self::SnowIce,
    ];

    pub const fn encode_id(self) -> u32 {
        match self {
            Self::Rock => 0,
            Self::Gravel => 1,
            Self::Sand => 2,
            Self::Silt => 3,
            Self::Clay => 4,
            Self::Loam => 5,
            Self::Peat => 6,
            Self::MarineSand => 7,
            Self::MarineMud => 8,
            Self::CarbonateShoal => 9,
            Self::SnowIce => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TerrainBedrockClass {
    #[default]
    Crystalline,
    Metamorphic,
    BasalticVolcanic,
    Volcaniclastic,
    OceanicBasalt,
    PassiveMarginSediment,
    CarbonatePlatform,
    Sandstone,
    MudstoneShale,
    Carbonate,
    BasinFill,
}

impl TerrainBedrockClass {
    pub const ALL: [Self; 11] = [
        Self::Crystalline,
        Self::Metamorphic,
        Self::BasalticVolcanic,
        Self::Volcaniclastic,
        Self::OceanicBasalt,
        Self::PassiveMarginSediment,
        Self::CarbonatePlatform,
        Self::Sandstone,
        Self::MudstoneShale,
        Self::Carbonate,
        Self::BasinFill,
    ];

    pub const fn encode_id(self) -> u32 {
        match self {
            Self::Crystalline => 0,
            Self::Metamorphic => 1,
            Self::BasalticVolcanic => 2,
            Self::Volcaniclastic => 3,
            Self::OceanicBasalt => 4,
            Self::PassiveMarginSediment => 5,
            Self::CarbonatePlatform => 6,
            Self::Sandstone => 7,
            Self::MudstoneShale => 8,
            Self::Carbonate => 9,
            Self::BasinFill => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TerrainCoverClass {
    #[default]
    None,
    Snow,
    Ice,
    SnowIce,
}

impl TerrainCoverClass {
    pub const ALL: [Self; 4] = [Self::None, Self::Snow, Self::Ice, Self::SnowIce];

    pub const fn encode_id(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Snow => 1,
            Self::Ice => 2,
            Self::SnowIce => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainSurfaceInputs {
    pub class: TerrainSurfaceClass,
    pub moisture: f32,
    pub organic_fraction: f32,
    pub carbonate_fraction: f32,
    pub tephra_fraction: f32,
    pub iron_oxide_fraction: f32,
}

impl Default for TerrainSurfaceInputs {
    fn default() -> Self {
        Self {
            class: TerrainSurfaceClass::Loam,
            moisture: 0.18,
            organic_fraction: 0.04,
            carbonate_fraction: 0.0,
            tephra_fraction: 0.0,
            iron_oxide_fraction: 0.025,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainRegolithInputs {
    pub origin: MaterialRegolithOrigin,
    pub parameters: Option<MaterialRegolithParameters>,
}

impl Default for TerrainRegolithInputs {
    fn default() -> Self {
        Self {
            origin: MaterialRegolithOrigin::Residual,
            parameters: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainBedrockInputs {
    pub class: TerrainBedrockClass,
    pub genesis: Option<MaterialStoneGenesis>,
    pub vein_content: Option<f32>,
    pub fracture_density: f32,
    pub weathering: f32,
    pub oxide_staining: f32,
    pub primary: Option<f32>,
    pub secondary: Option<f32>,
    pub tertiary: Option<f32>,
    pub quaternary: Option<f32>,
}

impl Default for TerrainBedrockInputs {
    fn default() -> Self {
        Self {
            class: TerrainBedrockClass::Crystalline,
            genesis: None,
            vein_content: None,
            fracture_density: 0.16,
            weathering: 0.10,
            oxide_staining: 0.06,
            primary: None,
            secondary: None,
            tertiary: None,
            quaternary: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainCoverInputs {
    pub class: TerrainCoverClass,
    pub compaction: f32,
    pub air_content: f32,
    pub impurity: f32,
    pub impurity_element: MaterialElement,
}

impl Default for TerrainCoverInputs {
    fn default() -> Self {
        Self {
            class: TerrainCoverClass::None,
            compaction: 0.18,
            air_content: 0.88,
            impurity: 0.02,
            impurity_element: MaterialElement::Stone,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TerrainMaterialInputs {
    pub surface: TerrainSurfaceInputs,
    pub regolith: TerrainRegolithInputs,
    pub bedrock: TerrainBedrockInputs,
    pub cover: TerrainCoverInputs,
    pub rarity_context: MaterialRarityContext,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainMaterialColumnRecipe {
    pub cover: Option<MaterialRecipe>,
    pub surface: MaterialRecipe,
    pub subsurface: MaterialRecipe,
    pub bedrock: MaterialRecipe,
}

pub fn derive_material_recipe(
    variant: MaterialVariant,
    soil_params: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    match variant {
        MaterialVariant::Soil => derive_soil_recipe(
            soil_params,
            TerrainRegolithInputs {
                origin: MaterialRegolithOrigin::Residual,
                parameters: Some(material_regolith_parameters_for_origin(
                    MaterialRegolithOrigin::Residual,
                )),
            },
            rarity_context,
        ),
        MaterialVariant::Stone => derive_stone_recipe(
            MaterialStoneGenesis::default(),
            MaterialStoneParameters::default(),
            rarity_context,
        ),
        MaterialVariant::Ice => derive_ice_recipe(MaterialIceParameters::default(), rarity_context),
        MaterialVariant::Water => {
            derive_water_recipe(MaterialWaterParameters::default(), rarity_context)
        }
        MaterialVariant::Ceramic => derive_ceramic_recipe(
            soil_params,
            MaterialRegolithOrigin::Residual,
            rarity_context,
        ),
        _ => derive_simple_recipe(variant, soil_params, rarity_context),
    }
}

pub fn derive_ceramic_recipe(
    soil_params: MaterialSoilParameters,
    regolith_origin: MaterialRegolithOrigin,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Ceramic,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: material_regolith_parameters_for_origin(regolith_origin),
        soil_params,
        rarity_context,
    };
    let representative_color =
        representative_color_for_ceramic(soil_params, regolith_origin, rarity_context);
    MaterialRecipe {
        class: MaterialClass::Elemental,
        variant: MaterialVariant::Ceramic,
        orientation: MaterialVariant::Ceramic.orientation_axis(),
        optical_class: MaterialOpticalClass::Opaque,
        representative_color,
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Soil {
            params: soil_params,
            bindings: MaterialSoilElementBindings::from_regolith_origin(regolith_origin),
            regolith_origin,
        },
    }
}

pub fn derive_terrain_material_column_recipe(
    inputs: TerrainMaterialInputs,
) -> TerrainMaterialColumnRecipe {
    let bedrock = derive_bedrock_recipe(inputs.bedrock, inputs.rarity_context);
    let surface = derive_surface_recipe(
        inputs.surface,
        inputs.regolith,
        inputs.rarity_context,
        bedrock,
    );
    let subsurface = derive_subsurface_recipe(
        inputs.surface,
        inputs.regolith,
        inputs.rarity_context,
        bedrock,
    );
    let cover = derive_cover_recipe(inputs.cover, inputs.rarity_context);
    TerrainMaterialColumnRecipe {
        cover,
        surface,
        subsurface,
        bedrock,
    }
}

pub fn derive_terrain_material_column_recipe_from_surface(
    surface: TerrainSurfaceInputs,
    regolith: TerrainRegolithInputs,
    bedrock: TerrainBedrockInputs,
    rarity_context: MaterialRarityContext,
) -> TerrainMaterialColumnRecipe {
    derive_terrain_material_column_recipe(TerrainMaterialInputs {
        surface,
        regolith,
        bedrock,
        cover: TerrainCoverInputs::default(),
        rarity_context,
    })
}

fn derive_simple_recipe(
    variant: MaterialVariant,
    soil_params: MaterialSoilParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let viewer_state = MaterialDerivationState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: material_regolith_parameters_for_origin(MaterialRegolithOrigin::Residual),
        soil_params,
        rarity_context,
    };
    MaterialRecipe {
        class: variant.material_class(),
        variant,
        orientation: variant.orientation_axis(),
        optical_class: optical_class_for_variant(variant, MaterialIceParameters::default()),
        representative_color: representative_color_for_variant(variant),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::None,
    }
}

fn derive_surface_recipe(
    surface: TerrainSurfaceInputs,
    regolith: TerrainRegolithInputs,
    rarity_context: MaterialRarityContext,
    bedrock: MaterialRecipe,
) -> MaterialRecipe {
    match surface.class {
        TerrainSurfaceClass::Rock => {
            let mut recipe = bedrock;
            if let MaterialRecipeParameters::Stone {
                genesis,
                mut params,
                ..
            } = recipe.parameters
            {
                params.weathering = (params.weathering + 0.18).clamp(0.0, 1.0);
                params.fracture_density = (params.fracture_density + 0.08).clamp(0.0, 1.0);
                recipe = derive_stone_recipe(genesis, params, rarity_context);
            }
            recipe
        }
        TerrainSurfaceClass::SnowIce => derive_ice_recipe(
            MaterialIceParameters::new(
                0.36,
                0.70,
                surface.tephra_fraction * 0.35,
                MaterialElement::Stone,
            ),
            rarity_context,
        ),
        _ => derive_soil_recipe(
            soil_parameters_for_surface(surface),
            regolith,
            rarity_context,
        ),
    }
}

fn derive_subsurface_recipe(
    surface: TerrainSurfaceInputs,
    regolith: TerrainRegolithInputs,
    rarity_context: MaterialRarityContext,
    bedrock: MaterialRecipe,
) -> MaterialRecipe {
    match surface.class {
        TerrainSurfaceClass::Rock | TerrainSurfaceClass::SnowIce => bedrock,
        _ => {
            let mut soil = soil_parameters_for_surface(surface);
            soil.organic_pct *= 0.35;
            soil.water_pct = (soil.water_pct * 0.72).clamp(0.0, soil.water_capacity());
            soil.clay_pct = (soil.clay_pct + 0.05).clamp(0.0, 1.0);
            soil.silt_pct = (soil.silt_pct + 0.02).clamp(0.0, 1.0);
            normalize_soil_composition(&mut soil);
            derive_soil_recipe(soil, regolith, rarity_context)
        }
    }
}

fn derive_bedrock_recipe(
    bedrock: TerrainBedrockInputs,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let (genesis, params) = stone_profile_for_bedrock_class(bedrock);
    let lithology = material_derive_stone_lithology(genesis, params);
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Stone,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: genesis,
        stone_params: params,
        regolith_params: material_regolith_parameters_for_origin(
            MaterialRegolithOrigin::ShallowBedrock,
        ),
        soil_params: MaterialSoilParameters::default(),
        rarity_context,
    };
    MaterialRecipe {
        class: MaterialClass::Elemental,
        variant: MaterialVariant::Stone,
        orientation: MaterialVariant::Stone.orientation_axis(),
        optical_class: MaterialOpticalClass::Opaque,
        representative_color: representative_color_for_stone(lithology, params),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Stone {
            genesis,
            params,
            lithology,
        },
    }
}

fn derive_cover_recipe(
    cover: TerrainCoverInputs,
    rarity_context: MaterialRarityContext,
) -> Option<MaterialRecipe> {
    let params = match cover.class {
        TerrainCoverClass::None => return None,
        TerrainCoverClass::Snow => MaterialIceParameters::new(
            cover.compaction.min(0.35),
            cover.air_content.max(0.60),
            cover.impurity,
            cover.impurity_element,
        ),
        TerrainCoverClass::Ice => MaterialIceParameters::new(
            cover.compaction.max(0.70),
            cover.air_content.min(0.25),
            cover.impurity,
            cover.impurity_element,
        ),
        TerrainCoverClass::SnowIce => MaterialIceParameters::new(
            cover.compaction.clamp(0.30, 0.75),
            cover.air_content.clamp(0.20, 0.72),
            cover.impurity,
            cover.impurity_element,
        ),
    };
    Some(derive_ice_recipe(params, rarity_context))
}

fn derive_soil_recipe(
    soil_params: MaterialSoilParameters,
    regolith: TerrainRegolithInputs,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Soil,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: regolith
            .parameters
            .unwrap_or(material_regolith_parameters_for_origin(regolith.origin)),
        soil_params,
        rarity_context,
    };
    let bindings = soil_element_bindings_for_recipe(soil_params, regolith.origin);
    MaterialRecipe {
        class: MaterialClass::Mixture,
        variant: MaterialVariant::Soil,
        orientation: MaterialVariant::Soil.orientation_axis(),
        optical_class: MaterialOpticalClass::Opaque,
        representative_color: representative_color_for_soil(soil_params, bindings),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Soil {
            params: soil_params,
            bindings,
            regolith_origin: regolith.origin,
        },
    }
}

fn derive_stone_recipe(
    genesis: MaterialStoneGenesis,
    params: MaterialStoneParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let lithology = material_derive_stone_lithology(genesis, params);
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Stone,
        ice_params: MaterialIceParameters::default(),
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: genesis,
        stone_params: params,
        regolith_params: material_regolith_parameters_for_origin(
            MaterialRegolithOrigin::ShallowBedrock,
        ),
        soil_params: MaterialSoilParameters::default(),
        rarity_context,
    };
    MaterialRecipe {
        class: MaterialClass::Elemental,
        variant: MaterialVariant::Stone,
        orientation: MaterialVariant::Stone.orientation_axis(),
        optical_class: MaterialOpticalClass::Opaque,
        representative_color: representative_color_for_stone(lithology, params),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Stone {
            genesis,
            params,
            lithology,
        },
    }
}

fn derive_ice_recipe(
    params: MaterialIceParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Ice,
        ice_params: params,
        water_params: MaterialWaterParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: material_regolith_parameters_for_origin(MaterialRegolithOrigin::Residual),
        soil_params: MaterialSoilParameters::default(),
        rarity_context,
    };
    MaterialRecipe {
        class: MaterialClass::Elemental,
        variant: MaterialVariant::Ice,
        orientation: MaterialVariant::Ice.orientation_axis(),
        optical_class: optical_class_for_variant(MaterialVariant::Ice, params),
        representative_color: representative_color_for_ice(params),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Ice { params },
    }
}

fn derive_water_recipe(
    params: MaterialWaterParameters,
    rarity_context: MaterialRarityContext,
) -> MaterialRecipe {
    let viewer_state = MaterialDerivationState {
        selected_variant: MaterialVariant::Water,
        ice_params: MaterialIceParameters::default(),
        water_params: params,
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: material_regolith_parameters_for_origin(MaterialRegolithOrigin::Residual),
        soil_params: MaterialSoilParameters::default(),
        rarity_context,
    };
    MaterialRecipe {
        class: MaterialClass::Elemental,
        variant: MaterialVariant::Water,
        orientation: MaterialVariant::Water.orientation_axis(),
        optical_class: MaterialOpticalClass::ThinTransmissive,
        representative_color: representative_color_for_water(params),
        common_properties: derive_material_common_properties_for_state(viewer_state),
        parameters: MaterialRecipeParameters::Water { params },
    }
}

fn optical_class_for_variant(
    variant: MaterialVariant,
    ice_params: MaterialIceParameters,
) -> MaterialOpticalClass {
    match variant {
        MaterialVariant::Metal | MaterialVariant::Crystal => MaterialOpticalClass::SpecularOpaque,
        MaterialVariant::Glass => MaterialOpticalClass::ThinTransmissive,
        MaterialVariant::Water => MaterialOpticalClass::ThinTransmissive,
        MaterialVariant::Ice => {
            let clarity = ice_params.compaction() * (1.0 - ice_params.air_content());
            if clarity >= 0.18 {
                MaterialOpticalClass::ThinTransmissive
            } else {
                MaterialOpticalClass::Opaque
            }
        }
        _ => MaterialOpticalClass::Opaque,
    }
}

fn representative_color_for_variant(variant: MaterialVariant) -> MaterialRepresentativeColor {
    match variant {
        MaterialVariant::Stone => MaterialRepresentativeColor::new(0.56, 0.54, 0.56),
        MaterialVariant::Metal => MaterialRepresentativeColor::new(0.62, 0.64, 0.68),
        MaterialVariant::Wood => MaterialRepresentativeColor::new(0.56, 0.38, 0.22),
        MaterialVariant::Foliage => MaterialRepresentativeColor::new(0.26, 0.44, 0.16),
        MaterialVariant::Glass => MaterialRepresentativeColor::new(0.72, 0.80, 0.84),
        MaterialVariant::Ice => MaterialRepresentativeColor::new(0.78, 0.88, 0.94),
        MaterialVariant::Water => {
            representative_color_for_water(MaterialWaterParameters::default())
        }
        MaterialVariant::Ceramic => MaterialRepresentativeColor::new(0.70, 0.46, 0.32),
        MaterialVariant::Crystal => MaterialRepresentativeColor::new(0.70, 0.62, 0.82),
        MaterialVariant::Soil => MaterialRepresentativeColor::new(0.42, 0.32, 0.22),
    }
}

fn representative_color_for_water(params: MaterialWaterParameters) -> MaterialRepresentativeColor {
    let purity = params.purity();
    let salinity = params.salinity();
    let sediment = params.sediment();
    let organic = params.organic_tint();
    let aeration = params.aeration();
    let clear = MaterialRepresentativeColor::new(0.56, 0.80, 0.94);
    let marine = MaterialRepresentativeColor::new(0.34, 0.66, 0.86);
    let tannin = MaterialRepresentativeColor::new(0.34, 0.44, 0.22);
    let silted = MaterialRepresentativeColor::new(0.58, 0.50, 0.28);
    let aerated = MaterialRepresentativeColor::new(0.76, 0.86, 0.88);
    let salt_mix = clear.mix(marine, salinity * 0.65);
    let loaded = salt_mix
        .mix(tannin, organic * 0.55)
        .mix(silted, sediment * 0.70);
    loaded
        .mix(aerated, aeration * 0.30)
        .mix(clear, purity * 0.25)
}

fn representative_color_for_element(element: MaterialElement) -> MaterialRepresentativeColor {
    match element {
        MaterialElement::Stone => MaterialRepresentativeColor::new(0.52, 0.48, 0.44),
        MaterialElement::Metal => MaterialRepresentativeColor::new(0.64, 0.66, 0.70),
        MaterialElement::Wood => MaterialRepresentativeColor::new(0.56, 0.38, 0.22),
        MaterialElement::Snow => MaterialRepresentativeColor::new(0.94, 0.96, 0.98),
        MaterialElement::Ice => MaterialRepresentativeColor::new(0.78, 0.88, 0.94),
        MaterialElement::Ceramic => MaterialRepresentativeColor::new(0.70, 0.46, 0.32),
        MaterialElement::Crystal => MaterialRepresentativeColor::new(0.72, 0.66, 0.84),
        MaterialElement::Glass => MaterialRepresentativeColor::new(0.74, 0.82, 0.86),
        MaterialElement::Carbonate => MaterialRepresentativeColor::new(0.82, 0.78, 0.66),
        MaterialElement::Tephra => MaterialRepresentativeColor::new(0.30, 0.28, 0.30),
        MaterialElement::Humus => MaterialRepresentativeColor::new(0.22, 0.15, 0.10),
        MaterialElement::IronOxide => MaterialRepresentativeColor::new(0.72, 0.34, 0.18),
    }
}

fn representative_color_for_soil(
    soil: MaterialSoilParameters,
    bindings: MaterialSoilElementBindings,
) -> MaterialRepresentativeColor {
    let mut rgb = [0.0; 3];
    for (weight, color) in [
        (
            soil.sand_pct,
            representative_color_for_element(bindings.sand),
        ),
        (
            soil.silt_pct,
            representative_color_for_element(bindings.silt),
        ),
        (
            soil.clay_pct,
            representative_color_for_element(bindings.clay),
        ),
        (
            soil.gravel_pct,
            representative_color_for_element(bindings.gravel),
        ),
        (
            soil.pebble_pct,
            representative_color_for_element(bindings.pebble),
        ),
        (
            soil.tephra_pct,
            representative_color_for_element(bindings.tephra),
        ),
        (
            soil.organic_pct,
            representative_color_for_element(bindings.organic),
        ),
        (
            soil.iron_oxide_pct.min(0.12) * 2.0,
            representative_color_for_element(bindings.iron_oxide),
        ),
    ] {
        rgb[0] += color.rgb[0] * weight;
        rgb[1] += color.rgb[1] * weight;
        rgb[2] += color.rgb[2] * weight;
    }
    let color = MaterialRepresentativeColor { rgb };
    color.scale(1.0 - soil.water_saturation() * 0.35)
}

fn representative_color_for_stone(
    lithology: MaterialStoneLithology,
    params: MaterialStoneParameters,
) -> MaterialRepresentativeColor {
    let base = match lithology {
        MaterialStoneLithology::Crystalline => MaterialRepresentativeColor::new(0.66, 0.66, 0.70),
        MaterialStoneLithology::Metamorphic => MaterialRepresentativeColor::new(0.54, 0.52, 0.58),
        MaterialStoneLithology::BasalticVolcanic => {
            MaterialRepresentativeColor::new(0.28, 0.29, 0.32)
        }
        MaterialStoneLithology::Volcaniclastic => {
            MaterialRepresentativeColor::new(0.46, 0.40, 0.36)
        }
        MaterialStoneLithology::OceanicBasalt => MaterialRepresentativeColor::new(0.20, 0.23, 0.26),
        MaterialStoneLithology::PassiveMarginSediment => {
            MaterialRepresentativeColor::new(0.68, 0.62, 0.54)
        }
        MaterialStoneLithology::CarbonatePlatform => {
            MaterialRepresentativeColor::new(0.82, 0.78, 0.68)
        }
        MaterialStoneLithology::Sandstone => MaterialRepresentativeColor::new(0.78, 0.66, 0.48),
        MaterialStoneLithology::MudstoneShale => MaterialRepresentativeColor::new(0.40, 0.36, 0.34),
        MaterialStoneLithology::Carbonate => MaterialRepresentativeColor::new(0.84, 0.82, 0.76),
        MaterialStoneLithology::BasinFill => MaterialRepresentativeColor::new(0.60, 0.54, 0.48),
    };
    let oxide = representative_color_for_element(MaterialElement::IronOxide);
    base.mix(oxide, params.oxide_staining * 0.22)
        .scale(1.0 - params.weathering * 0.16)
}

fn representative_color_for_ice(params: MaterialIceParameters) -> MaterialRepresentativeColor {
    let snow = representative_color_for_element(MaterialElement::Snow);
    let lake = MaterialRepresentativeColor::new(0.66, 0.82, 0.92);
    let dense = representative_color_for_element(MaterialElement::Ice);
    let clarity = params.compaction() * (1.0 - params.air_content());
    let snowy = dense.mix(snow, params.air_content().max(1.0 - params.compaction()));
    let clear = snowy.mix(lake, clarity);
    clear.mix(
        representative_color_for_element(params.impurity_element),
        params.impurity() * 0.22,
    )
}

fn representative_color_for_ceramic(
    soil: MaterialSoilParameters,
    regolith_origin: MaterialRegolithOrigin,
    rarity_context: MaterialRarityContext,
) -> MaterialRepresentativeColor {
    let soil_color = representative_color_for_soil(
        soil,
        MaterialSoilElementBindings::from_regolith_origin(regolith_origin),
    );
    let fired = MaterialRepresentativeColor::new(0.78, 0.52, 0.34);
    let south_bias = rarity_context.southness() as f32;
    soil_color
        .mix(fired, 0.38 + soil.iron_oxide_pct * 1.8 + south_bias * 0.10)
        .scale(0.92 + rarity_context.craft_quality as f32 * 0.08)
}

fn soil_element_bindings_for_recipe(
    soil: MaterialSoilParameters,
    regolith_origin: MaterialRegolithOrigin,
) -> MaterialSoilElementBindings {
    let mut bindings = MaterialSoilElementBindings::from_regolith_origin(regolith_origin);
    let carbonate_bias = matches!(
        regolith_origin,
        MaterialRegolithOrigin::MarineShelf
            | MaterialRegolithOrigin::MarinePelagic
            | MaterialRegolithOrigin::EstuarineDeltaic
    ) || (soil.sand_pct + soil.silt_pct) > 0.55
        && soil.iron_oxide_pct < 0.03
        && soil.tephra_pct < 0.05;
    if carbonate_bias {
        bindings.sand = MaterialElement::Carbonate;
        bindings.silt = MaterialElement::Carbonate;
    }
    bindings
}

fn soil_parameters_for_surface(surface: TerrainSurfaceInputs) -> MaterialSoilParameters {
    let mut soil = match surface.class {
        TerrainSurfaceClass::Rock => MaterialSoilParameters {
            sand_pct: 0.16,
            silt_pct: 0.10,
            clay_pct: 0.08,
            gravel_pct: 0.34,
            pebble_pct: 0.26,
            tephra_pct: 0.02,
            organic_pct: 0.04,
            water_pct: 0.06,
            iron_oxide_pct: 0.02,
        },
        TerrainSurfaceClass::Gravel => MaterialSoilParameters {
            sand_pct: 0.20,
            silt_pct: 0.10,
            clay_pct: 0.05,
            gravel_pct: 0.38,
            pebble_pct: 0.20,
            tephra_pct: 0.02,
            organic_pct: 0.05,
            water_pct: 0.06,
            iron_oxide_pct: 0.02,
        },
        TerrainSurfaceClass::Sand | TerrainSurfaceClass::MarineSand => MaterialSoilParameters {
            sand_pct: 0.72,
            silt_pct: 0.12,
            clay_pct: 0.05,
            gravel_pct: 0.05,
            pebble_pct: 0.02,
            tephra_pct: 0.01,
            organic_pct: 0.03,
            water_pct: 0.05,
            iron_oxide_pct: 0.02,
        },
        TerrainSurfaceClass::Silt => MaterialSoilParameters {
            sand_pct: 0.18,
            silt_pct: 0.56,
            clay_pct: 0.14,
            gravel_pct: 0.05,
            pebble_pct: 0.02,
            tephra_pct: 0.02,
            organic_pct: 0.03,
            water_pct: 0.10,
            iron_oxide_pct: 0.025,
        },
        TerrainSurfaceClass::Clay | TerrainSurfaceClass::MarineMud => MaterialSoilParameters {
            sand_pct: 0.08,
            silt_pct: 0.26,
            clay_pct: 0.46,
            gravel_pct: 0.05,
            pebble_pct: 0.02,
            tephra_pct: 0.03,
            organic_pct: 0.10,
            water_pct: 0.14,
            iron_oxide_pct: 0.03,
        },
        TerrainSurfaceClass::Loam => MaterialSoilParameters::default(),
        TerrainSurfaceClass::Peat => MaterialSoilParameters {
            sand_pct: 0.10,
            silt_pct: 0.18,
            clay_pct: 0.12,
            gravel_pct: 0.03,
            pebble_pct: 0.01,
            tephra_pct: 0.01,
            organic_pct: 0.55,
            water_pct: 0.18,
            iron_oxide_pct: 0.01,
        },
        TerrainSurfaceClass::CarbonateShoal => MaterialSoilParameters {
            sand_pct: 0.62,
            silt_pct: 0.12,
            clay_pct: 0.06,
            gravel_pct: 0.10,
            pebble_pct: 0.03,
            tephra_pct: 0.0,
            organic_pct: 0.07,
            water_pct: 0.06,
            iron_oxide_pct: 0.01,
        },
        TerrainSurfaceClass::SnowIce => MaterialSoilParameters::default(),
    };

    soil.organic_pct = surface
        .organic_fraction
        .clamp(0.0, 0.95)
        .max(soil.organic_pct);
    soil.tephra_pct = surface.tephra_fraction.clamp(0.0, 1.0).max(soil.tephra_pct);
    soil.water_pct = surface.moisture.clamp(0.0, 1.0);
    soil.iron_oxide_pct = surface.iron_oxide_fraction.clamp(0.0, 0.12);

    if surface.carbonate_fraction > 0.0 {
        soil.sand_pct += surface.carbonate_fraction * 0.22;
        soil.silt_pct += surface.carbonate_fraction * 0.08;
    }

    normalize_soil_composition(&mut soil);
    soil.water_pct = soil.water_pct.clamp(0.0, soil.water_capacity());
    soil
}

fn normalize_soil_composition(soil: &mut MaterialSoilParameters) {
    let total = soil.sand_pct
        + soil.silt_pct
        + soil.clay_pct
        + soil.gravel_pct
        + soil.pebble_pct
        + soil.tephra_pct
        + soil.organic_pct;
    if total <= 1.0e-6 {
        *soil = MaterialSoilParameters::default();
        return;
    }
    soil.sand_pct /= total;
    soil.silt_pct /= total;
    soil.clay_pct /= total;
    soil.gravel_pct /= total;
    soil.pebble_pct /= total;
    soil.tephra_pct /= total;
    soil.organic_pct /= total;
}

fn stone_profile_for_bedrock_class(
    bedrock: TerrainBedrockInputs,
) -> (MaterialStoneGenesis, MaterialStoneParameters) {
    let fracture_density = bedrock.fracture_density.clamp(0.0, 1.0);
    let weathering = bedrock.weathering.clamp(0.0, 1.0);
    let oxide_staining = bedrock.oxide_staining.clamp(0.0, 1.0);
    let (class_genesis, mut params) = match bedrock.class {
        TerrainBedrockClass::Crystalline => (
            MaterialStoneGenesis::Metamorphic,
            MaterialStoneParameters {
                vein_content: 0.28,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.62,
                secondary: 0.24,
                tertiary: 0.18,
                quaternary: 0.82,
            },
        ),
        TerrainBedrockClass::Metamorphic => (
            MaterialStoneGenesis::Metamorphic,
            MaterialStoneParameters {
                vein_content: 0.22,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.68,
                secondary: 0.58,
                tertiary: 0.56,
                quaternary: 0.54,
            },
        ),
        TerrainBedrockClass::BasalticVolcanic => (
            MaterialStoneGenesis::Igneous,
            MaterialStoneParameters {
                vein_content: 0.16,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.72,
                secondary: 0.56,
                tertiary: 0.40,
                quaternary: 0.18,
            },
        ),
        TerrainBedrockClass::Volcaniclastic => (
            MaterialStoneGenesis::Igneous,
            MaterialStoneParameters {
                vein_content: 0.12,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.44,
                secondary: 0.36,
                tertiary: 0.42,
                quaternary: 0.76,
            },
        ),
        TerrainBedrockClass::OceanicBasalt => (
            MaterialStoneGenesis::Igneous,
            MaterialStoneParameters {
                vein_content: 0.10,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.82,
                secondary: 0.62,
                tertiary: 0.18,
                quaternary: 0.14,
            },
        ),
        TerrainBedrockClass::PassiveMarginSediment => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.12,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.42,
                secondary: 0.40,
                tertiary: 0.18,
                quaternary: 0.48,
            },
        ),
        TerrainBedrockClass::CarbonatePlatform => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.10,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.62,
                secondary: 0.52,
                tertiary: 0.82,
                quaternary: 0.58,
            },
        ),
        TerrainBedrockClass::Sandstone => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.08,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.64,
                secondary: 0.52,
                tertiary: 0.22,
                quaternary: 0.68,
            },
        ),
        TerrainBedrockClass::MudstoneShale => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.05,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.18,
                secondary: 0.32,
                tertiary: 0.18,
                quaternary: 0.48,
            },
        ),
        TerrainBedrockClass::Carbonate => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.10,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.46,
                secondary: 0.34,
                tertiary: 0.74,
                quaternary: 0.46,
            },
        ),
        TerrainBedrockClass::BasinFill => (
            MaterialStoneGenesis::Sedimentary,
            MaterialStoneParameters {
                vein_content: 0.04,
                fracture_density,
                oxide_staining,
                weathering,
                primary: 0.24,
                secondary: 0.24,
                tertiary: 0.08,
                quaternary: 0.16,
            },
        ),
    };
    if let Some(value) = bedrock.vein_content {
        params.vein_content = value.clamp(0.0, 1.0);
    }
    if let Some(value) = bedrock.primary {
        params.primary = value.clamp(0.0, 1.0);
    }
    if let Some(value) = bedrock.secondary {
        params.secondary = value.clamp(0.0, 1.0);
    }
    if let Some(value) = bedrock.tertiary {
        params.tertiary = value.clamp(0.0, 1.0);
    }
    if let Some(value) = bedrock.quaternary {
        params.quaternary = value.clamp(0.0, 1.0);
    }
    (bedrock.genesis.unwrap_or(class_genesis), params)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_nearly_eq(lhs: f32, rhs: f32) {
        assert!((lhs - rhs).abs() <= 1.0e-6, "lhs={lhs}, rhs={rhs}");
    }

    #[test]
    fn terrain_loam_surface_derives_soil_surface_and_bedrock() {
        let column = derive_terrain_material_column_recipe(TerrainMaterialInputs::default());
        assert_eq!(column.surface.variant, MaterialVariant::Soil);
        assert_eq!(column.subsurface.variant, MaterialVariant::Soil);
        assert_eq!(column.bedrock.variant, MaterialVariant::Stone);
        assert!(column.cover.is_none());
    }

    #[test]
    fn terrain_snow_cover_derives_ice_cover_recipe() {
        let inputs = TerrainMaterialInputs {
            cover: TerrainCoverInputs {
                class: TerrainCoverClass::Snow,
                ..TerrainCoverInputs::default()
            },
            ..TerrainMaterialInputs::default()
        };
        let column = derive_terrain_material_column_recipe(inputs);
        let cover = column.cover.expect("snow cover recipe");
        assert_eq!(cover.variant, MaterialVariant::Ice);
        assert_eq!(cover.optical_class, MaterialOpticalClass::Opaque);
    }

    #[test]
    fn terrain_oceanic_basalt_maps_to_stone_recipe() {
        let recipe = derive_terrain_material_column_recipe(TerrainMaterialInputs {
            bedrock: TerrainBedrockInputs {
                class: TerrainBedrockClass::OceanicBasalt,
                ..TerrainBedrockInputs::default()
            },
            ..TerrainMaterialInputs::default()
        })
        .bedrock;
        match recipe.parameters {
            MaterialRecipeParameters::Stone { lithology, .. } => {
                assert_eq!(lithology, MaterialStoneLithology::OceanicBasalt);
            }
            _ => panic!("expected stone recipe"),
        }
    }

    #[test]
    fn terrain_regolith_parameters_override_changes_derived_properties() {
        let default_surface =
            derive_terrain_material_column_recipe(TerrainMaterialInputs::default()).surface;
        let override_surface = derive_terrain_material_column_recipe(TerrainMaterialInputs {
            regolith: TerrainRegolithInputs {
                origin: MaterialRegolithOrigin::Residual,
                parameters: Some(MaterialRegolithParameters {
                    transport: 0.98,
                    water_influence: 0.04,
                    marine_influence: 0.82,
                    angularity: 0.92,
                }),
            },
            ..TerrainMaterialInputs::default()
        })
        .surface;
        assert_ne!(
            default_surface.common_properties,
            override_surface.common_properties
        );
    }

    #[test]
    fn terrain_bedrock_optional_overrides_are_used() {
        let recipe = derive_terrain_material_column_recipe(TerrainMaterialInputs {
            bedrock: TerrainBedrockInputs {
                class: TerrainBedrockClass::Sandstone,
                genesis: Some(MaterialStoneGenesis::Igneous),
                vein_content: Some(0.91),
                fracture_density: 0.77,
                weathering: 0.18,
                oxide_staining: 0.63,
                primary: Some(0.12),
                secondary: Some(0.34),
                tertiary: Some(0.56),
                quaternary: Some(0.78),
            },
            ..TerrainMaterialInputs::default()
        })
        .bedrock;
        match recipe.parameters {
            MaterialRecipeParameters::Stone {
                genesis, params, ..
            } => {
                assert_eq!(genesis, MaterialStoneGenesis::Igneous);
                assert_nearly_eq(params.vein_content, 0.91);
                assert_nearly_eq(params.fracture_density, 0.77);
                assert_nearly_eq(params.weathering, 0.18);
                assert_nearly_eq(params.oxide_staining, 0.63);
                assert_nearly_eq(params.primary, 0.12);
                assert_nearly_eq(params.secondary, 0.34);
                assert_nearly_eq(params.tertiary, 0.56);
                assert_nearly_eq(params.quaternary, 0.78);
            }
            _ => panic!("expected stone recipe"),
        }
    }

    #[test]
    fn derive_ceramic_recipe_uses_elemental_class() {
        let recipe = derive_ceramic_recipe(
            MaterialSoilParameters::default(),
            MaterialRegolithOrigin::Aeolian,
            MaterialRarityContext::default(),
        );
        assert_eq!(recipe.class, MaterialClass::Elemental);
        assert_eq!(recipe.variant, MaterialVariant::Ceramic);
    }

    #[test]
    fn derive_foliage_recipe_uses_simple_opaque_elemental_path() {
        let recipe = derive_material_recipe(
            MaterialVariant::Foliage,
            MaterialSoilParameters::default(),
            MaterialRarityContext::default(),
        );
        assert_eq!(recipe.class, MaterialClass::Elemental);
        assert_eq!(recipe.variant, MaterialVariant::Foliage);
        assert_eq!(
            recipe.orientation,
            MaterialVariant::Foliage.orientation_axis()
        );
        assert_eq!(recipe.optical_class, MaterialOpticalClass::Opaque);
        assert_eq!(recipe.parameters, MaterialRecipeParameters::None);
    }

    #[test]
    fn terrain_material_schema_versions_are_stable() {
        assert_eq!(TERRAIN_MATERIAL_INPUT_SCHEMA_VERSION, 1);
        assert_eq!(TERRAIN_MATERIAL_RECIPE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn terrain_class_encode_ids_are_stable() {
        let surface_ids: Vec<_> = TerrainSurfaceClass::ALL
            .into_iter()
            .map(TerrainSurfaceClass::encode_id)
            .collect();
        let bedrock_ids: Vec<_> = TerrainBedrockClass::ALL
            .into_iter()
            .map(TerrainBedrockClass::encode_id)
            .collect();
        let cover_ids: Vec<_> = TerrainCoverClass::ALL
            .into_iter()
            .map(TerrainCoverClass::encode_id)
            .collect();

        assert_eq!(surface_ids, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(bedrock_ids, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(cover_ids, vec![0, 1, 2, 3]);
    }
}
