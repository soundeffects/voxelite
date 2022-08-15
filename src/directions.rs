use bevy::prelude::IVec3;

pub enum Directions {
	Zero,
	X,
	NegX,
	Y,
	NegY,
	Z,
	NegZ
}

impl Directions {
	pub fn all() -> [Directions; 7] {
		[
			Self::Zero,
			Self::X,
			Self::NegX,
			Self::Y,
			Self::NegY,
			Self::Z,
			Self::NegZ
		]
	}

	pub fn to_vector(&self) -> IVec3 {
		match self {
			Self::Zero => IVec3::ZERO,
			Self::X => IVec3::X,
			Self::NegX => IVec3::NEG_X,
			Self::Y => IVec3::Y,
			Self::NegY => IVec3::NEG_Y,
			Self::Z => IVec3::Z,
			Self::NegZ => IVec3::NEG_Z
		}
	}
}
