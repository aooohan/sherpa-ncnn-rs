use std::env;
use std::error::Error;
use std::str::FromStr;

use ncnn::{Mat, Net};

fn parse_dim(args: &[String], idx: usize, default: i32) -> Result<i32, Box<dyn Error>> {
    if args.len() > idx {
        Ok(i32::from_str(&args[idx])?)
    } else {
        Ok(default)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: cargo run -p ncnn --example simple -- <model.param> <model.bin> <input_blob> <output_blob> [w h c]",
        );
        eprintln!("Example: cargo run -p ncnn --example simple -- squeezenet.param squeezenet.bin data prob 227 227 3");
        return Ok(());
    }

    let param_path = &args[0];
    let model_path = &args[1];
    let input_blob = &args[2];
    let output_blob = &args[3];

    let w = parse_dim(&args, 4, 224)?;
    let h = parse_dim(&args, 5, 224)?;
    let c = parse_dim(&args, 6, 3)?;

    let mut net = Net::new();
    net.load_param(param_path)
        .map_err(|code| format!("load_param failed with code {code}"))?;
    net.load_model(model_path)
        .map_err(|code| format!("load_model failed with code {code}"))?;

    let mut ex = net.create_extractor();

    let mut input = Mat::new_3d(w, h, c, None);
    input.fill(0.0);

    ex.input(input_blob, &input)
        .map_err(|code| format!("input failed with code {code}"))?;

    let mut output = Mat::new();
    ex.extract(output_blob, &mut output)
        .map_err(|code| format!("extract failed with code {code}"))?;

    println!(
        "Output dims: {} x {} x {} (elemsize={} bytes, elempack={})",
        output.w(),
        output.h(),
        output.c(),
        output.elemsize(),
        output.elempack()
    );

    Ok(())
}
