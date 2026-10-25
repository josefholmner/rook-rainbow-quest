//! Letterboxed canvas mapping, independent of the renderer and Windows DPI.
pub struct Viewport {
    pub scale: f32,
    pub x: f32,
    pub y: f32,
}
/// Macroquad already applies Windows DPI to font_size; compensate exactly once.
/// Use the same plan for drawing and measurement, including rounded raster sizes.
pub fn font_raster(logical_size: f32, physical_scale: f32, dpi: f32) -> (u16, f32) {
    let size = (logical_size * physical_scale / dpi.max(0.1))
        .round()
        .clamp(1., 512.) as u16;
    (size, logical_size / size as f32)
}
/// Bound a circular edge's chord error to about one eighth of a physical pixel.
pub fn curve_segments(radius: f32, physical_scale: f32) -> u8 {
    let r = (radius * physical_scale).max(0.126);
    (std::f32::consts::PI / (1. - 0.125 / r).acos())
        .ceil()
        .clamp(12., 96.) as u8
}
impl Viewport {
    pub fn raster_size(&self, dpi: f32) -> (u32, u32) {
        let physical_scale = self.scale * dpi.max(0.1);
        (
            (1280. * physical_scale).round().max(1.) as u32,
            (800. * physical_scale).round().max(1.) as u32,
        )
    }
    pub fn new(width: f32, height: f32) -> Self {
        let scale = (width / 1280.).min(height / 800.).max(0.01);
        Self {
            scale,
            x: (width - 1280. * scale) / 2.,
            y: (height - 800. * scale) / 2.,
        }
    }
    pub fn to_virtual(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        let point = ((x - self.x) / self.scale, (y - self.y) / self.scale);
        if (0. ..1280.).contains(&point.0) && (0. ..800.).contains(&point.1) {
            Some(point)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curve_quality_stays_subpixel_at_window_and_high_dpi_sizes() {
        for radius in [1., 4., 12., 28., 48., 64.] {
            for scale in [0.75, 1., 1.35, 2., 3.] {
                let n = curve_segments(radius, scale);
                let error = radius * scale * (1. - (std::f32::consts::PI / n as f32).cos());
                assert!(error <= 0.13, "Curve error {error} px");
                assert!((12..=96).contains(&n));
            }
        }
    }
    #[test]
    fn text_raster_matches_settings_and_game_sizes_after_dpi_and_resize_changes() {
        for logical in [17., 19., 22., 24., 27., 35., 42.] {
            for (physical, dpi) in [(1., 1.), (3., 1.), (3., 2.), (1.5, 1.5), (0.75, 1.)] {
                let (size, scale) = font_raster(logical, physical, dpi);
                assert!((size as f32 * scale - logical).abs() < 0.001);
                assert!((size as f32 * dpi - logical * physical).abs() <= dpi * 0.51);
            }
        }
    }
    #[test]
    fn click_targets_survive_resizing_letterboxing_and_display_scaling() {
        for (w, h) in [
            (1280., 800.),
            (960., 600.),
            (1920., 1200.),
            (2560., 1600.),
            (1920., 1080.),
            (800., 900.),
        ] {
            let v = Viewport::new(w, h);
            // Centers of Play and each of the 16 board squares remain exact.
            for (x, y) in std::iter::once((239., 712.)).chain(
                (0..16).map(|i| (460. + (i % 4) as f32 * 120., 240. + (i / 4) as f32 * 120.)),
            ) {
                let actual = v.to_virtual(v.x + x * v.scale, v.y + y * v.scale).unwrap();
                assert!((actual.0 - x).abs() < 0.001);
                assert!((actual.1 - y).abs() < 0.001);
            }
        }
    }
    #[test]
    fn letterbox_bars_and_far_edges_are_not_clickable() {
        let v = Viewport::new(1920., 1080.);
        assert!(v.to_virtual(0., 540.).is_none());
        assert!(v.to_virtual(1919., 540.).is_none());
        assert!(v.to_virtual(v.x + 1280. * v.scale, 540.).is_none());
        assert!(v.to_virtual(960., 1080.).is_none());
    }
    #[test]
    fn fullscreen_targets_use_physical_pixels_including_high_dpi() {
        assert_eq!(Viewport::new(3840., 2400.).raster_size(1.), (3840, 2400));
        assert_eq!(Viewport::new(1280., 800.).raster_size(2.), (2560, 1600));
        assert_eq!(Viewport::new(1920., 1080.).raster_size(1.), (1728, 1080));
    }
}
