use serde::{Deserialize, Serialize};

use crate::openscenario::common::{Double, File, ParameterAssignment, Range, UnsignedInt};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProbabilityDistributionSetElement {
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProbabilityDistributionSet {
    #[serde(rename = "Element", skip_serializing_if = "Option::is_none")]
    pub element: Option<Vec<ProbabilityDistributionSetElement>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UniformDistribution {
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PoissonDistribution {
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<Range>>,
    #[serde(rename = "@expectedValue")]
    pub expected_value: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct None {}

#[derive(Deserialize, Serialize, Debug)]
pub struct NormalDistribution {
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<Range>>,
    #[serde(rename = "@expectedValue")]
    pub expected_value: Double,
    #[serde(rename = "@variance")]
    pub variance: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LogNormalDistribution {
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Vec<Range>>,
    #[serde(rename = "@expectedValue")]
    pub expected_value: Double,
    #[serde(rename = "@variance")]
    pub variance: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HistogramBin {
    #[serde(rename = "Range", skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Histogram {
    #[serde(rename = "Bin", skip_serializing_if = "Option::is_none")]
    pub bin: Option<Vec<HistogramBin>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StochasticDistributionType {
    #[serde(
        rename = "ProbabilityDistributionSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub probability_distribution_set: Option<ProbabilityDistributionSet>,
    #[serde(rename = "NormalDistribution", skip_serializing_if = "Option::is_none")]
    pub normal_distribution: Option<NormalDistribution>,
    #[serde(
        rename = "LogNormalDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_normal_distribution: Option<LogNormalDistribution>,
    #[serde(
        rename = "UniformDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub uniform_distribution: Option<UniformDistribution>,
    #[serde(
        rename = "PoissonDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub poisson_distribution: Option<PoissonDistribution>,
    #[serde(rename = "Histogram", skip_serializing_if = "Option::is_none")]
    pub histogram: Option<Histogram>,
    #[serde(
        rename = "UserDefinedDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_distribution: Option<UserDefinedDistribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterValueSet {
    #[serde(
        rename = "ParameterAssignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignment: Option<Vec<ParameterAssignment>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StochasticDistribution {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DistributionSetElement {
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DistributionSet {
    #[serde(rename = "Element", skip_serializing_if = "Option::is_none")]
    pub element: Option<Vec<DistributionSetElement>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DistributionRange {
    #[serde(rename = "Range")]
    pub range: Range,
    #[serde(rename = "@stepWidth")]
    pub step_width: Double,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValueSetDistribution {
    #[serde(rename = "ParameterValueSet", skip_serializing_if = "Option::is_none")]
    pub parameter_value_set: Option<Vec<ParameterValueSet>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ParameterValueDistribution {
    #[serde(rename = "ScenarioFile", skip_serializing_if = "Option::is_none")]
    pub scenario_file: Option<File>,

    // DistributionDefinition  one of this
    #[serde(rename = "Deterministic", skip_serializing_if = "Option::is_none")]
    pub deterministic: Option<Deterministic>,
    #[serde(rename = "Stochastic", skip_serializing_if = "Option::is_none")]
    pub stochastic: Option<Stochastic>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Deterministic {
    #[serde(
        rename = "DeterministicParameterDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_parameter_distribution: Option<DeterministicParameterDistribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeterministicParameterDistribution {
    #[serde(
        rename = "DeterministicMultiParameterDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_multi_parameter_distribution: Option<DeterministicMultiParameterDistribution>,
    #[serde(
        rename = "DeterministicSingleParameterDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_single_parameter_distribution:
    Option<DeterministicSingleParameterDistribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeterministicMultiParameterDistribution {
    #[serde(
        rename = "DeterministicMultiParameterDistributionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_multi_parameter_distribution_type:
    Option<DeterministicMultiParameterDistributionType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeterministicMultiParameterDistributionType {
    #[serde(
        rename = "ValueSetDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub value_set_distribution: Option<ValueSetDistribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeterministicSingleParameterDistribution {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,

    #[serde(
        rename = "DeterministicSingleParameterDistributionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_single_parameter_distribution_type:
    Option<DeterministicSingleParameterDistributionType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeterministicSingleParameterDistributionType {
    #[serde(rename = "DistributionSet", skip_serializing_if = "Option::is_none")]
    pub distribution_set: Option<DistributionSet>,
    #[serde(rename = "DistributionRange", skip_serializing_if = "Option::is_none")]
    pub distribution_range: Option<DistributionRange>,
    #[serde(
        rename = "UserDefinedDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_distribution: Option<UserDefinedDistribution>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserDefinedDistribution {
    #[serde(rename = "@type")]
    pub r#type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stochastic {
    #[serde(
        rename = "StochasticDistribution",
        skip_serializing_if = "Option::is_none"
    )]
    pub stochastic_distribution: Option<Vec<StochasticDistribution>>,
    #[serde(rename = "@numberOfTestRuns")]
    pub number_of_test_runs: UnsignedInt,
    #[serde(rename = "@randomSeed", skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<Double>,
}
