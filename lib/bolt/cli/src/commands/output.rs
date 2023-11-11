use std::collections::HashSet;

use anyhow::*;
use bolt_core::context::{ProjectContext, RunContext};
use clap::Parser;

/// Used to extract data from the Bolt configs. This gets called primarily in
/// shell scripts.
///
/// Bolt is intended to be the single source of truth, so this lets other tools
/// extract relevant information.
#[derive(Parser, Debug)]
pub enum SubCommand {
	Namespace,
	ProjectRoot,
	ServiceName {
		#[clap(index = 1, action = clap::ArgAction::Append)]
		service_names: Vec<String>,
	},
	ServicePath {
		#[clap(index = 1, action = clap::ArgAction::Append)]
		service_names: Vec<String>,
	},
	ServiceDatabases {
		#[clap(index = 1, action = clap::ArgAction::Append)]
		service_names: Vec<String>,
		#[clap(long)]
		test: bool,
	},
}

impl SubCommand {
	pub async fn execute(self, ctx: ProjectContext) -> Result<()> {
		match self {
			Self::Namespace => {
				println!("{}", ctx.ns_id());
			}
			Self::ProjectRoot => {
				print!("{}", ctx.path().display());
			}
			Self::ServiceName { service_names } => {
				for svc_ctx in ctx.services_with_patterns(&service_names).await {
					println!("{}", svc_ctx.name());
				}
			}
			Self::ServicePath { service_names } => {
				for svc_ctx in ctx.services_with_patterns(&service_names).await {
					println!("{}", svc_ctx.path().display());
				}
			}
			Self::ServiceDatabases {
				service_names,
				test,
			} => {
				let run_context = if test {
					RunContext::Test {
						test_id: String::new(),
					}
				} else {
					RunContext::Service {}
				};

				let mut databases = HashSet::new();

				// TODO: Use a stream iter instead
				for svc_ctx in ctx.services_with_patterns(&service_names).await {
					let dbs = svc_ctx.database_dependencies(&run_context).await;

					databases.extend(dbs.keys().cloned());
				}

				let mut list = databases.into_iter().collect::<Vec<_>>();
				list.sort();

				if list.is_empty() {
					eprintln!("no databases");
				} else {
					println!("{}", list.join("\n"));
				}
			}
		}
		Ok(())
	}
}
