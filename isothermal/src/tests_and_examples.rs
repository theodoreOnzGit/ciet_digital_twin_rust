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

extern crate approx;
#[test]
pub fn assert_ctah_behave_ok() {

    let mut mass_flowrate_kg_per_sec_vec: Vec<f64> = 
        vec![];

    mass_flowrate_kg_per_sec_vec.push(0.0);
    mass_flowrate_kg_per_sec_vec.push(0.2);
    mass_flowrate_kg_per_sec_vec.push(0.4);
    mass_flowrate_kg_per_sec_vec.push(0.7);
    mass_flowrate_kg_per_sec_vec.push(1.0);
    mass_flowrate_kg_per_sec_vec.push(-0.2);
    mass_flowrate_kg_per_sec_vec.push(-0.4);
    mass_flowrate_kg_per_sec_vec.push(-0.7);
    mass_flowrate_kg_per_sec_vec.push(-1.0);

    for mass_rate_kg_per_s in mass_flowrate_kg_per_sec_vec.iter() {

        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = 0.0;

        let reference_ctah_pressure_change = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                *mass_rate_kg_per_s,
                temperature_degrees_c,
                pump_pressure_pascals);

        // get a test version of ctah, the one based on traits

        let ctah_branch_factory = CTAHBranch::new();
        let pipe6a = ctah_branch_factory.get_pipe6a();
        let static_mixer_41 = ctah_branch_factory.get_static_mixer_41();
        let ctah_vertical = ctah_branch_factory.get_ctah_vertical();
        let ctah_horizontal = ctah_branch_factory.get_ctah_horizontal();
        let pipe_8a = ctah_branch_factory.get_pipe_8a();
        let static_mixer_40 = ctah_branch_factory.get_static_mixer_40();
        let pipe_9 = ctah_branch_factory.get_pipe_9();
        let pipe_10 = ctah_branch_factory.get_pipe_10();
        let pipe_11 = ctah_branch_factory.get_pipe_11();
        let pipe_12 = ctah_branch_factory.get_pipe_12();
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


        let mut ctah_branch_vector :Vec<&dyn FluidComponent> = vec![];
        // element number: 0 
        ctah_branch_vector.push(&pipe6a); 
        // 1
        ctah_branch_vector.push(&static_mixer_41);
        // 2
        ctah_branch_vector.push(&ctah_vertical);
        // 3
        ctah_branch_vector.push(&ctah_horizontal);
        // 4
        ctah_branch_vector.push(&pipe_8a);
        // 5
        ctah_branch_vector.push(&static_mixer_40);
        // 6
        ctah_branch_vector.push(&pipe_9);
        // 7
        ctah_branch_vector.push(&pipe_10);
        // 8
        ctah_branch_vector.push(&pipe_11);
        // 9
        ctah_branch_vector.push(&pipe_12);
        // 10
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_pressure_change = 
            ctah_branch.get_pressure_change(
                MassRate::new::<kilogram_per_second>(*mass_rate_kg_per_s));


        // assert

        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            test_ctah_pressure_change.value,
            max_relative = 0.01);
    }


    // now i want to test if the ctah branch 
    // can handle positive, negative and zero
    // pressure drops

    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);
    pressure_vec_pa.push(-1.0*10000.0);
    pressure_vec_pa.push(1.0*10000.0);
    pressure_vec_pa.push(-1.0*90000.0);
    pressure_vec_pa.push(1.0*90000.0);

    for pressure_change_value in pressure_vec_pa.iter(){
        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = 1000.0;


        // get a test version of ctah, the one based on traits

        let ctah_branch_factory = CTAHBranch::new();
        let pipe6a = ctah_branch_factory.get_pipe6a();
        let static_mixer_41 = ctah_branch_factory.get_static_mixer_41();
        let ctah_vertical = ctah_branch_factory.get_ctah_vertical();
        let ctah_horizontal = ctah_branch_factory.get_ctah_horizontal();
        let pipe_8a = ctah_branch_factory.get_pipe_8a();
        let static_mixer_40 = ctah_branch_factory.get_static_mixer_40();
        let pipe_9 = ctah_branch_factory.get_pipe_9();
        let pipe_10 = ctah_branch_factory.get_pipe_10();
        let pipe_11 = ctah_branch_factory.get_pipe_11();
        let pipe_12 = ctah_branch_factory.get_pipe_12();
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


        let mut ctah_branch_vector :Vec<&dyn FluidComponent> = vec![];
        // element number: 0 
        ctah_branch_vector.push(&pipe6a); 
        // 1
        ctah_branch_vector.push(&static_mixer_41);
        // 2
        ctah_branch_vector.push(&ctah_vertical);
        // 3
        ctah_branch_vector.push(&ctah_horizontal);
        // 4
        ctah_branch_vector.push(&pipe_8a);
        // 5
        ctah_branch_vector.push(&static_mixer_40);
        // 6
        ctah_branch_vector.push(&pipe_9);
        // 7
        ctah_branch_vector.push(&pipe_10);
        // 8
        ctah_branch_vector.push(&pipe_11);
        // 9
        ctah_branch_vector.push(&pipe_12);
        // 10
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_mass_flowrate = 
            ctah_branch.get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(*pressure_change_value));

        let reference_ctah_pressure_change: f64 = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                test_ctah_mass_flowrate.value,
                temperature_degrees_c,
                pump_pressure_pascals);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_ctah_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }

    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);
    pressure_vec_pa.push(-1.0*10000.0);
    pressure_vec_pa.push(1.0*10000.0);
    pressure_vec_pa.push(-1.0*1000000.0);
    pressure_vec_pa.push(1.0*1000000.0);

    for pressure_change_value in pressure_vec_pa.iter(){

        use fluid_mechanics_rust::prelude::*;
        use crate::CTAHBranch;

        // get a version of ctah i know is working
        let temperature_degrees_c = 21.0;

        let pump_pressure_pascals = *pressure_change_value;


        // get a test version of ctah, the one based on traits

        let ctah_branch_factory = CTAHBranch::new();
        let pipe6a = ctah_branch_factory.get_pipe6a();
        let static_mixer_41 = ctah_branch_factory.get_static_mixer_41();
        let ctah_vertical = ctah_branch_factory.get_ctah_vertical();
        let ctah_horizontal = ctah_branch_factory.get_ctah_horizontal();
        let pipe_8a = ctah_branch_factory.get_pipe_8a();
        let static_mixer_40 = ctah_branch_factory.get_static_mixer_40();
        let pipe_9 = ctah_branch_factory.get_pipe_9();
        let pipe_10 = ctah_branch_factory.get_pipe_10();
        let pipe_11 = ctah_branch_factory.get_pipe_11();
        let pipe_12 = ctah_branch_factory.get_pipe_12();
        let pipe_13 = ctah_branch_factory.get_pipe_13();
        let pipe_14 = ctah_branch_factory.get_pipe_14();
        let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
        let pipe_15 = ctah_branch_factory.get_pipe_15();
        let pipe_16 = ctah_branch_factory.get_pipe_16();
        let branch_17 = ctah_branch_factory.get_branch_17();



        let user_specified_pump_pressure = 
            Pressure::new::<pascal>(pump_pressure_pascals);
        let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
        mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


        let mut ctah_branch_vector :Vec<&dyn FluidComponent> = vec![];
        // element number: 0 
        ctah_branch_vector.push(&pipe6a); 
        // 1
        ctah_branch_vector.push(&static_mixer_41);
        // 2
        ctah_branch_vector.push(&ctah_vertical);
        // 3
        ctah_branch_vector.push(&ctah_horizontal);
        // 4
        ctah_branch_vector.push(&pipe_8a);
        // 5
        ctah_branch_vector.push(&static_mixer_40);
        // 6
        ctah_branch_vector.push(&pipe_9);
        // 7
        ctah_branch_vector.push(&pipe_10);
        // 8
        ctah_branch_vector.push(&pipe_11);
        // 9
        ctah_branch_vector.push(&pipe_12);
        // 10
        ctah_branch_vector.push(&mutable_ctah_pump);
        // 11
        ctah_branch_vector.push(&pipe_13);
        // 12
        ctah_branch_vector.push(&pipe_14);
        //13
        ctah_branch_vector.push(&flowmeter_40_14a);
        //14
        ctah_branch_vector.push(&pipe_15);
        //15
        ctah_branch_vector.push(&pipe_16);
        //16
        ctah_branch_vector.push(&branch_17);


        let mut ctah_branch = CTAHBranch::new();
        ctah_branch.set_fluid_component_vector(ctah_branch_vector);

        let test_ctah_mass_flowrate = 
            ctah_branch.get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(1000.0));

        let reference_ctah_pressure_change: f64 = 
            get_ctah_branch_isothermal_pressure_change_pascals(
                test_ctah_mass_flowrate.value,
                temperature_degrees_c,
                pump_pressure_pascals);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_ctah_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_ctah_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }

}

