# Protocol notes

```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment

usb.dst == "1.3.0" or usb.src == "1.3.0"
```

```
preamble; 1c00204adf0482daffff000000001b000001000300000248000000002109000200004000
                                                            | random data, forgot init?                                      | checksum?
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump quiet
3f101400ff05ffffffffff00000000000000000000000000ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff59
pump balanced
3fd81400ff05ffffffffff00000000000000000000000001ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff4d
pump extreme
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff43
coolant at 35.70 ish

fan1 balanced
3f581400ff05ffffffffff00000000000000000000000002ffff0000ff071c331e4f1f69208722ad23d424ff1e33204f2169238725ad29d42affffffffffff3a

fan1 extreme
3fa81400ff05ffffffffff00000000000000000000000002ffff0000ff071a591b6e1c871da31ebd1fdb20ff1e33204f2169238725ad29d42afffffffffffff2

fan1 quiet
3fc81400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42affffffffffff40

fan2 balanced
3f301400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1c331e4f1f69208722ad23d424fffffffffffff8

fan2 extreme
3fa01400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1a591b6e1c871da31ebd1fdb20ffffffffffffcf

fan2 quiet
3f081400ff05ffffffffff00000000000000000000000002ffff0000ff071e33204f2169238725ad29d42aff1e33204f2169238725ad29d42afffffffffffff7
```

Pump settings & fan settings start with `0x3fxx14`

Requests come back from another endpoint?
```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.1\") or (usb.src==\"1.3.1\")" -T fields -e frame.time -e usb.src -e usb.dst -e usb.capdata
ff:a0:12:08:00:17:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:81:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:44
ff:a8:12:08:00:18:13:92:24:00:00:a5:e8:03:a5:de:04:00:a5:e8:03:a5:ee:04:02:ff:00:00:ff:7f:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:a7:8b:66:00:05:2d:32:37:41:80:24:00:00:00:00:00:00:00:b2

tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment -e usb.capdata
```

Requests and responses in one:
```
tshark -r 2024_09_21_2343_capture_fan_and_pump_profile.pcapng -Y "(usb.dst == \"1.3.0\") or (usb.src==\"1.3.0\") or (usb.dst == \"1.3.1\") or (usb.src == \"1.3.1\") " -T fields -e frame.time -e usb.src -e usb.dst -e usb.data_fragment
```

Requesting data (fan speed, temperature):

```
Sep 21, 2024 19:42:56.154897000 EDT	host	1.3.0	3f:c0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:35	
Sep 21, 2024 19:42:56.155178000 EDT	1.3.0	host		
Sep 21, 2024 19:42:56.159726000 EDT	1.3.1	host		ff:88:12:08:00:9f:15:7d:23:00:00:90:e8:03:90:40:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:8b:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:40:e3:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d3
Sep 21, 2024 19:42:56.159755000 EDT	host	1.3.1		
Sep 21, 2024 19:42:57.669453000 EDT	host	1.3.0	3f:c8:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:da	
Sep 21, 2024 19:42:57.669702000 EDT	1.3.0	host		
Sep 21, 2024 19:42:57.675657000 EDT	1.3.1	host		ff:90:12:08:00:a0:15:7a:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:81:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:36:e9:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:d4
Sep 21, 2024 19:42:57.675664000 EDT	host	1.3.1		
Sep 21, 2024 19:42:59.199969000 EDT	host	1.3.0	3f:d0:ff:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:ec	
Sep 21, 2024 19:42:59.200252000 EDT	1.3.0	host		
Sep 21, 2024 19:42:59.203696000 EDT	1.3.1	host		ff:98:12:08:00:a1:15:77:23:00:00:90:e8:03:90:41:04:00:90:e8:03:90:52:04:02:ff:00:00:ff:79:0b:00:00:00:00:00:00:00:00:45:32:33:45:00:00:3c:ef:6f:00:05:2d:32:37:41:6a:23:00:00:00:00:00:00:00:81
```

Status maybe `0x3fxxff`


second byte is sequence number, but it always increments by 8?

Reply is in URB_INTERRUPT

## Checksum

```
./reveng -s -w 8 c0ff00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035    c8ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000da  b8ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000db 40ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ef 
width=8  poly=0x07  init=0x00  refin=false  refout=false  xorout=0x00  check=0xf4  residue=0x00  name="CRC-8"
```

On outgoing requests, first byte is missing from crc calculation.
Same on incoming data;
```
./reveng -s -w 8 881208009f157d23000090e8039040040090e80390520402ff0000ff8b0b000000000000000045323345000040e36f00052d3237416a2300000000000000d3 90120800a0157a23000090e8039041040090e80390520402ff0000ff810b000000000000000045323345000036e96f00052d3237416a2300000000000000d4
width=8  poly=0x07  init=0x00  refin=false  refout=false  xorout=0x00  check=0xf4  residue=0x00  name="CRC-8"
```

```




#pragma pattern_limit 200000

struct Values {
    u8 always_12;
    u8 always_08;
    u8 pad1[1];
    u16 msg_counter;
    u16 first_value;
    u8 pad2[2];
    u8 pad3;
    
    // these three bytes seem to alternate? what's the deal?
    u8 pair_val1;
    u8 pair_val2;
    u8 pad4_duty_perhaps;
    
    u16 more_value;
    
    u8 buf[46];
    u8 crc;
};

struct Msg {
    u8 cmd;
    u8 seq;
    if (cmd == 0xff) {
      Values v;
    } else {
     u8 zzz[62];
    }
};


Msg v[1000] @ 0;

```
