pub const CHUNK_SIZE: usize = 32;

/*
 * Voxel
 *
 * The world is subdivided into a cubic grid, where each cell is a voxel.
 * This struct stores the characteristics that every cell holds at a
 * minimum. This includes the material that fills the cell.
 */
struct Voxel(u16);
impl Default for Voxel {
	fn default() -> Self {
		Voxel(0)
	}
}

/*
 * Row
 */
pub trait Row {
	fn is_compressed() -> bool;
	fn compress_value(& self) -> Voxel;
	fn compress_mask(& self) -> u16;
	fn get(& self, index: usize) -> Voxel;
	fn put(& self, index: usize, voxel: Voxel);
}

/*
 * FullRow
 */
pub struct FullRow {
	voxels: [Voxel; CHUNK_SIZE]
}
impl Row for FullRow {
	fn is_compressed() -> bool {
		false
	}
	fn compress_value(& self) -> Voxel {
		Default::default()
	}
	fn compress_mask(& self) -> u16 {
		0
	}
	fn get(& self, index: usize) -> Voxel {
		self.voxels[index]
	}
	fn put(& self, index: usize, voxel: Voxel) {
		self.voxels[index] = voxel;
	}
}

/*
 * HalfRow {
 */
pub struct HalfRow {
	voxels: [u8; CHUNK_SIZE],
	half_voxel: u8,
	first_half: bool
}
impl Row for HalfRow {
	fn is_compressed() -> bool {
		true
	}
	fn compress_value(& self) -> Voxel {
		if self.first_half {

		} else {

		}
		Default::default()
	}
	fn compress_mask(& self) -> u16 {
		if self.first_half {

		} else {

		}
		0xffff
	}
	fn get(& self, index: usize) -> Voxel {
		if self.first_half {

		} else {

		}
		Default::default()
	}
	fn put(& self, index: usize, voxel: Voxel) {
		if self.first_half {

		} else {

		}
	}
}

/*
 * SingleRow
 */
pub struct SingleRow {
	voxel: Voxel
}
impl Row for SingleRow {
	fn is_compressed() -> bool {
		true
	}
	fn compress_value(& self) -> Voxel {
		self.voxel
	}
	fn compress_mask(& self) -> u16 {
		0xffff
	}
	fn get(& self, index: usize) -> Voxel {
		self.voxel
	}
	fn put(& self, index: usize, voxel: Voxel) {
		self.voxel = voxel;
	}
}
