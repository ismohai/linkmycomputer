use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmulatorPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportMapping {
    window_width: u32,
    window_height: u32,
    emulator_width: u32,
    emulator_height: u32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
}

impl ViewportMapping {
    pub fn for_letterboxed(
        window_width: u32,
        window_height: u32,
        emulator_width: u32,
        emulator_height: u32,
    ) -> Result<Self, MappingError> {
        if window_width == 0 || window_height == 0 || emulator_width == 0 || emulator_height == 0 {
            return Err(MappingError::InvalidDimensions);
        }

        let sx = window_width as f32 / emulator_width as f32;
        let sy = window_height as f32 / emulator_height as f32;
        let scale = sx.min(sy);

        let content_width = emulator_width as f32 * scale;
        let content_height = emulator_height as f32 * scale;
        let offset_x = (window_width as f32 - content_width) * 0.5;
        let offset_y = (window_height as f32 - content_height) * 0.5;

        Ok(Self {
            window_width,
            window_height,
            emulator_width,
            emulator_height,
            scale,
            offset_x,
            offset_y,
        })
    }

    pub fn normalized_to_emulator(&self, x: f32, y: f32) -> Result<EmulatorPoint, MappingError> {
        if !(0.0..=1.0).contains(&x) || !(0.0..=1.0).contains(&y) {
            return Err(MappingError::OutOfRangeNormalized);
        }

        let max_x = self.emulator_width.saturating_sub(1) as f32;
        let max_y = self.emulator_height.saturating_sub(1) as f32;

        Ok(EmulatorPoint {
            x: (x * max_x).round() as u32,
            y: (y * max_y).round() as u32,
        })
    }

    pub fn window_to_emulator(&self, x: u32, y: u32) -> Result<EmulatorPoint, MappingError> {
        if x >= self.window_width || y >= self.window_height {
            return Err(MappingError::OutOfWindowBounds);
        }

        let xf = x as f32;
        let yf = y as f32;
        let active_width = self.emulator_width as f32 * self.scale;
        let active_height = self.emulator_height as f32 * self.scale;

        if xf < self.offset_x
            || xf > self.offset_x + active_width
            || yf < self.offset_y
            || yf > self.offset_y + active_height
        {
            return Err(MappingError::OutsideActiveViewport);
        }

        let local_x = ((xf - self.offset_x) / self.scale)
            .clamp(0.0, self.emulator_width.saturating_sub(1) as f32);
        let local_y = ((yf - self.offset_y) / self.scale)
            .clamp(0.0, self.emulator_height.saturating_sub(1) as f32);

        Ok(EmulatorPoint {
            x: local_x.round() as u32,
            y: local_y.round() as u32,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MappingError {
    #[error("window/emulator dimensions must be > 0")]
    InvalidDimensions,
    #[error("normalized coordinates must be in range [0.0, 1.0]")]
    OutOfRangeNormalized,
    #[error("window coordinates are outside host window bounds")]
    OutOfWindowBounds,
    #[error("window coordinates are outside active emulator viewport")]
    OutsideActiveViewport,
}
