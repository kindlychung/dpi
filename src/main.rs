use std::env;


fn main() {
    let doc = r#"
dpi

Usage: 
    dpi  <inch> <dimension> <dimension>
    dpi  <dimension> <inch> <dimension>
    dpi  <dimension> <inch> <dimension>
    dpi --help
    dpi -h
    dpi
"#;

    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {
//            let mut args: Vec<f64> = args.split_off(1)
//                                         .into_iter()
//                                         .map(|s| s.parse().unwrap())
//                                         .collect();
            let mut args: Vec<f64> = args.into_iter().skip(1)
                                         .map(|s| s.parse().unwrap())
                                         .collect();
            args.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let dpi = (args[1].powi(2) + args[2].powi(2)).sqrt() / args[0];
            println!("DPI is {} for a {}x{} disaply.",
                     dpi.round() as i64,
                     args[1].round() as i64,
                     args[2].round() as i64);
        }
        _ => {
            println!("{}", doc);
        }
    }
}
