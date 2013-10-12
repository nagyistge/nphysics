#[link(name     = "boxes_vee3d"
       , vers   = "0.0"
       , author = "Sébastien Crozet"
       , uuid   = "4f1fb8c5-5f02-4d45-89f3-64acfd1718fa")];
#[crate_type = "bin"];
#[warn(non_camel_case_types)]

extern mod std;
extern mod extra;
extern mod kiss3d;
extern mod graphics3d;
extern mod nphysics;
extern mod nalgebra;
extern mod ncollide;

use kiss3d::window::Window;
use nalgebra::na;
use ncollide::geom::{Geom, Box, Plane};
use nphysics::world::BodyWorld;
use nphysics::aliases::dim3;
use nphysics::object::{RigidBody, Static, Dynamic, RB};
use graphics3d::engine::GraphicsManager;

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    GraphicsManager::simulate(boxes_vee_3d)
}

pub fn boxes_vee_3d(window: &mut Window, graphics: &mut GraphicsManager) -> dim3::BodyWorld3d<f64> {
    /*
     * World
     */
    let mut world = BodyWorld::new();
    world.set_gravity(na::vec3(0.0f64, -9.81, 0.0));

    /*
     * Plane
     */
    let geom = Geom::new_plane(Plane::new(na::vec3(0.0, 1.0, 0.0)));
    let body = @mut RB(RigidBody::new(geom, 0.0f64, Static, 0.3, 0.6));

    world.add_body(body);
    graphics.add(window, body);

    /*
     * Create the boxes
     */
    let num     = 8;
    let rad     = 1.0;
    let shift   = rad * 2.0;
    let centerx = shift * (num as f64) / 2.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f64) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            for k in range(0u, num) {
                let x = i as f64 * shift - centerx;
                let y = j as f64 * shift + centery;
                let z = k as f64 * shift - centerz;

                let geom   = Geom::new_box(Box::new(na::vec3(rad, rad, rad)));
                let mut rb = RigidBody::new(geom, 1.0f64, Dynamic, 0.3, 0.5);

                na::translate_by(&mut rb, &na::vec3(x, y, z));

                let body = @mut RB(rb);

                world.add_body(body);
                graphics.add(window, body);
            }
        }
    }

    /*
     * Set up the camera and that is it!
     */
    graphics.look_at(na::vec3(-30.0, 30.0, -30.0), na::vec3(0.0, 0.0, 0.0));

    world
}