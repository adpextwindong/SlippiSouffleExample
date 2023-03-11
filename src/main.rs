use std::{fs, io};

use peppi::model::frame::{Frame, PortData};
use peppi::model::game::Frames::*;

fn main() {
    let mut buf = io::BufReader::new(
        fs::File::open("singleUpsmash.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();
    //println!("{:#?}", game);

    let frames = game.frames;

    match frames {
        P2(vs) => analyse(vs),
        _ => println!{"We only analyse 1vs1s buddy"},
    }
}

fn analyse(frames : Vec<Frame<2>>) {
    //TODO dump these to their own csv's
    for f in &frames {
        let port1 = &f.ports[0];
        let port2 = &f.ports[1];

        preStateFrameInfo(1, port1, f.index);
        preStateFrameInfo(2, port2, f.index);

    }

    println!("----------------");
    for f in &frames {
        let port1 = &f.ports[0];
        let port2 = &f.ports[1];

        postStateFrameInfo(1, port1, f.index);
        postStateFrameInfo(2, port2, f.index);
    }

}
fn preStateFrameInfo(playerNumber : i32, port : &PortData, frameIndex : i32){
    println!("{:?}\t{:?}\t{:?}",
             playerNumber,
             frameIndex,
             port.leader.pre.state);
}

fn postStateFrameInfo(playerNumber : i32, port : &PortData, frameIndex : i32){
    println!("{:?}\t{:?}\t{:?}",
             playerNumber,
             frameIndex,
             port.leader.post.state);
}
