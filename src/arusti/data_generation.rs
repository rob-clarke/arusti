use std::mem;

extern crate nalgebra as na;
use na::{Vector3, UnitQuaternion, Matrix3};

use crate::types::{ElementType, Element, Figure, Sequence};

#[derive(Debug)]
pub struct DataPoint {
    pub position: Vector3<f32>,
    pub attitude: UnitQuaternion<f32>
    }

pub struct PerformanceOptions {
    pub roll_rate: f32,
    pub snap_rate: f32,
    pub spin_rate: f32,
    pub turn_rate: f32,
    pub hesitation_time: f32,
    }

pub struct DataPointGenerator {
    position: Vector3<f32>,
    body_velocity: Vector3<f32>,
    attitude: Vector3<f32>,
    omega_b: Vector3<f32>,
    wind: Vector3<f32>,
    sample_rate: f32,
    performance: PerformanceOptions,
    points: Vec<DataPoint>,
    }

impl DataPointGenerator {

    pub fn new(initial_velocity: Vector3<f32>, wind: Vector3<f32>, performance: PerformanceOptions) -> DataPointGenerator {
        DataPointGenerator {
            position: Vector3::<f32>::new(0.0,0.0,0.0),
            body_velocity: initial_velocity,
            attitude: Vector3::<f32>::new(0.0,0.0,0.0),
            omega_b: Vector3::<f32>::new(0.0,0.0,0.0),
            wind: wind,
            sample_rate: 50.0,
            performance: performance,
            points: Vec::<DataPoint>::new(),
            }
        }
    
    pub fn generate_points(&mut self, sequence: &Sequence) -> Vec<DataPoint> {
        for figure in sequence {
            self.generate_figure(figure);
            self.normalise_attitude();
            }
        return mem::replace(&mut self.points,Vec::<DataPoint>::new());
        }
    
    fn normalise_attitude(&mut self) {
        self.attitude = self.attitude.map( |e| (e + 360.0) % 360.0 );
        if 90.0 < self.attitude[1] && self.attitude[1] < 270.0 {
            // Pitch inverted. Normalise such that roll causes inversion
            self.attitude[0] = (self.attitude[0] + 180.0) % 360.0;
            if self.attitude[1] <= 180.0 {
                self.attitude[1] = 180.0 - self.attitude[1];
                }
            else {
                self.attitude[1] = 540.0 - self.attitude[1];
                }
            self.attitude[2] = (self.attitude[2] + 180.0) % 360.0;
            }
        }

    fn get_dcm_body2earth(&self) -> Matrix3<f32> {
        let roll_rad  = self.attitude[0].to_radians();
        let theta_rad = self.attitude[1].to_radians();
        let yaw_rad   = self.attitude[2].to_radians();

        let cr = roll_rad.cos();
        let sr = roll_rad.sin();

        let ct = theta_rad.cos();
        let st = theta_rad.sin();

        let cy = yaw_rad.cos();
        let sy = yaw_rad.sin();

        // Build the rotation matrix
        let rot_x = Matrix3::<f32>::new(
             1.0, 0.0, 0.0,
             0.0,  cr, -sr,
             0.0,  sr,  cr,
            );
        let rot_y = Matrix3::<f32>::new(
             ct, 0.0,  st,
            0.0, 1.0, 0.0,
            -st, 0.0,  ct,
            );
        let rot_z = Matrix3::<f32>::new(
             cy, -sy, 0.0,
             sy,  cy, 0.0,
            0.0, 0.0, 1.0
            );

        // Multiply the matrices together to get the combined matrix
        (rot_z * rot_y) * rot_x
        }

    fn get_jacobian_body2earth(&self) -> Matrix3<f32> {
        let phi = self.attitude[0].to_radians();
        let theta = self.attitude[1].to_radians();

        let sp = phi.sin();
        let cp = phi.cos();

        let st = theta.sin();
        let ct = theta.cos();
        let tt = theta.tan();

        Matrix3::<f32>::new(
            1.0, sp*tt, cp*tt,
            0.0,   cp ,  -sp ,
            0.0, sp/ct, cp/ct
            )
        }
    
    fn get_invjacobian_earth2body(&self) -> Matrix3<f32> {
        let phi = self.attitude[0].to_radians();
        let theta = self.attitude[1].to_radians();

        let sp = phi.sin();
        let cp = phi.cos();

        let st = theta.sin();
        let ct = theta.cos();

        Matrix3::<f32>::new(
            1.0, 0.0,   -st,
            0.0,  cp, ct*sp,
            0.0, -sp, ct*cp
            )
        }

    /// Generate the next DataPoint based on the current velocity and attitude rate
    fn generate_next_point(&mut self) {
        self.position += self.get_dcm_body2earth() * self.body_velocity * self.sample_rate.recip();
        self.attitude += self.get_jacobian_body2earth() * self.omega_b * self.sample_rate.recip();

        self.points.push(DataPoint {
            position: self.position,
            attitude: UnitQuaternion::from_euler_angles(
                self.attitude[0].to_radians(),
                self.attitude[1].to_radians(),
                self.attitude[2].to_radians()
                ),
            });
        }

    fn generate_figure(&mut self, figure: &Figure) {
        // Need to do some tailoring for some elements
        // Includes radius matching and line length matching
        for element in figure {
            eprintln!("Elem: {:?}",element);
            self.generate_element(&element);
            eprintln!("Att: {:?}",self.attitude.data);
            eprintln!("Vel: {:?}",self.body_velocity.data);
            }
        }

