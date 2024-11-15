use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the development main function.
    Develop,
    /// Pretty print sensors
    Sensors {
        /// Interval by which to poll
        #[arg(short, long, default_value_t = 0.1)]
        interval: f64,
    },

    /// Set the balanced profile.
    Balanced,
    // Extreme,
}

fn main() -> Result<(), h100i_tool::H100iError> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Develop => h100i_tool::main(),
        Commands::Balanced => {
            let mut d = h100i_tool::H100i::new()?;
            let mut config = h100i_tool::Config::balanced();
            // config.fans[0] = h100i_tool::CoolingCurve::extreme();
            // config.fans[1] = h100i_tool::CoolingCurve::extreme();
            d.set_config(&config)?;
            return Ok(());
        }
        Commands::Sensors { interval } => {
            let mut d = h100i_tool::H100i::new()?;
            loop {
                let status = d.get_status()?;
                // println!("Status: {status:#?}");
                println!();
                println!("uptime_ms: {}", status.uptime_ms);
                println!("msg_counter: {}", status.msg_counter);
                println!("temp1_c: {}", status.temperature_1.0);
                println!("temp2_c: {}", status.temperature_2.0);
                println!("fan0_rpm: {}", status.fans[0].speed.0);
                println!("fan0_duty: {}", status.fans[0].duty_cycle.0);
                println!("fan1_rpm: {}", status.fans[1].speed.0);
                println!("fan1_duty: {}", status.fans[1].duty_cycle.0);
                println!("pump_rpm: {}", status.fans[2].speed.0);
                println!("pump_duty: {}", status.fans[2].duty_cycle.0);
                std::thread::sleep(std::time::Duration::from_secs_f64(*interval));
            }
        }
    }
}
