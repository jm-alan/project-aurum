use crate::{
  js_panic, resolve_convex_polygon,
  types::intersect_resolver::IntersectResolver,
};

use super::IntersectType;

impl From<IntersectType> for IntersectResolver {
  fn from(intersect: IntersectType) -> Self {
    match intersect {
      IntersectType::None => Box::new(|_| false),
      IntersectType::Triangle(corner_one, corner_two, corner_three) => {
        resolve_convex_polygon!(corner_one, corner_two, corner_three)
      }
      IntersectType::Rectangle(
        corner_one,
        corner_two,
        corner_three,
        corner_four,
      ) => {
        resolve_convex_polygon!(
          corner_one,
          corner_two,
          corner_three,
          corner_four
        )
      }
      IntersectType::Circle(center, radius) => Box::new(move |point| {
        ((point.x - center.x).powf(2.0) + (point.y - center.y).powf(2.0))
          <= radius.powf(2.0)
      }),
      IntersectType::Ellipse(center, radius_x, radius_y) => {
        Box::new(move |point| {
          (((radius_y.powf(2.0)) * (point.x - center.x).powf(2.0))
            + (radius_x.powf(2.0) * (point.y - center.y).powf(2.0)))
            <= (radius_x.powf(2.0) * radius_y.powf(2.0))
        })
      }
      IntersectType::Polygon(vertices) => {
        let ceil = vertices.len();
        if ceil < 2 {
          js_panic!("Instantiated polygon with < 3 vertices");
        }
        Box::new(move |point| {
          for idx in 0..ceil {
            if (((vertices[idx + 1].x - vertices[idx].x)
              * (point.y - vertices[idx].y))
              - ((point.x - vertices[idx].x)
                * (vertices[idx + 1].y - vertices[idx].y)))
              < 0.0
            {
              return false;
            }
          }
          true
        })
      }
      IntersectType::Custom(resolver) => resolver,
    }
  }
}
