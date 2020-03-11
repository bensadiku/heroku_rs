//! Access the Teams portion of the Heroku API
imports!();
use crate::client::PutQueryBuilder;

new_type!(
    Teams
    Team
    TeamMember
    TeamInvitation
);

from!(
    @PutQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Team
        -> TeamMember = "members"  
        -> TeamInvitation = "invitations"    
);

impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Team
        |=> team_member ->  TeamMember
        |=> team_invitation ->  TeamInvitation
        |
);

exec!(Teams);
exec!(Team);
exec!(TeamMember);
exec!(TeamInvitation);