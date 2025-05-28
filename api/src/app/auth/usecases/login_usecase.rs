use crate::common::usecase::UseCase;

pub struct LoginUseCase {}

pub struct LoginDTO {
    pub email: String,
}

impl UseCase<LoginDTO, ()> for LoginUseCase {
    fn execute(input: LoginDTO) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
