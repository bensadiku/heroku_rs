 //!Update an existing team and it's properties.
imports!();
use crate::client::PatchQueryBuilder;

new_type!(
    Teams
    Team
    IdentityProviders
    IdentityProviderId
);

from!(
    @PatchQueryBuilder
        -> Teams = "teams"
    @Teams
        => Team
    @Team
        -> IdentityProviders = "identity-providers"
    @IdentityProviders
        => IdentityProviderId  
);

impl_macro!(
    @Teams
        |
        |=> team_name -> Team = team_name_str
        |=> team_id -> Team = team_id_str
    @Team
        |=> team_identity_provider ->  IdentityProviders
        |
    @IdentityProviders
        |
        |=> identity_provider_id -> IdentityProviderId = id
);

exec!(Teams);
exec!(Team);
exec!(IdentityProviders);
exec!(IdentityProviderId);