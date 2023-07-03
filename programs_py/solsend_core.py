# solsend_core
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('4FF8j7wY3uxZPbKKRA6RYzmpfkPnbvxGvWxNaBLeino5')

# A channel is analogous to a stream. A client may subscribe to the channel and receive 
# notifications from the said channel. 
# The creator of the channel may publish notifications, while other subscribers cannot.
class Channel(Account):
    channel_creator: Pubkey
    channel_id: str
    channel_readable_name: str
    channel_description: str
    channel_icon: str
    channel_is_hidden: bool

# A notification is well, a notification. It simply contains a title and a text, along with the
# channel it is a part of.
class Notification(Account):
    notif_sender: Pubkey
    notif_title: str
    notif_body: str
    notif_channel: Pubkey

# TODO: `channel_id` must be unique and it must be part of the seed. Therefore, making it unique
# for each channel created by each `creator`.
@instruction
def init_channel(creator: Signer, readable_name: str, description: str, icon: str, channel: Empty[Channel]):
    channel = channel.init(
        payer = creator,
        seeds = ['Channel', creator, readable_name]
    )
    channel.channel_creator = creator.key()
    channel.channel_readable_name = readable_name
    channel.channel_description = description
    channel.channel_icon = icon
    # By default, a channel is not hidden
    channel.channel_is_hidden = False
    # A channel id is just the hash of the channel's readable name
    channel.channel_id = "999"

@instruction
def publish_notification(sender: Signer, title: str, body: str, channel: Channel, notification: Empty[Notification]):
    assert sender.key() == channel.channel_creator, "Unauthorized: Client could not publish notification for channel"

    notif = notification.init(
        payer = sender,
        seeds = ['Notification', sender, title]
    )
    notif.notif_sender = sender.key()
    notif.notif_title = title
    notif.notif_body = body
    notif.notif_channel = channel.key()