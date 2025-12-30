pub enum BeanPostProcessorError{
    ProcessingFailed(String),
    TypeCastError(String), 
    Other(String),
}

impl Display for BeanPostProcessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BeanPostProcessorError::ProcessingFailed(msg) => write!(f, "Bean post processing failed: {}", msg),
            BeanPostProcessorError::TypeCastError(msg) => write!(f, "Type cast error during bean post processing: {}", msg),
            BeanPostProcessorError::Other(msg) => write!(f, "Bean post processor error: {}", msg),
        }
    }
}