#[test]
pub fn ctah_branch_manual_test(){


    use fluid_mechanics_rust::prelude::*;
    use crate::CTAHBranch;

    // get a version of ctah i know is working
    let temperature_degrees_c = 21.0;
    let pressure_change_value = 50000_f64;
    let pump_pressure_pascals = 0.0;


    // get a test version of ctah, the one based on traits

    let ctah_branch_factory = CTAHBranch::new();
    let pipe6a = ctah_branch_factory.get_pipe6a();
    let static_mixer_41 = ctah_branch_factory.get_static_mixer_41();
    let ctah_vertical = ctah_branch_factory.get_ctah_vertical();
    let ctah_horizontal = ctah_branch_factory.get_ctah_horizontal();
    let pipe_8a = ctah_branch_factory.get_pipe_8a();
    let static_mixer_40 = ctah_branch_factory.get_static_mixer_40();
    let pipe_9 = ctah_branch_factory.get_pipe_9();
    let pipe_10 = ctah_branch_factory.get_pipe_10();
    let pipe_11 = ctah_branch_factory.get_pipe_11();
    let pipe_12 = ctah_branch_factory.get_pipe_12();
    let pipe_13 = ctah_branch_factory.get_pipe_13();
    let pipe_14 = ctah_branch_factory.get_pipe_14();
    let flowmeter_40_14a = ctah_branch_factory.get_flowmeter_40_14a();
    let pipe_15 = ctah_branch_factory.get_pipe_15();
    let pipe_16 = ctah_branch_factory.get_pipe_16();
    let branch_17 = ctah_branch_factory.get_branch_17();



    let user_specified_pump_pressure = 
        Pressure::new::<pascal>(pump_pressure_pascals);
    let mut mutable_ctah_pump = ctah_branch_factory.get_ctah_pump();
    mutable_ctah_pump.set_internal_pressure_source(user_specified_pump_pressure);


    let mut ctah_branch_vector :Vec<&dyn FluidComponent> = vec![];
    // element number: 0 
    ctah_branch_vector.push(&pipe6a); 
    // 1
    ctah_branch_vector.push(&static_mixer_41);
    // 2
    ctah_branch_vector.push(&ctah_vertical);
    // 3
    ctah_branch_vector.push(&ctah_horizontal);
    // 4
    ctah_branch_vector.push(&pipe_8a);
    // 5
    ctah_branch_vector.push(&static_mixer_40);
    // 6
    ctah_branch_vector.push(&pipe_9);
    // 7
    ctah_branch_vector.push(&pipe_10);
    // 8
    ctah_branch_vector.push(&pipe_11);
    // 9
    ctah_branch_vector.push(&pipe_12);
    // 10
    ctah_branch_vector.push(&mutable_ctah_pump);
    // 11
    ctah_branch_vector.push(&pipe_13);
    // 12
    ctah_branch_vector.push(&pipe_14);
    //13
    ctah_branch_vector.push(&flowmeter_40_14a);
    //14
    ctah_branch_vector.push(&pipe_15);
    //15
    ctah_branch_vector.push(&pipe_16);
    //16
    ctah_branch_vector.push(&branch_17);


    let mut ctah_branch = CTAHBranch::new();
    ctah_branch.set_fluid_component_vector(ctah_branch_vector);

    let test_ctah_mass_flowrate = 
        ctah_branch.get_mass_flowrate_from_pressure_change(
            Pressure::new::<pascal>(pressure_change_value));

    let reference_ctah_pressure_change: f64 = 
        get_ctah_branch_isothermal_pressure_change_pascals(
            test_ctah_mass_flowrate.value,
            temperature_degrees_c,
            pump_pressure_pascals);

    // assert
    //

    if pressure_change_value == 0.0 {

        approx::assert_abs_diff_eq!(
            pressure_change_value,
            reference_ctah_pressure_change,
            epsilon = 10.0);

        return;
    }


    approx::assert_relative_eq!(
        reference_ctah_pressure_change,
        pressure_change_value,
        max_relative = 0.01);

}

