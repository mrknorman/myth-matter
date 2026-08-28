use crate::{MaterialCommonProperties, MaterialElement, MaterialMatterState};

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

    /// Snaps every lane onto the frozen 64-step semantic grid via
    /// [`crate::material_snap_unit_lane`] (MAT-1Q,
    /// docs/designs/materials/material_presentation_redesign.md section 8). Lanes live in the
    /// struct's native permille encoding, so each snapped lane holds the
    /// unique permille representative of its grid value: re-quantizing yields
    /// the same grid step and snapping again is a no-op. The discrete
    /// `impurity_element` binding is untouched.
    pub fn snap_to_semantic_grid(&mut self) {
        for parameter in MaterialIceParameter::ALL {
            self.set_parameter(
                parameter,
                crate::material_snap_unit_lane(self.value(parameter)),
            );
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
