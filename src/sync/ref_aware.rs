use actix::prelude::*;

pub trait RefAware: Message<Result = ()> + Send + 'static {
    fn get_ref(&self) -> u32;
}