#[test]
pub fn assert_dhx_branch_ok(){

    let mut mass_flowrate_kg_per_sec_vec: Vec<f64> = 
        vec![];

    mass_flowrate_kg_per_sec_vec.push(0.0);
    mass_flowrate_kg_per_sec_vec.push(0.2);
    mass_flowrate_kg_per_sec_vec.push(0.4);
    mass_flowrate_kg_per_sec_vec.push(0.7);
    mass_flowrate_kg_per_sec_vec.push(1.0);
    mass_flowrate_kg_per_sec_vec.push(-0.2);
    mass_flowrate_kg_per_sec_vec.push(-0.4);
    mass_flowrate_kg_per_sec_vec.push(-0.7);
    mass_flowrate_kg_per_sec_vec.push(-1.0);

    for mass_rate_kg_per_s in mass_flowrate_kg_per_sec_vec.iter() {

        use fluid_mechanics_rust::prelude::*;
        use crate::DHXBranch;

        // get a version of dhx i know is working
        let temperature_degrees_c = 21.0;


        let reference_dhx_pressure_change = 
            get_dhx_branch_isothermal_pressure_change_pascals(
                *mass_rate_kg_per_s,
                temperature_degrees_c);

        // get a test version of dhx, the one based on traits

        let dhx_branch_factory = DHXBranch::new();

        let pipe26 = dhx_branch_factory.get_pipe26();
        // item 25
        let static_mixer_21 = dhx_branch_factory.get_static_mixer_21();
        let pipe25a = dhx_branch_factory.get_pipe25a();
        // item 24
        let dhx_shell_side_heat_exchanger = dhx_branch_factory.get_dhx_shell_side_heat_exchanger();
        // item 23
        let static_mixer_20 = dhx_branch_factory.get_static_mixer_20();
        let pipe23a = dhx_branch_factory.get_pipe23a();
        let pipe22 = dhx_branch_factory.get_pipe22();
        // item 21a
        let flowmeter20 = dhx_branch_factory.get_flowmeter20();
        let pipe21 = dhx_branch_factory.get_pipe21();
        let pipe20 = dhx_branch_factory.get_pipe20();
        let pipe19 = dhx_branch_factory.get_pipe19();

        let mut dhx_branch_vector :Vec<&dyn FluidComponent> = vec![];

        dhx_branch_vector.push(&pipe26);
        dhx_branch_vector.push(&static_mixer_21);
        dhx_branch_vector.push(&pipe25a);
        dhx_branch_vector.push(&dhx_shell_side_heat_exchanger);
        dhx_branch_vector.push(&static_mixer_20);
        dhx_branch_vector.push(&pipe23a);
        dhx_branch_vector.push(&pipe22);
        dhx_branch_vector.push(&flowmeter20);
        dhx_branch_vector.push(&pipe21);
        dhx_branch_vector.push(&pipe20);
        dhx_branch_vector.push(&pipe19);

        let mut dhx_branch = DHXBranch::new();
        dhx_branch.set_fluid_component_vector(dhx_branch_vector);

        let test_dhx_pressure_change = 
            dhx_branch.get_pressure_change(
                MassRate::new::<kilogram_per_second>(*mass_rate_kg_per_s));


        // assert

        approx::assert_relative_eq!(
            reference_dhx_pressure_change,
            test_dhx_pressure_change.value,
            max_relative = 0.01);
    }

    // now for mass flowrate from pressure tests
    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);
    pressure_vec_pa.push(-1.0*10000.0);
    pressure_vec_pa.push(1.0*10000.0);
    pressure_vec_pa.push(-1.0*1000000.0);
    pressure_vec_pa.push(1.0*1000000.0);

    for pressure_change_value in pressure_vec_pa.iter(){
        use fluid_mechanics_rust::prelude::*;
        use crate::DHXBranch;

        // get a version of dhx i know is working
        let temperature_degrees_c = 21.0;



        // get a test version of dhx, the one based on traits

        let dhx_branch_factory = DHXBranch::new();

        let pipe26 = dhx_branch_factory.get_pipe26();
        // item 25
        let static_mixer_21 = dhx_branch_factory.get_static_mixer_21();
        let pipe25a = dhx_branch_factory.get_pipe25a();
        // item 24
        let dhx_shell_side_heat_exchanger = dhx_branch_factory.get_dhx_shell_side_heat_exchanger();
        // item 23
        let static_mixer_20 = dhx_branch_factory.get_static_mixer_20();
        let pipe23a = dhx_branch_factory.get_pipe23a();
        let pipe22 = dhx_branch_factory.get_pipe22();
        // item 21a
        let flowmeter20 = dhx_branch_factory.get_flowmeter20();
        let pipe21 = dhx_branch_factory.get_pipe21();
        let pipe20 = dhx_branch_factory.get_pipe20();
        let pipe19 = dhx_branch_factory.get_pipe19();

        let mut dhx_branch_vector :Vec<&dyn FluidComponent> = vec![];

        dhx_branch_vector.push(&pipe26);
        dhx_branch_vector.push(&static_mixer_21);
        dhx_branch_vector.push(&pipe25a);
        dhx_branch_vector.push(&dhx_shell_side_heat_exchanger);
        dhx_branch_vector.push(&static_mixer_20);
        dhx_branch_vector.push(&pipe23a);
        dhx_branch_vector.push(&pipe22);
        dhx_branch_vector.push(&flowmeter20);
        dhx_branch_vector.push(&pipe21);
        dhx_branch_vector.push(&pipe20);
        dhx_branch_vector.push(&pipe19);

        let mut dhx_branch = DHXBranch::new();
        dhx_branch.set_fluid_component_vector(dhx_branch_vector);

        let test_dhx_mass_flowrate = 
            dhx_branch.
            get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(*pressure_change_value));

        let reference_dhx_pressure_change: f64 = 
            get_dhx_branch_isothermal_pressure_change_pascals(
                test_dhx_mass_flowrate.value,
                temperature_degrees_c);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_dhx_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_dhx_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }
}

