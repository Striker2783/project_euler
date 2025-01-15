use common::tens;

pub mod eight;
pub mod five;
pub mod four;
pub mod nine;
pub mod one;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

tens!(&[
    blank,
    one::run,
    two::run,
    three::run,
    four::four,
    five::five,
    six::six,
    seven::seven,
    eight::eight,
    nine::nine,
]);
fn blank() {}
