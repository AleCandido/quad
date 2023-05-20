use crate::qag_1dvec_integration_result::*;
use crate::result_state::*;
use crate::qage_1dvec::*;

#[derive(Clone,Debug)]
pub struct Qag1DVecIntegratorResult {
    pub result_state : ResultState,
    pub integration_result : Qag1DVecIntegrationResult,
}



impl Qag1DVecIntegratorResult {
    pub fn new(result : f64, abserr : f64, neval : i32, list : Vec<Result>,
               last : usize) -> Self {
        Self{
            result_state : ResultState::Success,
            integration_result : Qag1DVecIntegrationResult::new(result, abserr, neval, list, last ),
        }
    }
    pub fn new_error(result_state : ResultState) -> Self{
        Self{
            result_state,
            integration_result : Qag1DVecIntegrationResult::new_error(),
        }
    }


    pub fn unwrap(&self) -> Qag1DVecIntegrationResult {
        match self.result_state{
            ResultState::Success => self.integration_result.clone(),
            ResultState::Failure => panic!("Generic Fail"),
            ResultState::MaxIteration => panic!("{}", MAX_ITERATION_ERROR_MESSAGE),
            ResultState::BadTolerance => panic!("{}", BAD_TOLERANCE_ERROR_MESSAGE),
            ResultState::Invalid => panic!("{}", INVALID_ERROR_MESSAGE),
            ResultState::BadFunction => panic!("{}", BAD_FUNCTION_ERROR_MESSAGE),
            ResultState::Diverge => panic!("{}", DIVERGE_ERROR_MESSAGE),

        }
    }
}