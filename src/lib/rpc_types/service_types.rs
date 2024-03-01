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

impl ServiceRequest for AddFluidRegulatorRequest {}
impl ServiceRequest for ModifyFluidRegulatorRequest {}
impl ServiceRequest for RemoveFluidRegulatorRequest {}
impl ServiceRequest for AddReceipeRequest {}
impl ServiceRequest for GetRecipeRequest {}
impl ServiceRequest for ModifyRecipeRequest {}
impl ServiceRequest for AddInstructionRequest {}
impl ServiceRequest for GetInstructionRequest {}
impl ServiceRequest for ModifyInstructionRequest {}
impl ServiceRequest for AddIngredientRequest {}
impl ServiceRequest for RemoveIngredientRequest {}
impl ServiceRequest for GetIngredientRequest {}
impl ServiceRequest for ResetRequest {}

impl ServiceResponse for AddFluidRegulatorResponse {}
impl ServiceResponse for ModifyFluidRegulatorResponse {}
impl ServiceResponse for RemoveFluidRegulatorResponse {}
impl ServiceResponse for AddReceipeResponse {}
impl ServiceResponse for GetRecipeResponse {}
impl ServiceResponse for ModifyRecipeResponse {}
impl ServiceResponse for AddInstructionResponse {}
impl ServiceResponse for GetInstructionResponse {}
impl ServiceResponse for ModifyInstructionResponse {}
impl ServiceResponse for AddIngredientResponse {}
impl ServiceResponse for RemoveIngredientResponse {}
impl ServiceResponse for GetIngredientResponse {}
impl ServiceResponse for ResetResponse {}
