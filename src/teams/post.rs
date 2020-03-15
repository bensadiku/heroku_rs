 //!Create new /teams implementation.
imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Teams
    Team
    TeamMember
    WhitelistedAddonServices
    IdentityProviders
    Apps
    App
    AppCollaborators
    TeamInvitation
    TeamInvitationToken
    TeamInvitationActionAccept
);

from!(
    @PostQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Teams
        -> Apps = "apps"
        -> TeamInvitation = "invitations"
    @Team
        -> TeamMember = "members"    
        -> WhitelistedAddonServices = "whitelisted-addon-services"    
        -> IdentityProviders = "identity-providers"   
    @Apps
        => App
    @App
        -> AppCollaborators = "collaborators"
    @TeamInvitation
        => TeamInvitationToken
    @TeamInvitationToken
        -> TeamInvitationActionAccept = "accept"
     
);

impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Teams
        |=> team_apps -> Apps
        |=> team_invitations -> TeamInvitation
        |
    @Apps
        |
        |=> app_name -> App = app_name_str
        |=> app_id -> App = app_id_str
    @App
        |=> app_collaborators ->  AppCollaborators
        |
    @Team
        |=> team_member ->  TeamMember
        |=> team_whitelist_addon_service ->  WhitelistedAddonServices
        |=> team_identity_provider ->  IdentityProviders
        |
    @TeamInvitation
        |
        |=> invitation_token -> TeamInvitationToken = token
    @TeamInvitationToken
        |=> invitation_accept -> TeamInvitationActionAccept
        |
    @TeamMember
        |
);

exec!(Teams);
exec!(Team);
exec!(TeamMember);
exec!(WhitelistedAddonServices);
exec!(IdentityProviders);
exec!(Apps);
exec!(App);
exec!(AppCollaborators);
exec!(TeamInvitation);
exec!(TeamInvitationToken);
exec!(TeamInvitationActionAccept);