#[test]
pub fn assert_heater_branch_ok(){

    let mut mass_flowrate_kg_per_sec_vec: Vec<f64> = 
        vec![];

    mass_flowrate_kg_per_sec_vec.push(0.0);
    mass_flowrate_kg_per_sec_vec.push(0.2);
    mass_flowrate_kg_per_sec_vec.push(0.4);
    mass_flowrate_kg_per_sec_vec.push(0.7);
    mass_flowrate_kg_per_sec_vec.push(1.0);
    mass_flowrate_kg_per_sec_vec.push(-0.2);
    mass_flowrate_kg_per_sec_vec.push(-0.4);
    mass_flowrate_kg_per_sec_vec.push(-0.7);
    mass_flowrate_kg_per_sec_vec.push(-1.0);

    for mass_rate_kg_per_s in mass_flowrate_kg_per_sec_vec.iter() {

        use fluid_mechanics_rust::prelude::*;
        use crate::HeaterBranch;

        // get a version of heater i know is working
        let temperature_degrees_c = 21.0;


        let reference_heater_pressure_change = 
            get_heater_branch_isothermal_pressure_change_pascals(
                *mass_rate_kg_per_s,
                temperature_degrees_c);

        // get a test version of heater, the one based on traits

        let heater_branch_factory = HeaterBranch::new();

        let branch5 = heater_branch_factory.get_branch5();
        let pipe4 = heater_branch_factory.get_pipe4();
        let pipe3 = heater_branch_factory.get_pipe3();
        let mixer10 = heater_branch_factory.get_mixer10();
        let pipe2a = heater_branch_factory.get_pipe2a();
        let heater_top_head_1a = heater_branch_factory.get_heater_top_head_1a();
        let ciet_heater = heater_branch_factory.get_ciet_heater();
        let heater_bottom_head_1b = heater_branch_factory.get_heater_bottom_head_1b();
        let pipe18 = heater_branch_factory.get_pipe18();

        let mut heater_branch_vector :Vec<&dyn FluidComponent> = vec![];

        heater_branch_vector.push(&branch5);
        heater_branch_vector.push(&pipe4);
        heater_branch_vector.push(&pipe3);
        heater_branch_vector.push(&mixer10);
        heater_branch_vector.push(&pipe2a);
        heater_branch_vector.push(&heater_top_head_1a);
        heater_branch_vector.push(&ciet_heater);
        heater_branch_vector.push(&heater_bottom_head_1b);
        heater_branch_vector.push(&pipe18);

        let mut heater_branch = HeaterBranch::new();
        heater_branch.set_fluid_component_vector(heater_branch_vector);

        let test_heater_pressure_change = 
            heater_branch.get_pressure_change(
                MassRate::new::<kilogram_per_second>(*mass_rate_kg_per_s));


        // assert

        approx::assert_relative_eq!(
            reference_heater_pressure_change,
            test_heater_pressure_change.value,
            max_relative = 0.01);
    }

    // now for mass flowrate from pressure tests
    let mut pressure_vec_pa: Vec<f64> = vec![];

    pressure_vec_pa.push(0.0);
    pressure_vec_pa.push(0.2*1000.0);
    pressure_vec_pa.push(0.4*1000.0);
    pressure_vec_pa.push(0.7*1000.0);
    pressure_vec_pa.push(1.0*1000.0);
    pressure_vec_pa.push(-0.2*1000.0);
    pressure_vec_pa.push(-0.4*1000.0);
    pressure_vec_pa.push(-0.7*1000.0);
    pressure_vec_pa.push(-1.0*1000.0);
    pressure_vec_pa.push(-1.0*10000.0);
    pressure_vec_pa.push(1.0*10000.0);
    pressure_vec_pa.push(-1.0*1000000.0);
    pressure_vec_pa.push(1.0*1000000.0);

    for pressure_change_value in pressure_vec_pa.iter(){
        use fluid_mechanics_rust::prelude::*;
        use crate::HeaterBranch;

        // get a version of heater i know is working
        let temperature_degrees_c = 21.0;



        // get a test version of heater, the one based on traits

        let heater_branch_factory = HeaterBranch::new();

        let branch5 = heater_branch_factory.get_branch5();
        let pipe4 = heater_branch_factory.get_pipe4();
        let pipe3 = heater_branch_factory.get_pipe3();
        let mixer10 = heater_branch_factory.get_mixer10();
        let pipe2a = heater_branch_factory.get_pipe2a();
        let heater_top_head_1a = heater_branch_factory.get_heater_top_head_1a();
        let ciet_heater = heater_branch_factory.get_ciet_heater();
        let heater_bottom_head_1b = heater_branch_factory.get_heater_bottom_head_1b();
        let pipe18 = heater_branch_factory.get_pipe18();

        let mut heater_branch_vector :Vec<&dyn FluidComponent> = vec![];

        heater_branch_vector.push(&branch5);
        heater_branch_vector.push(&pipe4);
        heater_branch_vector.push(&pipe3);
        heater_branch_vector.push(&mixer10);
        heater_branch_vector.push(&pipe2a);
        heater_branch_vector.push(&heater_top_head_1a);
        heater_branch_vector.push(&ciet_heater);
        heater_branch_vector.push(&heater_bottom_head_1b);
        heater_branch_vector.push(&pipe18);

        let mut heater_branch = HeaterBranch::new();
        heater_branch.set_fluid_component_vector(heater_branch_vector);

        let test_heater_mass_flowrate = 
            heater_branch.
            get_mass_flowrate_from_pressure_change(
                Pressure::new::<pascal>(*pressure_change_value));

        let reference_heater_pressure_change: f64 = 
            get_heater_branch_isothermal_pressure_change_pascals(
                test_heater_mass_flowrate.value,
                temperature_degrees_c);

        // assert
        //

        if *pressure_change_value == 0.0 {

            approx::assert_abs_diff_eq!(
                *pressure_change_value,
                reference_heater_pressure_change,
                epsilon = 10.0);

            return;
        }


        approx::assert_relative_eq!(
            reference_heater_pressure_change,
            pressure_change_value,
            max_relative = 0.01);

    }
}


extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;

/// here is reference code from the python maturin rust
/// opcua server
pub fn get_ctah_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64,
    pump_pressure_pascals: f64) -> f64 {

    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);


    // let's get pipe 6a and static mixer 41
    // which is pipe 6 on diagram
    //
    let pipe_6a = factory::Pipe6a::get();
    let static_mixer_41_6 = factory::StaticMixer41::get();

    // let's get the component for ctah
    let ctah_vertical_7a = factory::CTAHVertical::get();
    let ctah_horizontal_7b = factory::CTAHHorizontal::get();

    // let's get the static mixer and pipe 8 and 8a

    let pipe_8a = factory::Pipe8a::get();
    let static_mixer_40_8 = factory::StaticMixer40::get();

    // now let's get pipe 9 to 12

    let pipe_9 = factory::Pipe9::get();
    let pipe_10 = factory::Pipe10::get();
    let pipe_11 = factory::Pipe11::get();
    let pipe_12 = factory::Pipe12::get();

    // and now for the pump
    let ctah_pump = factory::CTAHPump::get(
        pump_pressure_pascals);

    // let's now get pipe 13 and 14
    let pipe_13 = factory::Pipe13::get();
    let pipe_14 = factory::Pipe14::get();

    // let's get flowmeter 14a
    let flowmeter_40_14a = factory::Flowmeter40::get();

    // let's get pipe 15 and 16
    let pipe_15 = factory::Pipe15::get();
    let pipe_16 = factory::Pipe16::get();

    // let's now get branch 17
    let branch_17 = factory::Branch17::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given

    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // pipe 6a, static mixer 6,
    // ctah 7a, 7b and static mixer 8 and pipe 8a
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_6a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &static_mixer_41_6,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_vertical_7a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_horizontal_7b,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_8a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &static_mixer_40_8,
            mass_flowrate,
            fluid_temp);

    // pipe 9 tp 12

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_9,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_10,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_11,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_12,
            mass_flowrate,
            fluid_temp);

    // ctah pump
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &ctah_pump,
            mass_flowrate,
            fluid_temp);

    // pipe 13 to 17

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_13,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_14,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &flowmeter_40_14a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_15,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_16,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &branch_17,
            mass_flowrate,
            fluid_temp);

    return pressure_change_total.get::<pascal>();
}

