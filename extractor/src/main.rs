use std::{fs, env};
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::message::ViewAngles;
use tf_demo_parser::demo::parser::{DemoHandler, RawPacketStream};
use tf_demo_parser::demo::packet::Packet;
use tf_demo_parser::{Demo, DemoParser};
use bitbuffer::BitRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Need instruction (header|full) and demo file path {}", args.len());
        return;
    }

    if let Ok(demo_data) = fs::read(&args[2]){
        let demo = Demo::new(&demo_data);

        match args[1].as_str() {
            "header" => header(&demo),
            "full" => dump_infos(&demo),
            "summary" => summary(&demo),
            _ => println!("Invalid instruction (header|full)")
        }
    }else{
        println!("Could not open demo file");
    }
}

fn header(demo: &Demo){
    let mut stream = demo.get_stream();
    let header = Header::read(&mut stream).unwrap();
    println!("{}", serde_json::to_string_pretty(&header).unwrap());
}

fn dump_infos(demo: &Demo) {
    let mut stream = demo.get_stream();
    let mut handler = DemoHandler::default();
    handler.handle_header(&Header::read(&mut stream).unwrap());

    let mut packets = RawPacketStream::new(stream.clone());
    
    let mut usercmds: Vec<(DemoTick, [ViewAngles; 2])> = Vec::new();

    while let Some(packet) = packets.next(&handler.state_handler).unwrap() {
        match &packet {
            Packet::Message(c) => {
                usercmds.push((c.tick, c.meta.view_angles.clone()));
                //println!("{:#?}", c.meta.view_angles);
                for m in &c.messages {
                    if let Message::PacketEntities(e) = &m {
                        println!("{:#?}", e);
                    }
                }
            },
            _ => () //println!("{:#?}", packet)
        };
        
        handler.handle_packet(packet).unwrap();
    }
    
    println!("{}", serde_json::to_string_pretty(&usercmds).unwrap());
}

fn summary(demo: &Demo){
    let parser = DemoParser::new(demo.get_stream());

    let (header, state) = parser.parse().unwrap();
    println!("{}",serde_json::to_string_pretty(&state).unwrap());
}