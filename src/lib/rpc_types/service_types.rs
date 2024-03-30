use crate::db::FluidRegulationSchema;
use crate::db::InstructionSchema;
use crate::error::UdmError;
use crate::UdmResult;
use log::debug;
use regex::Regex;
use sea_query::Expr;
use sea_query::SimpleExpr;
use tonic::Response;
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

pub trait CollectExpressions {
    fn get_expressions(&self) -> UdmResult<Vec<SimpleExpr>>;
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
impl ServiceRequest for CollectInstructionRequest {}

impl ServiceResponse for AddFluidRegulatorResponse {}
impl ServiceResponse for ModifyFluidRegulatorResponse {}
impl ServiceResponse for CollectFluidRegulatorsResponse {}
impl ServiceResponse for AddRecipeResponse {}
impl ServiceResponse for GetRecipeResponse {}
impl ServiceResponse for ModifyRecipeResponse {}
impl ServiceResponse for AddInstructionResponse {}
impl ServiceResponse for GetInstructionResponse {}
impl ServiceResponse for CollectInstructionResponse {}
impl ServiceResponse for ModifyInstructionResponse {}
impl ServiceResponse for AddIngredientResponse {}
impl ServiceResponse for GetIngredientResponse {}
impl ServiceResponse for ModifyIngredientResponse {}
impl ServiceResponse for ResetResponse {}
impl ServiceResponse for GenericRemovalResponse {}

impl FetchData {
    pub fn to_fetch_data_vec(user_input: &str) -> UdmResult<Vec<FetchData>> {
        let capture_regex: &str = r"(?P<field>[a-z_\s]+)(?P<operation>=|!=|in|!in|<|<=|>=|>|like|!like|is|!is)(?P<value>[a-zA-Z_\d\\s]+)(?:,|$)";
        let reg = Regex::new(capture_regex)?;
        let mut fetch_vec = Vec::new();
        for captures in reg.captures_iter(user_input) {
            fetch_vec.push(FetchData {
                column: captures["field"].to_owned(),
                operation: Operation::to_operation(&captures["operation"])
                    .unwrap()
                    .into(),
                values: captures["value"]
                    .split(',')
                    .map(|val| val.to_string())
                    .collect(),
            })
        }
        debug!("User input data: {:?}", &fetch_vec);
        Ok(fetch_vec)
    }
    pub fn to_simple_expr<T: sea_query::Iden + 'static>(&self, column: T) -> UdmResult<SimpleExpr> {
        let vals = self.values.to_owned();
        match Operation::try_from(self.operation)
            .map_err(|_| UdmError::InvalidInput("Could not parse the operation".to_string()))?
        {
            Operation::Unspecified => {
                Err(UdmError::ApiFailure("Operation not specified".to_string()))
            }
            Operation::Equal => Ok(Expr::col(column).eq(vals)),
            Operation::NotEqual => Ok(Expr::col(column).ne(vals)),
            Operation::In => Ok(Expr::col(column).is_in(vec![vals])),
            Operation::NotIn => Ok(Expr::col(column).is_not_in(vec![vals])),
            Operation::GreaterThan => Ok(Expr::col(column).gt(vals)),
            Operation::GreaterThanOrEqual => Ok(Expr::col(column).gte(vals)),
            Operation::LessThanOrEqual => Ok(Expr::col(column).lte(vals)),
            Operation::LessThan => Ok(Expr::col(column).lt(vals)),
            Operation::Like => Ok(Expr::col(column).like(vals)),
            Operation::NotLike => Ok(Expr::col(column).not_like(vals)),
            Operation::Is => Ok(Expr::col(column).is(vals)),
            Operation::NotIs => Ok(Expr::col(column).is_not(vals)),
        }
    }
}
impl CollectExpressions for CollectFluidRegulatorsRequest {
    fn get_expressions(&self) -> UdmResult<Vec<SimpleExpr>> {
        let mut exprs = Vec::new();
        for expr in &self.expressions {
            let cloned_data = expr.column.clone();
            let col = FluidRegulationSchema::try_from(cloned_data)?;
            let simple_expr = expr.to_simple_expr(col)?;
            debug!("Got simple expr: {:?}", simple_expr);
            exprs.push(simple_expr)
        }
        Ok(exprs)
    }
}
impl CollectExpressions for CollectInstructionRequest {
    fn get_expressions(&self) -> UdmResult<Vec<SimpleExpr>> {
        let mut exprs = Vec::new();
        for expr in &self.expressions {
            let cloned_data = expr.column.clone();
            let col = InstructionSchema::try_from(cloned_data)?;
            let simple_expr = expr.to_simple_expr(col)?;
            debug!("Got simple expr: {:?}", simple_expr);
            exprs.push(simple_expr)
        }
        Ok(exprs)
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
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Operation::Unspecified => "",
            Operation::Equal => "=",
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
