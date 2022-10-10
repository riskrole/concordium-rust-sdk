//! Test the `GetPassiveDelegationInfo` endpoint.
use anyhow::Context;
use clap::AppSettings;
use structopt::StructOpt;

use concordium_rust_sdk::{endpoints::Endpoint, v2};

#[derive(StructOpt)]
struct App {
    #[structopt(
        long = "node",
        help = "GRPC interface of the node.",
        default_value = "http://localhost:10001"
    )]
    endpoint: Endpoint,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let app = {
        let app = App::clap().global_setting(AppSettings::ColoredHelp);
        let matches = app.get_matches();
        App::from_clap(&matches)
    };

    let mut client = v2::Client::new(app.endpoint)
        .await
        .context("Cannot connect.")?;

    {
        let ai = client
            .get_passive_delegation_info(&v2::BlockIdentifier::Best)
            .await?;
        println!("Best block {:#?}", ai);
    }

    {
        let ai = client
            .get_passive_delegation_info(&v2::BlockIdentifier::LastFinal)
            .await?;
        println!("Last finalized {:#?}", ai);
    }

    Ok(())
}
