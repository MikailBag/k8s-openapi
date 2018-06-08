// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup

/// APIGroup contains the name, the supported versions, and the preferred version of a group.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIGroup {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// name is the name of the group.
    pub name: String,

    /// preferredVersion is the version preferred by the API server, which probably is the storage version.
    #[serde(rename = "preferredVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_version: Option<::v1_10::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,

    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    #[serde(rename = "serverAddressByClientCIDRs")]
    pub server_address_by_client_cid_rs: Vec<::v1_10::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR>,

    /// versions are the versions supported in this group.
    pub versions: Vec<::v1_10::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery>,
}