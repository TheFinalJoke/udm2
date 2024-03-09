use tonic::Response;
use regex::Regex;
use regex::Error::Syntax;
use crate::error::UdmError;
use crate::UdmResult;

tonic::include_proto!("service_types");

pub trait ServiceRequest {}
pub trait ServiceResponse: Clone {
    fn to_response(&self) -> Response<Self>
    where
        Self: Sized,
    {
        Response::new(self.clone())
    }
}

impl ServiceRequest for AddFluidRegulatorRequest {}
impl ServiceRequest for ModifyFluidRegulatorRequest {}
impl ServiceRequest for RemoveFluidRegulatorRequest {}
impl ServiceRequest for CollectFluidRegulatorsRequest {}
impl ServiceRequest for AddRecipeRequest {}
impl ServiceRequest for GetRecipeRequest {}
impl ServiceRequest for ModifyRecipeRequest {}
impl ServiceRequest for RemoveRecipeRequest {}
impl ServiceRequest for AddInstructionRequest {}
impl ServiceRequest for GetInstructionRequest {}
impl ServiceRequest for ModifyInstructionRequest {}
impl ServiceRequest for RemoveInstructionRequest {}
impl ServiceRequest for AddIngredientRequest {}
impl ServiceRequest for RemoveIngredientRequest {}
impl ServiceRequest for GetIngredientRequest {}
impl ServiceRequest for ModifyIngredientRequest {}
impl ServiceRequest for ResetRequest {}

impl ServiceResponse for AddFluidRegulatorResponse {}
impl ServiceResponse for ModifyFluidRegulatorResponse {}
impl ServiceResponse for CollectFluidRegulatorsResponse {}
impl ServiceResponse for AddRecipeResponse {}
impl ServiceResponse for GetRecipeResponse {}
impl ServiceResponse for ModifyRecipeResponse {}
impl ServiceResponse for AddInstructionResponse {}
impl ServiceResponse for GetInstructionResponse {}
impl ServiceResponse for ModifyInstructionResponse {}
impl ServiceResponse for AddIngredientResponse {}
impl ServiceResponse for GetIngredientResponse {}
impl ServiceResponse for ModifyIngredientResponse {}
impl ServiceResponse for ResetResponse {}
impl ServiceResponse for GenericRemovalResponse {}

impl FetchData {
    pub fn to_fetch_data(user_input: &str) -> UdmResult<FetchData> {
        let capture_regex: &str = r"(?P<field>[a-z_\s]+)(?P<operation>=|!=|in|!in|<|<=|>=|>|like|!like|is|!is)(?P<value>[\sa-zA-Z_\d]+)";
        let reg = Regex::new(capture_regex)?;
        let captures = reg.captures(user_input).ok_or_else(|| UdmError::ParsingError(Syntax("Error Parsing the input".to_string())))?;
        let operation = Operation::to_operation(captures.name("operation").unwrap().as_str()).ok_or_else(|| UdmError::ParsingError(Syntax("Error Parsing the input".to_string())))?;
        Ok(
            FetchData { 
                column: captures.name("field").unwrap().as_str().to_string(), 
                operation: operation.into(),
                values: vec![captures.name("value").unwrap().as_str().to_string()], 
            }
        )
    }
}

impl Operation {
    pub fn to_operation(user_input: &str) -> Option<Operation> {
        match user_input {
            "=" => Some(Operation::Equal),
            "!=" => Some(Operation::NotEqual),
            "in" => Some(Operation::In),
            "!in" => Some(Operation::NotIn),
            "<" => Some(Operation::LessThan),
            "<=" => Some(Operation::LessThanOrEqual),
            ">=" => Some(Operation::GreaterThanOrEqual),
            ">" => Some(Operation::GreaterThan),
            "like" => Some(Operation::Like),
            "!like" => Some(Operation::NotLike),
            "is" => Some(Operation::Is),
            "!is" => Some(Operation::NotIs),
            _ => None
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Operation::Unspecified => "",
            Operation::Equal => "",
            Operation::NotEqual => "!=",
            Operation::In => "IN",
            Operation::NotIn => "NOT IN",
            Operation::GreaterThan => "<",
            Operation::GreaterThanOrEqual => "<=",
            Operation::LessThanOrEqual => ">=",
            Operation::LessThan => ">",
            Operation::Like => "LIKE",
            Operation::NotLike => "NOT LIKE",
            Operation::Is => "IS",
            Operation::NotIs => "NOT IS",
        }
    }
}