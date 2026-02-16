#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TouchPoint {
    pub slot: u8,
    pub x: u32,
    pub y: u32,
    pub pressure: u32,
}

impl TouchPoint {
    pub fn new(slot: u8, x: u32, y: u32, pressure: u32) -> Self {
        Self {
            slot,
            x,
            y,
            pressure,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MinitouchBuilder {
    lines: Vec<String>,
}

impl MinitouchBuilder {
    pub fn down(mut self, point: TouchPoint) -> Self {
        self.lines.push(format!(
            "d {} {} {} {}",
            point.slot, point.x, point.y, point.pressure
        ));
        self
    }

    pub fn move_to(mut self, point: TouchPoint) -> Self {
        self.lines.push(format!(
            "m {} {} {} {}",
            point.slot, point.x, point.y, point.pressure
        ));
        self
    }

    pub fn up(mut self, slot: u8) -> Self {
        self.lines.push(format!("u {slot}"));
        self
    }

    pub fn commit(mut self) -> Self {
        self.lines.push("c".to_string());
        self
    }

    pub fn to_string(&self) -> String {
        if self.lines.is_empty() {
            return String::new();
        }

        let mut payload = self.lines.join("\n");
        payload.push('\n');
        payload
    }
}
