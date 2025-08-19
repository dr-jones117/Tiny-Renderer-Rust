pub mod algorithms;
pub mod line_raster;
pub mod triangle_raster;

pub use algorithms::Algorithms;
pub use line_raster::bresenhams_line_alg;
pub use line_raster::line_alg_with_floats;
pub use triangle_raster::rasterize_triangle;
