use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(WorldAssetRoot(
        asset_server.load("levels/DebugGym.glb#Scene0")
    ));

    let mut test_brush = make_test_brush();
    let vertices = generate_vertices(&test_brush);
    build_faces(&mut test_brush, &vertices);
    let mesh = build_mesh(&test_brush);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        Transform::default(),
        BrushHitTestFlag,
    ));
}

#[derive(Component)]
pub struct BrushHitTestFlag;

struct Plane { //defined by normal and distance from world origin
    normal: Vec3,
    distance: f32,
}

struct Brush {
    planes: Vec<Plane>,
    faces: Vec<Face>,
    collision: bool, //just true for starters, will update later
}

struct Face {
    plane_index: usize, //which plane the face is on
    vertices: Vec<Vec3>,
    material: usize,
}

//helper function to quickly produce signed plane distance
fn signed_plane_distance(plane: &Plane, point: Vec3) -> f32 {
    plane.normal.dot(point) - plane.distance //isn't this cool, batman, we can just ship the result
}

fn make_test_brush() -> Brush {
    Brush {
        planes: vec![
            Plane {
                normal: Vec3::X,
                distance: 3.0,
            },
            Plane {
                normal: -Vec3::X,
                distance: 1.0,
            },
            Plane {
                normal: Vec3::Y,
                distance: 4.0,
            },
            Plane {
                normal: -Vec3::Y,
                distance: -1.0,
            },
            Plane {
                normal: Vec3::Z,
                distance: 1.0,
            },
            Plane {
                normal: -Vec3::Z,
                distance: 1.0,
            },
        ],

        faces: Vec::new(),

        collision: true,
    }
}

fn intersect_three_planes(
    a: &Plane,
    b: &Plane,
    c: &Plane,
   ) -> Option<Vec3> {

    let denominator =
        a.normal.dot(b.normal.cross(c.normal));

    if denominator.abs() < 0.0001 {
        return None;
    }

    let numerator =
        a.distance * b.normal.cross(c.normal)
        + b.distance * c.normal.cross(a.normal)
        + c.distance * a.normal.cross(b.normal);

    Some(numerator / denominator)
}

fn point_inside_brush(
    brush: &Brush,
    point: Vec3,
   ) -> bool {

    for plane in &brush.planes {

        if signed_plane_distance(plane, point) > 0.0001 {
            return false;
        }
    }

    true
}

fn generate_vertices(brush: &Brush) -> Vec<Vec3> {
    let mut vertices = Vec::new();

    for i in 0..brush.planes.len() {
        for j in (i + 1)..brush.planes.len() {
            for k in (j + 1)..brush.planes.len() {

                let a = &brush.planes[i];
                let b = &brush.planes[j];
                let c = &brush.planes[k];

                if let Some(vertex) = intersect_three_planes(a, b, c) {
                    if point_inside_brush(brush, vertex) {
                        vertices.push(vertex);
                    }
                }
            }
        }
    }

    vertices
}

fn build_faces(
    brush: &mut Brush,
    vertices: &[Vec3],
) {
    brush.faces.clear();

    for (plane_index, plane) in brush.planes.iter().enumerate() {

        let mut face_vertices = Vec::new();

        // Find every vertex that lies on this plane.
        for &vertex in vertices {
            if signed_plane_distance(plane, vertex).abs() < 0.0001 {
                face_vertices.push(vertex);
            }
        }

        // Ignore planes that didn't produce enough vertices.
        if face_vertices.len() < 3 {
            continue;
        }

        sort_face_vertices(plane, &mut face_vertices);

        brush.faces.push(Face {
            plane_index,
            vertices: face_vertices,
            material: 0,
        });
    }
}

fn face_center(vertices: &[Vec3]) -> Vec3 {
    let mut center = Vec3::ZERO;

    for &vertex in vertices {
        center += vertex;
    }

    center / vertices.len() as f32
}

fn sort_face_vertices(
    plane: &Plane,
    vertices: &mut Vec<Vec3>,
) {
    let center = face_center(vertices);

    let u = plane.normal.any_orthonormal_vector();
    let v = plane.normal.cross(u);

    vertices.sort_by(|a, b| {

        let a_rel = *a - center;
        let b_rel = *b - center;

        let a_angle = a_rel.dot(v).atan2(a_rel.dot(u));
        let b_angle = b_rel.dot(v).atan2(b_rel.dot(u));

        a_angle.total_cmp(&b_angle)
    });
}

fn triangulate_face(face: &Face) -> Vec<[usize; 3]> {

    let mut triangles = Vec::new();

    for i in 1..face.vertices.len() - 1 {
        triangles.push([0, i, i + 1]);
    }

    triangles
}

fn build_mesh(
    brush: &Brush,
) -> Mesh {

    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    let mut indices = Vec::<u32>::new();

    for face in &brush.faces {

        let base = positions.len() as u32;

        for vertex in &face.vertices {

            positions.push(vertex.to_array());

            normals.push(
                brush.planes[face.plane_index]
                .normal
                .to_array()
            );

            uvs.push([0.0, 0.0]);
        }

        let triangles = triangulate_face(face);

        for triangle in triangles {
            indices.push(base + triangle[0] as u32);
            indices.push(base + triangle[1] as u32);
            indices.push(base + triangle[2] as u32);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        positions,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        uvs,
    );

    mesh.insert_indices(
        Indices::U32(indices),
    );

    mesh
}

pub fn setup_level_collision(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    query: Query<(Entity, &Mesh3d), Without<Collider>>,
) {
    for (entity, mesh_handle) in &query {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {

            if let Some(collider) =
                Collider::from_bevy_mesh(
                    mesh,
                    &ComputedColliderShape::TriMesh(
                        TriMeshFlags::default()
                    ),
                )
                {
                    commands.entity(entity)
                    .insert(RigidBody::Fixed)
                    .insert(collider);
                }
        }
    }
}

pub fn editor_pick(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
                   rapier_context: ReadRapierContext,
                   brush_query: Query<(), With<BrushHitTestFlag>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    // get cursor

    // build ray

    // rapier cast

    // see if hit entity has BrushHitTestFlag
}
