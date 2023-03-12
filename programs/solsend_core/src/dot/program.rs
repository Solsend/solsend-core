#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Channel {
    pub channel_creator: Pubkey,
    pub channel_id: String,
    pub channel_readable_name: String,
    pub channel_description: String,
    pub channel_icon: String,
    pub channel_is_hidden: bool,
}

impl<'info, 'entrypoint> Channel {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedChannel<'info, 'entrypoint>> {
        let channel_creator = account.channel_creator.clone();
        let channel_id = account.channel_id.clone();
        let channel_readable_name = account.channel_readable_name.clone();
        let channel_description = account.channel_description.clone();
        let channel_icon = account.channel_icon.clone();
        let channel_is_hidden = account.channel_is_hidden.clone();

        Mutable::new(LoadedChannel {
            __account__: account,
            __programs__: programs_map,
            channel_creator,
            channel_id,
            channel_readable_name,
            channel_description,
            channel_icon,
            channel_is_hidden,
        })
    }

    pub fn store(loaded: Mutable<LoadedChannel>) {
        let mut loaded = loaded.borrow_mut();
        let channel_creator = loaded.channel_creator.clone();

        loaded.__account__.channel_creator = channel_creator;

        let channel_id = loaded.channel_id.clone();

        loaded.__account__.channel_id = channel_id;

        let channel_readable_name = loaded.channel_readable_name.clone();

        loaded.__account__.channel_readable_name = channel_readable_name;

        let channel_description = loaded.channel_description.clone();

        loaded.__account__.channel_description = channel_description;

        let channel_icon = loaded.channel_icon.clone();

        loaded.__account__.channel_icon = channel_icon;

        let channel_is_hidden = loaded.channel_is_hidden.clone();

        loaded.__account__.channel_is_hidden = channel_is_hidden;
    }
}

#[derive(Debug)]
pub struct LoadedChannel<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Channel>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub channel_creator: Pubkey,
    pub channel_id: String,
    pub channel_readable_name: String,
    pub channel_description: String,
    pub channel_icon: String,
    pub channel_is_hidden: bool,
}

#[account]
#[derive(Debug)]
pub struct Notification {
    pub notif_sender: Pubkey,
    pub notif_title: String,
    pub notif_body: String,
    pub notif_channel: Pubkey,
}

impl<'info, 'entrypoint> Notification {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedNotification<'info, 'entrypoint>> {
        let notif_sender = account.notif_sender.clone();
        let notif_title = account.notif_title.clone();
        let notif_body = account.notif_body.clone();
        let notif_channel = account.notif_channel.clone();

        Mutable::new(LoadedNotification {
            __account__: account,
            __programs__: programs_map,
            notif_sender,
            notif_title,
            notif_body,
            notif_channel,
        })
    }

    pub fn store(loaded: Mutable<LoadedNotification>) {
        let mut loaded = loaded.borrow_mut();
        let notif_sender = loaded.notif_sender.clone();

        loaded.__account__.notif_sender = notif_sender;

        let notif_title = loaded.notif_title.clone();

        loaded.__account__.notif_title = notif_title;

        let notif_body = loaded.notif_body.clone();

        loaded.__account__.notif_body = notif_body;

        let notif_channel = loaded.notif_channel.clone();

        loaded.__account__.notif_channel = notif_channel;
    }
}

#[derive(Debug)]
pub struct LoadedNotification<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Notification>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub notif_sender: Pubkey,
    pub notif_title: String,
    pub notif_body: String,
    pub notif_channel: Pubkey,
}

pub fn init_channel_handler<'info>(
    mut creator: SeahorseSigner<'info, '_>,
    mut readable_name: String,
    mut description: String,
    mut icon: String,
    mut channel: Empty<Mutable<LoadedChannel<'info, '_>>>,
) -> () {
    let mut channel = channel.account.clone();

    assign!(channel.borrow_mut().channel_creator, creator.key());

    assign!(channel.borrow_mut().channel_readable_name, readable_name);

    assign!(channel.borrow_mut().channel_description, description);

    assign!(channel.borrow_mut().channel_icon, icon);

    assign!(channel.borrow_mut().channel_is_hidden, false);

    assign!(channel.borrow_mut().channel_id, "999".to_string());
}

pub fn publish_notification_handler<'info>(
    mut sender: SeahorseSigner<'info, '_>,
    mut title: String,
    mut body: String,
    mut channel: Mutable<LoadedChannel<'info, '_>>,
    mut notification: Empty<Mutable<LoadedNotification<'info, '_>>>,
) -> () {
    if !(sender.key() == channel.borrow().channel_creator) {
        panic!("Unauthorized: Client could not publish notification for channel");
    }

    let mut notif = notification.account.clone();

    assign!(notif.borrow_mut().notif_sender, sender.key());

    assign!(notif.borrow_mut().notif_title, title);

    assign!(notif.borrow_mut().notif_body, body);

    assign!(
        notif.borrow_mut().notif_channel,
        channel.borrow().__account__.key()
    );
}
