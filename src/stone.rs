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

    /// Stable identity id used by versioned semantic keys. Values are frozen
    /// independently of `shader_id` so shader retuning can never silently
    /// change material identity.
    pub const fn encode_id(self) -> u32 {
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

    /// Snaps every lane onto the frozen 64-step semantic grid via
    /// [`crate::material_snap_unit_lane`] (MAT-1Q,
    /// docs/material_presentation_redesign.md section 8). `set_parameter`'s
    /// `[0, 1]` clamp is a no-op on snapped values, so snapping is
    /// idempotent and every lane ends exactly on-grid.
    pub fn snap_to_semantic_grid(&mut self) {
        for parameter in MaterialStoneParameter::COMMON
            .into_iter()
            .chain(MaterialStoneParameter::SPECIFIC)
        {
            self.set_parameter(
                parameter,
                crate::material_snap_unit_lane(self.value(parameter)),
            );
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

    /// Stable identity id used by versioned semantic keys. Values are frozen
    /// independently of `shader_id` so shader retuning can never silently
    /// change material identity.
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
