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

// we will implement a few properties here for our therminol pipe
// for clarity we will list them in a
// supertrait
// This makes it easy to see what traits are being implemented here

/// This supertrait combines the necessary traits
/// for an isothermal therminol pipe
pub trait TherminolPipeTraits<'trait_lifetime> :
 FluidComponent
+ FluidPipeCalcPressureChange
+ FluidPipeCalcPressureLoss
{}

// first we create an therminol pipe struct
// and start implementing it
// 
/// this struct or class is to be instantiated into an object
/// which can represent therminol pipes
pub struct TherminolPipe {

    therminol_properties: TherminolVP1Properties,
    fluid_temp: ThermodynamicTemperature,
    fluid_mass_flowrate: MassRate,

    internal_pressure: Pressure,
    incline_angle: Angle,
    component_length: Length,
    hydraulic_diameter: Length,

    pressure_loss: Pressure,
    form_loss_k: f64,
    absolute_roughness: Length,
    name: String,

}

impl<'pipe_lifetime> 
TherminolPipeTraits<'pipe_lifetime> for TherminolPipe {}

impl<'pipe_lifetime> 
FluidPipeCalcPressureChange for TherminolPipe {
}

impl<'pipe_lifetime> 
FluidPipeCalcPressureLoss for TherminolPipe {

    fn get_pipe_form_loss_k(&mut self) -> f64 {
        return self.form_loss_k;
    }

    fn get_pipe_form_loss_k_immutable(&self) -> f64 {
        return self.form_loss_k;
    }

    /// return absolute roughness for pipe
    /// for a typical copper pipe
    /// it is 0.002 mm 
    /// i did a web search
    ///
    fn get_pipe_absolute_roughness(&mut self) -> Length {
        return self.absolute_roughness;
    }

    fn get_pipe_absolute_roughness_immutable(&self) -> Length {
        return self.absolute_roughness;
    }

}

