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

pub trait TherminolCustomComponentTraits<'trait_lifetime> :
 FluidComponent
+ FluidCustomComponentCalcPressureLoss<'trait_lifetime>
+ FluidCustomComponentCalcPressureChange<'trait_lifetime>
{}

// first we create an therminol pipe struct
// and start implementing it
pub struct TherminolCustomComponent<'pipe_lifetime> {

    therminol_properties: TherminolVP1Properties,
    fluid_temp: ThermodynamicTemperature,
    fluid_mass_flowrate: MassRate,

    internal_pressure: Pressure,
    incline_angle: Angle,
    component_length: Length,
    cross_sectional_area: Area,
    hydraulic_diameter: Length,

    pressure_loss: Pressure,
    absolute_roughness: Length,
    name: String,
    
    // these are trait object references which cannot be brought into
    // the scope
    custom_k: &'pipe_lifetime dyn Fn(f64) -> f64,
    custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) ->f64,

}

impl<'pipe_lifetime> 
TherminolCustomComponentTraits<'pipe_lifetime> for TherminolCustomComponent<'pipe_lifetime> {}


impl<'pipe_lifetime> 
FluidCustomComponentCalcPressureChange<'pipe_lifetime> 
for TherminolCustomComponent<'pipe_lifetime> {
}

impl<'pipe_lifetime> 
FluidCustomComponentCalcPressureLoss<'pipe_lifetime> 
for TherminolCustomComponent<'pipe_lifetime> {

    fn get_custom_component_absolute_roughness(
        &mut self) -> Length {

        return self.absolute_roughness;
    }

    fn get_custom_component_absolute_roughness_immutable(
        &self) -> Length {

        return self.absolute_roughness;
    }

    fn get_custom_darcy(&mut self) 
        -> &dyn Fn(f64, f64) -> f64 {

            return self.custom_darcy.clone();

        }


    fn get_custom_darcy_immutable(&self) 
        -> &dyn Fn(f64, f64) -> f64 {

            return self.custom_darcy.clone();

        }

    fn get_custom_k(&mut self) 
        -> &dyn Fn(f64) -> f64 {

            return self.custom_k.clone();

        }

    fn get_custom_k_immutable(&self) 
        -> &dyn Fn(f64) -> f64 {

            return self.custom_k.clone();

        }

    fn set_custom_k(
        &mut self,
        custom_k: &'pipe_lifetime dyn Fn(f64) -> f64){

        self.custom_k = custom_k;

    }

    fn set_custom_darcy(
        &mut self,
        custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64){

        self.custom_darcy = custom_darcy;
    }




}

