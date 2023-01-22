// This application was developed for use in my PhD thesis under supervision 
// of Professor Per F. Peterson. It is part of a thermal hydraulics
// library in Rust that is released under the GNU General Public License
// v 3.0. This is partly due to the fact that some of the libraries 
// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
// Public License v3.0.
//
// As such, the entire library is released under GNU GPL v3.0. It is a strong 
// copyleft license which means you cannot use it in proprietary software.
//
//
// License
//    This application is part ciet_digital_twin_rust which uses
//    fluid_mechanics_rust, a partial library of the
//    thermal hydraulics library written in rust meant to help with the
//    fluid mechanics aspects of the calculations
//     
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    ciet_digital_twin_rust is free software; you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    ciet_digital_twin_rust is distributed in the hope that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    ciet_digital_twin_rust relies on a thermal hydraulics library
//    also written in rust. The isothermal digital twin in particular
//    relies on fluid_mechanics_rust.
//    This library is part of a thermal hydraulics library in rust
//    and contains some code copied from GeN-Foam, and OpenFOAM derivative.
//    This offering is not approved or endorsed by the OpenFOAM Foundation nor
//    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
//    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
//    Nor is it endorsed by the authors and owners of GeN-Foam.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// © All rights reserved. Theodore Kay Chen Ong,
// Singapore Nuclear Research and Safety Initiative,
// Per F. Peterson,
// University of California, Berkeley Thermal Hydraulics Laboratory
//
// Main author of the code: Theodore Kay Chen Ong, supervised by
// Professor Per F. Peterson
extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

use super::therminol_pipe::*;
use super::therminol_component::*;


/// contains a class for the ctah branch
pub mod ctah_branch;
pub use ctah_branch::*;


/// contains a class for the heater branch
pub mod heater_branch;
pub use heater_branch::*;

/// contains a class for the dhx branch
pub mod dhx_branch;
pub use dhx_branch::*;

/// Pipe6a in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
/// It is a static mixer pipe
pub struct Pipe6a {

    // pipe 6a
    // otherwise known as the static mixer pipe 6a

}

impl<'pipe_lifetime> Pipe6a{

    pub fn new() -> Self {

        return Self {}

    }


    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_6a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.1526);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(51.526384);
        let form_loss_k = 5.05;


        let pipe_6a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_6a;
    }
}

/// static mixer 41
/// label component 6 
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
pub struct StaticMixer41 {
    // static mixer 41 (MX-41) on CIET diagram
    // in the pump and CTAH branch
    // just before CTAH (AKA IHX)
    // from top to bottom
    //
    // label 6 on diagram
}

impl StaticMixer41 {

    pub fn new() -> Self {

        return Self {};

    }

    /// custom darcy friction factor is 0
    /// MX-41 does not depend on L/D
    /// for friction factor
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K value for static mixer 41
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-41
    /// or component no.6
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_41_label_6";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(51.526384);

        let static_mixer_41: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_41;
    }

}

/// Vertical part of Coiled Tube Air Heater (CTAH)
/// label component 7a
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
pub struct CTAHVertical {

    // coiled tube air heater,
    // uses pipe friction factors but has a constant K value
    // also pipe isn't circular
    // so we'll have to use custom fldk to help
    // label 7a
    
}

/// CTAH vertical is actually an fldk type pipe
///
/// but because I was quickly copying templates from
/// other fldk components, it became easy just
/// to force the vertical CTAH to be a custom fldk component
///
impl CTAHVertical {


    /// CTAH has a darcy friction factor from churchill
    /// correlation
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// CTAH has a fixed K value of 3.9 
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 3.9;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_vertical_label_7a";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(1.19e-2);
        let component_length = Length::new::<meter>(0.3302);
        let cross_sectional_area = Area::new::<square_meter>(1.33e-3);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);

        let ctah_vertical: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return ctah_vertical;
    }
    pub fn new() -> Self {

        return Self {  }

    }
}

