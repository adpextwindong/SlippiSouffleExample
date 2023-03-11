use std::{fs, io};
use std::fs::File;
use std::io::Write;

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
    let mut preFactsFile = File::create("prestate.facts").unwrap();

    //TODO dump these to their own csv's
    for f in &frames {
        let port1 = &f.ports[0];
        let port2 = &f.ports[1];

        preStateFrameInfo(&mut preFactsFile, 1, port1, f.index);
        preStateFrameInfo(&mut preFactsFile, 2, port2, f.index);
    }

    let mut postFactsFile = File::create("poststate.facts").unwrap();
    for f in &frames {
        let port1 = &f.ports[0];
        let port2 = &f.ports[1];

        postStateFrameInfo(&mut postFactsFile, 1, port1, f.index);
        postStateFrameInfo(&mut postFactsFile, 2, port2, f.index);
    }

}
fn preStateFrameInfo(w : &mut File, playerNumber : i32, port : &PortData, frameIndex : i32){
    writeln!(w, "{:?}\t{:?}\t{:?}",
             playerNumber,
             frameIndex,
             port.leader.pre.state);
}

fn postStateFrameInfo(w: &mut File, playerNumber : i32, port : &PortData, frameIndex : i32){
    writeln!(w, "{:?}\t{:?}\t{:?}",
             playerNumber,
             frameIndex,
             port.leader.post.state);
}
