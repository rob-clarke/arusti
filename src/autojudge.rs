extern crate nalgebra as na;
use na::{Vector3, UnitQuaternion};

use core::f32::consts::PI;

use arusti;

struct FlightDataPoint {
    position: Vector3<f32>,
    attitude: UnitQuaternion<f32>
    }

fn generate_loop(start_point : Vector3<f32>, radius : f32, num_samples : i32) -> Vec<FlightDataPoint> {
    let mut flight_data_points : Vec<FlightDataPoint> = Vec::new();

    let origin = start_point + Vector3::<f32>::new(0.0,0.0,radius);

    for i in 0..num_samples {
        let theta = (i as f32) * 2.0*PI/ (num_samples as f32);
        
        let position = origin + Vector3::<f32>::new(
            radius * theta.sin(),
            0.0,
            -radius * theta.cos()
            );
        
        let attitude = UnitQuaternion::from_euler_angles( 0.0, theta, 0.0 );
        
        flight_data_points.push( FlightDataPoint {
            position: position,
            attitude: attitude 
            });
        }
    return flight_data_points;
    }
/*
fn fit_line(points: Vec<FlightDataPoint>) -> arusti::Element {
    // Arbitrary line in 3D space defined by a and b
    // a = mean of points.position
    let position_sum : Vector3<f32> = points.iter().map(|p| p.position).sum();
    let count = points.len();
    let a = position_sum / (count as f32);

    // b = (points[0].position - points[end].position).norm()
    let b = (points[0].position - points.last().unwrap().position).norm();
    // angle between b and xy plane -> line angle
    // roll should be either ~0 or ~180 for whole line

    return arusti::Element {
        elemType: arusti::ElementType::Line,
        inverted: false,
        angle: 0.0,
        points: 0,
        combining: Vec::<arusti::Element>::new()
        }

    }
*/

fn spilt_into_figures() {
    /*
    Split into time-based lengths
    For each length score for Line 0 & Line 0 inverted
    if score > threshold:
        split()
    */
    }

fn main() {
    
/*
    Split flight data into figures
        Look for level flight lengths (possibly inverted)
    Split figure into elements
    Judge each element
    */

    }