/// Horizontal part of Coiled Tube Air Heater (CTAH)
/// label component 7b
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
pub struct CTAHHorizontal {

    // coiled tube air heater
    // has fldk = 400 + 52,000/Re
    //
    // label is 7b
    // empirical data in page 48 on pdf viewer in Dr
    // Zweibaum thesis shows reverse flow has same
    // pressure drop characteristics as forward flow
    
}

impl CTAHHorizontal {


    /// custom darcy friction factor is 0
    /// the horizontal CTAH correlation does not depend on L/D
    /// for friction factor
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }


    /// coiled tube air heater (CTAH) horizontal component
    /// has fldk = 400 + 52,000/Re
    pub fn custom_k(mut reynolds_number: f64) -> f64 {

        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            400.0 + 52000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of the
    /// horizontal portion of CTAH

    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_horizontal_label_7b";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(1.19e-2);
        let component_length = Length::new::<meter>(1.2342);
        let cross_sectional_area = Area::new::<square_meter>(1.33e-3);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);

        let ctah_horizontal: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return ctah_horizontal;
    }
    pub fn new() -> Self {

        return Self {  }

    }
}

/// Static mixer pipe 8a
/// adjacent to MX-40 in the CTAH branch
pub struct Pipe8a {
    // pipe 8a
    // otherwise known as the static mixer pipe 8a
    
}

impl Pipe8a {

    /// returns and instance of pipe 8a
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_8a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.22245);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 3.75;


        let pipe_8a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_8a;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer 40 (MX-40) on CIET diagram
/// just after CTAH (AKA IHX)
/// from top to bottom
/// label 8 on diagram
///
/// forced convection flow direction is same as top to bottom
/// has a fldk of 21+4000/Re
pub struct StaticMixer40 {
    
}
impl StaticMixer40 {

    /// custom darcy is 0
    /// because fldk does not depend on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    
    /// has a fldk of 21+4000/Re
    /// it comes from the custom_k value
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-40
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_40_label_8";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);

        let static_mixer_40: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_40;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


/// pipe number 9 in CIET's CTAH branch
pub struct Pipe9 {
    // pipe 9
    
}

impl Pipe9 {

    /// returns instance of pipe 9
    /// returns and instance of pipe 8a
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_9";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.7112);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-42.73211);
        let form_loss_k = 0.8;


        let pipe_9 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_9;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe number 10 in CIET's CTAH branch
pub struct Pipe10 {
    // pipe 10
    
}

impl Pipe10 {

    /// returns instance of pipe 10
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_10";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(2.4511);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 0.45;


        let pipe_10 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_10;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}


/// pipe number 11 in CIET's CTAH branch
pub struct Pipe11 {
    // pipe 11
    
}

impl Pipe11 {

    /// returns instance of pipe 11
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_11";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.4826);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-63.47465);
        let form_loss_k = 2.4;


        let pipe_11 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_11;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe number 12 in CIET's CTAH branch
pub struct Pipe12 {
    // pipe 12
    
}

impl Pipe12 {

    /// returns instance of pipe 12
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_12";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.333375);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 21.65;


        let pipe_12 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_12;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// ctah pump is a custom therminol component with
/// ie no friction factor losses
/// but it provides a source pressure
///
/// it is located between pipe 12 and 13
pub struct CTAHPump {
    
}
impl CTAHPump {

    // let's import everything necessary:

    /// pump has no internal pressure loss
    /// so custom darcy friction factor is 0
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// pump has no internal pressure loss
    /// so custom k is 0
    pub fn custom_k(_reynolds_number: f64) -> f64 {
        return 0.0;
    }

    /// returns an instance of the pump with an internal
    /// pressure term set by the user in the get method
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "ctah_pump";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);

