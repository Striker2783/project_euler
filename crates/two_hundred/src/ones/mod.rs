mod eight;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod three;
mod two;
mod zero;

use common::tens;

tens!(&[
    zero::run,
    one::run,
    two::run,
    three::run,
    four::run,
    five::run,
    six::run,
    seven::run,
    eight::run,
    nine::run
]);
