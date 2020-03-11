 //! Access the Apps portion of the Heroku API
imports!();
use crate::client::DeleteQueryBuilder;

// Declaration of types representing the various items under apps
new_type!(
    Teams
    Team
    TeamInvitation
    TeamInvitationId
    TeamMembers
    TeamMemberEmail
    TeamMemberId
);

// From implementations for conversion
from!(
    @DeleteQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Team
        -> TeamInvitation = "invitations"
        -> TeamMembers = "members"
    @TeamInvitation
        => TeamInvitationId
    @TeamMembers
        => TeamMemberEmail
        => TeamMemberId      
);

// impls of each type
impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Team
        |=> team_members -> TeamMembers
        |=> team_invitations -> TeamInvitation
        |
    @TeamInvitation
        |
        |=> invitation_id -> TeamInvitationId = id
    @TeamMembers
        |
        |=> member_email -> TeamMemberEmail = email
        |=> member_id -> TeamMemberId = id
);

exec!(Teams);
exec!(Team);
exec!(TeamInvitation);
exec!(TeamInvitationId);
exec!(TeamMembers);
exec!(TeamMemberEmail);
exec!(TeamMemberId);