        let ctah_pump: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return ctah_pump;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// pipe number 13 in CIET's CTAH branch
/// just after the pump
pub struct Pipe13 {
    // pipe 13 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42,
    // F-40 and F-41 on CIET diagram
    
}

impl Pipe13 {

    /// returns an instance of pipe13
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_13";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(1.273175);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 12.95;


        let pipe_13 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_13;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe number 14 in CIET's CTAH branch
pub struct Pipe14 {
    // pipe 14 on the diagram in Nico Zweibaum nodalisation
    // probably some combination of V-42,
    // F-40 and F-41 on CIET diagram
    // it is inclined 90 degrees upwards in direction
    // of flow
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also 90 degrees
    
}

impl Pipe14 {

    /// returns an instance of pipe14
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_14";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.6687);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0);
        let form_loss_k = 2.4;


        let pipe_14 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_14;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// FM-40 Coriolis Flowmeter in CIET's CTAH branch
/// labelled 14a in simulation schmeatic
pub struct Flowmeter40 {
    // ctah line flowmeter 40
    // label 14a on simulation diagram
    // fldk = 18.0+93000/Re
    
}
impl Flowmeter40 {

    // let's import everything necessary:

    /// custom darcy is 0 because
    /// fldk does not depend on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// fldk = 18.0+93000/Re
    /// this is implemented by setting 
    /// K = = 18.0+93000/Re
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of FM-40 (14a)
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_40_14a";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0);

        let flowmeter_40_14a: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return flowmeter_40_14a;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


/// pipe number 15 in CIET's CTAH branch
pub struct Pipe15 {
    // pipe 15 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -49.36983 degrees
    
}

impl Pipe15 {

    /// returns an instance of pipe 15
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_15";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.3556);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-49.36983);
        let form_loss_k = 0.8;


        let pipe_15 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_15;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// pipe number 16 in CIET's CTAH branch
pub struct Pipe16 {
    // pipe 16 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -49.36983 degrees
    
}

impl Pipe16 {

    /// returns an instance of pipe 16
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_16";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.644525);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-90.0);
        let form_loss_k = 1.9;


        let pipe_16 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_16;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// Branch (or pipe 17) in CIET's CTAH branch
///
/// Approximations were made for this branch though,
/// technically branch 17a is part of CTAH branch
/// while 17b is part of the DHX branch,
/// I combined both for convenience
///
/// This is treated as a single pipe though
pub struct Branch17 {
    // pipe 17 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of F30 on CIET's P&ID
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is 0 degrees
    //
    
}

impl Branch17 {

    /// returns an instance of Branch 17
    pub fn get(&self) -> TherminolPipe{


        let name = "branch_17";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.473075);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0);
        let form_loss_k = 0.0;


        let branch_17 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return branch_17;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


// heater branch items start here (from top to bottom)


/// Branch 5 in the Heater Branch (top to bottom perspective)
/// 
/// Approximations were made for this branch though,
/// technically branch 5a is part of DHX branch
/// while 5b is part of the DHX branch,
/// I combined both for convenience
///
/// This is treated as a single pipe though
///
/// Now I'd probably made a mistake putting branch 5 in
/// the heater branch, it's probably better put inside the
/// CTAH branch, (as of Oct 2022)
/// I'll probably put this in the CTAH branch in future
///
/// But for forced isothermal circulation tests with only
/// the heater branch and CTAH branch, it doesn't really matter
/// since there are only two branches
///
/// So no matter which branch you put branch or pipe 5 in,
/// it is still the same set of pipes in series
/// calculations will still be the same numerically
///
/// 
// this is reverse order compared to table A-1 in
// the Zweibaum nodalised relap model
pub struct Branch5 {
    // pipe 5 on the diagram in Nico Zweibaum nodalisation
    // and from a top to bottom direction from pipe 5
    // to pipe 5, the incline angle is also
    // 0 degrees
    // i add 180 degrees so that it is
    // properly reversed in
    // inclination angle from top to bottom
    
}

impl Branch5 {

