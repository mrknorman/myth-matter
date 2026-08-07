use crate::{
    MATERIAL_SOIL_COMPOSITION_COMPONENT_COUNT, MATERIAL_SOIL_IRON_OXIDE_MAX,
    MATERIAL_SOIL_MAX_WATER_CAPACITY, MATERIAL_SOIL_MIN_WATER_CAPACITY, MaterialElement,
};

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
    pub const ALL: [Self; 12] = [
        Self::Residual,
        Self::Alluvial,
        Self::Colluvial,
        Self::Aeolian,
        Self::Lacustrine,
        Self::MarineShelf,
        Self::MarinePelagic,
        Self::EstuarineDeltaic,
        Self::GlacialTill,
        Self::Tephric,
        Self::OrganicPeat,
        Self::ShallowBedrock,
    ];

    /// Inverse of [`Self::encode_id`] for persisted identity (C-070 tables
    /// and any future encoded-id reader). Unknown ids are refused, never
    /// defaulted — an unknown identity must fail closed.
    pub fn from_encode_id(id: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|value| value.encode_id() == id)
    }
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

    /// Stable identity id used by versioned semantic keys. Values are frozen
    /// independently of `shader_id` so shader retuning can never silently
    /// change material identity.
    pub const fn encode_id(self) -> u32 {
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

pub const fn material_regolith_parameters_for_origin(
    origin: MaterialRegolithOrigin,
) -> MaterialRegolithParameters {
    match origin {
        MaterialRegolithOrigin::Residual => MaterialRegolithParameters {
            transport: 0.18,
            water_influence: 0.18,
            marine_influence: 0.0,
            angularity: 0.18,
        },
        MaterialRegolithOrigin::Alluvial => MaterialRegolithParameters {
            transport: 0.72,
            water_influence: 0.78,
            marine_influence: 0.08,
            angularity: 0.16,
        },
        MaterialRegolithOrigin::Colluvial => MaterialRegolithParameters {
            transport: 0.66,
            water_influence: 0.34,
            marine_influence: 0.02,
            angularity: 0.72,
        },
        MaterialRegolithOrigin::Aeolian => MaterialRegolithParameters {
            transport: 0.86,
            water_influence: 0.12,
            marine_influence: 0.0,
            angularity: 0.10,
        },
        MaterialRegolithOrigin::Lacustrine => MaterialRegolithParameters {
            transport: 0.54,
            water_influence: 0.84,
            marine_influence: 0.12,
            angularity: 0.14,
        },
        MaterialRegolithOrigin::MarineShelf => MaterialRegolithParameters {
            transport: 0.58,
            water_influence: 0.54,
            marine_influence: 0.82,
            angularity: 0.12,
        },
        MaterialRegolithOrigin::MarinePelagic => MaterialRegolithParameters {
            transport: 0.20,
            water_influence: 0.28,
            marine_influence: 0.94,
            angularity: 0.06,
        },
        MaterialRegolithOrigin::EstuarineDeltaic => MaterialRegolithParameters {
            transport: 0.74,
            water_influence: 0.82,
            marine_influence: 0.82,
            angularity: 0.14,
        },
        MaterialRegolithOrigin::GlacialTill => MaterialRegolithParameters {
            transport: 0.34,
            water_influence: 0.10,
            marine_influence: 0.0,
            angularity: 0.88,
        },
        MaterialRegolithOrigin::Tephric => MaterialRegolithParameters {
            transport: 0.16,
            water_influence: 0.10,
            marine_influence: 0.0,
            angularity: 0.28,
        },
        MaterialRegolithOrigin::OrganicPeat => MaterialRegolithParameters {
            transport: 0.10,
            water_influence: 0.72,
            marine_influence: 0.0,
            angularity: 0.10,
        },
        MaterialRegolithOrigin::ShallowBedrock => MaterialRegolithParameters {
            transport: 0.04,
            water_influence: 0.04,
            marine_influence: 0.0,
            angularity: 0.96,
        },
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

    /// Snaps every continuous lane onto the frozen 64-step semantic grid via
    /// [`crate::material_snap_unit_lane`] (MAT-1Q,
    /// docs/material_presentation_redesign.md section 8).
    ///
    /// Lanes are assigned directly instead of through `set_parameter`:
    /// the composition setter renormalizes the other components and the
    /// modifier setter clamps to the sub-grid `water_capacity()` /
    /// [`MATERIAL_SOIL_IRON_OXIDE_MAX`] limits, either of which would move an
    /// already snapped lane off-grid. A snapped modifier may therefore sit up
    /// to half a grid step above its soft capacity limit; identity and
    /// presentation read the lanes as-is.
    pub fn snap_to_semantic_grid(&mut self) {
        self.sand_pct = crate::material_snap_unit_lane(self.sand_pct);
        self.silt_pct = crate::material_snap_unit_lane(self.silt_pct);
        self.clay_pct = crate::material_snap_unit_lane(self.clay_pct);
        self.gravel_pct = crate::material_snap_unit_lane(self.gravel_pct);
        self.pebble_pct = crate::material_snap_unit_lane(self.pebble_pct);
        self.tephra_pct = crate::material_snap_unit_lane(self.tephra_pct);
        self.organic_pct = crate::material_snap_unit_lane(self.organic_pct);
        self.water_pct = crate::material_snap_unit_lane(self.water_pct);
        self.iron_oxide_pct = crate::material_snap_unit_lane(self.iron_oxide_pct);
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