fn get_heater_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    //import necessary things...
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);

    // let's get branch 5 and pipe 4
    //
    let branch_5 = factory::Branch5::get();
    let pipe_4 = factory::Pipe4::get();

    // lets get pipe 3 and static mixer 2 and pipe 2a
    let pipe_3 = factory::Pipe3::get();
    let mx10_2 = factory::StaticMixer10::get();
    let pipe_2a = factory::Pipe2a::get();

    // let's get the heater components 1a, 1 and 1b
    let heater_top_head_1a =
        factory::HeaterTopHead1a::get();
    let heater_version_1_1 =
        factory::CietHeaterVersion1::get();
    let heater_bottom_head_label_1b =
        factory::HeaterBottomHead1b::get();

    // now we'll get pipe 18

    let pipe_18 = factory::Pipe18::get();

    // now that we've gotten our items, we can
    // then sum up the pressure change contributions
    // given

    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // branch5 and pipe4
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &branch_5,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_4,
            mass_flowrate,
            fluid_temp);

    // pipe 3 and static mixer 2 and pipe 2a
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_3,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx10_2,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_2a,
            mass_flowrate,
            fluid_temp);

    // heater
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_top_head_1a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_version_1_1,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &heater_bottom_head_label_1b,
            mass_flowrate,
            fluid_temp);

    //pipe 18
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_18,
            mass_flowrate,
            fluid_temp);

    // convert the object to f64 and return
    return pressure_change_total.get::<pascal>();
}


