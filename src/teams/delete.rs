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
    TeamIdentityProviders
    TeamIdentityProviderId
    TeamApps
    TeamApp
    TeamAppCollaborators
    TeamAppCollaboratorEmail
    WhitelistedAddonServices
    WhitelistedAddonServiceId
    WhitelistedAddonServiceName
);

// From implementations for conversion
from!(
    @DeleteQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Teams
        -> TeamApps = "apps"
    @TeamApps
        => TeamApp
    @TeamApp
        -> TeamAppCollaborators = "collaborators"
    @TeamAppCollaborators
        => TeamAppCollaboratorEmail
    @Team
        -> TeamInvitation = "invitations"
        -> TeamMembers = "members"
        -> TeamIdentityProviders = "identity-providers"
        -> WhitelistedAddonServices = "whitelisted-addon-services"
    @TeamInvitation
        => TeamInvitationId
    @TeamMembers
        => TeamMemberEmail
        => TeamMemberId      
    @TeamIdentityProviders
        => TeamIdentityProviderId
    @WhitelistedAddonServices
        => WhitelistedAddonServiceId
        => WhitelistedAddonServiceName
);

// impls of each type
impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Teams
        |=> team_apps -> TeamApps
        |
    @TeamApps
        |
        |=> team_app_name -> TeamApp = team_app_name_str
    @TeamApp
        |=> app_collaborators -> TeamAppCollaborators
        |
    @TeamAppCollaborators
        |
        |=> collaborator_email -> TeamAppCollaboratorEmail = collaborator_email_str
    @Team
        |=> team_members -> TeamMembers
        |=> team_invitations -> TeamInvitation
        |=> team_identity_providers -> TeamIdentityProviders
        |=> team_whitelisted_addon_services -> WhitelistedAddonServices
        |
    @TeamInvitation
        |
        |=> invitation_id -> TeamInvitationId = id
    @TeamMembers
        |
        |=> member_email -> TeamMemberEmail = email
        |=> member_id -> TeamMemberId = id
    @TeamIdentityProviders
        |
        |=> identity_provider_id -> TeamIdentityProviderId = id
    @WhitelistedAddonServices
        |
        |=> whitelist_addon_service_id -> WhitelistedAddonServiceId =  id
        |=> whitelist_addon_service_name -> WhitelistedAddonServiceName =  name
);

exec!(Teams);
exec!(Team);
exec!(TeamInvitation);
exec!(TeamInvitationId);
exec!(TeamMembers);
exec!(TeamMemberEmail);
exec!(TeamMemberId);
exec!(TeamIdentityProviders);
exec!(TeamIdentityProviderId);
exec!(TeamApps);
exec!(TeamApp);
exec!(TeamAppCollaborators);
exec!(TeamAppCollaboratorEmail);
exec!(WhitelistedAddonServices);
exec!(WhitelistedAddonServiceId);
exec!(WhitelistedAddonServiceName);