impl<'pipe_lifetime> 
FluidComponent for TherminolPipe{
    fn get_pressure_loss(&mut self) -> Pressure {


        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k();
        let absolute_roughness = self.get_pipe_absolute_roughness();
        let cross_sectional_area = self.get_cross_sectional_area();
        let mass_flowrate = self.fluid_mass_flowrate;
        let hydraulic_diameter = self.get_hydraulic_diameter();
        let viscosity = self.get_fluid_viscosity();
        let density = self.get_fluid_density();
        let pipe_legnth = self.get_component_length();


        // calculate the pressure loss

        let pressure_loss = 
            Self::pipe_calc_pressure_loss(
                mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                viscosity,
                density,
                pipe_legnth,
                absolute_roughness,
                form_loss_k);

        // you can return the pressure loss straightaway
        // or set the struct variable first and then
        // return it

        self.pressure_loss = pressure_loss;

        return self.pressure_loss;
    }

    fn get_pressure_loss_immutable(
        &self,
        mass_flowrate: MassRate) -> Pressure {


        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k_immutable();
        let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
        let cross_sectional_area = self.get_cross_sectional_area_immutable();
        let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
        let viscosity = self.get_fluid_viscosity_immutable();
        let density = self.get_fluid_density_immutable();
        let pipe_legnth = self.get_component_length_immutable();


        // calculate the pressure loss

        let pressure_loss = 
            Self::pipe_calc_pressure_loss(
                mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                viscosity,
                density,
                pipe_legnth,
                absolute_roughness,
                form_loss_k);

        // you can return the pressure loss straightaway
        // or set the struct variable first and then
        // return it


        return pressure_loss;
    }
    fn set_pressure_loss(&mut self, pressure_loss: Pressure){
        self.pressure_loss = pressure_loss;
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
        self.fluid_mass_flowrate = mass_flowrate;
    }

    fn get_mass_flowrate(&mut self) -> MassRate {
        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k();
        let absolute_roughness = self.get_pipe_absolute_roughness();
        let cross_sectional_area = self.get_cross_sectional_area();
        let hydraulic_diameter = self.get_hydraulic_diameter();
        let fluid_viscosity = self.get_fluid_viscosity();
        let fluid_density = self.get_fluid_density();
        let pipe_length = self.get_component_length();
        let pressure_loss = self.pressure_loss;
        let incline_angle = self.get_incline_angle();
        let internal_pressure_source = self.get_internal_pressure_source();

        let pressure_change = 
            -pressure_loss 
            + internal_pressure_source 
            + self.get_hydrostatic_pressure_change();

        let mass_flowrate = 
            Self::pipe_calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                pipe_length, 
                absolute_roughness, 
                form_loss_k,
                incline_angle,
                internal_pressure_source);

        // you can return the mass flowrate straightaway
        // or set the struct variable first and then
        // return it

        self.set_mass_flowrate(mass_flowrate);

        return self.fluid_mass_flowrate;

    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self,
        pressure_loss: Pressure) -> MassRate {
        // get pipe parameters and flow conditions
        // from the get methods
        let form_loss_k = self.get_pipe_form_loss_k_immutable();
        let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
        let cross_sectional_area = self.get_cross_sectional_area_immutable();
        let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
        let fluid_viscosity = self.get_fluid_viscosity_immutable();
        let fluid_density = self.get_fluid_density_immutable();
        let pipe_length = self.get_component_length_immutable();
        let incline_angle = self.get_incline_angle_immutable();
        let internal_pressure_source = self.get_internal_pressure_source_immutable();

        let pressure_change = 
            -pressure_loss 
            + internal_pressure_source 
            + <Self as FluidPipeCalcPressureChange>::
            get_hydrostatic_pressure_change(
                pipe_length,
                incline_angle,
                fluid_density);

        let mass_flowrate = 
            Self::pipe_calculate_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                pipe_length, 
                absolute_roughness, 
                form_loss_k,
                incline_angle,
                internal_pressure_source);

        // you can return the mass flowrate straightaway
        // or set the struct variable first and then
        // return it


        return mass_flowrate;

    }

    fn get_cross_sectional_area(&mut self) -> Area {
        return self.get_hydraulic_diameter()*
            self.get_hydraulic_diameter()*
            PI/4.0_f64;
    }

    fn get_cross_sectional_area_immutable(&self) -> Area {
        return self.get_hydraulic_diameter_immutable()*
            self.get_hydraulic_diameter_immutable()*
            PI/4.0_f64;
    }

    fn get_hydraulic_diameter(&mut self) -> Length {

        return self.hydraulic_diameter;

    }

    fn get_hydraulic_diameter_immutable(&self) -> Length {


        return self.hydraulic_diameter;

    }


    fn get_fluid_viscosity(&mut self) -> DynamicViscosity {

        // get fluid temp first
        let fluid_temp = self.fluid_temp;

        // then the fluid properties

        let fluid_viscosity = self.therminol_properties.viscosity(fluid_temp);
        return fluid_viscosity;


    }

    fn get_fluid_viscosity_immutable(&self) -> DynamicViscosity {


        // get fluid temp first
        let fluid_temp = self.fluid_temp;

        // then the fluid properties

        let fluid_viscosity = self.therminol_properties.viscosity(fluid_temp);

        return fluid_viscosity;



    }

    fn get_fluid_density(&mut self) -> MassDensity {

        // get fluid temp first
        let fluid_temp = self.fluid_temp;

        // then the fluid properties

        let fluid_density = self.therminol_properties.density(fluid_temp);

        return fluid_density;


    }

    fn get_fluid_density_immutable(&self) -> MassDensity {


        // get fluid temp first
        let fluid_temp = self.fluid_temp;

        // then the fluid properties
        let fluid_density = self.therminol_properties.density(fluid_temp);

        return fluid_density;



    }

    fn get_component_length(&mut self) -> Length {

        return self.component_length;
    }

    fn get_component_length_immutable(&self) -> Length {

        return self.component_length;
    }

    fn get_incline_angle(&mut self) -> Angle {

        return self.incline_angle;
    }

    fn get_incline_angle_immutable(&self) -> Angle {

        return self.incline_angle;
    }



    fn get_internal_pressure_source(&mut self) -> Pressure {

        return self.internal_pressure;
    }

    fn get_internal_pressure_source_immutable(&self) -> Pressure {

        return self.internal_pressure;
    }

    fn set_internal_pressure_source(&mut self,
                                    internal_pressure: Pressure){

        self.internal_pressure = internal_pressure;
    }

}

impl<'pipe_lifetime> TherminolPipe{


    /// constructor for therminol pipes
    // let's implement a generic constructor
    pub fn new(name: &str,
           fluid_temp: ThermodynamicTemperature,
           incline_angle: Angle,
           component_length: Length,
           hydraulic_diameter: Length,
           form_loss_k: f64,
           absolute_roughness: Length) -> Self {

        return Self { 
            name: name.to_string(),
            therminol_properties: TherminolVP1Properties::new(),
            fluid_temp: fluid_temp, 
            fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            internal_pressure: Pressure::new::<pascal>(0.0), 
            incline_angle: incline_angle, 
            component_length: component_length ,
            hydraulic_diameter: hydraulic_diameter ,
            pressure_loss: Pressure::new::<pascal>(0.0),
            form_loss_k: form_loss_k ,
            absolute_roughness: absolute_roughness,
        };



    }
    /// gets the name of the therminol pipe as a string slice
    pub fn get_name(&self) -> &str {
        return &self.name;
    }
    /// sets the name for the therminol pipe as a string slice
    pub fn set_name(&mut self, name: &str) {

        self.name = name.to_string();
    }
}