    /// returns an instance of branch5
    pub fn get(&self) -> TherminolPipe{


        let name = "branch_5";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.7493);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0 + 180.0);
        let form_loss_k = 0.0;


        let branch_5 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return branch_5;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


/// pipe 4 within the heater branch
pub struct Pipe4 {
    // pipe 4 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of V11 and F12
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // 49.743387 +180.0 degrees
    
}

impl Pipe4 {

    /// returns an instance of pipe4
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_4";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.2413);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(49.743387 + 180.0);
        let form_loss_k = 2.4;


        let pipe_4 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_4;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// pipe3 within the heater branch
pub struct Pipe3 {
    // pipe 3 on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of V11 and F12
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // 90.0 +180.0 degrees
    
}

impl Pipe3 {

    /// returns an instance of pipe 3
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_3";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(1.2827);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);
        let form_loss_k = 3.15;


        let pipe_3 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_3;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// MX-10 within the heater branch
/// labelled as component 2
///
///
pub struct StaticMixer10 {
    // static mixer 10 (MX-10) on CIET diagram
    // just before the heater in the heater branch
    // from top to bottom
    // label 2 on diagram (fig A-1 on Nico Zweibaum thesis)
    // pg 125 on pdf viewer, pg 110 on printed page number on bottom right
    //
    // though in reality flow goes from bottom to
    // top in forced convection
    // so from a flow perspective it is before the
    // heater
    //
    
}
impl StaticMixer10 {


    /// darcy friction factor is 0
    ///
    /// This is because the MX-10 friction factor
    /// doesn't depend on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom k for MX-10
    ///
    /// fldk = 21 + 4000/Re
    ///
    /// this is done by setting 
    /// K = 21 + 4000/Re
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-10
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_10_label_2";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);

        let static_mixer_10: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_10;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer pipe2a in heater branch
///
/// adjacent to MX-10
pub struct Pipe2a {
    // pipe 2a on the diagram in Nico Zweibaum nodalisation
    // probably corresponds of V11 and F12
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // 90.0 +180.0 degrees
    
}

impl Pipe2a {

    /// returns an instance of pipe2a
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_2a_static_mixer";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.149425);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);
        let form_loss_k = 1.8;


        let pipe_2a_static_mixer = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_2a_static_mixer;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// heater top head 1a of heater branch in CIET
pub struct HeaterTopHead1a {

    // heater top head
    // diagram label 1a
    //
    // inclined at 90 degrees bottom to top
    // or 90 degrees + 180 top to bottom orientation
    
}

impl HeaterTopHead1a {


    /// custom darcy is taken from churchill friction factor
    ///
    /// Actually a dowtherm pipe would do,
    /// but I just copied and pasted the custom fldk component
    /// template
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// custom K is fixed at 3.75
    ///
    /// reverse flow logic means K is -3.75
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 3.75;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    /// returns an instance of heater top head 1a
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "heater_top_head_label_1a";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(6.60e-3);
        let component_length = Length::new::<meter>(0.0889);
        let cross_sectional_area = Area::new::<square_meter>(3.64e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);

        let heater_top_head_label_1a: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return heater_top_head_label_1a;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// This is the first version of CIET's heater
/// 
/// It is found in CIET's heater branch;
/// It has hydrodynamic losses similar to a pipe
pub struct CietHeaterVersion1 {

    // this is the first version of the ciet heater
    // without any insert within the heater
    // the heater behaves like a pipe
    //
    // inclined at 90 degrees bottom to top
    // or 90 degrees + 180 top to bottom orientation
    
}

impl CietHeaterVersion1 {


    /// custom darcy here is the same as churchill friction factor
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// K = 0 for CIET's heater version 1
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 0.0;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    /// returns an instance of CIET heater version 1
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "heater_version_1_label_1";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(6.60e-3);
        let component_length = Length::new::<meter>(1.6383);
        let cross_sectional_area = Area::new::<square_meter>(3.64e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);

