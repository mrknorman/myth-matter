use crate::{MaterialCommonProperties, MaterialMatterState};

#[cfg(test)]
use crate::{MaterialRarityContext, MaterialSoilParameters, derive_material_common_properties};

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
pub enum MaterialVariant {
    Stone,
    Metal,
    #[default]
    Wood,
    Foliage,
    Glass,
    Ice,
    Water,
    Ceramic,
    Crystal,
    Soil,
}

impl MaterialVariant {
    pub const ALL: [Self; 10] = [
        Self::Stone,
        Self::Metal,
        Self::Wood,
        Self::Foliage,
        Self::Glass,
        Self::Ice,
        Self::Water,
        Self::Ceramic,
        Self::Crystal,
        Self::Soil,
    ];

    pub const ELEMENTAL: [Self; 9] = [
        Self::Stone,
        Self::Metal,
        Self::Wood,
        Self::Foliage,
        Self::Glass,
        Self::Ice,
        Self::Water,
        Self::Ceramic,
        Self::Crystal,
    ];

    pub const MIXTURES: [Self; 1] = [Self::Soil];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Stone => "Stone",
            Self::Metal => "Metal",
            Self::Wood => "Wood",
            Self::Foliage => "Foliage",
            Self::Glass => "Glass",
            Self::Ice => "Ice",
            Self::Water => "Water",
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
            | Self::Foliage
            | Self::Glass
            | Self::Ice
            | Self::Water
            | Self::Ceramic
            | Self::Crystal => MaterialClass::Elemental,
        }
    }

    pub const fn example_label(self) -> &'static str {
        match self {
            Self::Stone => "Granite Vein",
            Self::Metal => "Weathered Iron",
            Self::Wood => "Pine End-Grain",
            Self::Foliage => "Broadleaf Canopy",
            Self::Glass => "Tinted Float Glass",
            Self::Ice => "Blue Glacial Ice",
            Self::Water => "Clear Freshwater",
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
            Self::Foliage => "material-foliage",
            Self::Glass => "material-glass",
            Self::Ice => "material-ice",
            Self::Water => "material-water",
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
            Self::Foliage => MaterialOrientationAxis::Y,
            Self::Glass => MaterialOrientationAxis::Y,
            Self::Ice => MaterialOrientationAxis::Y,
            Self::Water => MaterialOrientationAxis::Y,
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
            Self::Foliage => 9,
            Self::Water => 10,
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
            Self::Foliage => MaterialCommonProperties::new(
                260,
                120,
                180,
                240,
                720,
                560,
                180,
                360,
                90,
                30,
                180,
                760,
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
            Self::Water => MaterialCommonProperties::new(
                20,
                4,
                32,
                0,
                900,
                860,
                0,
                100,
                120,
                40,
                120,
                0,
                7,
                MaterialMatterState::Liquid,
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
