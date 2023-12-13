#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(iter_intersperse)]
#![allow(dead_code)]

#[macro_use] extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod grid;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;

aoc_lib! {year=2023}