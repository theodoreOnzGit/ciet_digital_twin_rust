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

use crate::{
    therminol_pipe::TherminolPipe, therminol_component::TherminolCustomComponent, 
    Pipe26, StaticMixer21, Pipe25a, DHXShellSideHeatExchanger, 
    StaticMixer20, Pipe23a, Pipe22, Flowmeter20WithHighKCheckValve, 
    Pipe21, Pipe20, Pipe19, Flowmeter20};

extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;

pub struct DHXBranch<'dhx_branch_lifetime> {

    pipe26: Pipe26,
    // item 25
    static_mixer_21: StaticMixer21,
    pipe25a: Pipe25a,
    // item 24
    dhx_shell_side_heat_exchanger: DHXShellSideHeatExchanger,
    // item 23
    static_mixer_20: StaticMixer20,
    pipe23a: Pipe23a,
    pipe22: Pipe22,
    // item 21a
    flowmeter20: Flowmeter20,
    flowmeter20_check_valve: Flowmeter20WithHighKCheckValve,
    pipe21: Pipe21,
    pipe20: Pipe20,
    pipe19: Pipe19,


    fluid_component_vector_immutable: 
        Vec<&'dhx_branch_lifetime dyn FluidComponent>
}

impl<'dhx_branch_lifetime> DHXBranch<'dhx_branch_lifetime> {

    /// constructor, returns an instance of the dhx branch
    pub fn new() -> Self {

        let empty_vec: Vec<&'dhx_branch_lifetime dyn FluidComponent> = vec![];
        

        // constructor will return the dhx branch with all its factories
        // but the vector will be empty
        //

        Self {
            pipe26: Pipe26::new(),
            // item 25
            static_mixer_21: StaticMixer21::new(),
            pipe25a: Pipe25a::new(),
            // item 24
            dhx_shell_side_heat_exchanger: DHXShellSideHeatExchanger::new(),
            // item 23
            static_mixer_20: StaticMixer20::new(),
            pipe23a: Pipe23a::new(),
            pipe22: Pipe22::new(),
            // item 21a
            flowmeter20: Flowmeter20::new(),
            flowmeter20_check_valve: Flowmeter20WithHighKCheckValve::new(),
            pipe21: Pipe21::new(),
            pipe20: Pipe20::new(),
            pipe19: Pipe19::new(),
            fluid_component_vector_immutable: empty_vec
        }
    }

    pub fn get_pipe26(&self) -> TherminolPipe {
        return self.pipe26.get();
    }

    pub fn get_static_mixer_21(&self) -> TherminolCustomComponent {
        return self.static_mixer_21.get();
    }

    pub fn get_pipe25a(&self) -> TherminolPipe {
        return self.pipe25a.get();
    }

    pub fn get_dhx_shell_side_heat_exchanger(&self) -> TherminolCustomComponent {
        return self.dhx_shell_side_heat_exchanger.get();
    }

    pub fn get_static_mixer_20(&self) -> TherminolCustomComponent {
        return self.static_mixer_20.get();
    }

    pub fn get_pipe23a(&self) -> TherminolPipe {
        return self.pipe23a.get();
    }

    pub fn get_pipe22(&self) -> TherminolPipe {
        return self.pipe22.get();
    }

    pub fn get_flowmeter20(&self) -> TherminolCustomComponent {
        return self.flowmeter20.get();
    }

    pub fn get_flowmeter20_check_valve(&self) -> TherminolCustomComponent {
        return self.flowmeter20_check_valve.get();
    }

    pub fn get_pipe21(&self) -> TherminolPipe {
        return self.pipe21.get();
    }

    pub fn get_pipe20(&self) -> TherminolPipe {
        return self.pipe20.get();
    }

    pub fn get_pipe19(&self) -> TherminolPipe {
        return self.pipe19.get();
    }
}


impl<'dhx_branch_lifetime> FluidComponentCollectionMethods for DHXBranch<'dhx_branch_lifetime> {

    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure{
        let fluid_component_collection_vector = 
            self.get_immutable_fluid_component_vector();

        let pressure_change = 
            <Self as FluidComponentCollectionSeriesAssociatedFunctions>
            ::calculate_pressure_change_from_mass_flowrate(
                fluid_mass_flowrate, 
                fluid_component_collection_vector);

        return pressure_change;
    }

    /// calculates mass flowrate from pressure change
    /// for dhx branch
    ///
    /// no check valve behaviour here
    /// it will be put in the ciet part
    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate{



        let fluid_component_collection_vector = 
            self.get_immutable_fluid_component_vector();

        // here is where i implement the check valve behaviour
        let zero_flowrate = 
            MassRate::new::<kilogram_per_second>(0.0);
        let dhx_branch_hydrostatic_pressure = 
            self.get_pressure_change(MassRate::new::<kilogram_per_second>(0.0));

        if pressure_change.value > dhx_branch_hydrostatic_pressure.value {

            return zero_flowrate;

        } 

        // i'm keeping bounds artificially low for ciet
        // -1 or +1 kg/s
        let upper_bound = MassRate::new::<kilogram_per_second>(1.0);


        let lower_bound = MassRate::new::<kilogram_per_second>(-1.0);


        // now we have a function comparing the pressure change
        // to the pressure change of the calculated value

        let mass_flow_from_pressure_chg_root = 
            |mass_flow_kg_per_s: f64| -> f64 {

            let mass_flow_kg_per_s_double = mass_flow_kg_per_s; 

            let mass_rate = 
                MassRate::new::<kilogram_per_second>(
                    mass_flow_kg_per_s_double);


            let pressure_change_tested = 
                Self::calculate_pressure_change_from_mass_flowrate(
                mass_rate, 
                fluid_component_collection_vector);

            // now i've obtained the pressure change, i convert it to f64

            let pressure_change_user_stipulated_pascals_f64 = 
                pressure_change.value;

            // since we are finding root, then we must also
            // subtract it from our pressure change value


            let pressure_change_error: f64 =
                pressure_change_user_stipulated_pascals_f64 - 
                pressure_change_tested.value;

            return pressure_change_error;

        };

        let mut convergency = SimpleConvergency { eps:1e-9_f64, max_iter:30 };

        let mass_flowrate_result 
            = find_root_brent(
                upper_bound.value,
                lower_bound.value,
                &mass_flow_from_pressure_chg_root,
                &mut convergency);

        return MassRate::new::<kilogram_per_second>(mass_flowrate_result.unwrap());
    }


}

impl<'dhx_branch_lifetime> FluidComponentCollection<'dhx_branch_lifetime> 
for DHXBranch<'dhx_branch_lifetime> {

            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'dhx_branch_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'dhx_branch_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

}

impl<'dhx_branch_lifetime> 
FluidComponentCollectionSeriesAssociatedFunctions for DHXBranch<'dhx_branch_lifetime> {}

