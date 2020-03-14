//! Access the Teams portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under Teams
new_type!(
   Teams
   Team
   Addons
   IdentityProviders
   Permissions
   PipelineCouplings
   TeamApps
   TeamApp
   TeamAppCollaborators
   TeamAppCollaboratorEmail
   TeamFeatures
   TeamFeature
   TeamInvitations
   TeamInvitationToken
   TeamInvoices
   TeamInvoiceNumber
   TeamMembers
   TeamMemberId
   TeamMemberEmail
   TeamPreferences
   TeamPreferenceId
   TeamPreferenceName
   TeamSpaces
   WhitelistedAddonServices
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
        => TeamPreferenceId
        => TeamPreferenceName
    @Teams 
        -> TeamApps = "apps"
        -> TeamInvitations = "invitations"
    @Team
        -> Addons = "addons"
        -> TeamApps = "apps"
        -> IdentityProviders = "identity-providers"
        -> Permissions = "permissions"
        -> PipelineCouplings = "pipeline-couplings"
        -> TeamFeatures = "features"
        -> TeamInvitations = "invitations"
        -> TeamInvoices = "invoices"
        -> TeamMembers = "members"
        -> TeamSpaces = "spaces"
        -> WhitelistedAddonServices = "whitelisted-addon-services"
    @TeamApps
        => TeamApp
    @TeamApp
        -> TeamAppCollaborators = "collaborators"
    @TeamAppCollaborators
        => TeamAppCollaboratorEmail
    @TeamFeatures
        => TeamFeature
    @TeamInvitations
        => TeamInvitationToken
    @TeamInvoices 
        => TeamInvoiceNumber
    @TeamMembers
        => TeamMemberId
        => TeamMemberEmail
    @TeamMemberId
        -> TeamApps = "apps"
    @TeamMemberEmail
        -> TeamApps = "apps"
    @TeamPreferenceId
        -> TeamPreferences = "preferences"
    @TeamPreferenceName
        -> TeamPreferences = "preferences"
);

// impls of each type
impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
        |=> team_preference_id -> TeamPreferenceId = team_preference_id_str
        |=> team_preference_name -> TeamPreferenceName = team_preference_name_str
    @Teams
        |=> team_apps -> TeamApps
        |=> team_invitations -> TeamInvitations
        |
    @Team
        |=> team_addons -> Addons
        |=> team_apps -> TeamApps
        |=> team_identity_providers -> IdentityProviders
        |=> team_permissions -> Permissions
        |=> team_pipeline_couplings -> PipelineCouplings
        |=> team_features -> TeamFeatures
        |=> team_invitations -> TeamInvitations
        |=> team_invoices -> TeamInvoices
        |=> team_members -> TeamMembers
        |=> team_spaces -> TeamSpaces
        |=> team_whitelisted_addon_services -> WhitelistedAddonServices
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
    @TeamFeatures
        |
        |=> feature_name -> TeamFeature = feature_name_str
        |=> feature_id -> TeamFeature = feature_id_str
    @TeamInvitations
        |
        |=> invitation_token -> TeamInvitationToken = token
    @TeamInvoices
        |
        |=> invoice_number -> TeamInvoiceNumber = invoice_number_str
    @TeamMembers
        |
        |=> member_email -> TeamMemberEmail =  member_email_str
        |=> member_id -> TeamMemberId =  member_id_str
    @TeamMemberEmail
        |=> team_apps -> TeamApps
        |
    @TeamMemberId
        |=> team_apps -> TeamApps
        |
    @TeamPreferenceId
        |=> preferences -> TeamPreferences
        |
    @TeamPreferenceName
        |=> preferences -> TeamPreferences
        |
);

exec!(Teams);
exec!(Team);
exec!(Addons);
exec!(IdentityProviders);
exec!(Permissions);
exec!(TeamApps);
exec!(TeamApp);
exec!(PipelineCouplings);
exec!(TeamAppCollaborators);
exec!(TeamAppCollaboratorEmail);
exec!(TeamFeatures);
exec!(TeamFeature);
exec!(TeamInvitations);
exec!(TeamInvitationToken);
exec!(TeamInvoices);
exec!(TeamInvoiceNumber);
exec!(TeamMembers);
exec!(TeamMemberId);
exec!(TeamMemberEmail);
exec!(TeamPreferences);
exec!(TeamPreferenceName);
exec!(TeamPreferenceId);
exec!(TeamSpaces);
exec!(WhitelistedAddonServices);