/// dhx branch from ciet isothermal digital twin v1
fn get_dhx_branch_isothermal_pressure_change_pascals(
    mass_rate_kg_per_s: f64,
    temperature_degrees_c: f64) -> f64 {
    use fluid_mechanics_rust::therminol_component::factory;
    use fluid_mechanics_rust::therminol_component::CalcPressureChange;

    // fluid temp and pressure
    let fluid_temp = ThermodynamicTemperature::new::<
        degree_celsius>(temperature_degrees_c);
    let mass_flowrate = MassRate::new::<
        kilogram_per_second>(mass_rate_kg_per_s);

    // pipe 26, static mixer pipe 25a and static mixer 25
    let pipe_26 = factory::Pipe26::get();
    let mx21_25 = factory::StaticMixer21::get();
    let pipe_25a = factory::Pipe25a::get();


    // DHX heat exchanger
    let dhx_shell_side_24 = factory::DHXShellSideHeatExchanger::get();


    // static mixer pipe 23a, static mixer 23, and pipe 22
    let mx20_23 = factory::StaticMixer20::get();
    let pipe_23a = factory::Pipe23a::get();
    let pipe_22 = factory::Pipe22::get();

    // flowmeter 21a (FM-20)
    let flowmeter_20_21a = factory::Flowmeter20::get();

    // pipe 21 to 19
    let pipe_21 = factory::Pipe21::get();
    let pipe_20 = factory::Pipe20::get();
    let pipe_19 = factory::Pipe19::get();


    let mut pressure_change_total =
        Pressure::new::<pascal>(0.0);

    // pipe 26, static mixer pipe 25a and static mixer 25
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_26,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx21_25,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_25a,
            mass_flowrate,
            fluid_temp);

    // DHX heat exchanger
    //
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &dhx_shell_side_24,
            mass_flowrate,
            fluid_temp);

    // static mixer pipe 23a, static mixer 23, and pipe 22
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &mx20_23,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_23a,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_22,
            mass_flowrate,
            fluid_temp);

    // flowmeter 21a (FM-20)

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &flowmeter_20_21a,
            mass_flowrate,
            fluid_temp);

    // pipe 21 to 19

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_21,
            mass_flowrate,
            fluid_temp);

    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_20,
            mass_flowrate,
            fluid_temp);
    pressure_change_total = pressure_change_total +
        CalcPressureChange::from_mass_rate(
            &pipe_19,
            mass_flowrate,
            fluid_temp);


    // convert to f64 and return
    return pressure_change_total.get::<pascal>();
}
