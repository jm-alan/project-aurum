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
      IntersectType::Ellipse(center, rotation, radius_x, radius_y) => {
        // https://www.maa.org/external_archive/joma/Volume8/Kalman/General.html
        Box::new(move |point| {
          let sin_rotation = rotation.sin();
          let cos_rotation = rotation.cos();
          let sin_sq_rotation = sin_rotation.powf(2.0);
          let cos_sq_rotation = 1.0 - sin_sq_rotation;
          let radius_x_sq = radius_x.powf(2.0);
          let radius_y_sq = radius_y.powf(2.0);

          (((cos_sq_rotation / radius_x_sq) + (sin_sq_rotation / radius_y_sq))
            * (point.x - center.x).powf(2.0))
            + (((sin_sq_rotation / radius_x_sq)
              + (cos_sq_rotation / radius_y_sq))
              * (point.y - center.y).powf(2.0))
            + (2.0
              * sin_rotation
              * cos_rotation
              * ((1.0 / radius_x_sq) - (1.0 / radius_y_sq))
              * (point.x - center.x)
              * (point.y - center.y))
            < 1.0
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
