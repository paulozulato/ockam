use clap::Args;
use colorful::Colorful;
use miette::IntoDiagnostic;
use tokio::sync::Mutex;
use tokio::try_join;
use tracing::debug;

use ockam::Context;
use ockam_api::cloud::share::{CreateInvitation, RoleInShare, SentInvitation, ShareScope};

use crate::node::util::delete_embedded_node;
use crate::util::api::{self, CloudOpts};
use crate::util::{node_rpc, Rpc};
use crate::{docs, fmt_ok, CommandGlobalOpts};

const PREVIEW_TAG: &str = include_str!("../static/preview_tag.txt");

#[derive(Clone, Debug, Args)]
#[command(
    before_help = docs::before_help(PREVIEW_TAG),
)]
pub struct CreateCommand {
    #[command(flatten)]
    pub cloud_opts: CloudOpts,
    #[arg(value_parser = clap::value_parser!(ShareScope))]
    pub scope: ShareScope,
    pub target_id: String,
    pub recipient_email: String,
    #[arg(default_value_t = RoleInShare::Admin, long, short = 'R', value_parser = clap::value_parser!(RoleInShare))]
    pub grant_role: RoleInShare,
    #[arg(long, short = 'x')]
    pub expires_at: Option<String>,
}

impl CreateCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        node_rpc(rpc, (options, self));
    }
}

impl From<CreateCommand> for CreateInvitation {
    fn from(val: CreateCommand) -> Self {
        let CreateCommand {
            expires_at,
            grant_role,
            recipient_email,
            scope,
            target_id,
            ..
        } = val;
        Self {
            expires_at,
            grant_role,
            recipient_email,
            remaining_uses: None,
            scope,
            target_id,
        }
    }
}

async fn rpc(
    mut ctx: Context,
    (opts, cmd): (CommandGlobalOpts, CreateCommand),
) -> miette::Result<()> {
    run_impl(&mut ctx, opts, cmd).await
}

async fn run_impl(
    ctx: &mut Context,
    opts: CommandGlobalOpts,
    cmd: CreateCommand,
) -> miette::Result<()> {
    let is_finished: Mutex<bool> = Mutex::new(false);
    let mut rpc = Rpc::embedded(ctx, &opts).await?;

    let get_sent_invitation = async {
        let req = cmd.into();
        debug!(?req);
        let invitation: SentInvitation = rpc
            .ask(api::share::create(req, &CloudOpts::route()))
            .await?;
        *is_finished.lock().await = true;
        Ok(invitation)
    };

    let output_messages = vec![format!("Creating invitation...\n",)];

    let progress_output = opts
        .terminal
        .progress_output(&output_messages, &is_finished);

    let (sent, _) = try_join!(get_sent_invitation, progress_output)?;

    debug!(?sent);

    delete_embedded_node(&opts, rpc.node_name()).await;

    let plain = fmt_ok!(
        "Invite {} to {} {} created, expiring at {}. {} will be notified via email.",
        sent.id,
        sent.scope,
        sent.target_id,
        sent.expires_at,
        sent.recipient_email
    );
    let json = serde_json::to_string_pretty(&sent).into_diagnostic()?;
    opts.terminal
        .stdout()
        .plain(plain)
        .json(json)
        .write_line()?;

    Ok(())
}