        let heater_version_1_label_1: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return heater_version_1_label_1;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// heater bottom head 1b within CIET's heater branch
pub struct HeaterBottomHead1b {

    // heater top head
    // diagram label 1b
    //
    // inclined at 90 degrees bottom to top
    // or 90 degrees + 180 top to bottom orientation
    
}

impl HeaterBottomHead1b {


    /// custom darcy here is the same as churchill friction factor
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// custom K is fixed at 3.95
    ///
    /// reverse flow logic means K is -3.95
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 3.95;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    /// returns an instance of heater bottom head 1b
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "heater_bottom_head_label_1b";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(6.60e-3);
        let component_length = Length::new::<meter>(0.19685);
        let cross_sectional_area = Area::new::<square_meter>(3.64e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);

        let heater_bottom_head_label_1b: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return heater_bottom_head_label_1b;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// pipe 18 within CIET's heater branch
pub struct Pipe18 {
    // pipe 18 on the diagram in Nico Zweibaum nodalisation
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -40.00520 +180.0 degrees
    
}

impl Pipe18 {

    /// returns an instance of pipe 18
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_18";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.1778);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-40.00520 + 180.0);
        let form_loss_k = 5.15;


        let pipe_18 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_18;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

// These components belong to the Dracs Heat Exchanger (DHX) branch
//
//
//
//
//
//

/// pipe 26 in DHX Branch from Top to Bottom orientation
///
pub struct Pipe26 {
    // pipe 26 on the diagram in Nico Zweibaum nodalisation
    //
    // and from a top to bottom direction from pipe 5
    // to pipe 17, the incline angle is also
    // -40.00520 +180.0 degrees
    
}

impl Pipe26 {

    /// returns an instance of pipe 26
    pub fn get(&self) -> TherminolPipe{


        let name = "pipe_26";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.2159);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(52.571994 + 180.0);
        let form_loss_k = 1.75;


        let pipe_26 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return pipe_26;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer 21 (MX-21) on CIET diagram
/// in the DHX branch in primary loop
/// just before the DRACS heat exchanger
/// from top to bottom
/// label 25
pub struct StaticMixer21 {
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // also in the DRACS
    // loop there are flow diodes to make
    // it such that flow going from bottom to top
    // encounters more resistance
    //
    
}
impl StaticMixer21 {


    /// custom darcy is 0 
    ///
    /// this is because fldk has no dependence on L/D
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 21.0 + 4000/Re
    ///
    /// This is because fldk = = 21.0 + 4000/Re
    /// And we don't have L/D dependence
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-21
    ///
    /// It is labelled 25 on diagram
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_21_label_25";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);

        let static_mixer_21_label_25: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_21_label_25;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


/// Static mixer pipe 25a adjacent to MX-21
/// in DHX branch
pub struct Pipe25a {
    // pipe 25a
    // otherwise known as the static mixer pipe 25a
    
}

impl Pipe25a {

    /// returns an instance of static mixer pipe 25a
    /// adjacent to MX-21
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_25a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.22245);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);
        let form_loss_k = 1.35;


        let static_mixer_pipe_25a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_25a;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// this is the heat exchanger
/// in the DHX branch, labelled 24
///
/// It is shell side heat exchanger which allows
/// for heat to be transferred to natural circulation loop
/// or DRACS Loop
/// inclined at 90 degrees bottom to top
/// or 90 degrees + 180 top to bottom orientation
///
pub struct DHXShellSideHeatExchanger {

    
}

impl DHXShellSideHeatExchanger {


    /// custom darcy here is same as churchill friction factor
    pub fn custom_darcy(mut reynolds_number: f64, roughness_ratio: f64) -> f64 {

        if roughness_ratio < 0.0 {
            panic!("roughness_ratio < 0.0");
        }

        use fluid_mechanics_rust::churchill_friction_factor;
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let darcy = churchill_friction_factor::darcy(reynolds_number,
                                                     roughness_ratio);

        if reverse_flow {
            return -darcy;
        }
        return darcy;
    }

