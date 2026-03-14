use bevy::prelude::*;

pub mod uniform;
pub use uniform::{MaterialUniform, MaterialUniformPlugin};

pub const MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT: usize = 7;
pub const MATERIAL_SOIL_IRON_OXIDE_MAX: f32 = 0.12;
pub const MATERIAL_SOIL_MIN_WATER_CAPACITY: f32 = 0.04;
pub const MATERIAL_SOIL_MAX_WATER_CAPACITY: f32 = 0.34;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialOrientationAxis {
    X,
    #[default]
    Y,
    Z,
}

impl MaterialOrientationAxis {
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    pub const fn label(self) -> &'static str {
        match self {
            Self::X => "X-axis",
            Self::Y => "Y-axis",
            Self::Z => "Z-axis",
        }
    }

    pub const fn shader_id(self) -> u32 {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialPreviewLod {
    #[default]
    Auto,
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    Far,
}

impl MaterialPreviewLod {
    pub const ALL: [Self; 8] = [
        Self::Auto,
        Self::L0,
        Self::L1,
        Self::L2,
        Self::L3,
        Self::L4,
        Self::L5,
        Self::Far,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::L0 => "L0",
            Self::L1 => "L1",
            Self::L2 => "L2",
            Self::L3 => "L3",
            Self::L4 => "L4",
            Self::L5 => "L5",
            Self::Far => "Far",
        }
    }

    pub const fn shader_id(self) -> u32 {
        match self {
            Self::Auto => 0,
            Self::L0 => 1,
            Self::L1 => 2,
            Self::L2 => 3,
            Self::L3 => 4,
            Self::L4 => 5,
            Self::L5 => 6,
            Self::Far => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialClass {
    Elemental,
    Mixture,
}

impl MaterialClass {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Elemental => "Elemental",
            Self::Mixture => "Mixture",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialElement {
    Stone,
    Metal,
    Wood,
    Snow,
    Ice,
    Ceramic,
    Crystal,
    Glass,
    Carbonate,
    Tephra,
    Humus,
    IronOxide,
}

impl MaterialElement {
    pub const ALL: [Self; 12] = [
        Self::Stone,
        Self::Metal,
        Self::Wood,
        Self::Snow,
        Self::Ice,
        Self::Ceramic,
        Self::Crystal,
        Self::Glass,
        Self::Carbonate,
        Self::Tephra,
        Self::Humus,
        Self::IronOxide,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Stone => "Stone",
            Self::Metal => "Metal",
            Self::Wood => "Wood",
            Self::Snow => "Snow",
            Self::Ice => "Ice",
            Self::Ceramic => "Ceramic",
            Self::Crystal => "Crystal",
            Self::Glass => "Glass",
            Self::Carbonate => "Carbonate",
            Self::Tephra => "Tephra",
            Self::Humus => "Humus",
            Self::IronOxide => "Iron Oxide",
        }
    }

    pub const fn shader_id(self) -> u32 {
        match self {
            Self::Stone => 0,
            Self::Metal => 1,
            Self::Wood => 2,
            Self::Snow => 3,
            Self::Ice => 4,
            Self::Ceramic => 5,
            Self::Crystal => 6,
            Self::Glass => 7,
            Self::Carbonate => 8,
            Self::Tephra => 9,
            Self::Humus => 10,
            Self::IronOxide => 11,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialIceForm {
    Snow,
    #[default]
    GlacialIce,
}

impl MaterialIceForm {
    pub const ALL: [Self; 2] = [Self::Snow, Self::GlacialIce];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Snow => "Snow",
            Self::GlacialIce => "Glacial Ice",
        }
    }

    pub const fn benchmark_case_label(self) -> &'static str {
        match self {
            Self::Snow => "material-snow",
            Self::GlacialIce => "material-ice",
        }
    }

    pub const fn base_common_properties(self) -> MaterialCommonProperties {
        match self {
            Self::Snow => MaterialCommonProperties::new(
                80,
                40,
                100,
                50,
                60,
                240,
                60,
                280,
                100,
                0,
                100,
                820,
                6,
                MaterialMatterState::Solid,
            ),
            Self::GlacialIce => MaterialCommonProperties::new(
                160,
                220,
                280,
                100,
                120,
                300,
                80,
                300,
                220,
                20,
                220,
                100,
                7,
                MaterialMatterState::Solid,
            ),
        }
    }

    pub const fn parameters(self) -> MaterialIceParameters {
        match self {
            Self::Snow => MaterialIceParameters::new(0.18, 0.88, 0.04, MaterialElement::Stone),
            Self::GlacialIce => {
                MaterialIceParameters::new(0.92, 0.08, 0.02, MaterialElement::Stone)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialIceParameter {
    Compaction,
    AirContent,
    Impurity,
}

impl MaterialIceParameter {
    pub const ALL: [Self; 3] = [Self::Compaction, Self::AirContent, Self::Impurity];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Compaction => "Compaction",
            Self::AirContent => "Air Content",
            Self::Impurity => "Impurity",
        }
    }

    pub fn accent_color(self) -> Color {
        match self {
            Self::Compaction => Color::srgb(0.45, 0.72, 0.86),
            Self::AirContent => Color::srgb(0.86, 0.88, 0.92),
            Self::Impurity => Color::srgb(0.72, 0.46, 0.26),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialIceParameters {
    pub compaction_pct: u16,
    pub air_content_pct: u16,
    pub impurity_pct: u16,
    pub impurity_element: MaterialElement,
}

impl Default for MaterialIceParameters {
    fn default() -> Self {
        MaterialIceForm::default().parameters()
    }
}

impl MaterialIceParameters {
    pub const fn new(
        compaction: f32,
        air_content: f32,
        impurity: f32,
        impurity_element: MaterialElement,
    ) -> Self {
        Self {
            compaction_pct: Self::encode_unit_interval(compaction),
            air_content_pct: Self::encode_unit_interval(air_content),
            impurity_pct: Self::encode_unit_interval(impurity),
            impurity_element,
        }
    }

    const fn encode_unit_interval(value: f32) -> u16 {
        let scaled = (value * 1000.0) + 0.5;
        if scaled <= 0.0 {
            0
        } else if scaled >= 1000.0 {
            1000
        } else {
            scaled as u16
        }
    }

    const fn decode_unit_interval(value: u16) -> f32 {
        value as f32 / 1000.0
    }

    pub const fn value(self, parameter: MaterialIceParameter) -> f32 {
        match parameter {
            MaterialIceParameter::Compaction => Self::decode_unit_interval(self.compaction_pct),
            MaterialIceParameter::AirContent => Self::decode_unit_interval(self.air_content_pct),
            MaterialIceParameter::Impurity => Self::decode_unit_interval(self.impurity_pct),
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialIceParameter, value: f32) {
        let encoded = Self::encode_unit_interval(value.clamp(0.0, 1.0));
        match parameter {
            MaterialIceParameter::Compaction => self.compaction_pct = encoded,
            MaterialIceParameter::AirContent => self.air_content_pct = encoded,
            MaterialIceParameter::Impurity => self.impurity_pct = encoded,
        }
    }

    pub const fn compaction(self) -> f32 {
        Self::decode_unit_interval(self.compaction_pct)
    }

    pub const fn air_content(self) -> f32 {
        Self::decode_unit_interval(self.air_content_pct)
    }

    pub const fn impurity(self) -> f32 {
        Self::decode_unit_interval(self.impurity_pct)
    }

    pub fn snowiness(self) -> f32 {
        let compaction = self.compaction();
        let air = self.air_content();
        ((1.0 - compaction) * 0.38 + air * 0.78).clamp(0.0, 1.0)
    }

    pub fn clarity(self) -> f32 {
        let compaction = self.compaction();
        let air = self.air_content();
        let impurity = self.impurity();
        (compaction * (1.0 - air) * (1.0 - impurity * 0.65)).clamp(0.0, 1.0)
    }

    pub fn representative_form(self) -> MaterialIceForm {
        if self.snowiness() >= 0.68 {
            MaterialIceForm::Snow
        } else {
            MaterialIceForm::GlacialIce
        }
    }

    pub fn visual_label(self) -> &'static str {
        let snowiness = self.snowiness();
        let clarity = self.clarity();
        if snowiness >= 0.84 {
            "Fluffy Snow"
        } else if snowiness >= 0.62 {
            "Packed Snow"
        } else if self.compaction() >= 0.80 {
            "Dense Packed Ice"
        } else if clarity >= 0.72 {
            "Clear Lake Ice"
        } else {
            "Cloudy Firn"
        }
    }
}

pub fn material_ice_anchor_weights(ice: MaterialIceParameters) -> (f32, f32, f32) {
    let compaction = ice.compaction();
    let air = ice.air_content();
    let dense = (compaction * (0.35 + (1.0 - air) * 0.65)).clamp(0.0, 1.0);
    let snow =
        (air * (0.55 + (1.0 - compaction) * 0.45) + (1.0 - compaction) * 0.18).clamp(0.0, 1.0);
    let lake =
        (compaction * (1.0 - air) * (1.0 - ((compaction - 0.72).abs() / 0.42))).clamp(0.0, 1.0);
    let total = (dense + lake + snow).max(1.0e-6);
    (dense / total, lake / total, snow / total)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialStoneGenesis {
    #[default]
    Igneous,
    Sedimentary,
    Metamorphic,
}

impl MaterialStoneGenesis {
    pub const ALL: [Self; 3] = [Self::Igneous, Self::Sedimentary, Self::Metamorphic];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Igneous => "Igneous",
            Self::Sedimentary => "Sedimentary",
            Self::Metamorphic => "Metamorphic",
        }
    }

    pub const fn shader_id(self) -> u32 {
        match self {
            Self::Igneous => 0,
            Self::Sedimentary => 1,
            Self::Metamorphic => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialStoneParameter {
    VeinContent,
    FractureDensity,
    OxideStaining,
    Weathering,
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
}

impl MaterialStoneParameter {
    pub const COMMON: [Self; 4] = [
        Self::VeinContent,
        Self::FractureDensity,
        Self::OxideStaining,
        Self::Weathering,
    ];
    pub const SPECIFIC: [Self; 4] = [
        Self::Primary,
        Self::Secondary,
        Self::Tertiary,
        Self::Quaternary,
    ];

    pub const fn label(self, genesis: MaterialStoneGenesis) -> &'static str {
        match self {
            Self::VeinContent => "Vein Content",
            Self::FractureDensity => "Fracture Density",
            Self::OxideStaining => "Oxide Staining",
            Self::Weathering => "Weathering",
            Self::Primary => match genesis {
                MaterialStoneGenesis::Igneous => "Maficity",
                MaterialStoneGenesis::Sedimentary => "Grain Size",
                MaterialStoneGenesis::Metamorphic => "Grade",
            },
            Self::Secondary => match genesis {
                MaterialStoneGenesis::Igneous => "Cooling Rate",
                MaterialStoneGenesis::Sedimentary => "Sorting",
                MaterialStoneGenesis::Metamorphic => "Foliation",
            },
            Self::Tertiary => match genesis {
                MaterialStoneGenesis::Igneous => "Volatile Content",
                MaterialStoneGenesis::Sedimentary => "Carbonate Content",
                MaterialStoneGenesis::Metamorphic => "Banding",
            },
            Self::Quaternary => match genesis {
                MaterialStoneGenesis::Igneous => "Fragmental Content",
                MaterialStoneGenesis::Sedimentary => "Cementation",
                MaterialStoneGenesis::Metamorphic => "Recrystallization",
            },
        }
    }

    pub fn accent_color(self, genesis: MaterialStoneGenesis) -> Color {
        match self {
            Self::VeinContent => Color::srgb(0.70, 0.66, 0.58),
            Self::FractureDensity => Color::srgb(0.64, 0.38, 0.32),
            Self::OxideStaining => Color::srgb(0.76, 0.34, 0.18),
            Self::Weathering => Color::srgb(0.58, 0.50, 0.40),
            Self::Primary => match genesis {
                MaterialStoneGenesis::Igneous => Color::srgb(0.34, 0.38, 0.44),
                MaterialStoneGenesis::Sedimentary => Color::srgb(0.78, 0.64, 0.42),
                MaterialStoneGenesis::Metamorphic => Color::srgb(0.62, 0.50, 0.74),
            },
            Self::Secondary => match genesis {
                MaterialStoneGenesis::Igneous => Color::srgb(0.52, 0.62, 0.74),
                MaterialStoneGenesis::Sedimentary => Color::srgb(0.56, 0.70, 0.76),
                MaterialStoneGenesis::Metamorphic => Color::srgb(0.72, 0.58, 0.74),
            },
            Self::Tertiary => match genesis {
                MaterialStoneGenesis::Igneous => Color::srgb(0.80, 0.45, 0.18),
                MaterialStoneGenesis::Sedimentary => Color::srgb(0.90, 0.86, 0.72),
                MaterialStoneGenesis::Metamorphic => Color::srgb(0.54, 0.52, 0.68),
            },
            Self::Quaternary => match genesis {
                MaterialStoneGenesis::Igneous => Color::srgb(0.46, 0.26, 0.20),
                MaterialStoneGenesis::Sedimentary => Color::srgb(0.68, 0.58, 0.48),
                MaterialStoneGenesis::Metamorphic => Color::srgb(0.78, 0.74, 0.82),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialStoneParameters {
    pub vein_content: f32,
    pub fracture_density: f32,
    pub oxide_staining: f32,
    pub weathering: f32,
    pub primary: f32,
    pub secondary: f32,
    pub tertiary: f32,
    pub quaternary: f32,
}

impl Default for MaterialStoneParameters {
    fn default() -> Self {
        Self {
            vein_content: 0.34,
            fracture_density: 0.18,
            oxide_staining: 0.12,
            weathering: 0.16,
            primary: 0.22,
            secondary: 0.24,
            tertiary: 0.10,
            quaternary: 0.08,
        }
    }
}

impl MaterialStoneParameters {
    pub const fn value(self, parameter: MaterialStoneParameter) -> f32 {
        match parameter {
            MaterialStoneParameter::VeinContent => self.vein_content,
            MaterialStoneParameter::FractureDensity => self.fracture_density,
            MaterialStoneParameter::OxideStaining => self.oxide_staining,
            MaterialStoneParameter::Weathering => self.weathering,
            MaterialStoneParameter::Primary => self.primary,
            MaterialStoneParameter::Secondary => self.secondary,
            MaterialStoneParameter::Tertiary => self.tertiary,
            MaterialStoneParameter::Quaternary => self.quaternary,
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialStoneParameter, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        match parameter {
            MaterialStoneParameter::VeinContent => self.vein_content = clamped,
            MaterialStoneParameter::FractureDensity => self.fracture_density = clamped,
            MaterialStoneParameter::OxideStaining => self.oxide_staining = clamped,
            MaterialStoneParameter::Weathering => self.weathering = clamped,
            MaterialStoneParameter::Primary => self.primary = clamped,
            MaterialStoneParameter::Secondary => self.secondary = clamped,
            MaterialStoneParameter::Tertiary => self.tertiary = clamped,
            MaterialStoneParameter::Quaternary => self.quaternary = clamped,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialStoneLithology {
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

impl MaterialStoneLithology {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Crystalline => "Crystalline",
            Self::Metamorphic => "Metamorphic",
            Self::BasalticVolcanic => "Basaltic Volcanic",
            Self::Volcaniclastic => "Volcaniclastic",
            Self::OceanicBasalt => "Oceanic Basalt",
            Self::PassiveMarginSediment => "Passive Margin Sediment",
            Self::CarbonatePlatform => "Carbonate Platform",
            Self::Sandstone => "Sandstone",
            Self::MudstoneShale => "Mudstone/Shale",
            Self::Carbonate => "Carbonate",
            Self::BasinFill => "Basin Fill",
        }
    }

    pub const fn shader_id(self) -> u32 {
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
pub enum MaterialRegolithOrigin {
    #[default]
    Residual,
    Alluvial,
    Colluvial,
    Aeolian,
    Lacustrine,
    MarineShelf,
    MarinePelagic,
    EstuarineDeltaic,
    GlacialTill,
    Tephric,
    OrganicPeat,
    ShallowBedrock,
}

impl MaterialRegolithOrigin {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Residual => "Residual",
            Self::Alluvial => "Alluvial",
            Self::Colluvial => "Colluvial",
            Self::Aeolian => "Aeolian",
            Self::Lacustrine => "Lacustrine",
            Self::MarineShelf => "Marine Shelf",
            Self::MarinePelagic => "Marine Pelagic",
            Self::EstuarineDeltaic => "Estuarine/Deltaic",
            Self::GlacialTill => "Glacial Till",
            Self::Tephric => "Tephric",
            Self::OrganicPeat => "Organic Peat",
            Self::ShallowBedrock => "Shallow Bedrock",
        }
    }

    pub const fn shader_id(self) -> u32 {
        match self {
            Self::Residual => 0,
            Self::Alluvial => 1,
            Self::Colluvial => 2,
            Self::Aeolian => 3,
            Self::Lacustrine => 4,
            Self::MarineShelf => 5,
            Self::MarinePelagic => 6,
            Self::EstuarineDeltaic => 7,
            Self::GlacialTill => 8,
            Self::Tephric => 9,
            Self::OrganicPeat => 10,
            Self::ShallowBedrock => 11,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialRegolithParameter {
    Transport,
    WaterInfluence,
    MarineInfluence,
    Angularity,
}

impl MaterialRegolithParameter {
    pub const ALL: [Self; 4] = [
        Self::Transport,
        Self::WaterInfluence,
        Self::MarineInfluence,
        Self::Angularity,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Transport => "Transport",
            Self::WaterInfluence => "Water Influence",
            Self::MarineInfluence => "Marine Influence",
            Self::Angularity => "Angularity",
        }
    }

    pub fn accent_color(self) -> Color {
        match self {
            Self::Transport => Color::srgb(0.74, 0.62, 0.36),
            Self::WaterInfluence => Color::srgb(0.24, 0.48, 0.76),
            Self::MarineInfluence => Color::srgb(0.24, 0.66, 0.72),
            Self::Angularity => Color::srgb(0.58, 0.56, 0.54),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialRegolithParameters {
    pub transport: f32,
    pub water_influence: f32,
    pub marine_influence: f32,
    pub angularity: f32,
}

impl Default for MaterialRegolithParameters {
    fn default() -> Self {
        Self {
            transport: 0.18,
            water_influence: 0.18,
            marine_influence: 0.0,
            angularity: 0.18,
        }
    }
}

impl MaterialRegolithParameters {
    pub const fn value(self, parameter: MaterialRegolithParameter) -> f32 {
        match parameter {
            MaterialRegolithParameter::Transport => self.transport,
            MaterialRegolithParameter::WaterInfluence => self.water_influence,
            MaterialRegolithParameter::MarineInfluence => self.marine_influence,
            MaterialRegolithParameter::Angularity => self.angularity,
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialRegolithParameter, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        match parameter {
            MaterialRegolithParameter::Transport => self.transport = clamped,
            MaterialRegolithParameter::WaterInfluence => self.water_influence = clamped,
            MaterialRegolithParameter::MarineInfluence => self.marine_influence = clamped,
            MaterialRegolithParameter::Angularity => self.angularity = clamped,
        }
    }
}

pub fn material_derive_regolith_origin(
    soil: MaterialSoilParameters,
    params: MaterialRegolithParameters,
) -> MaterialRegolithOrigin {
    let saturation = soil.water_saturation();
    if soil.organic_pct >= 0.20 && (saturation >= 0.32 || params.water_influence >= 0.45) {
        return MaterialRegolithOrigin::OrganicPeat;
    }
    if soil.tephra_pct >= 0.14 {
        return MaterialRegolithOrigin::Tephric;
    }
    if params.marine_influence >= 0.74 {
        if params.water_influence >= 0.70 && params.transport >= 0.60 {
            return MaterialRegolithOrigin::EstuarineDeltaic;
        }
        if params.water_influence >= 0.42 {
            return MaterialRegolithOrigin::MarineShelf;
        }
        return MaterialRegolithOrigin::MarinePelagic;
    }
    if params.angularity >= 0.82 && params.transport <= 0.30 {
        return MaterialRegolithOrigin::ShallowBedrock;
    }
    if params.angularity >= 0.62 {
        if params.transport >= 0.46 {
            return MaterialRegolithOrigin::GlacialTill;
        }
        return MaterialRegolithOrigin::Colluvial;
    }
    if params.water_influence >= 0.72 {
        if params.transport >= 0.56 {
            return MaterialRegolithOrigin::Alluvial;
        }
        return MaterialRegolithOrigin::Lacustrine;
    }
    if params.transport >= 0.68 && params.water_influence <= 0.34 {
        return MaterialRegolithOrigin::Aeolian;
    }
    MaterialRegolithOrigin::Residual
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialSoilElementBindings {
    pub sand: MaterialElement,
    pub silt: MaterialElement,
    pub clay: MaterialElement,
    pub gravel: MaterialElement,
    pub pebble: MaterialElement,
    pub tephra: MaterialElement,
    pub organic: MaterialElement,
    pub iron_oxide: MaterialElement,
}

impl Default for MaterialSoilElementBindings {
    fn default() -> Self {
        Self {
            sand: MaterialElement::Stone,
            silt: MaterialElement::Stone,
            clay: MaterialElement::Stone,
            gravel: MaterialElement::Stone,
            pebble: MaterialElement::Stone,
            tephra: MaterialElement::Tephra,
            organic: MaterialElement::Humus,
            iron_oxide: MaterialElement::IronOxide,
        }
    }
}

impl MaterialSoilElementBindings {
    pub const fn from_regolith_origin(origin: MaterialRegolithOrigin) -> Self {
        let mineral = match origin {
            MaterialRegolithOrigin::MarineShelf | MaterialRegolithOrigin::MarinePelagic => {
                MaterialElement::Carbonate
            }
            _ => MaterialElement::Stone,
        };
        Self {
            sand: mineral,
            silt: mineral,
            clay: mineral,
            gravel: mineral,
            pebble: mineral,
            tephra: MaterialElement::Tephra,
            organic: MaterialElement::Humus,
            iron_oxide: MaterialElement::IronOxide,
        }
    }
}

impl MaterialSoilElementBindings {
    pub const fn composition_element(
        self,
        parameter: MaterialSoilParameter,
    ) -> Option<MaterialElement> {
        match parameter {
            MaterialSoilParameter::Sand => Some(self.sand),
            MaterialSoilParameter::Silt => Some(self.silt),
            MaterialSoilParameter::Clay => Some(self.clay),
            MaterialSoilParameter::Gravel => Some(self.gravel),
            MaterialSoilParameter::Pebble => Some(self.pebble),
            MaterialSoilParameter::Tephra => Some(self.tephra),
            MaterialSoilParameter::Organic => Some(self.organic),
            MaterialSoilParameter::Water | MaterialSoilParameter::IronOxide => None,
        }
    }

    pub fn summary(self) -> String {
        let sand = self
            .composition_element(MaterialSoilParameter::Sand)
            .expect("sand binding must be present")
            .label();
        let silt = self
            .composition_element(MaterialSoilParameter::Silt)
            .expect("silt binding must be present")
            .label();
        let clay = self
            .composition_element(MaterialSoilParameter::Clay)
            .expect("clay binding must be present")
            .label();
        let gravel = self
            .composition_element(MaterialSoilParameter::Gravel)
            .expect("gravel binding must be present")
            .label();
        let pebble = self
            .composition_element(MaterialSoilParameter::Pebble)
            .expect("pebble binding must be present")
            .label();
        let organic = self
            .composition_element(MaterialSoilParameter::Organic)
            .expect("organic binding must be present")
            .label();
        let tephra = self
            .composition_element(MaterialSoilParameter::Tephra)
            .expect("tephra binding must be present")
            .label();
        format!(
            "sand -> {}  silt -> {}  clay -> {}  gravel -> {}  pebble -> {}  tephra -> {}  organic -> {}  oxide -> {}",
            sand,
            silt,
            clay,
            gravel,
            pebble,
            tephra,
            organic,
            self.iron_oxide.label(),
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialMatterState {
    #[default]
    Solid,
    Liquid,
    Gas,
    Plasma,
    Powder,
}

impl MaterialMatterState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Solid => "Solid",
            Self::Liquid => "Liquid",
            Self::Gas => "Gas",
            Self::Plasma => "Plasma",
            Self::Powder => "Powder",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialCommonProperty {
    Toughness,
    Hardness,
    CompressiveStrength,
    TensileStrength,
    Elasticity,
    HeatCapacity,
    MeltingPoint,
    BoilingPoint,
    ThermalConductivity,
    ElectricalConductivity,
    Density,
    Porosity,
    Ph,
}

impl MaterialCommonProperty {
    pub const MECHANICAL: [Self; 5] = [
        Self::Toughness,
        Self::Hardness,
        Self::CompressiveStrength,
        Self::TensileStrength,
        Self::Elasticity,
    ];

    pub const THERMAL: [Self; 4] = [
        Self::HeatCapacity,
        Self::MeltingPoint,
        Self::BoilingPoint,
        Self::ThermalConductivity,
    ];

    pub const PHYSICAL: [Self; 3] = [Self::ElectricalConductivity, Self::Density, Self::Porosity];

    pub const CHEMICAL: [Self; 1] = [Self::Ph];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Toughness => "Toughness",
            Self::Hardness => "Hardness",
            Self::CompressiveStrength => "Compressive Strength",
            Self::TensileStrength => "Tensile Strength",
            Self::Elasticity => "Elasticity",
            Self::HeatCapacity => "Heat Capacity",
            Self::MeltingPoint => "Melting Point",
            Self::BoilingPoint => "Boiling Point",
            Self::ThermalConductivity => "Thermal Conductivity",
            Self::ElectricalConductivity => "Electrical Conductivity",
            Self::Density => "Density",
            Self::Porosity => "Porosity",
            Self::Ph => "pH",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialCommonProperties {
    pub toughness: u64,
    pub hardness: u64,
    pub compressive_strength: u64,
    pub tensile_strength: u64,
    pub elasticity: u64,
    pub heat_capacity: u64,
    pub melting_point: u64,
    pub boiling_point: u64,
    pub thermal_conductivity: u64,
    pub electrical_conductivity: u64,
    pub density: u64,
    pub porosity: u64,
    pub ph: u64,
    pub state: MaterialMatterState,
}

impl MaterialCommonProperties {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        toughness: u64,
        hardness: u64,
        compressive_strength: u64,
        tensile_strength: u64,
        elasticity: u64,
        heat_capacity: u64,
        melting_point: u64,
        boiling_point: u64,
        thermal_conductivity: u64,
        electrical_conductivity: u64,
        density: u64,
        porosity: u64,
        ph: u64,
        state: MaterialMatterState,
    ) -> Self {
        Self {
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
        }
    }

    pub const fn value(self, property: MaterialCommonProperty) -> u64 {
        match property {
            MaterialCommonProperty::Toughness => self.toughness,
            MaterialCommonProperty::Hardness => self.hardness,
            MaterialCommonProperty::CompressiveStrength => self.compressive_strength,
            MaterialCommonProperty::TensileStrength => self.tensile_strength,
            MaterialCommonProperty::Elasticity => self.elasticity,
            MaterialCommonProperty::HeatCapacity => self.heat_capacity,
            MaterialCommonProperty::MeltingPoint => self.melting_point,
            MaterialCommonProperty::BoilingPoint => self.boiling_point,
            MaterialCommonProperty::ThermalConductivity => self.thermal_conductivity,
            MaterialCommonProperty::ElectricalConductivity => self.electrical_conductivity,
            MaterialCommonProperty::Density => self.density,
            MaterialCommonProperty::Porosity => self.porosity,
            MaterialCommonProperty::Ph => self.ph,
        }
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialContextParameter {
    RarityBits,
    Seed,
    Depth,
    CenterProximity,
    NorthSouthAxis,
    Height,
    RadialDistance,
    CraftQuality,
}

impl MaterialContextParameter {
    pub const ALL: [Self; 8] = [
        Self::RarityBits,
        Self::Seed,
        Self::Depth,
        Self::CenterProximity,
        Self::NorthSouthAxis,
        Self::Height,
        Self::RadialDistance,
        Self::CraftQuality,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RarityBits => "Rarity",
            Self::Seed => "Seed",
            Self::Depth => "Depth",
            Self::CenterProximity => "Center",
            Self::NorthSouthAxis => "North/South",
            Self::Height => "Height",
            Self::RadialDistance => "Radial",
            Self::CraftQuality => "Craft Quality",
        }
    }

    pub fn accent_color(self) -> Color {
        match self {
            Self::RarityBits | Self::Seed => Color::srgb(0.78, 0.56, 0.20),
            Self::Depth | Self::RadialDistance => Color::srgb(0.53, 0.62, 0.77),
            Self::CenterProximity | Self::NorthSouthAxis | Self::Height => {
                Color::srgb(0.43, 0.68, 0.52)
            }
            Self::CraftQuality => Color::srgb(0.76, 0.42, 0.32),
        }
    }

    pub const fn min_value(self) -> f64 {
        match self {
            Self::NorthSouthAxis => -1.0,
            _ => 0.0,
        }
    }

    pub const fn max_value(self) -> f64 {
        match self {
            Self::RarityBits => 24.0,
            Self::Seed => 4095.0,
            Self::Depth
            | Self::CenterProximity
            | Self::Height
            | Self::RadialDistance
            | Self::CraftQuality
            | Self::NorthSouthAxis => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialRarityContext {
    pub rarity_bits: f64,
    pub seed: u64,
    pub depth: f64,
    pub center_proximity: f64,
    pub north_south_axis: f64,
    pub height: f64,
    pub radial_distance: f64,
    pub craft_quality: f64,
}

impl Default for MaterialRarityContext {
    fn default() -> Self {
        Self {
            rarity_bits: 0.0,
            seed: 0,
            depth: 0.0,
            center_proximity: 0.0,
            north_south_axis: 0.0,
            height: 0.0,
            radial_distance: 0.0,
            craft_quality: 0.0,
        }
    }
}

impl MaterialRarityContext {
    pub fn value(self, parameter: MaterialContextParameter) -> f64 {
        match parameter {
            MaterialContextParameter::RarityBits => self.rarity_bits,
            MaterialContextParameter::Seed => self.seed as f64,
            MaterialContextParameter::Depth => self.depth,
            MaterialContextParameter::CenterProximity => self.center_proximity,
            MaterialContextParameter::NorthSouthAxis => self.north_south_axis,
            MaterialContextParameter::Height => self.height,
            MaterialContextParameter::RadialDistance => self.radial_distance,
            MaterialContextParameter::CraftQuality => self.craft_quality,
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialContextParameter, value: f64) {
        let clamped = value.clamp(parameter.min_value(), parameter.max_value());
        match parameter {
            MaterialContextParameter::RarityBits => self.rarity_bits = clamped,
            MaterialContextParameter::Seed => self.seed = clamped.round() as u64,
            MaterialContextParameter::Depth => self.depth = clamped,
            MaterialContextParameter::CenterProximity => self.center_proximity = clamped,
            MaterialContextParameter::NorthSouthAxis => self.north_south_axis = clamped,
            MaterialContextParameter::Height => self.height = clamped,
            MaterialContextParameter::RadialDistance => self.radial_distance = clamped,
            MaterialContextParameter::CraftQuality => self.craft_quality = clamped,
        }
    }

    pub fn northness(self) -> f64 {
        self.north_south_axis.max(0.0)
    }

    pub fn southness(self) -> f64 {
        (-self.north_south_axis).max(0.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MaterialDebugPropertyOverrides {
    pub toughness: i64,
    pub hardness: i64,
    pub compressive_strength: i64,
    pub tensile_strength: i64,
    pub elasticity: i64,
    pub heat_capacity: i64,
    pub melting_point: i64,
    pub boiling_point: i64,
    pub thermal_conductivity: i64,
    pub electrical_conductivity: i64,
    pub density: i64,
    pub porosity: i64,
    pub ph: i64,
}

impl MaterialDebugPropertyOverrides {
    pub const fn value(self, property: MaterialCommonProperty) -> i64 {
        match property {
            MaterialCommonProperty::Toughness => self.toughness,
            MaterialCommonProperty::Hardness => self.hardness,
            MaterialCommonProperty::CompressiveStrength => self.compressive_strength,
            MaterialCommonProperty::TensileStrength => self.tensile_strength,
            MaterialCommonProperty::Elasticity => self.elasticity,
            MaterialCommonProperty::HeatCapacity => self.heat_capacity,
            MaterialCommonProperty::MeltingPoint => self.melting_point,
            MaterialCommonProperty::BoilingPoint => self.boiling_point,
            MaterialCommonProperty::ThermalConductivity => self.thermal_conductivity,
            MaterialCommonProperty::ElectricalConductivity => self.electrical_conductivity,
            MaterialCommonProperty::Density => self.density,
            MaterialCommonProperty::Porosity => self.porosity,
            MaterialCommonProperty::Ph => self.ph,
        }
    }

    pub fn set_value(&mut self, property: MaterialCommonProperty, value: i64) {
        let clamped = value.clamp(
            -material_debug_override_abs_max(property),
            material_debug_override_abs_max(property),
        );
        match property {
            MaterialCommonProperty::Toughness => self.toughness = clamped,
            MaterialCommonProperty::Hardness => self.hardness = clamped,
            MaterialCommonProperty::CompressiveStrength => self.compressive_strength = clamped,
            MaterialCommonProperty::TensileStrength => self.tensile_strength = clamped,
            MaterialCommonProperty::Elasticity => self.elasticity = clamped,
            MaterialCommonProperty::HeatCapacity => self.heat_capacity = clamped,
            MaterialCommonProperty::MeltingPoint => self.melting_point = clamped,
            MaterialCommonProperty::BoilingPoint => self.boiling_point = clamped,
            MaterialCommonProperty::ThermalConductivity => self.thermal_conductivity = clamped,
            MaterialCommonProperty::ElectricalConductivity => {
                self.electrical_conductivity = clamped
            }
            MaterialCommonProperty::Density => self.density = clamped,
            MaterialCommonProperty::Porosity => self.porosity = clamped,
            MaterialCommonProperty::Ph => self.ph = clamped,
        }
    }

    pub fn apply(self, properties: MaterialCommonProperties) -> MaterialCommonProperties {
        MaterialCommonProperties {
            toughness: material_add_signed_u64(properties.toughness, self.toughness),
            hardness: material_add_signed_u64(properties.hardness, self.hardness),
            compressive_strength: material_add_signed_u64(
                properties.compressive_strength,
                self.compressive_strength,
            ),
            tensile_strength: material_add_signed_u64(
                properties.tensile_strength,
                self.tensile_strength,
            ),
            elasticity: material_add_signed_u64(properties.elasticity, self.elasticity),
            heat_capacity: material_add_signed_u64(properties.heat_capacity, self.heat_capacity),
            melting_point: material_add_signed_u64(properties.melting_point, self.melting_point),
            boiling_point: material_add_signed_u64(properties.boiling_point, self.boiling_point),
            thermal_conductivity: material_add_signed_u64(
                properties.thermal_conductivity,
                self.thermal_conductivity,
            ),
            electrical_conductivity: material_add_signed_u64(
                properties.electrical_conductivity,
                self.electrical_conductivity,
            ),
            density: material_add_signed_u64(properties.density, self.density),
            porosity: material_add_signed_u64(properties.porosity, self.porosity),
            ph: material_add_signed_u64(properties.ph, self.ph).clamp(0, 14),
            state: properties.state,
        }
    }

    pub fn any_active(self) -> bool {
        self != Self::default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialSoilParameter {
    Sand,
    Silt,
    Clay,
    Gravel,
    Pebble,
    Tephra,
    Organic,
    Water,
    IronOxide,
}

impl MaterialSoilParameter {
    pub const COMPOSITION: [Self; MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT] = [
        Self::Sand,
        Self::Silt,
        Self::Clay,
        Self::Gravel,
        Self::Pebble,
        Self::Tephra,
        Self::Organic,
    ];

    pub const MODIFIERS: [Self; 2] = [Self::Water, Self::IronOxide];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Sand => "Sand",
            Self::Silt => "Silt",
            Self::Clay => "Clay",
            Self::Gravel => "Gravel",
            Self::Pebble => "Pebble",
            Self::Tephra => "Tephra",
            Self::Organic => "Organic",
            Self::Water => "Water",
            Self::IronOxide => "Iron Oxide",
        }
    }

    pub fn accent_color(self) -> Color {
        match self {
            Self::Sand => Color::srgb(0.82, 0.68, 0.42),
            Self::Silt => Color::srgb(0.66, 0.58, 0.48),
            Self::Clay => Color::srgb(0.72, 0.36, 0.24),
            Self::Gravel => Color::srgb(0.50, 0.52, 0.54),
            Self::Pebble => Color::srgb(0.62, 0.56, 0.48),
            Self::Tephra => Color::srgb(0.38, 0.34, 0.36),
            Self::Organic => Color::srgb(0.26, 0.19, 0.12),
            Self::Water => Color::srgb(0.18, 0.46, 0.76),
            Self::IronOxide => Color::srgb(0.74, 0.33, 0.18),
        }
    }

    const fn composition_index(self) -> Option<usize> {
        match self {
            Self::Sand => Some(0),
            Self::Silt => Some(1),
            Self::Clay => Some(2),
            Self::Gravel => Some(3),
            Self::Pebble => Some(4),
            Self::Tephra => Some(5),
            Self::Organic => Some(6),
            Self::Water | Self::IronOxide => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialSoilParameters {
    pub sand_pct: f32,
    pub silt_pct: f32,
    pub clay_pct: f32,
    pub gravel_pct: f32,
    pub pebble_pct: f32,
    pub tephra_pct: f32,
    pub organic_pct: f32,
    pub water_pct: f32,
    pub iron_oxide_pct: f32,
}

impl Default for MaterialSoilParameters {
    fn default() -> Self {
        Self {
            sand_pct: 0.40,
            silt_pct: 0.28,
            clay_pct: 0.18,
            gravel_pct: 0.06,
            pebble_pct: 0.04,
            tephra_pct: 0.02,
            organic_pct: 0.02,
            water_pct: 0.08,
            iron_oxide_pct: 0.025,
        }
    }
}

impl MaterialSoilParameters {
    pub const fn value(self, parameter: MaterialSoilParameter) -> f32 {
        match parameter {
            MaterialSoilParameter::Sand => self.sand_pct,
            MaterialSoilParameter::Silt => self.silt_pct,
            MaterialSoilParameter::Clay => self.clay_pct,
            MaterialSoilParameter::Gravel => self.gravel_pct,
            MaterialSoilParameter::Pebble => self.pebble_pct,
            MaterialSoilParameter::Tephra => self.tephra_pct,
            MaterialSoilParameter::Organic => self.organic_pct,
            MaterialSoilParameter::Water => self.water_pct,
            MaterialSoilParameter::IronOxide => self.iron_oxide_pct,
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialSoilParameter, value: f32) {
        if let Some(selected_index) = parameter.composition_index() {
            let clamped_value = value.clamp(0.0, 1.0);
            let mut components = self.composition_array();
            let other_total = components
                .iter()
                .enumerate()
                .filter_map(|(index, component)| (index != selected_index).then_some(*component))
                .sum::<f32>();
            let remaining = 1.0 - clamped_value;

            components[selected_index] = clamped_value;
            if other_total > 1.0e-6 {
                let scale = remaining / other_total;
                for (index, component) in components.iter_mut().enumerate() {
                    if index != selected_index {
                        *component *= scale;
                    }
                }
            } else {
                let fill = remaining / (MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT as f32 - 1.0);
                for (index, component) in components.iter_mut().enumerate() {
                    if index != selected_index {
                        *component = fill;
                    }
                }
            }

            let total = components.iter().sum::<f32>();
            if total > 1.0e-6 {
                for component in &mut components {
                    *component /= total;
                }
            }
            self.set_composition_from_array(components);
            self.clamp_modifier_ranges();
            return;
        }

        match parameter {
            MaterialSoilParameter::Water => {
                self.water_pct = value.clamp(0.0, self.water_capacity());
            }
            MaterialSoilParameter::IronOxide => {
                self.iron_oxide_pct = value.clamp(0.0, MATERIAL_SOIL_IRON_OXIDE_MAX);
            }
            _ => {}
        }
    }

    pub fn composition_total(self) -> f32 {
        self.composition_array().into_iter().sum()
    }

    pub fn water_capacity(self) -> f32 {
        (MATERIAL_SOIL_MIN_WATER_CAPACITY
            + self.sand_pct * 0.02
            + self.silt_pct * 0.14
            + self.clay_pct * 0.32
            + self.tephra_pct * 0.26
            + self.organic_pct * 1.60
            - self.gravel_pct * 0.18
            - self.pebble_pct * 0.22)
            .clamp(
                MATERIAL_SOIL_MIN_WATER_CAPACITY,
                MATERIAL_SOIL_MAX_WATER_CAPACITY,
            )
    }

    pub fn water_saturation(self) -> f32 {
        (self.water_pct / self.water_capacity()).clamp(0.0, 1.0)
    }

    pub fn category_label(self) -> String {
        let coarse_total = self.gravel_pct + self.pebble_pct;
        let moisture = self.water_saturation();
        if moisture >= 0.68
            && (self.silt_pct + self.clay_pct + self.organic_pct) >= 0.38
            && coarse_total < 0.42
        {
            return "Mud".to_string();
        }
        if self.gravel_pct >= 0.45 {
            return "Gravel".to_string();
        }
        if self.pebble_pct >= 0.45 {
            return "Pebbles".to_string();
        }

        let matrix_total =
            (self.sand_pct + self.silt_pct + self.clay_pct + self.tephra_pct).max(0.0001);
        let sand = self.sand_pct / matrix_total;
        let silt = self.silt_pct / matrix_total;
        let clay = self.clay_pct / matrix_total;
        let tephra = self.tephra_pct / matrix_total;

        let base = if tephra >= 0.20 {
            "Tephric Loam"
        } else if clay >= 0.40 && sand < 0.45 {
            "Clay"
        } else if sand >= 0.70 && clay < 0.15 {
            "Sand"
        } else if silt >= 0.65 && clay < 0.20 {
            "Silt"
        } else if sand >= 0.52 && clay <= 0.20 {
            "Sand Loam"
        } else if silt >= 0.50 && clay <= 0.27 {
            "Silty Loam"
        } else if clay >= 0.27 {
            "Clay Loam"
        } else {
            "Loam"
        };

        if coarse_total >= 0.32 {
            let prefix = if self.gravel_pct >= self.pebble_pct {
                "Gravelly "
            } else {
                "Pebbly "
            };
            return format!("{prefix}{base}");
        }

        base.to_string()
    }

    pub fn parameter_max(self, parameter: MaterialSoilParameter) -> f32 {
        match parameter {
            MaterialSoilParameter::Water => self.water_capacity(),
            MaterialSoilParameter::IronOxide => MATERIAL_SOIL_IRON_OXIDE_MAX,
            _ => 1.0,
        }
    }

    pub fn composition_array(self) -> [f32; MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT] {
        [
            self.sand_pct,
            self.silt_pct,
            self.clay_pct,
            self.gravel_pct,
            self.pebble_pct,
            self.tephra_pct,
            self.organic_pct,
        ]
    }

    fn set_composition_from_array(
        &mut self,
        values: [f32; MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT],
    ) {
        self.sand_pct = values[0];
        self.silt_pct = values[1];
        self.clay_pct = values[2];
        self.gravel_pct = values[3];
        self.pebble_pct = values[4];
        self.tephra_pct = values[5];
        self.organic_pct = values[6];
    }

    fn clamp_modifier_ranges(&mut self) {
        self.water_pct = self.water_pct.clamp(0.0, self.water_capacity());
        self.iron_oxide_pct = self.iron_oxide_pct.clamp(0.0, MATERIAL_SOIL_IRON_OXIDE_MAX);
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MaterialVariant {
    Stone,
    Metal,
    #[default]
    Wood,
    Glass,
    Ice,
    Ceramic,
    Crystal,
    Soil,
}

impl MaterialVariant {
    pub const ALL: [Self; 8] = [
        Self::Stone,
        Self::Metal,
        Self::Wood,
        Self::Glass,
        Self::Ice,
        Self::Ceramic,
        Self::Crystal,
        Self::Soil,
    ];

    pub const ELEMENTAL: [Self; 7] = [
        Self::Stone,
        Self::Metal,
        Self::Wood,
        Self::Glass,
        Self::Ice,
        Self::Ceramic,
        Self::Crystal,
    ];

    pub const MIXTURES: [Self; 1] = [Self::Soil];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Stone => "Stone",
            Self::Metal => "Metal",
            Self::Wood => "Wood",
            Self::Glass => "Glass",
            Self::Ice => "Ice",
            Self::Ceramic => "Ceramic",
            Self::Crystal => "Crystal",
            Self::Soil => "Soil",
        }
    }

    pub const fn material_class(self) -> MaterialClass {
        match self {
            Self::Soil => MaterialClass::Mixture,
            Self::Stone
            | Self::Metal
            | Self::Wood
            | Self::Glass
            | Self::Ice
            | Self::Ceramic
            | Self::Crystal => MaterialClass::Elemental,
        }
    }

    pub const fn example_label(self) -> &'static str {
        match self {
            Self::Stone => "Granite Vein",
            Self::Metal => "Weathered Iron",
            Self::Wood => "Pine End-Grain",
            Self::Glass => "Tinted Float Glass",
            Self::Ice => "Blue Glacial Ice",
            Self::Ceramic => "Speckled Terracotta",
            Self::Crystal => "Amethyst Banding",
            Self::Soil => "Rich Garden Soil",
        }
    }

    pub const fn benchmark_case_label(self) -> &'static str {
        match self {
            Self::Stone => "material-stone",
            Self::Metal => "material-metal",
            Self::Wood => "material-wood",
            Self::Glass => "material-glass",
            Self::Ice => "material-ice",
            Self::Ceramic => "material-ceramic",
            Self::Crystal => "material-crystal",
            Self::Soil => "material-soil",
        }
    }

    pub const fn orientation_axis(self) -> MaterialOrientationAxis {
        match self {
            Self::Stone => MaterialOrientationAxis::Y,
            Self::Metal => MaterialOrientationAxis::Z,
            Self::Wood => MaterialOrientationAxis::Y,
            Self::Glass => MaterialOrientationAxis::Y,
            Self::Ice => MaterialOrientationAxis::Y,
            Self::Ceramic => MaterialOrientationAxis::Y,
            Self::Crystal => MaterialOrientationAxis::Z,
            Self::Soil => MaterialOrientationAxis::Y,
        }
    }

    pub const fn shader_family_id(self) -> u32 {
        match self {
            Self::Stone => 0,
            Self::Metal => 1,
            Self::Wood => 2,
            Self::Ice => 4,
            Self::Ceramic => 5,
            Self::Crystal => 6,
            Self::Glass => 7,
            Self::Soil => 8,
        }
    }

    pub const fn base_common_properties(self) -> MaterialCommonProperties {
        match self {
            Self::Stone => MaterialCommonProperties::new(
                620,
                740,
                880,
                260,
                180,
                460,
                920,
                980,
                380,
                40,
                680,
                180,
                8,
                MaterialMatterState::Solid,
            ),
            Self::Metal => MaterialCommonProperties::new(
                840,
                720,
                830,
                780,
                440,
                420,
                880,
                960,
                860,
                940,
                860,
                40,
                7,
                MaterialMatterState::Solid,
            ),
            Self::Wood => MaterialCommonProperties::new(
                580,
                280,
                420,
                550,
                480,
                580,
                340,
                460,
                120,
                50,
                340,
                520,
                6,
                MaterialMatterState::Solid,
            ),
            Self::Glass => MaterialCommonProperties::new(
                220,
                700,
                740,
                120,
                100,
                400,
                760,
                900,
                240,
                20,
                580,
                10,
                7,
                MaterialMatterState::Solid,
            ),
            Self::Ice => MaterialCommonProperties::new(
                160,
                220,
                280,
                100,
                120,
                300,
                80,
                300,
                220,
                20,
                220,
                100,
                7,
                MaterialMatterState::Solid,
            ),
            Self::Ceramic => MaterialCommonProperties::new(
                260,
                780,
                860,
                180,
                80,
                380,
                940,
                990,
                200,
                10,
                620,
                90,
                7,
                MaterialMatterState::Solid,
            ),
            Self::Crystal => MaterialCommonProperties::new(
                200,
                820,
                720,
                160,
                90,
                340,
                900,
                980,
                180,
                40,
                640,
                30,
                7,
                MaterialMatterState::Solid,
            ),
            Self::Soil => MaterialCommonProperties::new(
                180,
                80,
                160,
                120,
                220,
                560,
                280,
                420,
                140,
                60,
                360,
                480,
                7,
                MaterialMatterState::Powder,
            ),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn derived_common_properties(
        self,
        soil_params: MaterialSoilParameters,
        rarity_context: MaterialRarityContext,
    ) -> MaterialCommonProperties {
        derive_material_common_properties(self, soil_params, rarity_context)
    }
}

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct MaterialViewerState {
    pub selected_variant: MaterialVariant,
    pub ice_params: MaterialIceParameters,
    pub selected_stone_genesis: MaterialStoneGenesis,
    pub stone_params: MaterialStoneParameters,
    pub regolith_params: MaterialRegolithParameters,
    pub selected_orientation: MaterialOrientationAxis,
    pub preview_lod: MaterialPreviewLod,
    pub soil_params: MaterialSoilParameters,
    pub rarity_context: MaterialRarityContext,
    pub debug_property_overrides: MaterialDebugPropertyOverrides,
}

impl Default for MaterialViewerState {
    fn default() -> Self {
        Self {
            selected_variant: MaterialVariant::Wood,
            ice_params: MaterialIceParameters::default(),
            selected_stone_genesis: MaterialStoneGenesis::default(),
            stone_params: MaterialStoneParameters::default(),
            regolith_params: MaterialRegolithParameters::default(),
            selected_orientation: MaterialVariant::Wood.orientation_axis(),
            preview_lod: MaterialPreviewLod::Auto,
            soil_params: MaterialSoilParameters::default(),
            rarity_context: MaterialRarityContext::default(),
            debug_property_overrides: MaterialDebugPropertyOverrides::default(),
        }
    }
}

impl MaterialViewerState {
    pub const fn selected_shader_family_id(self) -> u32 {
        self.selected_variant.shader_family_id()
    }

    pub fn benchmark_case_label(self) -> &'static str {
        match self.selected_variant {
            MaterialVariant::Ice => self.ice_params.representative_form().benchmark_case_label(),
            _ => self.selected_variant.benchmark_case_label(),
        }
    }

    pub fn example_label(self) -> &'static str {
        match self.selected_variant {
            MaterialVariant::Ice => self.ice_params.visual_label(),
            MaterialVariant::Stone => self.derived_stone_lithology().label(),
            _ => self.selected_variant.example_label(),
        }
    }

    pub fn set_ice_form_preset(&mut self, preset: MaterialIceForm) {
        self.ice_params = preset.parameters();
    }

    pub fn derived_regolith_origin(self) -> MaterialRegolithOrigin {
        material_derive_regolith_origin(self.soil_params, self.regolith_params)
    }

    pub fn soil_element_bindings(self) -> MaterialSoilElementBindings {
        MaterialSoilElementBindings::from_regolith_origin(self.derived_regolith_origin())
    }

    pub const fn selected_orientation_axis(self) -> MaterialOrientationAxis {
        self.selected_orientation
    }

    pub fn derived_stone_lithology(self) -> MaterialStoneLithology {
        material_derive_stone_lithology(self.selected_stone_genesis, self.stone_params)
    }

    pub fn derived_common_properties(self) -> MaterialCommonProperties {
        self.debug_property_overrides
            .apply(derive_material_common_properties_for_state(self))
    }

    pub fn effective_rarity_bits(self) -> f64 {
        material_effective_rarity_bits_for_state(self)
    }

    pub fn rarity_budget(self) -> f64 {
        material_rarity_budget_for_state(self)
    }
}

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
    derive_material_common_properties_for_state(MaterialViewerState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        selected_orientation: variant.orientation_axis(),
        preview_lod: MaterialPreviewLod::Auto,
        soil_params: soil,
        rarity_context,
        debug_property_overrides: MaterialDebugPropertyOverrides::default(),
    })
}

pub fn derive_material_common_properties_for_state(
    viewer_state: MaterialViewerState,
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

pub fn material_effective_rarity_bits_for_state(viewer_state: MaterialViewerState) -> f64 {
    let rarity_context = viewer_state.rarity_context;
    let family_bias = match viewer_state.selected_variant {
        MaterialVariant::Metal => rarity_context.depth * 4.2,
        MaterialVariant::Stone => rarity_context.depth * 3.6,
        MaterialVariant::Wood => rarity_context.northness() * 4.0,
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
    material_effective_rarity_bits_for_state(MaterialViewerState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        selected_orientation: variant.orientation_axis(),
        preview_lod: MaterialPreviewLod::Auto,
        soil_params: soil,
        rarity_context,
        debug_property_overrides: MaterialDebugPropertyOverrides::default(),
    })
}

fn material_rarity_budget_for_state(viewer_state: MaterialViewerState) -> f64 {
    let scale = match viewer_state.selected_variant {
        MaterialVariant::Metal => 14.0,
        MaterialVariant::Stone => 13.0,
        MaterialVariant::Wood => 12.0,
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
        MaterialVariant::Glass => 10.5,
        MaterialVariant::Ice => 10.0,
        MaterialVariant::Ceramic => 13.0,
        MaterialVariant::Crystal => 12.5,
        MaterialVariant::Soil => 11.0,
    };
    scale * (((1.0 + effective_rarity_bits).powf(1.7)) - 1.0)
}

fn material_aspect_weights_for_state(viewer_state: MaterialViewerState) -> MaterialAspectVector {
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
    material_aspect_weights_for_state(MaterialViewerState {
        selected_variant: variant,
        ice_params: MaterialIceParameters::default(),
        selected_stone_genesis: MaterialStoneGenesis::default(),
        stone_params: MaterialStoneParameters::default(),
        regolith_params: MaterialRegolithParameters::default(),
        selected_orientation: variant.orientation_axis(),
        preview_lod: MaterialPreviewLod::Auto,
        soil_params: soil,
        rarity_context,
        debug_property_overrides: MaterialDebugPropertyOverrides::default(),
    })
}

fn material_aspect_budget(
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

fn material_apply_aspects(
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

fn material_ceramic_feedstock_score_with_origin(
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
