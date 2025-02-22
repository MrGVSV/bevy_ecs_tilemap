use crate::helpers::hex_grid::consts::{DOUBLE_INV_SQRT_3, HALF_SQRT_3, INV_SQRT_3};
use crate::helpers::hex_grid::cube::{CubePos, FractionalCubePos};
use crate::helpers::hex_grid::offset::{ColEvenPos, ColOddPos, RowEvenPos, RowOddPos};
use crate::tiles::TilePos;
use crate::{TilemapGridSize, TilemapSize};
use bevy::math::{Mat2, Vec2};
use std::ops::{Add, Mul, Sub};

/// A position in a hex grid labelled according to [`HexCoordSystem::Row`] or
/// [`HexCoordSystem::Column`]. It is composed of a pair of `i32` digits named `q` and `r`. When
/// converting from a [`TilePos`], `TilePos.x` is mapped to `q`, while `TilePos.y` is mapped to `r`.
///
/// It is vector-like. In others: two `AxialPos` can be added/subtracted, and it can be multiplied
/// by an `i32` scalar.
///
/// Since this position type covers both [`HexCoordSystem::Row`] and [`HexCoordSystem::Column`],
/// it has `*_col` and `*_row` variants for important methods.
///
/// It can be converted from/into [`RowOddPos`], [`RowEvenPos`], [`ColOddPos`] and [`ColEvenPos`].
/// It can also be converted from/into [`CubePos`].
///
/// For more information, including interactive diagrams, see
/// [Red Blob Games](https://www.redblobgames.com/grids/hexagons/#coordinates-axial) (RBG). Note
/// however, that while positive `r` goes "downward" in RBG's article, we consider it as going
/// "upward".
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AxialPos {
    pub q: i32,
    pub r: i32,
}

impl From<&TilePos> for AxialPos {
    fn from(tile_pos: &TilePos) -> Self {
        AxialPos {
            q: tile_pos.x as i32,
            r: tile_pos.y as i32,
        }
    }
}

impl From<CubePos> for AxialPos {
    fn from(cube_pos: CubePos) -> Self {
        let CubePos { q, r, .. } = cube_pos;
        AxialPos { q, r }
    }
}

impl Add<AxialPos> for AxialPos {
    type Output = AxialPos;

