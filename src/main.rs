use std::io;

use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use rtmp::channels::ChannelsManager;
use rtmp::rtmp::RtmpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    gst::init()?;

    // Your RTMP server configuration
    let rtmp_server_ip = "127.0.0.1";
    let rtmp_server_port = 1936;
    let rtmp_server_app = "live";
    let rtmp_server_stream = "stream";

    let mut channel = ChannelsManager::new(None);
    let producer = channel.get_channel_event_producer();

    let listen_port = 1936;
    let address = format!("127.0.0.1:{port}", port = listen_port);

    let mut rtmp_server = RtmpServer::new(address, producer);
    tokio::spawn(async move {
        if let Err(err) = rtmp_server.run().await {
            log::error!("rtmp server error: {}\n", err);
        }
    });

    tokio::spawn(async move { channel.run().await });

    // Create GStreamer pipeline with video resizing
    let pipeline_str = format!(
        "videotestsrc ! videoconvert ! queue ! videoscale ! video/x-raw,width=800,height=800 ! queue ! x264enc tune=zerolatency ! flvmux ! rtmpsink location=rtmp://{}:{}/{}/{}",
        rtmp_server_ip, rtmp_server_port, rtmp_server_app, rtmp_server_stream
    );
    let pipeline = gst::parse_launch(&pipeline_str)?;

    // Start GStreamer pipeline
    pipeline.set_state(gst::State::Playing)?;
    println!(
        "stream at rtmp://{}:{}/{}/{}",
        rtmp_server_ip, rtmp_server_port, rtmp_server_app, rtmp_server_stream
    );

    println!("Press 'q' and Enter to stop the pipeline.");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            break;
        }
    }

    // Stop GStreamer pipeline
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}
