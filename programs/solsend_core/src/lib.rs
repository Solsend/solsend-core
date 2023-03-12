#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("88Vv88x5T9HvAxu8b1ya9KazRzcNkqkdcAU5VAH9fjkG");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod solsend_core {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (readable_name : String , description : String , icon : String)]
    pub struct InitChannel<'info> {
        #[account(mut)]
        pub creator: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Channel > () + 8 , payer = creator , seeds = ["Channel" . as_bytes () . as_ref () , creator . key () . as_ref () , readable_name . as_bytes () . as_ref ()] , bump)]
        pub channel: Box<Account<'info, dot::program::Channel>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn init_channel(
        ctx: Context<InitChannel>,
        readable_name: String,
        description: String,
        icon: String,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let creator = SeahorseSigner {
            account: &ctx.accounts.creator,
            programs: &programs_map,
        };

        let channel = Empty {
            account: dot::program::Channel::load(&mut ctx.accounts.channel, &programs_map),
            bump: ctx.bumps.get("channel").map(|bump| *bump),
        };

        init_channel_handler(
            creator.clone(),
            readable_name,
            description,
            icon,
            channel.clone(),
        );

        dot::program::Channel::store(channel.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (title : String , body : String)]
    pub struct PublishNotification<'info> {
        #[account(mut)]
        pub sender: Signer<'info>,
        #[account(mut)]
        pub channel: Box<Account<'info, dot::program::Channel>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Notification > () + 8 , payer = sender , seeds = ["Notification" . as_bytes () . as_ref () , sender . key () . as_ref () , title . as_bytes () . as_ref ()] , bump)]
        pub notification: Box<Account<'info, dot::program::Notification>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn publish_notification(
        ctx: Context<PublishNotification>,
        title: String,
        body: String,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let sender = SeahorseSigner {
            account: &ctx.accounts.sender,
            programs: &programs_map,
        };

        let channel = dot::program::Channel::load(&mut ctx.accounts.channel, &programs_map);
        let notification = Empty {
            account: dot::program::Notification::load(
                &mut ctx.accounts.notification,
                &programs_map,
            ),
            bump: ctx.bumps.get("notification").map(|bump| *bump),
        };

        publish_notification_handler(
            sender.clone(),
            title,
            body,
            channel.clone(),
            notification.clone(),
        );

        dot::program::Channel::store(channel);

        dot::program::Notification::store(notification.account);

        return Ok(());
    }
}