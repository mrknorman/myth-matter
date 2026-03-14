use crate::{material_add_signed_u64, material_debug_override_abs_max};

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
