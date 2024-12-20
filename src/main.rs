use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Dry run? Don't actually communicate to the usb device.
    #[clap(long, short, action)]
    dry_run: bool,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PumpArg {
    Quiet,
    #[default]
    Balanced,
    Extreme,
}
impl Into<h100i_tool::PumpMode> for PumpArg {
    fn into(self) -> h100i_tool::PumpMode {
        match self {
            PumpArg::Quiet => h100i_tool::PumpMode::Quiet,
            PumpArg::Balanced => h100i_tool::PumpMode::Balanced,
            PumpArg::Extreme => h100i_tool::PumpMode::Extreme,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FanArg {
    Quiet,
    #[default]
    Balanced,
    Extreme,
}
impl Into<h100i_tool::CoolingCurve> for FanArg {
    fn into(self) -> h100i_tool::CoolingCurve {
        match self {
            FanArg::Quiet => h100i_tool::CoolingCurve::quiet(),
            FanArg::Balanced => h100i_tool::CoolingCurve::balanced(),
            FanArg::Extreme => h100i_tool::CoolingCurve::extreme(),
        }
    }
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

    /// Set the quiet profile.
    Quiet,
    /// Set the balanced profile.
    Balanced,
    /// Set the extreme profile.
    Extreme,
    /// Manually set the configuration
    Config {
        #[arg(short, long)]
        pump_level: PumpArg,
        // #[arg(short, long, default_value_t = FanArg::Balanced)]
        #[arg(short, long)]
        fans: FanArg,
    },
}

fn main() -> Result<(), h100i_tool::H100iError> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Develop => h100i_tool::main(),
        Commands::Balanced => {
            let mut d = h100i_tool::H100i::new(cli.dry_run)?;
            let config = h100i_tool::Config::balanced();
            d.set_config(&config)?;
            return Ok(());
        }
        Commands::Extreme => {
            let mut d = h100i_tool::H100i::new(cli.dry_run)?;
            let config = h100i_tool::Config::extreme();
            d.set_config(&config)?;
            return Ok(());
        }
        Commands::Quiet => {
            let mut d = h100i_tool::H100i::new(cli.dry_run)?;
            let config = h100i_tool::Config::quiet();
            d.set_config(&config)?;
            return Ok(());
        }
        Commands::Config { pump_level, fans } => {
            let mut d = h100i_tool::H100i::new(cli.dry_run)?;
            let mut config = h100i_tool::Config::balanced();
            config.pump = (*pump_level).into();
            config.fans[0] = (*fans).into();
            config.fans[1] = (*fans).into();
            d.set_config(&config)?;
            return Ok(());
        }
        Commands::Sensors { interval } => {
            let mut d = h100i_tool::H100i::new(cli.dry_run)?;
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
