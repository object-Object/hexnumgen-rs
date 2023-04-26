use crate::{errors::HexResult, hex_math::Coord, Direction};

use png::EncodingError;
use tiny_skia::*;
use tiny_skia_path::PathStroker;

// TODO: return iterator instead? might be faster (need to benchmark)
pub fn pattern_to_points(direction: Direction, pattern: &str) -> HexResult<Vec<Coord>> {
    let mut compass = direction;
    let mut cursor: Coord = compass.into();

    let mut points = vec![Coord::origin(), cursor];
    points.reserve(pattern.len());
    for c in pattern.chars() {
        compass = compass.rotated(c.try_into()?);
        cursor += compass;
        points.push(cursor);
    }
    Ok(points)
}

pub struct PatternPlotter<'a> {
    stroker: PathStroker,
    resolution_scale: f32,

    pixmap: Pixmap,
    paint: Paint<'a>,
    stroke: Stroke,
}

impl PatternPlotter<'_> {
    pub fn new(width: u32, height: u32, scale: Option<f32>) -> Option<Self> {
        Some(Self {
            stroker: PathStroker::new(),
            resolution_scale: scale
                .map(|s| PathStroker::compute_resolution_scale(&Transform::from_scale(s, s)))
                .unwrap_or(1.),
            pixmap: Pixmap::new(width, height)?,
            paint: Paint { anti_alias: true, ..Default::default() },
            stroke: Stroke { line_cap: LineCap::Round, line_join: LineJoin::Round, ..Default::default() },
        })
    }

    pub fn plot_monochrome_line(
        &mut self,
        points: &Vec<Coord>,
        width: f32,
        color: Color,
        transform: Option<Transform>,
    ) -> Option<()> {
        let path = {
            let mut pb = PathBuilder::new();
            let (x, y) = Coord::origin().pixel();
            pb.move_to(x, y);

            for point in points {
                let (x, y) = point.pixel();
                pb.line_to(x, y);
            }

            pb.finish()?
        };
        self.paint.set_color(color);
        self.stroke.width = width;

        self.stroke_path(&path, transform)
    }

    pub fn plot_monochrome_points(
        &mut self,
        points: &Vec<Coord>,
        width: f32,
        color: Color,
        transform: Option<Transform>,
    ) -> Option<()> {
        let radius = width / 2.;
        self.paint.set_color(color);
        self.stroke.width = 0.;

        for point in points {
            let (x, y) = point.pixel();
            let path = PathBuilder::from_circle(x, y, radius)?;
            self.pixmap.fill_path(&path, &self.paint, FillRule::Winding, transform.unwrap_or_default(), None);
        }

        Some(())
    }

    pub fn encode_png(&self) -> Result<Vec<u8>, EncodingError> {
        self.pixmap.encode_png()
    }

    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), EncodingError> {
        self.pixmap.save_png(path)
    }

    fn stroke_path(&mut self, path: &Path, transform: Option<Transform>) -> Option<()> {
        let stroked = self.stroker.stroke(&path, &self.stroke, self.resolution_scale)?;
        self.pixmap.fill_path(&stroked, &self.paint, FillRule::Winding, transform.unwrap_or_default(), None);
        Some(())
    }
}
