#![allow(dead_code)]

use std::fs;
use std::path;

fn main() {
    println!("Hello, world!");
}

fn p1_load(path: &path::Path) -> anyhow::Result<Vec<u32>> {
    let r = fs::read_to_string(path)?
        .lines()
        .map(|f| f.parse())
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(r)
}

fn p1_1(path: &path::Path) -> anyhow::Result<usize> {
    let vals = p1_load(path)?;
    Ok(vals
        .windows(2)
        .filter_map(|f| (f[1] > f[0]).then(|| ()))
        .count())
}

fn p1_2(path: &path::Path) -> anyhow::Result<usize> {
    let vals = p1_load(path)?;

    let vals = vals
        .windows(3)
        .map(|f| f[0] + f[1] + f[2])
        .collect::<Vec<_>>();

    Ok(vals
        .windows(2)
        .filter_map(|f| (f[1] > f[0]).then(|| ()))
        .count())
}

#[test]
fn p1test() -> anyhow::Result<()> {
    let test_input = "p1test.txt";
    let input = "p1.txt";
    let p1 = p1_1;
    let p2 = p1_2;

    assert_eq!(p1(&path::PathBuf::from(test_input))?, 7);
    assert_eq!(p2(&path::PathBuf::from(test_input))?, 5);
    println!("p1_1: {}", p1(&path::PathBuf::from(input))?);
    println!("p1_2: {}", p2(&path::PathBuf::from(input))?);
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn p2_load(path: &path::Path) -> anyhow::Result<Vec<(String, u32)>> {
    let r = fs::read_to_string(path)?
        .lines()
        .map(|f| -> anyhow::Result<_> {
            let mut i = f.split_whitespace();
            Ok((i.next().unwrap().to_string(), i.next().unwrap().parse()?))
        })
        .collect::<Result<Vec<(String, u32)>, _>>()?;
    Ok(r)
}

fn p2_1(path: &path::Path) -> anyhow::Result<u32> {
    let vals = p2_load(path)?;

    let count_direction = |direction| {
        vals.iter()
            .filter(|(s, _)| s == direction)
            .map(|(_, v)| v)
            .sum()
    };

    let forward: u32 = count_direction("forward");
    let up: u32 = count_direction("up");
    let down: u32 = count_direction("down");

    Ok((down - up) * forward)
}

fn p2_2(path: &path::Path) -> anyhow::Result<u32> {
    let vals = p2_load(path)?;

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (command, value) in vals {
        match command.as_ref() {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("unhandled case"),
        }
    }

    Ok(horizontal * depth)
}

#[test]
fn p2test() -> anyhow::Result<()> {
    let test_input = "p2test.txt";
    let input = "p2.txt";
    let p1 = p2_1;
    let p2 = p2_2;

    assert_eq!(p1(&path::PathBuf::from(test_input))?, 150);
    assert_eq!(p2(&path::PathBuf::from(test_input))?, 900);
    println!("p2_1: {}", p1(&path::PathBuf::from(input))?);
    println!("p2_2: {}", p2(&path::PathBuf::from(input))?);
    Ok(())
}
