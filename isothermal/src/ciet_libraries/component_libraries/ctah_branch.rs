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

use crate::{Pipe6a, StaticMixer41, CTAHVertical, CTAHHorizontal, 
    Pipe8a, StaticMixer40, Pipe9, Pipe10, Pipe11, Pipe12, CTAHPump, Pipe13, Pipe14, 
    therminol_pipe::TherminolPipe, therminol_component::TherminolCustomComponent, Pipe16, Pipe15, Branch17, Flowmeter40};

extern crate roots;
use roots::find_root_brent;
use roots::SimpleConvergency;

pub struct CTAHBranch<'ctah_branch_lifetime> {

    pipe6a: Pipe6a, 
    // component 6
    static_mixer_41: StaticMixer41, 
    // 7a
    ctah_vertical: CTAHVertical, 
    // 7b
    ctah_horizontal: CTAHHorizontal, 
    // 8a
    pipe_8a: Pipe8a,
    // 8
    static_mixer_40: StaticMixer40, // 8
    // 9
    pipe_9: Pipe9,
    // 10
    pipe_10: Pipe10,
    // 11
    pipe_11: Pipe11,
    //12
    pipe_12: Pipe12,
    // between 12 and 13 
    ctah_pump: CTAHPump,
    //13
    pipe_13: Pipe13,
    //14
    pipe_14: Pipe14,
    //15 
    flowmeter_40_14a: Flowmeter40,
    //16
    pipe_15: Pipe15,
    //17
    pipe_16: Pipe16,
    //18
    branch_17: Branch17,

    fluid_component_vector_immutable: 
        Vec<&'ctah_branch_lifetime dyn FluidComponent>
}

impl<'ctah_branch_lifetime> CTAHBranch<'ctah_branch_lifetime> {

    /// constructor, returns an instance of the ctah branch
    pub fn new() -> Self {

        

        // constructor will return the CTAH branch with all its items
        // but the vector will be empty
        let ctah_branch_vector_empty: Vec<&'ctah_branch_lifetime dyn FluidComponent>
            = vec![];

        return Self { 
            pipe6a: Pipe6a::new(),
            static_mixer_41: StaticMixer41::new(),
            ctah_vertical: CTAHVertical::new(),
            ctah_horizontal: CTAHHorizontal::new(),
            pipe_8a: Pipe8a::new(),
            static_mixer_40: StaticMixer40::new(),
            pipe_9: Pipe9::new(),
            pipe_10: Pipe10::new(),
            pipe_11: Pipe11::new(),
            pipe_12: Pipe12::new(),
            ctah_pump: CTAHPump::new(), 
            pipe_13: Pipe13::new(),
            pipe_14: Pipe14::new(),
            flowmeter_40_14a: Flowmeter40::new(),
            pipe_15: Pipe15::new(),
            pipe_16: Pipe16::new(),
            branch_17: Branch17::new(),
            fluid_component_vector_immutable: ctah_branch_vector_empty,
        }
    }




    /// sets the ctah pump pressure to whatever value the user specifies
    ///
    /// it basically deletes the existing ctah pump and instantiates a
    /// new one at the correct position
    ///
    pub fn set_ctah_pump_pressure(&mut self,
                                  user_specified_pressure: Pressure,
                                  ctah_pump: &'ctah_branch_lifetime mut 
                                  TherminolCustomComponent){

        // should we do max/min pressure??? IDK
        // i'll just have an actual ctah pump object
        
        // the ctah pump will be usually at 10th element of the vector starting
        // from 0


        ctah_pump.set_internal_pressure_source(user_specified_pressure);

        self.fluid_component_vector_immutable[10] = ctah_pump;
        // inside the CTAH branch i should have all my components
        // so for ease of use and readability, i may want to nest the 
        // actual component objects within the ctah branch

    }

    /// these help to return the components to the environment
    ///
    /// what you are supposed to do is first
    /// to 

    pub fn get_pipe6a(&self) -> TherminolPipe {
        return self.pipe6a.get();
    }

    pub fn get_static_mixer_41(&self) -> TherminolCustomComponent {
        return self.static_mixer_41.get();
    }

    pub fn get_ctah_vertical(&self) -> TherminolCustomComponent {
        return self.ctah_vertical.get();
    }

    pub fn get_ctah_horizontal(&self) -> TherminolCustomComponent{
        return self.ctah_horizontal.get();
    }

    pub fn get_pipe_8a(&self) -> TherminolPipe {
        return self.pipe_8a.get();
    }

    pub fn get_static_mixer_40(&self) -> TherminolCustomComponent {
        return self.static_mixer_40.get();
    }
    pub fn get_pipe_9(&self) -> TherminolPipe {
        return self.pipe_9.get();
    }

    pub fn get_pipe_10(&self) -> TherminolPipe {
        return self.pipe_10.get();
    }

    pub fn get_pipe_11(&self) -> TherminolPipe {
        return self.pipe_11.get();
    }

    pub fn get_pipe_12(&self) -> TherminolPipe {
        return self.pipe_12.get();
    }

    pub fn get_ctah_pump(&self) -> TherminolCustomComponent {
        return self.ctah_pump.get();
    }

    pub fn get_pipe_13(&self) -> TherminolPipe {
        return self.pipe_13.get();
    }
    pub fn get_pipe_14(&self) -> TherminolPipe {
        return self.pipe_14.get();
    }
    pub fn get_flowmeter_40_14a(&self) -> TherminolCustomComponent {
        return self.flowmeter_40_14a.get();
    }

    pub fn get_pipe_15(&self) -> TherminolPipe {
        return self.pipe_15.get();
    }
    pub fn get_pipe_16(&self) -> TherminolPipe {
        return self.pipe_16.get();
    }
    pub fn get_branch_17(&self) -> TherminolPipe {
        return self.branch_17.get();
    }

}

impl<'ctah_branch_lifetime> FluidComponentCollectionMethods for CTAHBranch<'ctah_branch_lifetime> {

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

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate{


        let fluid_component_collection_vector = 
            self.get_immutable_fluid_component_vector();


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

impl<'ctah_branch_lifetime> FluidComponentCollection<'ctah_branch_lifetime> 
for CTAHBranch<'ctah_branch_lifetime> {

            fn get_immutable_fluid_component_vector(&self)
                -> &Vec<&'ctah_branch_lifetime dyn FluidComponent> {

                    return &self.fluid_component_vector_immutable;
                }

            fn set_fluid_component_vector(
                &mut self, 
                fluid_component_vector: 
                Vec<&'ctah_branch_lifetime dyn FluidComponent>){

                self.fluid_component_vector_immutable = 
                    fluid_component_vector;

            }

}

impl<'ctah_branch_lifetime> 
FluidComponentCollectionSeriesAssociatedFunctions for CTAHBranch<'ctah_branch_lifetime> {}