impl<'pipe_lifetime> 
FluidComponent for TherminolCustomComponent<'pipe_lifetime>{
    fn get_pressure_loss(&mut self) -> Pressure {

        let fluid_mass_flowrate = 
            self.fluid_mass_flowrate;

        let cross_sectional_area = 
            self.get_cross_sectional_area();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter();

        let fluid_viscosity = 
            self.get_fluid_viscosity();

        let fluid_density = 
            self.get_fluid_density();

        let component_length = 
            self.get_component_length();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness();

        // i need to make some immutable borrows here...
        let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;

        let pressure_loss =
            Self::
            fluid_custom_component_calc_pressure_loss(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                custom_darcy, custom_k);

        self.pressure_loss = pressure_loss;

        return pressure_loss;


    }

    fn get_pressure_loss_immutable(
        &self,
        mass_flowrate: MassRate) -> Pressure {

        let fluid_mass_flowrate = 
            mass_flowrate;

        let cross_sectional_area = 
            self.get_cross_sectional_area_immutable();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter_immutable();

        let fluid_viscosity = 
            self.get_fluid_viscosity_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();

        let component_length = 
            self.get_component_length_immutable();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness_immutable();

        // i need to make some immutable borrows here...
        let custom_darcy: &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;

        let pressure_loss =
            Self:: fluid_custom_component_calc_pressure_loss(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                custom_darcy, custom_k);


        return pressure_loss;

    }
    fn set_pressure_loss(&mut self, pressure_loss: Pressure){
        self.pressure_loss = pressure_loss;
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
        self.fluid_mass_flowrate = mass_flowrate;
    }

    fn get_mass_flowrate(&mut self) -> MassRate {


        //i'll have to get the pressure change
        //
        // pressure_change = 
        // - pressure_change
        // + hydrostatic pressure change
        // + internal pressure source
        //

        // internal pressure source
        let internal_pressure_source = 
            self.get_internal_pressure_source();

        // hydrostatic pressure
        let incline_angle = 
            self.get_incline_angle();

        let hydrostatic_pressure_change =
            self.get_hydrostatic_pressure_change();

        // pressure_loss term
        //
        //
        let pressure_loss = 
            self.get_pressure_loss();

        // now we get pressure change

        let pressure_change =
            - pressure_loss
            + hydrostatic_pressure_change
            + internal_pressure_source;

        let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;


        let cross_sectional_area = 
            self.get_cross_sectional_area();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter();

        let fluid_viscosity = 
            self.get_fluid_viscosity();

        let fluid_density = 
            self.get_fluid_density();

        let component_length = 
            self.get_component_length();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness();

        let source_pressure = 
            self.get_internal_pressure_source();

        let mass_flowrate =
            Self::
            fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                incline_angle, 
                source_pressure, 
                custom_darcy, 
                custom_k);

        self.fluid_mass_flowrate = mass_flowrate;

        return mass_flowrate;
    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self,
        pressure_loss: Pressure) -> MassRate {


        //i'll have to get the pressure change
        //
        // pressure_change = 
        // - pressure_change
        // + hydrostatic pressure change
        // + internal pressure source
        //

        // internal pressure source
        let internal_pressure_source = 
            self.get_internal_pressure_source_immutable();

        // hydrostatic pressure

        let incline_angle = 
            self.get_incline_angle_immutable();


        let hydrostatic_pressure_change =
            self.get_hydrostatic_pressure_change_immutable();


        // now we get pressure change

        let pressure_change =
            - pressure_loss
            + hydrostatic_pressure_change
            + internal_pressure_source;

        let custom_darcy : &dyn Fn(f64, f64) -> f64 = 
            self.custom_darcy;

        let custom_k : &dyn Fn(f64) -> f64 =
            self.custom_k;


        let cross_sectional_area = 
            self.get_cross_sectional_area_immutable();

        let hydraulic_diameter = 
            self.get_hydraulic_diameter_immutable();

        let fluid_viscosity = 
            self.get_fluid_viscosity_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable();

        let component_length = 
            self.get_component_length_immutable();

        let absolute_roughness = 
            self.get_custom_component_absolute_roughness_immutable();

        let source_pressure = 
            self.get_internal_pressure_source_immutable();

        let mass_flowrate =
            Self::
            fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                component_length, 
                absolute_roughness, 
                incline_angle, 
                source_pressure, 
                custom_darcy, 
                custom_k);

        return mass_flowrate;
    }

    fn get_cross_sectional_area(&mut self) -> Area {
        return self.cross_sectional_area;
    }

    fn get_cross_sectional_area_immutable(&self) -> Area {
        return self.cross_sectional_area;
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


impl<'pipe_lifetime> TherminolCustomComponent<'pipe_lifetime>{

    // let's implement a generic constructor
    pub fn new(name: &str,
               fluid_temp: ThermodynamicTemperature,
               incline_angle: Angle,
               component_length: Length,
               cross_sectional_area: Area,
               hydraulic_diameter: Length,
               absolute_roughness: Length,
               custom_k: &'pipe_lifetime dyn Fn(f64)-> f64 ,
               custom_darcy: &'pipe_lifetime dyn Fn(f64,f64) -> f64 ) -> Self {

        return Self { 
            name: name.to_string(),
            therminol_properties: TherminolVP1Properties::new(),
            fluid_temp: fluid_temp, 
            fluid_mass_flowrate: MassRate::new::<kilogram_per_second>(0.0), 
            internal_pressure: Pressure::new::<pascal>(0.0), 
            incline_angle: incline_angle, 
            component_length: component_length ,
            hydraulic_diameter: hydraulic_diameter,
            cross_sectional_area: cross_sectional_area,
            pressure_loss: Pressure::new::<pascal>(0.0),
            absolute_roughness: absolute_roughness,
            custom_k: custom_k,
            custom_darcy: custom_darcy,
        };

    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn set_name(&mut self, name: &str) {

        self.name = name.to_string();
    }

}

