//!Update an existing team and it's properties.
imports!();
use crate::client::PatchQueryBuilder;

new_type!(
    Teams
    Team
    IdentityProviders
    IdentityProviderId
    TeamApps
    TeamApp
    TeamAppCollaborators
    TeamAppCollaboratorEmail
    TeamMembers
    TeamPreferences
    TeamPreferenceId
    TeamPreferenceName

);

from!(
    @PatchQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
        => TeamPreferenceId
        => TeamPreferenceName
    @Teams
        -> TeamApps = "apps"
    @Team
        -> IdentityProviders = "identity-providers"
        -> TeamMembers = "members"
    @IdentityProviders
        => IdentityProviderId  
    @TeamApps
        => TeamApp
    @TeamApp
        -> TeamAppCollaborators = "collaborators"
    @TeamAppCollaborators
        => TeamAppCollaboratorEmail
    @TeamPreferenceId
        -> TeamPreferences = "preferences"
    @TeamPreferenceName
        -> TeamPreferences = "preferences"
);

impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
        |=> team_preference_id -> TeamPreferenceId = team_preference_id_str
        |=> team_preference_name -> TeamPreferenceName = team_preference_name_str
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
        |=> team_identity_provider ->  IdentityProviders
        |=> team_members ->  TeamMembers
        |
    @IdentityProviders
        |
        |=> identity_provider_id -> IdentityProviderId = id
    @TeamPreferenceId
        |=> preferences -> TeamPreferences
        |
    @TeamPreferenceName
        |=> preferences -> TeamPreferences
        |
);

exec!(Teams);
exec!(Team);
exec!(IdentityProviders);
exec!(IdentityProviderId);
exec!(TeamApps);
exec!(TeamApp);
exec!(TeamAppCollaborators);
exec!(TeamAppCollaboratorEmail);
exec!(TeamMembers);
exec!(TeamPreferences);
exec!(TeamPreferenceName);
exec!(TeamPreferenceId);