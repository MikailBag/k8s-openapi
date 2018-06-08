// Generated from definition io.k8s.kubernetes.pkg.apis.batch.v2alpha1.JobTemplateSpec

/// JobTemplateSpec describes the data a Job should have when created from a template
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::v1_7::kubernetes::pkg::apis::batch::v1::JobSpec>,
}