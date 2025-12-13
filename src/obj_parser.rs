use std::{collections::HashMap, convert::Infallible, str::FromStr};

use crate::{
    Point, Vector, point,
    shape::{Group, Shape, group, smooth_triangle, triangle},
    vector,
};

pub struct ObjParser {
    pub vertices: Vec<Point>,
    pub normals: Vec<Vector>,
    pub default_group: Shape,
    pub groups: HashMap<String, Shape>,
    pub ignored_lines: usize,
    root_group: Shape,
}

#[derive(Clone, Copy)]
struct FaceVertex {
    vertex_idx: usize,
    normal_idx: Option<usize>,
}

fn parse_face_vertex(s: &str) -> Option<FaceVertex> {
    let parts: Vec<&str> = s.split('/').collect();
    let vertex_idx: usize = parts.first()?.parse().ok()?;
    let normal_idx = if parts.len() >= 3 && !parts[2].is_empty() {
        parts[2].parse().ok()
    } else {
        None
    };
    Some(FaceVertex {
        vertex_idx,
        normal_idx,
    })
}

fn fan_triangulate(
    face_vertices: &[FaceVertex],
    vertices: &[Point],
    normals: &[Vector],
) -> Vec<Shape> {
    if face_vertices.len() < 3 {
        return vec![];
    }

    (1..face_vertices.len() - 1)
        .map(|i| {
            let v1 = &face_vertices[0];
            let v2 = &face_vertices[i];
            let v3 = &face_vertices[i + 1];

            let p1 = vertices[v1.vertex_idx];
            let p2 = vertices[v2.vertex_idx];
            let p3 = vertices[v3.vertex_idx];

            match (v1.normal_idx, v2.normal_idx, v3.normal_idx) {
                (Some(n1_idx), Some(n2_idx), Some(n3_idx)) => smooth_triangle(
                    p1,
                    p2,
                    p3,
                    normals[n1_idx],
                    normals[n2_idx],
                    normals[n3_idx],
                )
                .build(),
                _ => triangle(p1, p2, p3).build(),
            }
        })
        .collect()
}

impl FromStr for ObjParser {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vertices = vec![point(0, 0, 0)];
        let mut normals = vec![vector(0, 0, 0)];
        let default_group = group().build();
        let mut groups: HashMap<String, Shape> = HashMap::new();
        let mut current_group: Option<String> = None;
        let mut ignored_lines = 0;

        for line in s.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" if parts.len() >= 4 => {
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                        parts[3].parse::<f32>(),
                    ) {
                        vertices.push(point(x, y, z));
                    } else {
                        ignored_lines += 1;
                    }
                }
                "vn" if parts.len() >= 4 => {
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                        parts[3].parse::<f32>(),
                    ) {
                        normals.push(vector(x, y, z));
                    } else {
                        ignored_lines += 1;
                    }
                }
                "g" if parts.len() >= 2 => {
                    let name = parts[1].to_string();
                    groups
                        .entry(name.clone())
                        .or_insert_with(|| group().build());
                    current_group = Some(name);
                }
                "f" if parts.len() >= 4 => {
                    let face_vertices: Vec<FaceVertex> = parts[1..]
                        .iter()
                        .filter_map(|s| parse_face_vertex(s))
                        .collect();

                    if face_vertices.len() >= 3 {
                        let triangles = fan_triangulate(&face_vertices, &vertices, &normals);
                        let target_group = current_group
                            .as_ref()
                            .and_then(|name| groups.get(name))
                            .unwrap_or(&default_group);

                        for tri in triangles {
                            target_group.add_child(tri);
                        }
                    } else {
                        ignored_lines += 1;
                    }
                }
                _ => {
                    ignored_lines += 1;
                }
            }
        }

        let root_group = build_root_group(&default_group, &groups);

        Ok(ObjParser {
            vertices,
            normals,
            default_group,
            groups,
            ignored_lines,
            root_group,
        })
    }
}

fn build_root_group(default_group: &Shape, groups: &HashMap<String, Shape>) -> Shape {
    let result = group().build();

    {
        let default_inner = default_group.inner();
        let default_geom = default_inner
            .geometry
            .as_any()
            .downcast_ref::<Group>()
            .expect("default_group is a Group");

        if !default_geom.is_empty() {
            drop(default_inner);
            result.add_child(default_group.clone());
        }
    }

    for group_shape in groups.values() {
        result.add_child(group_shape.clone());
    }

    result
}

