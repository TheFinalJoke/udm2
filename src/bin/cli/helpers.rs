use lib::UdmResult;

pub trait UdmGrpcActions {
    fn sanatize_input(&self) -> UdmResult<()>;
}

pub trait MainCommandHandler<T> {
    fn handle_command(&self) -> T;
}