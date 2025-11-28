mod zero;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;

use common::tens;

tens!(&[zero::run, one::run, two::run, three::run, four::run, five::run, six::run, seven::run]);