    fn add(self, rhs: AxialPos) -> Self::Output {
        AxialPos {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub<AxialPos> for AxialPos {
    type Output = AxialPos;

    fn sub(self, rhs: AxialPos) -> Self::Output {
        AxialPos {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Mul<AxialPos> for i32 {
    type Output = AxialPos;

    fn mul(self, rhs: AxialPos) -> Self::Output {
        AxialPos {
            q: self * rhs.q,
            r: self * rhs.r,
        }
    }
}

fn ceiled_division_by_2(x: i32) -> i32 {
    if x < 0 {
        (x - 1) / 2
    } else {
        (x + 1) / 2
    }
}

impl From<AxialPos> for RowOddPos {
    fn from(axial_pos: AxialPos) -> Self {
        let AxialPos { q, r } = axial_pos;
        let delta = r / 2;
        RowOddPos { q: q + delta, r }
    }
}

impl From<RowOddPos> for AxialPos {
    fn from(offset_pos: RowOddPos) -> Self {
        let RowOddPos { q, r } = offset_pos;
        let delta = r / 2;
        AxialPos { q: q - delta, r }
    }
}

impl From<AxialPos> for RowEvenPos {
    fn from(axial_pos: AxialPos) -> Self {
        let AxialPos { q, r } = axial_pos;
        // (n + 1) / 2 is a ceil'ed rather than floored division
        let delta = ceiled_division_by_2(r);
        RowEvenPos { q: q + delta, r }
    }
}

impl From<RowEvenPos> for AxialPos {
    fn from(offset_pos: RowEvenPos) -> Self {
        let RowEvenPos { q, r } = offset_pos;
        let delta = ceiled_division_by_2(r);
        AxialPos { q: q - delta, r }
    }
}

impl From<AxialPos> for ColOddPos {
    fn from(axial_pos: AxialPos) -> Self {
        let AxialPos { q, r } = axial_pos;
        let delta = q / 2;
        ColOddPos { q, r: r + delta }
    }
}

impl From<ColOddPos> for AxialPos {
    fn from(offset_pos: ColOddPos) -> Self {
        let ColOddPos { q, r } = offset_pos;
        let delta = q / 2;
        AxialPos { q, r: r - delta }
    }
}

impl From<AxialPos> for ColEvenPos {
    fn from(axial_pos: AxialPos) -> Self {
        let AxialPos { q, r } = axial_pos;
        let delta = ceiled_division_by_2(q);
        ColEvenPos { q, r: r + delta }
    }
}

impl From<ColEvenPos> for AxialPos {
    fn from(offset_pos: ColEvenPos) -> Self {
        let ColEvenPos { q, r } = offset_pos;
        let delta = ceiled_division_by_2(q);
        AxialPos { q, r: r - delta }
    }
}

/// The matrix for mapping from [`AxialPos`](AxialPos), to world position when hexes are arranged
/// in row format ("pointy top" per Red Blob Games). See
/// [Size and Spacing](https://www.redblobgames.com/grids/hexagons/#size-and-spacing)
/// at Red Blob Games for an interactive visual explanation, but note that:
///     1) we consider increasing-y to be the same as "going up", while RBG considers increasing-y to be "going down",
///     2) our vectors have magnitude 1 (in order to allow for easy scaling based on grid-size)
pub const ROW_BASIS: Mat2 = Mat2::from_cols(Vec2::new(1.0, 0.0), Vec2::new(0.5, HALF_SQRT_3));

/// The inverse of [`ROW_BASIS`](ROW_BASIS).
pub const INV_ROW_BASIS: Mat2 = Mat2::from_cols(
    Vec2::new(1.0, 0.0),
    Vec2::new(-1.0 * INV_SQRT_3, DOUBLE_INV_SQRT_3),
);

/// The matrix for mapping from [`AxialPos`](AxialPos), to world position when hexes are arranged
/// in column format ("flat top" per Red Blob Games). See
/// [Size and Spacing](https://www.redblobgames.com/grids/hexagons/#size-and-spacing)
/// at Red Blob Games for an interactive visual explanation, but note that:
///     1) we consider increasing-y to be the same as "going up", while RBG considers increasing-y to be "going down",
///     2) our vectors have magnitude 1 (in order to allow for easy scaling based on grid-size)
pub const COL_BASIS: Mat2 = Mat2::from_cols(Vec2::new(HALF_SQRT_3, 0.5), Vec2::new(0.0, 1.0));

/// The inverse of [`COL_BASIS`](COL_BASIS).
pub const INV_COL_BASIS: Mat2 = Mat2::from_cols(
    Vec2::new(DOUBLE_INV_SQRT_3, -1.0 * INV_SQRT_3),
    Vec2::new(0.0, 1.0),
);

pub const UNIT_Q: AxialPos = AxialPos { q: 1, r: 0 };

pub const UNIT_R: AxialPos = AxialPos { q: 0, r: -1 };

pub const UNIT_S: AxialPos = AxialPos { q: 1, r: -1 };

impl AxialPos {
    /// The magnitude of a cube position is its distance away from the `(0, 0)` hex_grid.
    ///
    /// See the Red Blob Games article for a [helpful interactive diagram](https://www.redblobgames.com/grids/hexagons/#distances-cube).
    pub fn magnitude(&self) -> i32 {
        let cube_pos = CubePos::from(*self);
        cube_pos.magnitude()
    }

    /// Returns the hex_grid distance between `self` and `other`.
    pub fn distance_from(&self, other: &AxialPos) -> i32 {
        (*self - *other).magnitude()
    }

    /// Returns the center of the hex_grid in world space, assuming that:
    ///     1) tiles are row-oriented ("pointy top"),
    ///     2) the center of the hex_grid with index `(0, 0)` is located at `[0.0, 0.0]`.
    pub fn center_in_world_row(&self, grid_size: &TilemapGridSize) -> Vec2 {
        let unscaled_pos = ROW_BASIS * Vec2::new(self.q as f32, self.r as f32);
        Vec2::new(
            grid_size.x * unscaled_pos.x,
            ROW_BASIS.y_axis.y * grid_size.y * unscaled_pos.y,
        )
    }

    /// Returns the center of the hex_grid in world space, assuming that:
    ///     1) tiles are column-oriented ("flat top"),
    ///     2) the center of the hex_grid with index `(0, 0)` is located at `[0.0, 0.0]`.
    pub fn center_in_world_col(&self, grid_size: &TilemapGridSize) -> Vec2 {
        let unscaled_pos = COL_BASIS * Vec2::new(self.q as f32, self.r as f32);
        Vec2::new(
            COL_BASIS.x_axis.x * grid_size.x * unscaled_pos.x,
            grid_size.y * unscaled_pos.y,
        )
    }

    /// Returns the axial position of the hex_grid containing the given world position, assuming that:
    ///     1) tiles are row-oriented ("pointy top") and that
    ///     2) the world position corresponding to `[0.0, 0.0]` lies in the hex_grid indexed `(0, 0)`.
    pub fn from_world_pos_row(world_pos: &Vec2, grid_size: &TilemapGridSize) -> AxialPos {
        let normalized_world_pos = Vec2::new(
            world_pos.x / grid_size.x,
            world_pos.y / (ROW_BASIS.y_axis.y * grid_size.y),
        );
        let frac_pos = FractionalAxialPos::from(INV_ROW_BASIS * normalized_world_pos);
        frac_pos.round()
    }

    /// Returns the axial position of the hex_grid containing the given world position, assuming that:
    ///     1) tiles are column-oriented ("flat top") and that
    ///     2) the world position corresponding to `[0.0, 0.0]` lies in the hex_grid indexed `(0, 0)`.
    pub fn from_world_pos_col(world_pos: &Vec2, grid_size: &TilemapGridSize) -> AxialPos {
        let normalized_world_pos = Vec2::new(
            world_pos.x / (COL_BASIS.x_axis.x * grid_size.x),
            world_pos.y / grid_size.y,
        );
        let frac_pos = FractionalAxialPos::from(INV_COL_BASIS * normalized_world_pos);
        frac_pos.round()
    }

    /// Try converting into a [`TilePos`].
    ///
    /// Returns `None` if either one of `q` or `r` is negative, or lies out of the bounds of
    /// `map_size`.
    pub fn as_tile_pos(&self, map_size: &TilemapSize) -> Option<TilePos> {
        TilePos::from_i32_pair(self.q, self.r, map_size)
    }
}

/// A fractional axial position can represent a point that lies inside a hexagon. It is typically
/// the result of mapping a world position into hexagonal space.
///
/// It can be rounded into an [`AxialPos`].
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct FractionalAxialPos {
    pub q: f32,
    pub r: f32,
}

impl FractionalAxialPos {
    fn round(&self) -> AxialPos {
        let frac_cube_pos = FractionalCubePos::from(*self);
        let cube_pos = frac_cube_pos.round();
        cube_pos.into()
    }
}

impl From<Vec2> for FractionalAxialPos {
    fn from(v: Vec2) -> Self {
        FractionalAxialPos { q: v.x, r: v.y }
    }
}