impl AsRef<Shape> for ObjParser {
    fn as_ref(&self) -> &Shape {
        &self.root_group
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    use crate::{EPSILON, point, shape::Triangle, vector};

    #[test]
    fn ignoring_unrecognized_lines() {
        let gibberish = "\
There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.
";
        let parser: ObjParser = gibberish.parse().unwrap();
        assert_eq!(parser.ignored_lines, 5);
    }

    #[test]
    fn vertex_records() {
        let file = "\
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
";
        let parser: ObjParser = file.parse().unwrap();
        assert_eq!(parser.vertices.len(), 5); // 1-based indexing, index 0 unused
        assert_eq!(parser.vertices[1], point(-1, 1, 0));
        assert_relative_eq!(parser.vertices[2].x(), -1.0, epsilon = EPSILON);
        assert_relative_eq!(parser.vertices[2].y(), 0.5, epsilon = EPSILON);
        assert_relative_eq!(parser.vertices[2].z(), 0.0, epsilon = EPSILON);
        assert_eq!(parser.vertices[3], point(1, 0, 0));
        assert_eq!(parser.vertices[4], point(1, 1, 0));
    }

    #[test]
    fn parsing_triangle_faces() {
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
";
        let parser: ObjParser = file.parse().unwrap();

        let inner = parser.default_group.inner();
        let group = inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("default_group should be a Group");

        let children = group.children();
        assert_eq!(children.len(), 2);

        let t1_inner = children[0].inner();
        let t1 = t1_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);

        let t2_inner = children[1].inner();
        let t2 = t2_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);
    }

    #[test]
    fn triangulating_polygons() {
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5
";
        let parser: ObjParser = file.parse().unwrap();

        let inner = parser.default_group.inner();
        let group = inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("default_group should be a Group");

        let children = group.children();
        assert_eq!(children.len(), 3);

        let t1_inner = children[0].inner();
        let t1 = t1_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);

        let t2_inner = children[1].inner();
        let t2 = t2_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);

        let t3_inner = children[2].inner();
        let t3 = t3_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t3.p1, parser.vertices[1]);
        assert_eq!(t3.p2, parser.vertices[4]);
        assert_eq!(t3.p3, parser.vertices[5]);
    }

    #[test]
    fn triangles_in_groups() {
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";
        let parser: ObjParser = file.parse().unwrap();

        let g1 = parser
            .groups
            .get("FirstGroup")
            .expect("FirstGroup should exist");
        let g1_inner = g1.inner();
        let g1_group = g1_inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("FirstGroup should be a Group");
        let g1_children = g1_group.children();
        assert_eq!(g1_children.len(), 1);

        let t1_inner = g1_children[0].inner();
        let t1 = t1_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);

        let g2 = parser
            .groups
            .get("SecondGroup")
            .expect("SecondGroup should exist");
        let g2_inner = g2.inner();
        let g2_group = g2_inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("SecondGroup should be a Group");
        let g2_children = g2_group.children();
        assert_eq!(g2_children.len(), 1);

        let t2_inner = g2_children[0].inner();
        let t2 = t2_inner
            .geometry
            .as_any()
            .downcast_ref::<Triangle>()
            .expect("child should be Triangle");
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);
    }

    #[test]
    fn converting_obj_to_group() {
        let file = "\
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";
        let parser: ObjParser = file.parse().unwrap();
        let g = parser.as_ref().clone();

        let inner = g.inner();
        let group = inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("result should be a Group");

        let children = group.children();
        assert!(children.contains(parser.groups.get("FirstGroup").unwrap()));
        assert!(children.contains(parser.groups.get("SecondGroup").unwrap()));
    }

    #[test]
    fn vertex_normal_records() {
        let file = "\
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3
";
        let parser: ObjParser = file.parse().unwrap();
        assert_eq!(parser.normals.len(), 4); // 1-based indexing, index 0 unused
        assert_eq!(parser.normals[1], vector(0, 0, 1));
        assert_relative_eq!(parser.normals[2].x(), 0.707, epsilon = EPSILON);
        assert_relative_eq!(parser.normals[2].y(), 0.0, epsilon = EPSILON);
        assert_relative_eq!(parser.normals[2].z(), -0.707, epsilon = EPSILON);
        assert_eq!(parser.normals[3], vector(1, 2, 3));
    }

    #[test]
    fn faces_with_normals() {
        let file = "\
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1//3 3//2 2//1
";
        let parser: ObjParser = file.parse().unwrap();

        let inner = parser.default_group.inner();
        let group = inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::Group>()
            .expect("default_group should be a Group");

        let children = group.children();
        assert_eq!(children.len(), 2);

        let t1_inner = children[0].inner();
        let t1 = t1_inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::SmoothTriangle>()
            .expect("child should be SmoothTriangle");
        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);
        assert_eq!(t1.n1, parser.normals[3]);
        assert_eq!(t1.n2, parser.normals[1]);
        assert_eq!(t1.n3, parser.normals[2]);

        let t2_inner = children[1].inner();
        let t2 = t2_inner
            .geometry
            .as_any()
            .downcast_ref::<crate::shape::SmoothTriangle>()
            .expect("child should be SmoothTriangle");
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[2]);
        assert_eq!(t2.n1, parser.normals[3]);
        assert_eq!(t2.n2, parser.normals[2]);
        assert_eq!(t2.n3, parser.normals[1]);
    }
}
