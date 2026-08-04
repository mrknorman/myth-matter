use crate::{MaterialCommonProperties, MaterialMatterState};

pub const MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M: f32 = 0.5;
pub const MATERIAL_WATER_MAX_MAX_PATH_LENGTH_M: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialWaterParameter {
    Purity,
    Salinity,
    Sediment,
    OrganicTint,
    Aeration,
    MaxPathLength,
}

impl MaterialWaterParameter {
    pub const ALL: [Self; 6] = [
        Self::Purity,
        Self::Salinity,
        Self::Sediment,
        Self::OrganicTint,
        Self::Aeration,
        Self::MaxPathLength,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Purity => "Purity",
            Self::Salinity => "Salinity",
            Self::Sediment => "Sediment",
            Self::OrganicTint => "Organic Tint",
            Self::Aeration => "Aeration",
            Self::MaxPathLength => "Max Path",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialWaterParameters {
    pub purity_pct: u16,
    pub salinity_pct: u16,
    pub sediment_pct: u16,
    pub organic_tint_pct: u16,
    pub aeration_pct: u16,
    pub max_path_length_pct: u16,
}

impl Default for MaterialWaterParameters {
    fn default() -> Self {
        Self::new(0.88, 0.16, 0.08, 0.04, 0.05, 8.0)
    }
}

impl MaterialWaterParameters {
    pub fn new(
        purity: f32,
        salinity: f32,
        sediment: f32,
        organic_tint: f32,
        aeration: f32,
        max_path_length_m: f32,
    ) -> Self {
        Self {
            purity_pct: Self::encode_unit_interval(purity),
            salinity_pct: Self::encode_unit_interval(salinity),
            sediment_pct: Self::encode_unit_interval(sediment),
            organic_tint_pct: Self::encode_unit_interval(organic_tint),
            aeration_pct: Self::encode_unit_interval(aeration),
            max_path_length_pct: Self::encode_max_path_length(max_path_length_m),
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

    fn encode_max_path_length(value_m: f32) -> u16 {
        let span = MATERIAL_WATER_MAX_MAX_PATH_LENGTH_M - MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M;
        let normalized = if span <= 0.0 {
            0.0
        } else {
            ((value_m - MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M) / span).clamp(0.0, 1.0)
        };
        Self::encode_unit_interval(normalized)
    }

    fn decode_max_path_length(value: u16) -> f32 {
        MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M
            + (MATERIAL_WATER_MAX_MAX_PATH_LENGTH_M - MATERIAL_WATER_MIN_MAX_PATH_LENGTH_M)
                * Self::decode_unit_interval(value)
    }

    pub const fn purity(self) -> f32 {
        Self::decode_unit_interval(self.purity_pct)
    }

    pub const fn salinity(self) -> f32 {
        Self::decode_unit_interval(self.salinity_pct)
    }

    pub const fn sediment(self) -> f32 {
        Self::decode_unit_interval(self.sediment_pct)
    }

    pub const fn organic_tint(self) -> f32 {
        Self::decode_unit_interval(self.organic_tint_pct)
    }

    pub const fn aeration(self) -> f32 {
        Self::decode_unit_interval(self.aeration_pct)
    }

    pub fn max_path_length_m(self) -> f32 {
        Self::decode_max_path_length(self.max_path_length_pct)
    }

    pub fn value(self, parameter: MaterialWaterParameter) -> f32 {
        match parameter {
            MaterialWaterParameter::Purity => self.purity(),
            MaterialWaterParameter::Salinity => self.salinity(),
            MaterialWaterParameter::Sediment => self.sediment(),
            MaterialWaterParameter::OrganicTint => self.organic_tint(),
            MaterialWaterParameter::Aeration => self.aeration(),
            MaterialWaterParameter::MaxPathLength => {
                Self::decode_unit_interval(self.max_path_length_pct)
            }
        }
    }

    pub fn set_parameter(&mut self, parameter: MaterialWaterParameter, value: f32) {
        let encoded = Self::encode_unit_interval(value.clamp(0.0, 1.0));
        match parameter {
            MaterialWaterParameter::Purity => self.purity_pct = encoded,
            MaterialWaterParameter::Salinity => self.salinity_pct = encoded,
            MaterialWaterParameter::Sediment => self.sediment_pct = encoded,
            MaterialWaterParameter::OrganicTint => self.organic_tint_pct = encoded,
            MaterialWaterParameter::Aeration => self.aeration_pct = encoded,
            MaterialWaterParameter::MaxPathLength => self.max_path_length_pct = encoded,
        }
    }

    /// Snaps every lane onto the frozen 64-step semantic grid via
    /// [`crate::material_snap_unit_lane`] (MAT-1Q,
    /// docs/material_presentation_redesign.md section 8). The max path length
    /// lane snaps in its normalized `[0, 1]` encoding, matching the semantic
    /// key. Lanes live in the struct's native permille encoding, so each
    /// snapped lane holds the unique permille representative of its grid
    /// value: re-quantizing yields the same grid step and snapping again is a
    /// no-op.
    pub fn snap_to_semantic_grid(&mut self) {
        for parameter in MaterialWaterParameter::ALL {
            self.set_parameter(
                parameter,
                crate::material_snap_unit_lane(self.value(parameter)),
            );
        }
    }

    pub fn visual_label(self) -> &'static str {
        if self.sediment() >= 0.55 {
            "Sediment-Laden Water"
        } else if self.organic_tint() >= 0.45 {
            "Tannin Water"
        } else if self.aeration() >= 0.45 {
            "Aerated Water"
        } else if self.salinity() >= 0.45 {
            "Clear Coastal Water"
        } else if self.purity() >= 0.80 {
            "Clear Freshwater"
        } else {
            "Clouded Water"
        }
    }
}

pub fn derive_water_base_common_properties(
    water: MaterialWaterParameters,
) -> MaterialCommonProperties {
    let purity = water.purity();
    let salinity = water.salinity();
    let sediment = water.sediment();
    let organic_tint = water.organic_tint();
    let aeration = water.aeration();

    MaterialCommonProperties::new(
        (18.0 + purity * 8.0 + sediment * 6.0).round() as u64,
        (4.0 + sediment * 4.0).round() as u64,
        (26.0 + salinity * 16.0 + sediment * 14.0).round() as u64,
        0,
        (920.0 - sediment * 40.0 + aeration * 20.0).round() as u64,
        (860.0 - salinity * 50.0 - sediment * 30.0 + purity * 20.0).round() as u64,
        0,
        100,
        (120.0 - aeration * 20.0 + sediment * 12.0).round() as u64,
        (40.0 + salinity * 260.0 + sediment * 80.0 + organic_tint * 20.0).round() as u64,
        (120.0 + salinity * 40.0 + sediment * 35.0 - aeration * 18.0).round() as u64,
        0,
        (7.0 + salinity * 0.7 - organic_tint * 1.1)
            .clamp(5.0, 9.0)
            .round() as u64,
        MaterialMatterState::Liquid,
    )
}
