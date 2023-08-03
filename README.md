# Rtmp Server
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FSir-Thom%2FrtmpserverMagic-eye.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FSir-Thom%2FrtmpserverMagic-eye?ref=badge_shield)


## Description

this is the rtmp server for the live streaming
you send a rtmp stream use something like

```bash
gst-launch-1.0 videotestsrc ! videoconvert ! videoscale ! video/x-raw,width=800,height=800 ! x264enc tune=zerolatency ! flvmux ! rtmpsink location=rtmp://{your ip}:{port}/live/stream
```

## API

## Create a new stream

```plain text
http://{ip}:3030/rtmp/rtmp_servers/create_rtmp_server/{number_of_streams}
```

## get all streams

```plain text
http://{ip}:3030/rtmp/rtmp_servers/
```

## get a specific stream

```plain text
http://{ip}:3030/rtmp/rtmp_servers/{stream_id}
```


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FSir-Thom%2FrtmpserverMagic-eye.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FSir-Thom%2FrtmpserverMagic-eye?ref=badge_large)