    /// custom K is fixed at 23.9
    ///
    /// reverse flow logic means K is -23.9
    pub fn custom_k(reynolds_number: f64) -> f64 {

        let custom_k_value = 23.9;

        if reynolds_number < 0.0 {
            return -custom_k_value
        }

        return custom_k_value;

    }

    /// returns an instance of dhx shell side
    /// heat exchanger 24
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "dhx_shell_side_label_24";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(5.65e-3);
        let component_length = Length::new::<meter>(1.18745);
        let cross_sectional_area = Area::new::<square_meter>(9.43e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 + 180.0);

        let dhx_shell_side_label_24: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return dhx_shell_side_label_24;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer 20 (MX-20) on CIET diagram
/// in the DRACS branch in primary loop
/// just after the DRACS heat exchanger
/// from top to bottom
/// label 23
///
/// in reality flow goes from bottom to
/// top in natural convection
/// also in the DRACS
/// loop there are flow diodes to make
/// it such that flow going from bottom to top
/// encounters more resistance
///
/// original angle is is 90 degrees
/// but i orientate from top to bottom
pub struct StaticMixer20 {
    
}
impl StaticMixer20 {


    /// custom darcy is 0 
    ///
    /// because fldk is independent of L/D
    /// so we set custom darcy = 0
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 21.0 + 4000/Re
    ///
    /// This is because fldk = = 21.0 + 4000/Re
    /// And we don't have L/D dependence
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-20
    /// label 23
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_20_label_23";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);

        let static_mixer_20_label_23: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_20_label_23;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer pipe 23a in DHX branch in CIET
///
/// otherwise known as the static mixer pipe 
/// to MX-20
pub struct Pipe23a {
    
}

impl Pipe23a {

    /// returns an instance of static mixer pipe 23a
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_23a";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.0891);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);
        let form_loss_k = 1.35;


        let static_mixer_pipe_23a = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_23a;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe 22 within DHX branch in CIEt
pub struct Pipe22 {
    // pipe 22
    // otherwise known as the static mixer pipe 22
    
}

impl Pipe22 {

    /// returns an intance of pipe 22
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_22";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.69215);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);
        let form_loss_k = 9.95;


        let static_mixer_pipe_22 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_22;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// FM-20 DHX branch flow coriolis flowmeter 20
///
/// natural convection heat exchanger in primary loop
/// diagram label is 21a
pub struct Flowmeter20 {
    // we use the convention of top of bypass branch to bottom
    // hence degree is -90
    //
    // However in DHX, i also expect there to be
    // a check valve which only allows flow from top to bottom
    //
    // That is the forward direction of flow for FM20,
    //
    
}
impl Flowmeter20 {

    // let's import everything necessary:

    /// custom darcy = 0 
    /// 
    /// as fldk has no dependence on L/D
    /// not explicitly anyway
    /// it is an empirical correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 18 + 93000/(Re^1.35)
    ///
    /// because
    /// fldk = 18 + 93000/(Re^1.35)
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an isntance of 
    /// FM-20 (label 21a)
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_20_label_21a";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 - 180.0);

        let flowmeter_20_label_21a: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return flowmeter_20_label_21a;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// FM-20 DHX branch flow coriolis flowmeter 20
///
/// natural convection heat exchanger in primary loop
/// diagram label is 21a
///
/// However, i put an artificial check valve behaviour
/// here, in that when flow is reversed from normal pump direction
/// a huge K value is put in
/// at
/// -1.0e10 - 1.0e10/Re
///
/// This is of course with reverse flow taken into account already
pub struct Flowmeter20WithHighKCheckValve {
    // DHX flow flowmeter 20
    // natural convection heat exchanger in primary loop
    // diagram label is 21a
    // we use the convention of top of bypass branch to bottom
    // hence degree is -90
    //
    // However in DHX, i also expect there to be
    // a check valve which only allows flow from top to bottom
    //
    // That is the forward direction of flow for FM20,
    //
    
}
impl Flowmeter20WithHighKCheckValve {