    fn generate_element(&mut self, element: &Element) {
        match element.elem_type {
            ElementType::Line => self.generate_line(element),
            ElementType::Radius => self.generate_radius(element),
            ElementType::Turn => self.generate_turn(element),
            ElementType::Roll => self.generate_roll(element),
            ElementType::Flick => self.generate_flick(element),
            ElementType::Spin => self.generate_spin(element),
            _ => unreachable!()
            }
        }
    
    fn generate_line(&mut self, element: &Element) {
        // Angle defines angle between forward direction and ground
        // Argument is length (m)
        self.omega_b = Vector3::<f32>::new(0.0,0.0,0.0);

        let mut current_time = self.sample_rate.recip();

        while current_time < 1.0 {
            current_time += self.sample_rate.recip();
            self.generate_next_point();
            }
        return;
        /*
        if element.angle == 0.0 {
            // CGT line, Set velocity, compute attitude with wind
            self.velocity = Vector3::<f32>::new(0.0,0.0,0.0);
            }
        else {
            // ZLA line. Set attitude, compute velocity with wind
            if element.angle.abs() > 50.0 {
                // Vertical line

                }
            else {
                self.attitude[0] = if element.inverted { 180.0 } else { 0.0 };
                }
            }
            */
        // Attitude and velocity set, propogate for appropriate length
        // TODO: Speed model?
        
        }
    
    fn generate_radius(&mut self, element: &Element) {
        assert_eq!(element.elem_type, ElementType::Radius);

        // Use turn rate * 2 for now...
        let radius_time = element.angle.abs() * (self.performance.turn_rate * 2.0).recip();
        self.omega_b = Vector3::<f32>::new(0.0,(self.performance.turn_rate * 2.0).copysign(element.angle),0.0);

        let mut current_time = self.sample_rate.recip();

        while current_time < radius_time {
            current_time += self.sample_rate.recip();
            self.generate_next_point();
            }
        }

    /// Generate points for the turn defined by element
    fn generate_turn(&mut self, element: &Element) {
        assert_eq!(element.elem_type, ElementType::Turn);
        if element.argument == 0.0 {
            // No rolls in turn, generate 75° entry/exit rolls

            // Entry roll
            self.generate_roll(&Element {
                elem_type: ElementType::Roll,
                inverted: element.inverted,
                angle: 75.0,
                argument: 0.0,
                });

            // Main turn
            let turn_time = element.angle.abs() * self.performance.turn_rate.recip();
            let attitude_rate = Vector3::<f32>::new( 0.0, 0.0, self.performance.turn_rate.copysign(element.angle) );

            let mut current_time = self.sample_rate.recip();

            while current_time < turn_time {
                current_time += self.sample_rate.recip();
                self.omega_b = self.get_invjacobian_earth2body() * attitude_rate;
                self.generate_next_point();
                }

            // Exit roll
            self.generate_roll(&Element {
                elem_type: ElementType::Roll,
                inverted: element.inverted,
                angle: -75.0,
                argument: 0.0,
                });
            }
        else {
            // Rolling turn
            let turn_time = element.angle.abs() * self.performance.turn_rate.recip();

            let attitude_rate = Vector3::<f32>::new(
                element.argument / turn_time,
                0.0,
                self.performance.turn_rate.copysign(element.angle)
                );

            let mut current_time = self.sample_rate.recip();

            while current_time < turn_time {
                current_time += self.sample_rate.recip();
                self.omega_b = self.get_invjacobian_earth2body() * attitude_rate;
                self.generate_next_point();
                }
            }
        }

    /// Generate points for the roll defined by Element
    ///  The aircraft velocity remains unchanged (i.e. follows CGT), therefore it needs to be set
    ///  as part of the tailoring in the generate_figure
    fn generate_roll(&mut self, element: &Element) {
        assert_eq!(element.elem_type, ElementType::Roll);

        if element.argument == 0.0 {
            // Pure roll
            let roll_time = element.angle.abs() * self.performance.roll_rate.recip();
            self.omega_b = Vector3::<f32>::new(self.performance.roll_rate.copysign(element.angle),0.0,0.0);

            let mut current_time = self.sample_rate.recip();

            while current_time < roll_time {
                current_time += self.sample_rate.recip();
                self.generate_next_point();
                }
            }
        else {
            // Hesitation roll, recursive call
            let divisions = element.argument as i8;
            for _i in 1..divisions {
                self.generate_roll(&Element {
                    elem_type: ElementType::Roll,
                    inverted: element.inverted,
                    angle: element.angle / element.argument,
                    argument: 0.0,
                    });
                // Advance on CGT for hesitation_time
                self.omega_b = Vector3::<f32>::new(0.0,0.0,0.0);
                let mut current_time = self.sample_rate.recip();
                while current_time < self.performance.hesitation_time {
                    current_time += self.sample_rate.recip();
                    self.generate_next_point();
                    }
                }
            // Generate final rolling element
            self.generate_roll(&Element {
                elem_type: ElementType::Roll,
                inverted: element.inverted,
                angle: element.angle / element.argument,
                argument: 0.0,
                });
            }
        // In a hesitation roll, should hesitations modulo 180° be judged on ZLA?
        self.normalise_attitude();
        }

    fn generate_flick(&mut self, element: &Element) {
        unimplemented!();
        }

    fn generate_spin(&mut self, element: &Element) {
        unimplemented!();
        }

    }