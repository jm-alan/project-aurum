#[macro_export]
macro_rules! resolve_convex_polygon {
  ($first:expr, $sec:expr, $($el:expr),+) => {
    {
      let vertices = [$first, $sec, $($el),*, $first];
      let ceil = vertices.len() - 1;
      Box::new(move |point: CanvasPoint| {
        for idx in 0..ceil {
          if (
              ((vertices[idx + 1].x - vertices[idx].x)
              * (point.y - vertices[idx].y))
              - ((point.x - vertices[idx].x)
              * (vertices[idx + 1].y - vertices[idx].y))
          ) < 0.0 { return false };
        }
        true
      })
    }
  };
}