    // let's import everything necessary:

    /// custom darcy = 0 
    /// 
    /// as fldk has no dependence on L/D
    /// not explicitly anyway
    /// it is an empirical correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K 
    ///
    /// It is set to 18 + 93000/(Re^1.35) 
    /// in pump reverse flow direction (or normal natural 
    /// convection direction)
    ///
    /// because
    /// fldk = 18 + 93000/(Re^1.35)
    ///
    ///
    /// but in pump forward flow direction
    ///
    /// fldk = 1.0e10 + 1.0e10 / Re
    ///
    /// This enables the flow resistance to be extremely high
    /// even during laminar regime.
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -1.0e10/reynolds_number - 1.0e10;
        }

        return custom_k_value;

    }

    /// returns an instance of FM-20
    /// with artificial check valve behaviour
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_20_label_21a_with_check_valve";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 - 180.0);

        let flowmeter_20_label_21a_with_check_valve: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return flowmeter_20_label_21a_with_check_valve;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// pipe 21 within CIET DHX loop
pub struct Pipe21 {
    // pipe 21
    
}

impl Pipe21 {

    /// returns an instance of pipe21
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_21";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.487725);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0-180.0);
        let form_loss_k = 4.4;


        let static_mixer_pipe_21 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_21;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe 20 within CIET DHX loop
pub struct Pipe20 {
    // pipe 20
    
}

impl Pipe20 {

    /// returns an instance of pipe 20
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_20";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33655);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(0.0 - 180.0);
        let form_loss_k = 0.0;


        let static_mixer_pipe_20 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_20;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}

/// pipe 19 within CIET DHX loop
pub struct Pipe19 {
    // pipe 19
    
}

impl Pipe19 {

    /// returns an instance of pipe 19
    pub fn get(&self) -> TherminolPipe{


        let name = "static_mixer_pipe_19";

        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.219075);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-31.44898 - 180.0);
        let form_loss_k = 7.5;


        let static_mixer_pipe_19 = TherminolPipe::new(
            name, 
            fluid_temp, 
            incline_angle, 
            component_length, 
            hydraulic_diameter, 
            form_loss_k, 
            absolute_roughness, 
            );

        return static_mixer_pipe_19;
    }

    pub fn new() -> Self {

        return Self {  }

    }

}


// these components are for bypass flow branch
//
//
//
//
//
// list is currently incomplete due to lack of data

pub struct Flowmeter30 {
    // not labelled on diagram
    // we use the convention of top of bypass branch to bottom
    // hence degree is a 180-90 degrees = -90 degrees
    
}
impl Flowmeter30 {

    // let's import everything necessary:

    /// darcy = 0
    /// 
    /// as fldk is empirical and indpendent of L/D
    /// in that correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 18 + 93000/(Re^1.35)
    ///
    /// because
    /// fldk = 18 + 93000/(Re^1.35)
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of FM-30
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_30";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 - 180.0);

        let flowmeter_30: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return flowmeter_30;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


// DRACS Loop (secondary loop for natural convection) 
// components are here
//
//
//
//
//
//

/// DRACS loop flow flowmeter 60
/// natural convection heat exchanger in DRACS loop
/// this is the secondary loop equivalent for
/// decay heat removal
///
/// diagram label 37a on simulation model
pub struct Flowmeter60 {
    // we use the convention of top of bypass branch to bottom (Tank 2)
    // hence degree is -90
    
}
impl Flowmeter60 {

    // let's import everything necessary:

    /// darcy = 0
    /// 
    /// as fldk is empirical and indpendent of L/D
    /// in that correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 18 + 93000/(Re^1.35)
    ///
    /// because
    /// fldk = 18 + 93000/(Re^1.35)
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            18.0 + 93000.0/reynolds_number.powf(1.35);
        // coriolis flowmeter

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of FM-60 within DRACS loop
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "flowmeter_60_label_37a";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.36);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 - 180.0);

        let flowmeter_60_label_37a: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return flowmeter_60_label_37a;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}


/// static mixer MX-60 within DRACS loop
///
/// label 36 on diagram
pub struct StaticMixer60 {
    // static mixer 60 (MX-60) on CIET diagram
    // in the NDHX branch in secondary DRACS loop
    // just after the NDHX heat exchanger
    // from top to bottom
    // ie this is where hot fluid gets cooled by a fan
    // label 36
    //
    // in reality flow goes from top to
    // bottom in natural convection
    //
    
}
impl StaticMixer60 {


    /// darcy = 0
    /// 
    /// as fldk is empirical and indpendent of L/D
    /// in that correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 21.0 + 4000/Re
    ///
    /// This is because fldk = = 21.0 + 4000/Re
    /// And we don't have L/D dependence
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of MX-60
    /// static mixer 
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_60_label_36";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(-58.99728);

        let static_mixer_60_label_36: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_60_label_36;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

/// static mixer MX-61 within DRACS loop
pub struct StaticMixer61 {
    // static mixer 61 (MX-61) on CIET diagram
    // in the DHX branch in secondary DRACS loop
    // just before the DHX heat exchanger
    // from top to bottom
    // ie this is where cool fluid gets heated by the
    // primary loop heat exchanger
    // label 31
    //
    // in reality flow goes from bottom to
    // top in natural convection
    // so it is actually after the DHX from perspective of flow
    //
    
}
impl StaticMixer61 {


    /// darcy = 0
    /// 
    /// as fldk is empirical and indpendent of L/D
    /// in that correlation
    pub fn custom_darcy(_reynolds_number: f64, _roughness_ratio: f64) -> f64 {
        return 0.0;
    }

    /// custom K = 21.0 + 4000/Re
    ///
    /// This is because fldk = = 21.0 + 4000/Re
    /// And we don't have L/D dependence
    pub fn custom_k(mut reynolds_number: f64) -> f64 {
        let mut reverse_flow = false;

        // the user account for reverse flow scenarios...
        if reynolds_number < 0.0 {
            reverse_flow = true;
            reynolds_number = reynolds_number * -1.0;
        }

        let custom_k_value =
            21.0 + 4000.0/reynolds_number;

        if reverse_flow {
            return -custom_k_value;
        }

        return custom_k_value;

    }

    /// returns an instance of static mixer 61
    pub fn get(&self) -> TherminolCustomComponent {

        let name = "static_mixer_61_label_31";

        
        let fluid_temp = ThermodynamicTemperature::new::<degree_celsius>(21.0);

        let hydraulic_diameter = Length::new::<meter>(2.79e-2);
        let component_length = Length::new::<meter>(0.33);
        let cross_sectional_area = Area::new::<square_meter>(6.11e-4);
        // note that aboslute roughness doesn't matter here really
        // because we are having laminar flow in the experimental data range
        let absolute_roughness = Length::new::<millimeter>(0.015);
        let incline_angle = Angle::new::<degree>(90.0 - 180.0);

        let static_mixer_61_label_31: TherminolCustomComponent
            = TherminolCustomComponent::new(
                name, 
                fluid_temp, 
                incline_angle, 
                component_length, 
                cross_sectional_area, 
                hydraulic_diameter, 
                absolute_roughness, 
                &Self::custom_k, 
                &Self::custom_darcy);

        return static_mixer_61_label_31;
    }

    pub fn new() -> Self {

        return Self {  }

    }
}

// miscellaneous items are here
//
//
//
//
// list is currently empty


